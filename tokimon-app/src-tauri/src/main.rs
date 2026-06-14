#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod growth;
mod tray;

use std::time::Duration;

use commands::window_control;
use growth::{PetState, StarterSpecies};
use rusqlite::Connection;
use serde::Serialize;
use tauri::{Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};
use tokepet_collector::{Collector, CollectorSnapshot, PollSummary};

/// Shared runtime state: the embedded token collector. The growth tables live
/// in the same SQLite file (`collector.db_path()`), so commands open a fresh
/// connection to it as needed.
struct AppState {
    collector: Collector,
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
    let summary = state.collector.poll_all_once()?;
    let conn = state.connection()?;
    let (fed, _) = growth::process_new_usage(&conn)?;
    if fed > 0 {
        let _ = app.emit("pet:fed", fed);
    }
    Ok(summary)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window_control::hide_to_tray,
            window_control::set_tray_icon,
            dashboard_snapshot,
            current_pet,
            select_starter,
            poll_now,
        ])
        .setup(|app| {
            // 펫 선택 팝업: 화면 중앙, macOS 기본 데코레이션, 고정 크기
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("TokiMon")
                .inner_size(640.0, 460.0)
                .center()
                .resizable(false)
                .build()?;

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

            // 5초마다: 모든 소스 폴링 → 신규 usage_events를 펫 성장으로 변환 → pet:fed emit.
            let poll_collector = collector.clone();
            let poll_app = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(Duration::from_secs(5));
                if poll_collector.poll_all_once().is_err() {
                    continue;
                }
                if let Ok(conn) = Connection::open(poll_collector.db_path()) {
                    if let Ok((fed, _)) = growth::process_new_usage(&conn) {
                        if fed > 0 {
                            let _ = poll_app.emit("pet:fed", fed);
                        }
                    }
                }
            });

            app.manage(AppState { collector });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
