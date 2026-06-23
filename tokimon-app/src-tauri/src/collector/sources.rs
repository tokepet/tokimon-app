use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::event::UsageEvent;

// ---------------------------------------------------------------------------
// Default source paths
// ---------------------------------------------------------------------------

pub fn default_claude_projects_path() -> Option<PathBuf> {
    std::env::var_os("CLAUDE_PROJECTS_DIR")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".claude").join("projects")))
}

pub fn default_gemini_telemetry_path() -> Option<PathBuf> {
    std::env::var_os("GEMINI_TELEMETRY_OUTFILE")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".gemini").join("telemetry.log")))
}

pub fn default_codex_sessions_path() -> Option<PathBuf> {
    std::env::var_os("CODEX_SESSIONS_DIR")
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".codex").join("sessions")))
}

// ---------------------------------------------------------------------------
// Gemini telemetry
// ---------------------------------------------------------------------------

pub fn parse_gemini_telemetry_events(content: &str) -> Vec<UsageEvent> {
    content
        .lines()
        .filter_map(parse_gemini_telemetry_line)
        .collect()
}

fn parse_gemini_telemetry_line(line: &str) -> Option<UsageEvent> {
    let value: Value = serde_json::from_str(line).ok()?;
    if value.get("name")?.as_str()? != "gemini_cli.api_response" {
        return None;
    }
    let attributes = value.get("attributes")?;
    let timestamp = value
        .get("timestamp")
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(now_utc_timestamp);
    let model = attributes
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    Some(UsageEvent {
        provider: "gemini".to_string(),
        tool: "gemini-cli".to_string(),
        model,
        timestamp,
        session_id: None,
        prompt_id: attributes
            .get("prompt_id")
            .and_then(Value::as_str)
            .map(ToString::to_string),
        input_tokens: json_i64(attributes, "input_token_count"),
        output_tokens: json_i64(attributes, "output_token_count"),
        cached_input_tokens: json_i64(attributes, "cached_content_token_count"),
        cache_creation_tokens: 0,
        reasoning_tokens: 0,
        thoughts_tokens: json_i64(attributes, "thoughts_token_count"),
        tool_tokens: json_i64(attributes, "tool_token_count"),
        total_tokens: json_i64(attributes, "total_token_count"),
        source_type: "telemetry_file".to_string(),
    })
}

// ---------------------------------------------------------------------------
// Claude Code transcripts
// ---------------------------------------------------------------------------

pub fn claude_transcript_paths(projects_path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    collect_claude_transcript_paths(projects_path, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_claude_transcript_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_claude_transcript_paths(&path, paths)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("jsonl") {
            paths.push(path);
        }
    }
    Ok(())
}

pub fn parse_claude_transcript_events(content: &str) -> Vec<UsageEvent> {
    content
        .lines()
        .filter_map(parse_claude_transcript_line)
        .collect()
}

fn parse_claude_transcript_line(line: &str) -> Option<UsageEvent> {
    let value: Value = serde_json::from_str(line).ok()?;
    if value.get("type")?.as_str()? != "assistant" {
        return None;
    }
    let message = value.get("message")?;
    let usage = message.get("usage")?;
    let input_tokens = json_i64(usage, "input_tokens");
    let output_tokens = json_i64(usage, "output_tokens");
    let cache_creation_tokens = json_i64(usage, "cache_creation_input_tokens");
    let cached_input_tokens = json_i64(usage, "cache_read_input_tokens");
    let total_tokens = input_tokens + output_tokens + cache_creation_tokens + cached_input_tokens;
    if total_tokens <= 0 {
        return None;
    }
    let timestamp = value
        .get("timestamp")
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(now_utc_timestamp);
    let model = message
        .get("model")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let prompt_id = value
        .get("uuid")
        .and_then(Value::as_str)
        .or_else(|| message.get("id").and_then(Value::as_str))
        .map(ToString::to_string);
    let session_id = value
        .get("sessionId")
        .and_then(Value::as_str)
        .or_else(|| value.get("session_id").and_then(Value::as_str))
        .map(ToString::to_string);

    Some(UsageEvent {
        provider: "claude".to_string(),
        tool: "claude-code".to_string(),
        model,
        timestamp,
        session_id,
        prompt_id,
        input_tokens,
        output_tokens,
        cached_input_tokens,
        cache_creation_tokens,
        reasoning_tokens: 0,
        thoughts_tokens: 0,
        tool_tokens: 0,
        total_tokens,
        source_type: "local_parser".to_string(),
    })
}

