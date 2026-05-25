// 투명·프레임리스·Always-on-top 같은 윈도우 옵션을 Builder 패턴으로 조립한다.
// main.rs가 직접 WebviewWindowBuilder를 다루지 않도록 한 겹 감싸서,
// 새 윈도우 종류가 추가될 때 main.rs는 그대로 두고 여기만 확장한다.

use tauri::{App, WebviewUrl, WebviewWindowBuilder};

pub struct TransparentWindow {
    label: String,
    title: String,
    width: f64,
    height: f64,
    transparent: bool,
    always_on_top: bool,
    decorations: bool,
}

impl TransparentWindow {
    pub fn new(label: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            title: title.into(),
            width: 240.0,
            height: 240.0,
            transparent: false,
            always_on_top: false,
            decorations: true,
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_transparent(mut self, value: bool) -> Self {
        self.transparent = value;
        self
    }

    pub fn with_always_on_top(mut self, value: bool) -> Self {
        self.always_on_top = value;
        self
    }

    pub fn with_decorations(mut self, value: bool) -> Self {
        self.decorations = value;
        self
    }

    pub fn build(self, app: &mut App) -> tauri::Result<()> {
        // 윈도우를 보이지 않게 먼저 생성 → 위치 계산 후 show. 첫 프레임이
        // OS 디폴트 위치에 잠깐 보이는 깜빡임을 막는다.
        let window = WebviewWindowBuilder::new(
            app,
            &self.label,
            WebviewUrl::App("index.html".into()),
        )
        .title(&self.title)
        .inner_size(self.width, self.height)
        .transparent(self.transparent)
        .always_on_top(self.always_on_top)
        .decorations(self.decorations)
        .resizable(false)
        .visible(false)
        .build()?;

        // primary monitor의 우하단으로 위치를 잡는다. monitor.size()는 physical
        // pixel이라 scale_factor로 logical로 정규화해야 다양한 해상도/Retina에서
        // 동일하게 동작한다.
        if let Some(monitor) = window.primary_monitor()? {
            let size = monitor.size();
            let pos = monitor.position();
            let scale = monitor.scale_factor();
            let mon_x = pos.x as f64 / scale;
            let mon_y = pos.y as f64 / scale;
            let mon_w = size.width as f64 / scale;
            let mon_h = size.height as f64 / scale;

            const MARGIN: f64 = 16.0;
            let target_x = mon_x + mon_w - self.width - MARGIN;
            let target_y = mon_y + mon_h - self.height - MARGIN;

            window.set_position(tauri::LogicalPosition::new(target_x, target_y))?;
        }

        window.show()?;
        Ok(())
    }
}
