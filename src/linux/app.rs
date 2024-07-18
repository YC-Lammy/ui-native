use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, BoxExt, GtkWindowExt, WidgetExt};
use gtk4::ApplicationWindow;
use parking_lot::RwLock;

use crate::native_tree::NativeTree;
use crate::private::ElementLike;
use crate::shadow_tree::command::Command;
use crate::shadow_tree::commit::commit_tree;
use crate::shadow_tree::component::CoreComponent;
use crate::state::GLOBAL_STATE_CHANGED;
use crate::Context;

pub struct GtkApp {
    app: gtk4::Application,
    title: String,
    width: u32,
    height: u32,

    inner: Arc<AppInner>,
}

struct AppInner {
    comitted_tree: RwLock<Option<CoreComponent>>,
    command_sender: Sender<Vec<Command>>,
    command_receiver: Receiver<Vec<Command>>,
}

impl Default for AppInner {
    fn default() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();

        Self {
            comitted_tree: Default::default(),
            command_sender: tx,
            command_receiver: rx,
        }
    }
}

impl GtkApp {
    pub fn new(builder: crate::AppBuilder) -> Self {
        // initialise gtk
        gtk4::init().expect("failed to initialise GTK");

        // create a new gtk app
        let app = gtk4::Application::builder()
            .application_id("com.ui.native")
            .build();

        Self {
            app: app,
            title: builder.title.unwrap_or_default(),
            width: builder.width.unwrap_or(320),
            height: builder.width.unwrap_or(320),

            inner: Default::default(),
        }
    }

    pub fn launch<T, F>(self, on_active: F)
    where
        F: Fn(&Context) -> T + Send + Sync + 'static,
        T: ElementLike,
    {
        let width = self.width;
        let height = self.height;
        let title = self.title;

        let root_view = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

        root_view.set_hexpand(true);
        root_view.set_vexpand(true);

        let cloned_root_view = root_view.clone();

        let window = Rc::new(RefCell::new(None));
        let cloned_window = window.clone();

        self.app.connect_activate(move |app| {
            // create new window
            let window = ApplicationWindow::builder()
                .application(app)
                .title(title.as_str())
                .default_width(width as _)
                .default_height(height as _)
                .build();

            // set content as root view
            window.set_child(Some(&cloned_root_view));

            // present the window
            window.present();

            cloned_window.borrow_mut().replace(window);
        });

        let app_inner = self.inner.clone();

        // create new thread for rendering
        std::thread::spawn(move || {
            // create new context
            let ctx = Context::new();

            // indefinite loop for rendering
            loop {
                
                // call on active
                let mut widget = on_active(&ctx);

                if GLOBAL_STATE_CHANGED.load(Ordering::SeqCst) {
                    widget.on_state_change(&Context::new())
                }
                // render the widget
                let mut elem = widget.render();

                let mut comp = loop {
                    match elem {
                        Err(mut e) => {
                            elem = e.render();
                        }
                        Ok(c) => {
                            break c;
                        }
                    }
                };

                // acquire read lock to the old tree
                let mut old = app_inner.comitted_tree.upgradable_read();

                // encode the new tree into commands by comparing to the old tree
                let commands = commit_tree(&mut comp, old.as_ref());

                // ungrade the read lock and write the tree
                old.with_upgraded(|comit| {
                    *comit = Some(comp);
                });

                // drop the read lock
                drop(old);

                // send the commands to the main thread
                app_inner
                    .command_sender
                    .send(commands)
                    .expect("app closed unexpectedly");

                // sleep 18ms
                std::thread::sleep(Duration::from_millis(20));
            }
        });

        let app_inner = self.inner.clone();

        // create a new native tree
        let native_tree = NativeTree::get();

        // force callback every 18ms
        glib::timeout_add_local(Duration::from_millis(20), move || {
            // has commands been made to tree
            let mut commands_recieved = false;

            // try to fetch commands
            while let Ok(commands) = app_inner.command_receiver.try_recv() {
                commands_recieved = true;
                // execute commands
                native_tree.execute_commands(commands);
            }

            // only need to check root if any command has been made
            if commands_recieved {
                let window = window.borrow();
                let window = window.as_ref().unwrap();

                // recalculate layout
                native_tree.compute_layout(window.width() as _, window.height() as _);

                // get the root view from native tree
                if let Some(root) = native_tree.get_root_node() {
                    let root = root.component();

                    if let Some(c) = root_view.first_child() {
                        // if the child is not the same object,
                        // the new child must be mounted
                        if !c.eq(root.widget().as_gtk4_widget()) {
                            // remove all the child
                            while let Some(c) = root_view.first_child() {
                                root_view.remove(&c);
                            }
                            // add root widget
                            root_view.append(root.widget().as_gtk4_widget());
                        }
                        // other wise the root has the same object
                    } else {
                        // the root has not been mounted
                        root_view.append(root.widget().as_gtk4_widget());
                    }
                } else {
                    // remove all the child
                    while let Some(c) = root_view.first_child() {
                        root_view.remove(&c);
                    }
                }
            }

            // indefinite loop
            glib::ControlFlow::Continue
        });

        // run the main loop
        self.app.run();
    }
}
