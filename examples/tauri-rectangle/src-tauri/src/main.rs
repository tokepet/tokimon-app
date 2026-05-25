#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod window;

use commands::window_control;
use window::TransparentWindow;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            window_control::close,
            window_control::minimize,
        ])
        .setup(|app| {
            TransparentWindow::new("main", "Rectangle")
                .with_size(240.0, 240.0)
                .with_transparent(true)
                .with_always_on_top(true)
                .with_decorations(false)
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
