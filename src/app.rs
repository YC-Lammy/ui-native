use alloc::string::String;

use crate::imp::NativeApp;
use crate::private::ElementLike;
use crate::Context;

pub struct App {
    app: NativeApp,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn launch<T, F>(self, on_active: F)
    where
        F: Fn(&Context) -> T + Send + Sync + 'static,
        T: ElementLike,
    {
        self.app.launch(on_active)
    }
}

#[derive(Default)]
pub struct AppBuilder {
    pub(crate) title: Option<String>,
    pub(crate) width: Option<u32>,
    pub(crate) height: Option<u32>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
    #[cfg(not(all(target_os = "android")))]
    pub fn build(self) -> Result<App, ()> {
        let app = NativeApp::new(self);
        Ok(App { app: app })
    }

    #[cfg(target_os = "android")]
    pub fn build_android(self, app: AndroidApp) -> Result<App, ()> {}
}
