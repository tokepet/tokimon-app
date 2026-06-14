// webview(React) ↔ OS 윈도우/트레이 제어 경계.
// 펫 선택 시 메뉴바 트레이 아이콘을 선택한 펫 스프라이트로 교체하고,
// 메인 윈도우는 작고 프레임 없는 떠다니는 펫 창으로 전환한다.

use tauri::{image::Image, AppHandle, LogicalPosition, LogicalSize, WebviewWindow};

const PET_WINDOW_W: f64 = 240.0;
const PET_WINDOW_H: f64 = 120.0;
const PET_WINDOW_RIGHT_MARGIN: f64 = 16.0;
const PET_WINDOW_BOTTOM_MARGIN: f64 = 120.0;
const SELECTION_WINDOW_W: f64 = 640.0;
const SELECTION_WINDOW_H: f64 = 460.0;

#[tauri::command]
pub fn hide_to_tray(window: WebviewWindow) {
    let _ = window.hide();
}

#[tauri::command]
pub fn enter_selection_mode(window: WebviewWindow) -> Result<(), String> {
    window
        .set_size(LogicalSize::new(SELECTION_WINDOW_W, SELECTION_WINDOW_H))
        .map_err(|e| e.to_string())?;
    window.set_decorations(true).map_err(|e| e.to_string())?;
    let _ = window.set_shadow(true);
    window.set_always_on_top(false).map_err(|e| e.to_string())?;
    center_window(&window, SELECTION_WINDOW_W, SELECTION_WINDOW_H)?;
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn enter_pet_mode(window: WebviewWindow) -> Result<(), String> {
    window
        .set_size(LogicalSize::new(PET_WINDOW_W, PET_WINDOW_H))
        .map_err(|e| e.to_string())?;
    window.set_decorations(false).map_err(|e| e.to_string())?;
    let _ = window.set_shadow(false);
    window.set_always_on_top(true).map_err(|e| e.to_string())?;

    with_primary_monitor(&window, |mon_x, mon_y, mon_w, mon_h| {
        let target_x = mon_x + mon_w - PET_WINDOW_W - PET_WINDOW_RIGHT_MARGIN;
        let target_y = mon_y + mon_h - PET_WINDOW_H - PET_WINDOW_BOTTOM_MARGIN;
        window
            .set_position(LogicalPosition::new(target_x, target_y))
            .map_err(|e| e.to_string())
    })?;

    window.show().map_err(|e| e.to_string())?;
    Ok(())
}

fn center_window(window: &WebviewWindow, width: f64, height: f64) -> Result<(), String> {
    with_primary_monitor(window, |mon_x, mon_y, mon_w, mon_h| {
        let target_x = mon_x + (mon_w - width) / 2.0;
        let target_y = mon_y + (mon_h - height) / 2.0;
        window
            .set_position(LogicalPosition::new(target_x, target_y))
            .map_err(|e| e.to_string())
    })
}

fn with_primary_monitor(
    window: &WebviewWindow,
    update_position: impl FnOnce(f64, f64, f64, f64) -> Result<(), String>,
) -> Result<(), String> {
    if let Some(monitor) = window.primary_monitor().map_err(|e| e.to_string())? {
        let size = monitor.size();
        let pos = monitor.position();
        let scale = monitor.scale_factor();
        let mon_x = pos.x as f64 / scale;
        let mon_y = pos.y as f64 / scale;
        let mon_w = size.width as f64 / scale;
        let mon_h = size.height as f64 / scale;
        update_position(mon_x, mon_y, mon_w, mon_h)?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_tray_icon(
    app: AppHandle,
    rgba: Vec<u8>,
    width: u32,
    height: u32,
) -> Result<(), String> {
    // rgba 길이와 width*height*4가 맞지 않으면 Image가 잘못 해석되니 사전 검증.
    let expected = (width as usize) * (height as usize) * 4;
    if rgba.len() != expected {
        return Err(format!(
            "rgba 길이 {}바이트는 {}x{}x4={}바이트와 일치해야 합니다",
            rgba.len(),
            width,
            height,
            expected
        ));
    }

    let tray = app
        .tray_by_id("main-tray")
        .ok_or_else(|| "main-tray를 찾을 수 없습니다".to_string())?;
    let icon = Image::new_owned(rgba, width, height);
    tray.set_icon(Some(icon)).map_err(|e| e.to_string())?;
    Ok(())
}
