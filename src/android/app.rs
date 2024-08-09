use std::marker::PhantomData;
use std::time::{Duration, Instant};

use android_activity::input::InputEvent;
use android_activity::{AndroidApp, InputStatus, MainEvent, PollEvent};
use jni_bind::JReturnType;

use crate::native_tree::{context::Context, NativeTree};
use crate::shadow_tree::commit::commit_tree;
use crate::{AppBuilder, Application};

pub struct NativeApp {
    app: AndroidApp,
}

impl NativeApp {
    pub fn new(_builder: AppBuilder, app: AndroidApp) -> Self {
        Self { app: app }
    }
    pub fn launch<A>(self, mut app: A)
    where
        A: Application + Send + Sync,
    {
        let jvm = unsafe {
            jni::JavaVM::from_raw(self.app.vm_as_ptr() as _).expect("faild to acquire java vm")
        };
        let mut env = jvm.get_env().expect("failed to initialise jni environment");
        let activity = unsafe {
            crate::android::binding::android::app::NativeActivity::from_jvalue(
                &mut env,
                jni::sys::jvalue {
                    l: self.app.activity_as_ptr() as _,
                },
            )
        };

        let mut context = Context {
            jni_env: env,
            android_activity: activity,
            _mark: PhantomData,
        };

        let mut window = None;

        let native_tree = NativeTree::get(&mut context);
        let mut shadow_tree = None;

        let mut has_input = false;
        let mut last_input_handle = Instant::now();

        self.app
            .poll_events(Some(Duration::from_millis(100)), |event| {
                // true if screen should rerender
                let mut should_render = false;

                match event {
                    PollEvent::Timeout => {
                        // only rerender if window is present
                        should_render = window.is_some();
                    }
                    PollEvent::Wake => {}
                    PollEvent::Main(MainEvent::InputAvailable) => {
                        has_input = true;
                    }
                    PollEvent::Main(MainEvent::InitWindow { .. }) => {
                        // assign the window
                        window = self.app.native_window();
                        // rerender the screen
                        should_render = true;
                    }
                    PollEvent::Main(MainEvent::TerminateWindow { .. }) => {
                        window = None;
                    }
                    PollEvent::Main(MainEvent::WindowResized { .. }) => {
                        should_render = true;
                    }
                    PollEvent::Main(MainEvent::RedrawNeeded { .. }) => {
                        should_render = true;
                    }
                    PollEvent::Main(MainEvent::ContentRectChanged { .. }) => {
                        should_render = true;
                    }
                    PollEvent::Main(MainEvent::GainedFocus) => {}
                    PollEvent::Main(MainEvent::LostFocus) => {}
                    PollEvent::Main(MainEvent::ConfigChanged { .. }) => {}
                    PollEvent::Main(MainEvent::LowMemory) => {}
                    PollEvent::Main(MainEvent::Start) => {}
                    PollEvent::Main(MainEvent::Resume { loader: _, .. }) => {}
                    PollEvent::Main(MainEvent::SaveState { saver: _, .. }) => {}
                    PollEvent::Main(MainEvent::Pause) => {}
                    PollEvent::Main(MainEvent::Stop) => {}
                    PollEvent::Main(MainEvent::Destroy) => {}
                    PollEvent::Main(MainEvent::InsetsChanged { .. }) => {}
                    _ => {}
                }

                if should_render {
                    // call render on the user defined app
                    let mut element = app.render();

                    // render the elements until a core component is returned
                    let mut component = loop {
                        match element.render() {
                            Ok(c) => break c,
                            Err(e) => element = e,
                        }
                    };

                    // generate command by comparing the new and old shadow tree
                    let commands = commit_tree(&mut component, shadow_tree.as_ref());
                    // set the old shadow tree to the new shadow tree
                    shadow_tree = Some(component);

                    // execute the changes on the native tree
                    native_tree.execute_commands(&mut context, commands);
                    // compute the style of the components
                    native_tree.compute_style();

                    // compute the layout
                    if let Some(window) = &window {
                        native_tree.compute_layout(
                            &mut context,
                            window.width() as f64,
                            window.height() as f64,
                        );
                    }
                }

                if has_input {
                    last_input_handle = Instant::now();
                    has_input = false;

                    if let Ok(mut events) = self.app.input_events_iter() {
                        while events.next(|event| {
                            match event {
                                InputEvent::MotionEvent(_) => InputStatus::Unhandled,
                                InputEvent::TextEvent(_) => InputStatus::Unhandled,
                                InputEvent::KeyEvent(k) => {
                                    // dispatch the event to the native tree
                                    super::event::dispatch_event(native_tree, k);
                                    InputStatus::Unhandled
                                }
                                _ => InputStatus::Unhandled,
                            }
                        }) {}
                    }
                }
            });
    }
}
