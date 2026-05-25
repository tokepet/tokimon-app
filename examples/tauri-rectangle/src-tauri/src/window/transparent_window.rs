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
        WebviewWindowBuilder::new(app, &self.label, WebviewUrl::App("index.html".into()))
            .title(&self.title)
            .inner_size(self.width, self.height)
            .transparent(self.transparent)
            .always_on_top(self.always_on_top)
            .decorations(self.decorations)
            .resizable(false)
            .build()?;
        Ok(())
    }
}
