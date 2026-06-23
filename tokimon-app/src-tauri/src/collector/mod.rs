//! TokePet token-usage collector.
//!
//! Parses local CLI telemetry from Claude Code, Gemini CLI, and Codex CLI and
//! ingests normalized [`UsageEvent`]s into a local SQLite database. The crate is
//! deliberately scoped to *collection* only — pet state and growth logic live in
//! the consuming application, not here.
//!
//! Privacy: only token counts and identifiers are stored. Prompt, response, and
//! code content are never persisted.

mod event;
mod sources;
mod store;

pub use store::ProviderStats;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Result of polling a single source once.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollResult {
    pub inserted: i64,
    pub duplicates: i64,
    pub cursor: i64,
}

impl PollResult {
    pub fn empty() -> Self {
        Self {
            inserted: 0,
            duplicates: 0,
            cursor: 0,
        }
    }
}

/// Result of polling every configured source once.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PollSummary {
    pub claude: PollResult,
    pub gemini: PollResult,
    pub codex: PollResult,
}

impl PollSummary {
    pub fn empty() -> Self {
        Self {
            claude: PollResult::empty(),
            gemini: PollResult::empty(),
            codex: PollResult::empty(),
        }
    }

    #[cfg(test)]
    pub fn inserted(&self) -> i64 {
        self.claude.inserted + self.gemini.inserted + self.codex.inserted
    }
}

/// Token-usage source watched for local telemetry changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WatchSource {
    Claude,
    Gemini,
    Codex,
}

/// Options for event-driven collection.
#[derive(Debug, Clone, Copy)]
pub struct WatchOptions {
    /// How long to wait for related filesystem events to settle before polling.
    pub debounce: Duration,
    /// Safety poll interval in case the operating system drops a file event.
    pub backup_poll: Duration,
}

impl Default for WatchOptions {
    fn default() -> Self {
        Self {
            debounce: Duration::from_millis(300),
            backup_poll: Duration::from_secs(30),
        }
    }
}

impl WatchOptions {
    fn normalized(self) -> Self {
        Self {
            debounce: if self.debounce == Duration::from_millis(0) {
                Duration::from_millis(1)
            } else {
                self.debounce
            },
            backup_poll: if self.backup_poll == Duration::from_millis(0) {
                Duration::from_secs(30)
            } else {
                self.backup_poll
            },
        }
    }
}

/// Keeps filesystem watchers and the background debounce worker alive.
pub struct CollectorWatch {
    tx: mpsc::Sender<WatchMessage>,
    worker: Option<JoinHandle<()>>,
    _watchers: Vec<RecommendedWatcher>,
}

impl Drop for CollectorWatch {
    fn drop(&mut self) {
        let _ = self.tx.send(WatchMessage::Stop);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

enum WatchMessage {
    Source(WatchSource),
    Stop,
}

/// A point-in-time view of collected usage, independent of any pet state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectorSnapshot {
    pub today_tokens: i64,
    pub status: String,
    pub active_source_count: i64,
    pub gemini_telemetry_path: Option<String>,
    pub claude_projects_path: Option<String>,
    pub codex_sessions_path: Option<String>,
    pub claude_stats: ProviderStats,
    pub gemini_stats: ProviderStats,
    pub codex_stats: ProviderStats,
}

/// Owns the SQLite database path, configured source paths, and a status string.
/// Cloning shares the underlying status via an `Arc<Mutex<_>>`.
#[derive(Clone)]
pub struct Collector {
    db_path: PathBuf,
    status: Arc<Mutex<String>>,
    gemini_telemetry_path: Option<PathBuf>,
    claude_projects_path: Option<PathBuf>,
    codex_sessions_path: Option<PathBuf>,
}

impl Collector {
    /// Open (or create) the database at `db_path`, run migrations, and resolve
    /// source paths from the environment / well-known defaults.
    pub fn start(db_path: PathBuf) -> Result<Self, String> {
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        Self::with_sources(
            db_path,
            sources::default_gemini_telemetry_path(),
            sources::default_claude_projects_path(),
            sources::default_codex_sessions_path(),
        )
    }

