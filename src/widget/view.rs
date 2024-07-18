use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, ViewNode};
use crate::state::StateLike;
use crate::Context;
use crate::State;

pub struct View {
    /// shadow tree
    tree_node: ViewNode,
    /// attached states
    states: Vec<Box<dyn StateLike>>,
    children: Vec<Box<dyn ElementLike>>,
}

impl View {
    pub fn new() -> Self {
        Self {
            tree_node: ViewNode::default(),
            states: Vec::new(),
            children: Vec::new(),
        }
    }
    pub fn with_child<T>(mut self, child: T) -> Self
    where
        T: ElementLike,
    {
        self.add_child(child);
        return self;
    }
    pub fn add_child<T>(&mut self, child: T)
    where
        T: ElementLike + 'static,
    {
        let child = Box::new(child);
        self.add_child_dyn(child)
    }

    fn add_child_dyn(&mut self, child: Box<dyn ElementLike>) {
        self.children.push(child)
    }

    pub fn attach_state<T: 'static>(&mut self, state: State<T>) {
        self.attach_state_dyn(Box::new(state))
    }

    fn attach_state_dyn(&mut self, state: Box<dyn StateLike>) {
        self.states.push(state);
    }

    pub fn remove_state<T: 'static>(&mut self, state: State<T>) -> bool {
        let old_len = self.states.len();
        // remove all same state
        self.states
            .retain(|s| !s.is_same_state(unsafe { core::mem::transmute(&state) }));
        return self.states.len() != old_len;
    }
}

impl NativeElement for View {
    fn on_state_change(&mut self, ctx: &Context) {
        for child in &mut self.children {
            child.on_state_change(ctx);
        }
    }

    fn core_component(&self) -> CoreComponent {
        CoreComponent::View(Box::new(self.tree_node.clone()))
    }

    fn render(&mut self) {
        for child in &mut self.children {
            // render the child
            let mut elem = child.render();
            // keep rendering until core component is reached
            let comp = loop {
                match elem {
                    Ok(c) => break c,
                    Err(mut e) => {
                        elem = e.render();
                    }
                }
            };

            self.tree_node.children.push(comp);
        }
    }
}

impl ElementLike for View {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}

pub struct ViewBuilder {
    children: Vec<Box<dyn ElementLike>>,
    states: Vec<Box<dyn StateLike>>,
}

impl ViewBuilder {
    pub fn with_child<T: ElementLike>(&mut self, child: T) -> &mut Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn with_state<T: 'static>(&mut self, state: State<T>) -> &mut Self {
        self.states.push(Box::new(state));
        self
    }
    pub fn build(self) -> View {
        let mut view = View::new();

        for child in self.children {
            view.add_child_dyn(child);
        }

        for state in self.states {
            view.attach_state_dyn(state);
        }

        return view;
    }
}
