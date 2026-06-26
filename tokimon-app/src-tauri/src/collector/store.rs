use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::event::UsageEvent;

const SCHEMA_VERSION: i64 = 1;

/// Per-provider aggregate stats for a given date.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStats {
    pub events_today: i64,
    pub tokens_today: i64,
    pub last_event_at: Option<String>,
}

/// Apply the collector schema. This owns only the tables the collector is
/// responsible for: usage events and per-source file cursors. Pet state and
/// growth live in the consuming application, not the collector.
pub fn apply_migrations(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(&format!(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS usage_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider TEXT NOT NULL,
            tool TEXT NOT NULL,
            model TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            session_id TEXT,
            prompt_id TEXT,
            input_tokens INTEGER NOT NULL DEFAULT 0,
            output_tokens INTEGER NOT NULL DEFAULT 0,
            cached_input_tokens INTEGER NOT NULL DEFAULT 0,
            cache_creation_tokens INTEGER NOT NULL DEFAULT 0,
            reasoning_tokens INTEGER NOT NULL DEFAULT 0,
            thoughts_tokens INTEGER NOT NULL DEFAULT 0,
            tool_tokens INTEGER NOT NULL DEFAULT 0,
            total_tokens INTEGER NOT NULL DEFAULT 0,
            source_type TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE UNIQUE INDEX IF NOT EXISTS idx_usage_events_identity
            ON usage_events (
                provider,
                tool,
                source_type,
                COALESCE(prompt_id, ''),
                COALESCE(session_id, ''),
                timestamp,
                model
            );
        CREATE INDEX IF NOT EXISTS idx_usage_events_daily
            ON usage_events (substr(timestamp, 1, 10), provider, tool, model);
        CREATE TABLE IF NOT EXISTS collector_cursors (
            source TEXT NOT NULL,
            path TEXT NOT NULL,
            cursor INTEGER NOT NULL DEFAULT 0,
            updated_at TEXT NOT NULL,
            PRIMARY KEY (source, path)
        );
        CREATE TABLE IF NOT EXISTS collector_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );
        INSERT OR IGNORE INTO schema_migrations (version, applied_at)
            VALUES ({SCHEMA_VERSION}, CURRENT_TIMESTAMP);
        "#
    ))
    .map_err(|error| error.to_string())
}

/// Insert a usage event, deduping on the identity index. Returns `true` when a
/// new row was inserted, `false` when it was a duplicate.
pub fn insert_usage_event(conn: &Connection, event: &UsageEvent) -> Result<bool, String> {
    let changed = conn
        .execute(
            "INSERT OR IGNORE INTO usage_events (
                provider, tool, model, timestamp, session_id, prompt_id,
                input_tokens, output_tokens, cached_input_tokens, cache_creation_tokens,
                reasoning_tokens, thoughts_tokens, tool_tokens, total_tokens, source_type
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                event.provider,
                event.tool,
                event.model,
                event.timestamp,
                event.session_id,
                event.prompt_id,
                event.input_tokens,
                event.output_tokens,
                event.cached_input_tokens,
                event.cache_creation_tokens,
                event.reasoning_tokens,
                event.thoughts_tokens,
                event.tool_tokens,
                event.total_tokens,
                event.source_type
            ],
        )
        .map_err(|error| error.to_string())?;
    Ok(changed == 1)
}

pub fn daily_total_tokens(conn: &Connection, date: &str) -> Result<i64, String> {
    conn.query_row(
        "SELECT COALESCE(SUM(total_tokens), 0) FROM usage_events WHERE substr(timestamp, 1, 10) = ?1",
        [date],
        |row| row.get(0),
    )
    .map_err(|error| error.to_string())
}

pub fn provider_stats(
    conn: &Connection,
    provider: &str,
    date: &str,
) -> Result<ProviderStats, String> {
    let (events_today, tokens_today): (i64, i64) = conn
        .query_row(
            "SELECT COUNT(*), COALESCE(SUM(total_tokens), 0)
             FROM usage_events
             WHERE provider = ?1 AND substr(timestamp, 1, 10) = ?2",
            params![provider, date],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|error| error.to_string())?;
    let last_event_at: Option<String> = conn
        .query_row(
            "SELECT MAX(timestamp) FROM usage_events WHERE provider = ?1",
            params![provider],
            |row| row.get(0),
        )
        .map_err(|error| error.to_string())?;
    Ok(ProviderStats {
        events_today,
        tokens_today,
        last_event_at,
    })
}

/// Read a collector setting. Returns `None` when the key has never been set.
pub fn load_setting(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT value FROM collector_settings WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|error| error.to_string())
}

/// Write a collector setting, overwriting any existing value.
pub fn save_setting(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO collector_settings (key, value, updated_at)
         VALUES (?1, ?2, CURRENT_TIMESTAMP)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![key, value],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

/// Delete all collected usage events and per-file cursors so collection starts
/// fresh. Settings are preserved.
pub fn clear_usage_events(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "DELETE FROM usage_events;
         DELETE FROM collector_cursors;",
    )
    .map_err(|error| error.to_string())
}

pub fn load_cursor(conn: &Connection, source: &str, path: &Path) -> Result<i64, String> {
    let path = path.to_string_lossy();
    conn.query_row(
        "SELECT cursor FROM collector_cursors WHERE source = ?1 AND path = ?2",
        params![source, path.as_ref()],
        |row| row.get(0),
    )
    .optional()
    .map_err(|error| error.to_string())
    .map(|cursor| cursor.unwrap_or(0))
}

pub fn save_cursor(
    conn: &Connection,
    source: &str,
    path: &Path,
    cursor: i64,
) -> Result<(), String> {
    let path = path.to_string_lossy();
    conn.execute(
        "INSERT INTO collector_cursors (source, path, cursor, updated_at)
         VALUES (?1, ?2, ?3, CURRENT_TIMESTAMP)
         ON CONFLICT(source, path) DO UPDATE SET cursor = excluded.cursor, updated_at = excluded.updated_at",
        params![source, path.as_ref(), cursor],
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}
