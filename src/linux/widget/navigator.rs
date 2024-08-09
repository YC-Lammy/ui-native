use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, TryRecvError};
use gtk4::prelude::*;

use crate::custom::MeasuredSize;
use crate::native_tree::context::Context;
use crate::native_tree::{AvalableSpace, NativeNavigatorImp, NativeStyledElement};
use crate::shadow_tree::component::NavigatorCommand;
use crate::shadow_tree::NodeID;
use crate::style::*;

use super::NativeElement;

struct NavigatorState {
    dropped: bool,
    children: Vec<(String, NodeID)>,
}

pub struct NativeNavigator {
    stack: gtk4::Stack,
    state: Rc<RefCell<NavigatorState>>,
}

impl NativeElement for NativeNavigator {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.stack.as_ref()
    }
}

impl NativeNavigatorImp for NativeNavigator {
    fn new(_ctx: &mut Context, command_recv: Receiver<NavigatorCommand>) -> Self {
        // constuct the state
        let state = Rc::new(RefCell::new(NavigatorState {
            dropped: false,
            children: Vec::new(),
        }));
        let state1 = state.clone();

        let stack = gtk4::Stack::new();
        let stack1 = stack.clone();

        stack.set_overflow(gtk4::Overflow::Hidden);
        stack.set_hhomogeneous(true);
        stack.set_vhomogeneous(true);

        stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);

        // check navigator actions at regular intervals
        glib::timeout_add_local(Duration::from_millis(5), move || {
            let mut state = state1.borrow_mut();

            loop {
                match command_recv.try_recv() {
                    Ok(cmd) => match cmd {
                        NavigatorCommand::Goto(name) => {
                            // find the id of page
                            if let Some(_) = state.children.iter().find(|(n, _p)| n == &name) {
                                // set the page
                                stack1.set_visible_child_name(&name);
                            }
                        }
                        _ => unimplemented!(),
                    },
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        state.dropped = true;
                        return glib::ControlFlow::Break;
                    }
                }
            }

            return glib::ControlFlow::Continue;
        });

        Self {
            stack: stack,
            state,
        }
    }

    fn add_child(&self, _ctx: &mut Context, child: &dyn NativeElement, name: &str, id: NodeID) {
        // add the named page
        let _page = self.stack.add_named(child.as_gtk4_widget(), Some(name));

        // get mutable reference to state
        let mut state = self.state.borrow_mut();

        // push page
        state.children.push((name.to_string(), id));

        // if no child is visible, set as the first child
        if self.stack.visible_child().is_none() {
            if let Some((name, _)) = state.children.first() {
                // set the page
                self.stack
                    .set_visible_child_full(name, gtk4::StackTransitionType::None);
            }
        }
    }

    fn remove_child(&self, _ctx: &mut Context, name: &str) {
        if let Some(w) = self.stack.child_by_name(name) {
            self.stack.remove(&w);

            let mut state = self.state.borrow_mut();

            for (i, (n, _id)) in state.children.iter().enumerate() {
                if &name == n {
                    state.children.remove(i);
                    break;
                }
            }
        }
    }

    fn visible_child(&self, _ctx: &mut Context) -> Option<NodeID> {
        let state = self.state.borrow();

        let name = self.stack.visible_child_name()?;

        if let Some((_, id)) = state.children.iter().find(|(n, _)| n == &name) {
            return Some(*id);
        }

        return None;
    }

    fn should_retain(&self, _ctx: &mut Context) -> bool {
        let state = self.state.borrow();

        return !state.dropped;
    }

    fn layout_child(
        &self,
        _ctx: &mut Context,
        child: &dyn NativeElement,
        _x: f32,
        _y: f32,
        width: f32,
        height: f32,
    ) {
        let w = child.as_gtk4_widget();
        w.set_width_request(width as i32);
        w.set_height_request(height as i32);
    }
}

impl NativeStyledElement for NativeNavigator {
    fn measure(
        &self,
        _ctx: &mut Context,
        known_width: AvalableSpace,
        known_height: AvalableSpace,
    ) -> anyhow::Result<MeasuredSize> {
        // measure width
        let (min_width, natural_width, _, _) = self.stack.measure(
            gtk4::Orientation::Horizontal,
            match known_height {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );
        // measure height
        let (min_height, natural_height, _, _) = self.stack.measure(
            gtk4::Orientation::Vertical,
            match known_width {
                AvalableSpace::AtMost(f) => f as i32,
                AvalableSpace::Exact(f) => f as i32,
                AvalableSpace::Unknown => -1,
            },
        );

        return Ok(MeasuredSize {
            min_width: min_width as f32,
            natural_width: natural_width as f32,
            min_height: min_height as f32,
            natural_height: natural_height as f32,
        });
    }
    fn set_visible(&self, _ctx: &mut Context, visible: bool) {
        self.stack.set_visible(visible)
    }
    fn set_backface_visible(&self, _visible: bool) {}

    fn set_colour(&self, _colour: Colour) {}
    fn set_background_colour(&self, _colour: Colour) {}

    fn set_border_top_width(&self, _width: f32) {}
    fn set_border_bottom_width(&self, _width: f32) {}
    fn set_border_left_width(&self, _width: f32) {}
    fn set_border_right_width(&self, _width: f32) {}

    fn set_border_top_left_radius(&self, _radius: f32) {}
    fn set_border_top_right_radius(&self, _radius: f32) {}
    fn set_border_bottom_left_radius(&self, _radius: f32) {}
    fn set_border_bottom_right_radius(&self, _radius: f32) {}

    fn set_border_top_colour(&self, _colour: Colour) {}
    fn set_border_bottom_colour(&self, _colour: Colour) {}
    fn set_border_left_colour(&self, _colour: Colour) {}
    fn set_border_right_colour(&self, _colour: Colour) {}

    fn set_border_style(&self, _style: BorderStyle) {}

    fn set_opacity(&self, _opacity: f32) {}
    fn set_points_event(&self, _event: PointEvents) {}

    fn set_font_size(&self, _size: f32) {}
    fn set_font_style(&self, _style: FontStyle) {}
    fn set_font_weight(&self, _weight: FontWeight) {}

    fn set_letter_spacing(&self, _spacing: f32) {}
    fn set_line_height(&self, _height: f32) {}

    fn set_text_align(&self, _align: TextAlign) {}
    fn set_text_decloration_line(&self, _line: TextDecorationLine) {}
    fn set_text_decloration_colour(&self, _colour: Colour) {}
    fn set_text_shadow_colour(&self, _colour: Colour) {}
    fn set_text_shadow_radius(&self, _radius: f32) {}
    fn set_text_transform(&self, _transform: TextTransform) {}
}
