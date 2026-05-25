// webview(React) ↔ OS 윈도우 제어 경계.
// Rust에서 노출해야만 webview JS가 OS 기능을 호출할 수 있다.

use tauri::WebviewWindow;

#[tauri::command]
pub fn close(window: WebviewWindow) {
    let _ = window.close();
}

#[tauri::command]
pub fn minimize(window: WebviewWindow) {
    let _ = window.minimize();
}