// ---------------------------------------------------------------------------
// Codex rollouts
// ---------------------------------------------------------------------------

pub fn codex_rollout_paths(sessions_path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    collect_codex_rollout_paths(sessions_path, &mut paths)?;
    paths.sort();
    Ok(paths)
}

fn collect_codex_rollout_paths(dir: &Path, paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_codex_rollout_paths(&path, paths)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("jsonl") {
            // Codex rollout files are named rollout-<date>-<uuid>.jsonl
            if path
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("rollout-"))
                .unwrap_or(false)
            {
                paths.push(path);
            }
        }
    }
    Ok(())
}

/// Parse a Codex rollout file. The file is parsed from the start so that
/// `session_meta` and `turn_context` lines (carrying session id and the
/// active model) are resolved correctly, but `UsageEvent`s are only emitted
/// for `event_msg`/`token_count` lines whose byte offset is past `cursor`.
pub fn parse_codex_rollout_events(content: &str, cursor: i64) -> Vec<UsageEvent> {
    let mut events = Vec::new();
    let mut session_id: Option<String> = None;
    let mut current_model: Option<String> = None;
    let mut byte_offset: i64 = 0;

    for line in content.split_inclusive('\n') {
        let line_offset = byte_offset;
        byte_offset += line.len() as i64;
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if trimmed.is_empty() {
            continue;
        }

        let value: Value = match serde_json::from_str(trimmed) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let line_type = value.get("type").and_then(Value::as_str).unwrap_or("");

        match line_type {
            "session_meta" => {
                if let Some(id) = value
                    .get("payload")
                    .and_then(|payload| payload.get("id"))
                    .and_then(Value::as_str)
                {
                    session_id = Some(id.to_string());
                }
            }
            "turn_context" => {
                if let Some(model) = value
                    .get("payload")
                    .and_then(|payload| payload.get("model"))
                    .and_then(Value::as_str)
                {
                    current_model = Some(model.to_string());
                }
            }
            "event_msg" => {
                // Only emit events past the cursor so re-reads don't re-insert.
                if line_offset < cursor {
                    continue;
                }
                if let Some(event) = parse_codex_token_count_event(
                    &value,
                    line_offset,
                    session_id.as_deref(),
                    current_model.as_deref(),
                ) {
                    events.push(event);
                }
            }
            _ => {}
        }
    }

    events
}

fn parse_codex_token_count_event(
    value: &Value,
    byte_offset: i64,
    session_id: Option<&str>,
    current_model: Option<&str>,
) -> Option<UsageEvent> {
    let payload = value.get("payload")?;
    if payload.get("type").and_then(Value::as_str)? != "token_count" {
        return None;
    }
    let info = payload.get("info")?;
    if info.is_null() {
        return None;
    }
    let last = info.get("last_token_usage")?;
    let input_tokens = json_i64(last, "input_tokens");
    let output_tokens = json_i64(last, "output_tokens");
    let cached_input_tokens = json_i64(last, "cached_input_tokens");
    let reasoning_tokens = json_i64(last, "reasoning_output_tokens");
    let total_tokens = json_i64(last, "total_tokens");
    if total_tokens <= 0 {
        return None;
    }
    let timestamp = value
        .get("timestamp")
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .unwrap_or_else(now_utc_timestamp);

    Some(UsageEvent {
        provider: "openai".to_string(),
        tool: "codex-cli".to_string(),
        model: current_model.unwrap_or("unknown").to_string(),
        timestamp,
        session_id: session_id.map(ToString::to_string),
        prompt_id: Some(format!("codex:{byte_offset}")),
        input_tokens,
        output_tokens,
        cached_input_tokens,
        cache_creation_tokens: 0,
        reasoning_tokens,
        thoughts_tokens: 0,
        tool_tokens: 0,
        total_tokens,
        source_type: "codex_rollout".to_string(),
    })
}

// ---------------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------------

fn json_i64(value: &Value, key: &str) -> i64 {
    value.get(key).and_then(Value::as_i64).unwrap_or(0)
}

pub fn today_utc_date() -> String {
    chrono::Utc::now().format("%Y-%m-%d").to_string()
}

pub fn now_utc_timestamp() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