    /// Open (or create) the database with explicit source paths. Useful for
    /// tests and for callers that resolve paths themselves.
    pub fn with_sources(
        db_path: PathBuf,
        gemini_telemetry_path: Option<PathBuf>,
        claude_projects_path: Option<PathBuf>,
        codex_sessions_path: Option<PathBuf>,
    ) -> Result<Self, String> {
        let handle = Self {
            db_path,
            status: Arc::new(Mutex::new("ready".to_string())),
            gemini_telemetry_path,
            claude_projects_path,
            codex_sessions_path,
        };
        handle.initialize()?;
        Ok(handle)
    }

    /// Path of the SQLite database this collector writes to. Consumers can open
    /// the same file (e.g. to read `usage_events` for their own growth logic).
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn status(&self) -> String {
        self.status
            .lock()
            .map(|status| status.clone())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    pub fn snapshot(&self) -> Result<CollectorSnapshot, String> {
        self.snapshot_for_date(&sources::today_utc_date())
    }

    pub fn snapshot_for_date(&self, date: &str) -> Result<CollectorSnapshot, String> {
        let conn = self.connection()?;
        let today_tokens = store::daily_total_tokens(&conn, date)?;
        Ok(CollectorSnapshot {
            today_tokens,
            status: self.status(),
            active_source_count: i64::from(self.gemini_telemetry_path.is_some())
                + i64::from(self.claude_projects_path.is_some())
                + i64::from(self.codex_sessions_path.is_some()),
            gemini_telemetry_path: self
                .gemini_telemetry_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string()),
            claude_projects_path: self
                .claude_projects_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string()),
            codex_sessions_path: self
                .codex_sessions_path
                .as_ref()
                .map(|path| path.to_string_lossy().to_string()),
            claude_stats: store::provider_stats(&conn, "claude", date)?,
            gemini_stats: store::provider_stats(&conn, "gemini", date)?,
            codex_stats: store::provider_stats(&conn, "openai", date)?,
        })
    }

    /// Poll every configured source once.
    pub fn poll_all_once(&self) -> Result<PollSummary, String> {
        Ok(PollSummary {
            claude: self.poll_claude_once()?,
            gemini: self.poll_gemini_once()?,
            codex: self.poll_codex_once()?,
        })
    }

    pub fn poll_gemini_once(&self) -> Result<PollResult, String> {
        let Some(path) = &self.gemini_telemetry_path else {
            self.set_status("Gemini telemetry path not configured");
            return Ok(PollResult {
                inserted: 0,
                duplicates: 0,
                cursor: 0,
            });
        };
        self.poll_gemini_path_once(path)
    }

    pub fn poll_claude_once(&self) -> Result<PollResult, String> {
        let Some(projects_path) = &self.claude_projects_path else {
            self.set_status("Claude projects path not configured");
            return Ok(PollResult {
                inserted: 0,
                duplicates: 0,
                cursor: 0,
            });
        };
        self.poll_claude_projects_once(projects_path)
    }

    pub fn poll_codex_once(&self) -> Result<PollResult, String> {
        let Some(sessions_path) = &self.codex_sessions_path else {
            self.set_status("Codex sessions path not configured");
            return Ok(PollResult {
                inserted: 0,
                duplicates: 0,
                cursor: 0,
            });
        };
        self.poll_codex_sessions_once(sessions_path)
    }

    /// Start event-driven collection. Filesystem changes are debounced, while a
    /// periodic full poll remains as a safety net for missed events.
    pub fn watch<F>(&self, options: WatchOptions, on_poll: F) -> Result<CollectorWatch, String>
    where
        F: FnMut(Result<PollSummary, String>) + Send + 'static,
    {
        let options = options.normalized();
        let (tx, rx) = mpsc::channel();
        let mut watchers = Vec::new();

        if let Some(path) = &self.gemini_telemetry_path {
            if let Some(watcher) = watch_source_path(WatchSource::Gemini, path, false, &tx)? {
                watchers.push(watcher);
            }
        }

        if let Some(path) = &self.claude_projects_path {
            if let Some(watcher) = watch_source_path(WatchSource::Claude, path, true, &tx)? {
                watchers.push(watcher);
            }
        }

        if let Some(path) = &self.codex_sessions_path {
            if let Some(watcher) = watch_source_path(WatchSource::Codex, path, true, &tx)? {
                watchers.push(watcher);
            }
        }

        let collector = self.clone();
        let worker = thread::spawn(move || run_watch_worker(collector, options, rx, on_poll));

        Ok(CollectorWatch {
            tx,
            worker: Some(worker),
            _watchers: watchers,
        })
    }

