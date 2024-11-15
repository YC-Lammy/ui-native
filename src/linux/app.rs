use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use gtk4::prelude::{ApplicationExt, ApplicationExtManual, GtkWindowExt, WidgetExt};
use gtk4::ApplicationWindow;
use parking_lot::RwLock;

use crate::app::Application;
use crate::native_tree::context::Context;
use crate::native_tree::NativeTree;
use crate::shadow_tree::command::Command;
use crate::shadow_tree::commit::commit_tree;
use crate::shadow_tree::component::CoreComponent;

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

    pub fn launch<A: Application>(self, mut app: A) {
        let width = self.width;
        let height = self.height;
        let title = self.title;

        // the root view is a workaround to show excess content
        // and allows the window to shrink below its content
        let root_view = gtk4::ScrolledWindow::new();

        root_view.set_hexpand(true);
        root_view.set_vexpand(true);

        // clone reference to root view
        let cloned_root_view = root_view.clone();

        // create cell to store the window
        let window = Rc::new(RefCell::new(None));
        let cloned_window = window.clone();

        // create and initialise the window when app is active
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

            // store the window in cell
            cloned_window.borrow_mut().replace(window);
        });

        // clone app inner
        let app_inner = self.inner.clone();

        // create new thread for rendering
        std::thread::spawn(move || {
            // indefinite loop for rendering
            loop {
                // call on active
                let mut widget = app.render();

                // render the widget
                let mut elem = widget.render();

                // get the native component from the rendered element
                let mut comp = loop {
                    match elem {
                        // element is not native
                        Err(mut e) => {
                            // keep rendering the element
                            elem = e.render();
                        }
                        // element is native
                        Ok(c) => {
                            // return the native component
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

        // just a dummy context
        let mut ctx = Context::<'static>::dummy();

        // create a new native tree
        let native_tree = NativeTree::get(&mut ctx);

        let mut last_child: Option<gtk4::Widget> = None;

        // force callback every 18ms
        glib::timeout_add_local(Duration::from_millis(20), move || {
            // has commands been made to tree
            let mut commands_recieved = false;

            // try to fetch commands
            while let Ok(commands) = app_inner.command_receiver.try_recv() {
                commands_recieved = true;
                // execute commands
                native_tree.execute_commands(&mut Context::dummy(), commands);
            }

            // only need to check root if any command has been made
            if commands_recieved {
                let window = window.borrow();
                let window = window.as_ref().unwrap();

                // check update
                native_tree.check_update();

                // recalculate layout
                native_tree.compute_layout(
                    &mut Context::dummy(),
                    window.width() as _,
                    window.height() as _,
                );
                // recalculate the style
                native_tree.compute_style();

                // get the root view from native tree
                if let Some(root) = native_tree.get_root_node() {
                    // get the gtk widget
                    let widget = root.widget().as_gtk4_widget();

                    if let Some(w) = &last_child {
                        // set the child if not the same widget
                        if w != widget {
                            //println!("set child");
                            root_view.set_child(Some(widget));
                            last_child = Some(widget.clone());
                        }
                    } else {
                        root_view.set_child(Some(widget));
                        last_child = Some(widget.clone());
                    }
                } else {
                    root_view.set_child_visible(false);
                }
            }

            // indefinite loop
            glib::ControlFlow::Continue
        });

        // run the main loop
        self.app.run();
    }
}
