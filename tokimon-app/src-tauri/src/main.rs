#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod tray;

use commands::window_control;
use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window_control::hide_to_tray,
            window_control::set_tray_icon,
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
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