    fn poll_codex_sessions_once(&self, sessions_path: &Path) -> Result<PollResult, String> {
        let conn = self.connection()?;
        let mut inserted = 0;
        let mut duplicates = 0;
        let mut total_cursor = 0;

        let rollout_paths = match sources::codex_rollout_paths(sessions_path) {
            Ok(paths) => paths,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.set_status("Codex sessions directory not found");
                return Ok(PollResult {
                    inserted: 0,
                    duplicates: 0,
                    cursor: 0,
                });
            }
            Err(error) => return Err(error.to_string()),
        };

        for path in rollout_paths {
            let mut cursor = store::load_cursor(&conn, "codex", &path)?;
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error.to_string()),
            };
            let content_len = content.len() as i64;
            if cursor > content_len {
                cursor = 0;
            }
            // Codex rollout lines reference earlier session_meta/turn_context lines for
            // session id and model. Always parse the file from the start to recover that
            // context, then only insert events whose byte offset is past the cursor.
            for event in sources::parse_codex_rollout_events(&content, cursor) {
                if store::insert_usage_event(&conn, &event)? {
                    inserted += 1;
                } else {
                    duplicates += 1;
                }
            }
            store::save_cursor(&conn, "codex", &path, content_len)?;
            total_cursor += content_len;
        }

        self.set_status(format!(
            "Codex watcher: {inserted} new event(s), {duplicates} duplicate(s)"
        ));
        Ok(PollResult {
            inserted,
            duplicates,
            cursor: total_cursor,
        })
    }

    fn poll_claude_projects_once(&self, projects_path: &Path) -> Result<PollResult, String> {
        let conn = self.connection()?;
        let mut inserted = 0;
        let mut duplicates = 0;
        let mut total_cursor = 0;

        let transcript_paths = match sources::claude_transcript_paths(projects_path) {
            Ok(paths) => paths,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.set_status("Claude projects directory not found");
                return Ok(PollResult {
                    inserted: 0,
                    duplicates: 0,
                    cursor: 0,
                });
            }
            Err(error) => return Err(error.to_string()),
        };

        for path in transcript_paths {
            let mut cursor = store::load_cursor(&conn, "claude", &path)?;
            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
                Err(error) => return Err(error.to_string()),
            };
            let content_len = content.len() as i64;
            if cursor > content_len {
                cursor = 0;
            }
            let unread = content.get(cursor as usize..).unwrap_or_default();
            for event in sources::parse_claude_transcript_events(unread) {
                if store::insert_usage_event(&conn, &event)? {
                    inserted += 1;
                } else {
                    duplicates += 1;
                }
            }
            store::save_cursor(&conn, "claude", &path, content_len)?;
            total_cursor += content_len;
        }

        self.set_status(format!(
            "Claude watcher: {inserted} new event(s), {duplicates} duplicate(s)"
        ));
        Ok(PollResult {
            inserted,
            duplicates,
            cursor: total_cursor,
        })
    }

    fn poll_gemini_path_once(&self, path: &Path) -> Result<PollResult, String> {
        let conn = self.connection()?;
        let mut cursor = store::load_cursor(&conn, "gemini", path)?;
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                self.set_status("Gemini telemetry file not found");
                return Ok(PollResult {
                    inserted: 0,
                    duplicates: 0,
                    cursor,
                });
            }
            Err(error) => return Err(error.to_string()),
        };

        let content_len = content.len() as i64;
        if cursor > content_len {
            cursor = 0;
        }
        let unread = content.get(cursor as usize..).unwrap_or_default();
        let events = sources::parse_gemini_telemetry_events(unread);
        let mut inserted = 0;
        let mut duplicates = 0;

        for event in events {
            if store::insert_usage_event(&conn, &event)? {
                inserted += 1;
            } else {
                duplicates += 1;
            }
        }

        store::save_cursor(&conn, "gemini", path, content_len)?;
        self.set_status(format!(
            "Gemini watcher: {inserted} new event(s), {duplicates} duplicate(s)"
        ));

        Ok(PollResult {
            inserted,
            duplicates,
            cursor: content_len,
        })
    }

    fn connection(&self) -> Result<Connection, String> {
        Connection::open(&self.db_path).map_err(|error| error.to_string())
    }

    fn initialize(&self) -> Result<(), String> {
        let conn = self.connection()?;
        store::apply_migrations(&conn)
    }

    fn set_status(&self, status: impl Into<String>) {
        if let Ok(mut current) = self.status.lock() {
            *current = status.into();
        }
    }
}

