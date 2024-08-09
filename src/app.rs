use crate::imp::NativeApp;
use crate::private::ElementLike;

pub trait Application: Send + Sync + 'static {
    fn render(&mut self) -> crate::ElementLike;
}

impl<F, E> Application for F
where
    F: Fn() -> E + Send + Sync + 'static,
    E: ElementLike,
{
    fn render(&mut self) -> crate::ElementLike {
        Box::new((self)())
    }
}

pub struct App {
    app: NativeApp,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub fn launch<A: Application>(self, app: A) {
        self.app.launch(app)
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
    pub fn build_android(self, app: android_activity::AndroidApp) -> Result<App, ()> {
        let app = NativeApp::new(self, app);
        Ok(App { app: app })
    }
}
