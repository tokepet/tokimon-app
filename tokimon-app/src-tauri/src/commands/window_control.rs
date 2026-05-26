// webview(React) ↔ OS 윈도우/트레이 제어 경계.
// 펫 선택 시 메인 윈도우를 hide하고, 메뉴바 트레이 아이콘을 선택한 펫 스프라이트로 교체한다.

use tauri::{image::Image, AppHandle, WebviewWindow};

#[tauri::command]
pub fn hide_to_tray(window: WebviewWindow) {
    let _ = window.hide();
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