fn watch_source_path(
    source: WatchSource,
    path: &Path,
    recursive_when_directory: bool,
    tx: &mpsc::Sender<WatchMessage>,
) -> Result<Option<RecommendedWatcher>, String> {
    let (watch_path, recursive_mode) = if path.is_dir() {
        (
            path.to_path_buf(),
            if recursive_when_directory {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            },
        )
    } else if let Some(parent) = path.parent().filter(|parent| parent.exists()) {
        (parent.to_path_buf(), RecursiveMode::NonRecursive)
    } else {
        return Ok(None);
    };

    let event_tx = tx.clone();
    let mut watcher = RecommendedWatcher::new(
        move |event: notify::Result<notify::Event>| match event {
            Ok(event) if is_relevant_file_event(&event) => {
                let _ = event_tx.send(WatchMessage::Source(source));
            }
            Ok(_) => {}
            Err(error) => eprintln!("tokepet collector watcher error: {error}"),
        },
        Config::default(),
    )
    .map_err(|error| error.to_string())?;

    watcher
        .watch(&watch_path, recursive_mode)
        .map_err(|error| error.to_string())?;

    Ok(Some(watcher))
}

fn is_relevant_file_event(event: &notify::Event) -> bool {
    use notify::event::{AccessKind, AccessMode, EventKind};

    match event.kind {
        EventKind::Access(AccessKind::Close(AccessMode::Write)) => true,
        EventKind::Access(_) => false,
        EventKind::Any
        | EventKind::Create(_)
        | EventKind::Modify(_)
        | EventKind::Remove(_)
        | EventKind::Other => true,
    }
}

