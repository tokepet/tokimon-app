#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod collector;
mod commands;
mod growth;
mod tray;

use std::{sync::Mutex, time::Duration};

use collector::{Collector, CollectorSnapshot, CollectorWatch, PollSummary, WatchOptions};
use commands::window_control;
use growth::{PetState, StarterSpecies};
use rusqlite::Connection;
use serde::Serialize;
use tauri::{Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

/// Shared runtime state: the embedded token collector. The growth tables live
/// in the same SQLite file (`collector.db_path()`), so commands open a fresh
/// connection to it as needed.
struct AppState {
    collector: Collector,
    _watcher: Mutex<Option<CollectorWatch>>,
}

impl AppState {
    fn connection(&self) -> Result<Connection, String> {
        Connection::open(self.collector.db_path()).map_err(|error| error.to_string())
    }
}

/// Collector usage view plus the current pet — everything a dashboard needs.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DashboardSnapshot {
    collector: CollectorSnapshot,
    pet: PetState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TokenUsageSnapshot {
    event_count: i64,
    input_tokens: i64,
    reasoning_tokens: i64,
    thoughts_tokens: i64,
    tool_tokens: i64,
    total_tokens: i64,
    last_event_at: Option<String>,
}

#[tauri::command]
fn dashboard_snapshot(state: State<'_, AppState>) -> Result<DashboardSnapshot, String> {
    let collector = state.collector.snapshot()?;
    let conn = state.connection()?;
    let pet = growth::load_pet(&conn)?;
    Ok(DashboardSnapshot { collector, pet })
}

#[tauri::command]
fn current_pet(state: State<'_, AppState>) -> Result<PetState, String> {
    let conn = state.connection()?;
    growth::load_pet(&conn)
}

#[tauri::command]
fn token_usage(state: State<'_, AppState>) -> Result<TokenUsageSnapshot, String> {
    let conn = state.connection()?;
    conn.query_row(
        "SELECT
            COUNT(*),
            COALESCE(SUM(input_tokens), 0),
            COALESCE(SUM(reasoning_tokens), 0),
            COALESCE(SUM(thoughts_tokens), 0),
            COALESCE(SUM(tool_tokens), 0),
            COALESCE(SUM(total_tokens), 0),
            MAX(timestamp)
         FROM usage_events",
        [],
        |row| {
            Ok(TokenUsageSnapshot {
                event_count: row.get(0)?,
                input_tokens: row.get(1)?,
                reasoning_tokens: row.get(2)?,
                thoughts_tokens: row.get(3)?,
                tool_tokens: row.get(4)?,
                total_tokens: row.get(5)?,
                last_event_at: row.get(6)?,
            })
        },
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn select_starter(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    species: String,
) -> Result<PetState, String> {
    let species_enum = StarterSpecies::try_from(species.as_str())?;
    let conn = state.connection()?;
    let pet = growth::select_starter(&conn, species_enum)?;
    let _ = app.emit("pet:species-changed", &species);
    Ok(pet)
}

/// Manual one-shot poll: collect from all sources, then feed the pet.
#[tauri::command]
fn poll_now(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<PollSummary, String> {
    poll_and_feed(&app, &state.collector)
}

fn poll_and_feed(app: &tauri::AppHandle, collector: &Collector) -> Result<PollSummary, String> {
    let summary = collector.poll_all_once()?;
    feed_new_usage(app, collector)?;
    Ok(summary)
}

fn feed_new_usage(app: &tauri::AppHandle, collector: &Collector) -> Result<(), String> {
    let conn = Connection::open(collector.db_path()).map_err(|error| error.to_string())?;
    let (fed, _) = growth::process_new_usage(&conn)?;
    if fed > 0 {
        let _ = app.emit("pet:fed", fed);
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window_control::hide_to_tray,
            window_control::enter_selection_mode,
            window_control::enter_pet_mode,
            window_control::set_tray_icon,
            dashboard_snapshot,
            current_pet,
            token_usage,
            select_starter,
            poll_now,
        ])
        .setup(|app| {
            // 펫 선택 팝업: 화면 중앙, macOS 기본 데코레이션, 고정 크기
            let window =
                WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                    .title("TokiMon")
                    .inner_size(640.0, 460.0)
                    .transparent(true)
                    .center()
                    .resizable(false)
                    .build()?;
            window.show()?;
            window.set_focus()?;

            tray::create_tray(app)?;

            // 수집기 시작: app data 디렉터리의 SQLite에 usage_events를 적재한다.
            let app_data_dir = app
                .path()
                .app_data_dir()
                .map_err(|error| error.to_string())?;
            let db_path = app_data_dir.join("tokimon.sqlite3");
            let collector = Collector::start(db_path).map_err(|error| error.to_string())?;

            // 앱이 소유하는 펫 테이블 마이그레이션.
            {
                let conn = Connection::open(collector.db_path())?;
                growth::apply_migrations(&conn).map_err(|error| error.to_string())?;
            }

            // 파일 변경 이벤트 기반 수집: 300ms debounce + 30초 안전망 poll.
            let watch_collector = collector.clone();
            let watch_app = app.handle().clone();
            let watcher = collector
                .watch(
                    WatchOptions {
                        debounce: Duration::from_millis(300),
                        backup_poll: Duration::from_secs(30),
                    },
                    move |poll_result| match poll_result {
                        Ok(_) => {
                            if let Err(error) = feed_new_usage(&watch_app, &watch_collector) {
                                eprintln!("failed to feed pet from token usage: {error}");
                            }
                        }
                        Err(error) => eprintln!("failed to collect token usage: {error}"),
                    },
                )
                .map_err(|error| error.to_string())?;

            app.manage(AppState {
                collector,
                _watcher: Mutex::new(Some(watcher)),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