fn run_watch_worker<F>(
    collector: Collector,
    options: WatchOptions,
    rx: mpsc::Receiver<WatchMessage>,
    mut on_poll: F,
) where
    F: FnMut(Result<PollSummary, String>),
{
    let mut batch = DebouncedSources::new(options.debounce);
    let mut next_backup = Instant::now() + options.backup_poll;

    on_poll(collector.poll_all_once());

    loop {
        let now = Instant::now();
        if let Some(sources) = batch.take_due(now) {
            on_poll(poll_sources(&collector, &sources));
        }
        if now >= next_backup {
            on_poll(collector.poll_all_once());
            next_backup = Instant::now() + options.backup_poll;
        }

        let timeout = next_timeout(batch.deadline(), next_backup);
        match rx.recv_timeout(timeout) {
            Ok(WatchMessage::Source(source)) => batch.push(source, Instant::now()),
            Ok(WatchMessage::Stop) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}

fn poll_sources(collector: &Collector, sources: &[WatchSource]) -> Result<PollSummary, String> {
    let sources: BTreeSet<_> = sources.iter().copied().collect();
    if sources.len() == 3 {
        return collector.poll_all_once();
    }

    let mut summary = PollSummary::empty();
    for source in sources {
        match source {
            WatchSource::Claude => summary.claude = collector.poll_claude_once()?,
            WatchSource::Gemini => summary.gemini = collector.poll_gemini_once()?,
            WatchSource::Codex => summary.codex = collector.poll_codex_once()?,
        }
    }
    Ok(summary)
}

fn next_timeout(debounce_deadline: Option<Instant>, next_backup: Instant) -> Duration {
    let now = Instant::now();
    let next = debounce_deadline
        .map(|deadline| deadline.min(next_backup))
        .unwrap_or(next_backup);
    next.checked_duration_since(now).unwrap_or_default()
}

struct DebouncedSources {
    debounce: Duration,
    deadline: Option<Instant>,
    pending: BTreeSet<WatchSource>,
}

impl DebouncedSources {
    fn new(debounce: Duration) -> Self {
        Self {
            debounce,
            deadline: None,
            pending: BTreeSet::new(),
        }
    }

    fn push(&mut self, source: WatchSource, now: Instant) {
        self.pending.insert(source);
        self.deadline = Some(now + self.debounce);
    }

    fn deadline(&self) -> Option<Instant> {
        self.deadline
    }

    fn take_due(&mut self, now: Instant) -> Option<Vec<WatchSource>> {
        if self.pending.is_empty()
            || match self.deadline {
                Some(deadline) => now < deadline,
                None => true,
            }
        {
            return None;
        }

        self.deadline = None;
        Some(std::mem::take(&mut self.pending).into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn api_response(prompt_id: &str, total_tokens: i64) -> String {
        serde_json::json!({
            "timestamp": "2026-05-17T14:21:00.000Z",
            "name": "gemini_cli.api_response",
            "attributes": {
                "model": "gemini-2.5-pro",
                "prompt_id": prompt_id,
                "input_token_count": 1000,
                "output_token_count": 50,
                "thoughts_token_count": 500,
                "tool_token_count": 500,
                "total_token_count": total_tokens
            }
        })
        .to_string()
    }

    fn claude_assistant_message(uuid: &str, input_tokens: i64, output_tokens: i64) -> String {
        serde_json::json!({
            "type": "assistant",
            "uuid": uuid,
            "sessionId": "claude-session-1",
            "timestamp": "2026-05-17T16:30:00.000Z",
            "message": {
                "id": "msg_01tokepet",
                "model": "claude-sonnet-4-5-20250929",
                "usage": {
                    "input_tokens": input_tokens,
                    "output_tokens": output_tokens,
                    "cache_creation_input_tokens": 300,
                    "cache_read_input_tokens": 700
                }
            }
        })
        .to_string()
    }

    #[test]
    fn initializes_sqlite() {
        let dir = tempdir().unwrap();
        let collector =
            Collector::with_sources(dir.path().join("tokepet.sqlite3"), None, None, None).unwrap();

        let snapshot = collector.snapshot_for_date("2026-05-17").unwrap();

        assert_eq!(snapshot.today_tokens, 0);
        assert_eq!(snapshot.status, "ready");
        assert_eq!(snapshot.active_source_count, 0);
    }

    #[test]
    fn polls_gemini_file_persists_usage_and_cursor() {
        let dir = tempdir().unwrap();
        let telemetry = dir.path().join("telemetry.log");
        fs::write(
            &telemetry,
            format!("{}\n", api_response("prompt-1", 10_000)),
        )
        .unwrap();
        let collector = Collector::with_sources(
            dir.path().join("tokepet.sqlite3"),
            Some(telemetry.clone()),
            None,
            None,
        )
        .unwrap();

        let first = collector.poll_gemini_once().unwrap();
        let second = collector.poll_gemini_once().unwrap();
        let snapshot = collector.snapshot_for_date("2026-05-17").unwrap();

        assert_eq!(first.inserted, 1);
        assert_eq!(first.duplicates, 0);
        assert_eq!(
            first.cursor,
            fs::read_to_string(&telemetry).unwrap().len() as i64
        );
        assert_eq!(second.inserted, 0);
        assert_eq!(snapshot.today_tokens, 10_000);
        assert_eq!(snapshot.gemini_stats.events_today, 1);
        assert_eq!(
            snapshot.gemini_stats.last_event_at.as_deref(),
            Some("2026-05-17T14:21:00.000Z")
        );
    }

    #[test]
    fn poll_recovers_after_log_rotation() {
        let dir = tempdir().unwrap();
        let telemetry = dir.path().join("telemetry.log");
        fs::write(&telemetry, format!("{}\n", api_response("before", 400))).unwrap();
        let collector = Collector::with_sources(
            dir.path().join("tokepet.sqlite3"),
            Some(telemetry.clone()),
            None,
            None,
        )
        .unwrap();
        collector.poll_gemini_once().unwrap();

        fs::write(&telemetry, format!("{}\n", api_response("after", 600))).unwrap();
        let result = collector.poll_gemini_once().unwrap();
        let snapshot = collector.snapshot_for_date("2026-05-17").unwrap();

        assert_eq!(result.inserted, 1);
        assert_eq!(snapshot.today_tokens, 1_000);
    }

    #[test]
    fn polls_claude_project_jsonl_persists_usage_and_cursor() {
        let dir = tempdir().unwrap();
        let projects_dir = dir.path().join("projects");
        let project_dir = projects_dir.join("-home-user-work");
        fs::create_dir_all(&project_dir).unwrap();
        let transcript = project_dir.join("session.jsonl");
        fs::write(
            &transcript,
            format!("{}\n", claude_assistant_message("assistant-1", 1_200, 345)),
        )
        .unwrap();
        let collector = Collector::with_sources(
            dir.path().join("tokepet.sqlite3"),
            None,
            Some(projects_dir.clone()),
            None,
        )
        .unwrap();

        let first = collector.poll_claude_once().unwrap();
        let second = collector.poll_claude_once().unwrap();
        let snapshot = collector.snapshot_for_date("2026-05-17").unwrap();

        assert_eq!(first.inserted, 1);
        assert_eq!(first.duplicates, 0);
        assert_eq!(second.inserted, 0);
        assert_eq!(snapshot.today_tokens, 2_545);
        assert_eq!(snapshot.claude_stats.events_today, 1);
        assert_eq!(
            snapshot.claude_projects_path.as_deref(),
            Some(projects_dir.to_string_lossy().as_ref())
        );
    }

    fn codex_session_meta(id: &str) -> String {
        serde_json::json!({
            "timestamp": "2026-03-15T16:48:00.000Z",
            "type": "session_meta",
            "payload": { "id": id, "originator": "codex_cli_rs" }
        })
        .to_string()
    }

    fn codex_turn_context(model: &str) -> String {
        serde_json::json!({
            "timestamp": "2026-03-15T16:48:01.000Z",
            "type": "turn_context",
            "payload": { "model": model }
        })
        .to_string()
    }

    fn codex_token_count(
        timestamp: &str,
        input: i64,
        cached: i64,
        output: i64,
        reasoning: i64,
        total: i64,
    ) -> String {
        serde_json::json!({
            "timestamp": timestamp,
            "type": "event_msg",
            "payload": {
                "type": "token_count",
                "info": {
                    "last_token_usage": {
                        "input_tokens": input,
                        "cached_input_tokens": cached,
                        "output_tokens": output,
                        "reasoning_output_tokens": reasoning,
                        "total_tokens": total
                    },
                    "total_token_usage": {
                        "input_tokens": input,
                        "cached_input_tokens": cached,
                        "output_tokens": output,
                        "reasoning_output_tokens": reasoning,
                        "total_tokens": total
                    },
                    "model_context_window": 258400
                },
                "rate_limits": null
            }
        })
        .to_string()
    }

    fn codex_null_info_event() -> String {
        serde_json::json!({
            "timestamp": "2026-03-15T16:48:00.500Z",
            "type": "event_msg",
            "payload": {
                "type": "token_count",
                "info": null,
                "rate_limits": { "primary": null }
            }
        })
        .to_string()
    }

    #[test]
    fn codex_parser_extracts_token_usage_with_session_and_model() {
        let content = format!(
            "{}\n{}\n{}\n{}\n",
            codex_session_meta("019cf25c-987f-7a23-91a5-62cffa1cea53"),
            codex_null_info_event(),
            codex_turn_context("gpt-5.2-codex"),
            codex_token_count("2026-03-15T16:49:25.074Z", 25685, 24192, 110, 0, 25795),
        );
        let events = sources::parse_codex_rollout_events(&content, 0);
        assert_eq!(events.len(), 1);
        let event = &events[0];
        assert_eq!(event.provider, "openai");
        assert_eq!(event.tool, "codex-cli");
        assert_eq!(event.model, "gpt-5.2-codex");
        assert_eq!(
            event.session_id.as_deref(),
            Some("019cf25c-987f-7a23-91a5-62cffa1cea53")
        );
        assert_eq!(event.input_tokens, 25685);
        assert_eq!(event.output_tokens, 110);
        assert_eq!(event.cached_input_tokens, 24192);
        assert_eq!(event.reasoning_tokens, 0);
        assert_eq!(event.total_tokens, 25795);
        assert!(event
            .prompt_id
            .as_deref()
            .map(|id| id.starts_with("codex:"))
            .unwrap_or(false));
    }

    #[test]
    fn codex_parser_skips_events_before_cursor() {
        let header = format!(
            "{}\n{}\n",
            codex_session_meta("session-A"),
            codex_turn_context("gpt-5.2-codex"),
        );
        let first_event = format!(
            "{}\n",
            codex_token_count("2026-03-15T16:49:25.074Z", 100, 50, 25, 0, 175)
        );
        let second_event = format!(
            "{}\n",
            codex_token_count("2026-03-15T16:50:00.000Z", 200, 80, 30, 5, 315)
        );
        let content = format!("{header}{first_event}{second_event}");

        let cursor = (header.len() + first_event.len()) as i64;
        let events = sources::parse_codex_rollout_events(&content, cursor);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].total_tokens, 315);
    }

    #[test]
    fn polls_codex_rollout_persists_usage_and_cursor() {
        let dir = tempdir().unwrap();
        let sessions_dir = dir.path().join("sessions");
        let day_dir = sessions_dir.join("2026").join("03").join("15");
        fs::create_dir_all(&day_dir).unwrap();
        let rollout = day_dir.join("rollout-2026-03-15T16-48-00-session-A.jsonl");
        fs::write(
            &rollout,
            format!(
                "{}\n{}\n{}\n",
                codex_session_meta("session-A"),
                codex_turn_context("gpt-5.2-codex"),
                codex_token_count("2026-03-15T16:49:25.074Z", 1_000, 200, 100, 50, 1_350),
            ),
        )
        .unwrap();

        let collector = Collector::with_sources(
            dir.path().join("tokepet.sqlite3"),
            None,
            None,
            Some(sessions_dir.clone()),
        )
        .unwrap();

        let first = collector.poll_codex_once().unwrap();
        let second = collector.poll_codex_once().unwrap();
        let snapshot = collector.snapshot_for_date("2026-03-15").unwrap();

        assert_eq!(first.inserted, 1);
        assert_eq!(first.duplicates, 0);
        // Second poll must not duplicate the event even though we re-parse the
        // file from the start (session_meta/turn_context recovery).
        assert_eq!(second.inserted, 0);
        assert_eq!(snapshot.today_tokens, 1_350);
        assert_eq!(snapshot.codex_stats.events_today, 1);
        assert_eq!(
            snapshot.codex_sessions_path.as_deref(),
            Some(sessions_dir.to_string_lossy().as_ref())
        );
    }

    #[test]
    fn poll_all_once_aggregates_inserted() {
        let dir = tempdir().unwrap();
        let telemetry = dir.path().join("telemetry.log");
        fs::write(&telemetry, format!("{}\n", api_response("prompt-1", 5_000))).unwrap();
        let collector = Collector::with_sources(
            dir.path().join("tokepet.sqlite3"),
            Some(telemetry),
            None,
            None,
        )
        .unwrap();

        let summary = collector.poll_all_once().unwrap();
        assert_eq!(summary.inserted(), 1);
        assert_eq!(summary.gemini.inserted, 1);
        assert_eq!(summary.claude.inserted, 0);
        assert_eq!(summary.codex.inserted, 0);
    }

    #[test]
    fn debounce_coalesces_repeated_source_events() {
        let mut batch = DebouncedSources::new(Duration::from_millis(300));
        let start = Instant::now();

        batch.push(WatchSource::Gemini, start);
        batch.push(WatchSource::Gemini, start + Duration::from_millis(100));

        assert!(batch.take_due(start + Duration::from_millis(399)).is_none());
        assert_eq!(
            batch.take_due(start + Duration::from_millis(400)),
            Some(vec![WatchSource::Gemini])
        );
        assert!(batch.take_due(start + Duration::from_millis(500)).is_none());
    }

    #[test]
    fn debounce_batches_distinct_sources_once() {
        let mut batch = DebouncedSources::new(Duration::from_millis(300));
        let start = Instant::now();

        batch.push(WatchSource::Codex, start);
        batch.push(WatchSource::Claude, start + Duration::from_millis(50));
        batch.push(WatchSource::Codex, start + Duration::from_millis(75));

        assert_eq!(
            batch.take_due(start + Duration::from_millis(375)),
            Some(vec![WatchSource::Claude, WatchSource::Codex])
        );
    }
}
