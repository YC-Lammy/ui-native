use crossbeam_channel::{Receiver, Sender};

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, NavigatorCommand, StackNavigaterNode};
use crate::shadow_tree::NodeID;
use crate::style::StyleRef;

pub enum StackNavigatorTransition{
    None,
    Fade,
    Slide
}

#[derive(Debug, Clone)]
pub struct StackNavigator {
    /// node id of the corresponding navigator widget
    id: NodeID,
    command_sender: Sender<NavigatorCommand>,
    command_recv: Receiver<NavigatorCommand>,
}

impl StackNavigator {
    pub fn new() -> Self {
        let (rx, tx) = crossbeam_channel::unbounded();

        Self {
            id: NodeID::new_unique(),
            command_sender: rx,
            command_recv: tx,
        }
    }

    /// should be called during `render` to construct a widget
    pub fn navigator(&self) -> StackNavigatorElement {
        return StackNavigatorElement {
            id: self.id,
            style: StyleRef::DEFAULT,
            command_sender: self.command_sender.clone(),
            command_recv: self.command_recv.clone(),
            children: Vec::new(),

            rendered_children: Vec::new(),
            child_names: Vec::new(),
        };
    }

    pub fn push(&self, name: &str) {
        let _ = self
            .command_sender
            .send(NavigatorCommand::Push(name.to_string()));
    }

    /// go to the page in index, noop if index is not valid
    pub fn goto(&self, name: &str) {
        let _ = self
            .command_sender
            .send(NavigatorCommand::Goto(name.to_string()));
    }

    /// goback to the last page, noop if cannot go back
    pub fn goback(&self) {
        let _ = self.command_sender.send(NavigatorCommand::Goback);
    }
}

impl Default for StackNavigator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StackNavigatorElement {
    id: NodeID,
    style: StyleRef,
    command_sender: Sender<NavigatorCommand>,
    command_recv: Receiver<NavigatorCommand>,
    children: Vec<(String, Box<dyn ElementLike>)>,

    rendered_children: Vec<CoreComponent>,
    child_names: Vec<String>,
}

impl StackNavigatorElement {
    pub fn with_page<F, T>(mut self, name: &str, component: F) -> Self
    where
        F: Fn(StackNavigator) -> T,
        T: ElementLike,
    {
        let component = component(StackNavigator {
            id: self.id,
            command_sender: self.command_sender.clone(),
            command_recv: self.command_recv.clone(),
        });

        self.children.push((name.to_string(), Box::new(component)));
        return self;
    }

    pub fn add_page<F, T>(&mut self, name: &str, component: F)
    where
        F: Fn(StackNavigator) -> T,
        T: ElementLike,
    {
        let component = component(StackNavigator {
            id: self.id,
            command_sender: self.command_sender.clone(),
            command_recv: self.command_recv.clone(),
        });

        self.children.push((name.to_string(), Box::new(component)));
    }

    pub fn with_style<S: Into<StyleRef>>(mut self, style: S) -> Self {
        self.set_style(style);
        return self;
    }

    pub fn set_style<S: Into<StyleRef>>(&mut self, style: S) {
        self.style = style.into();
    }
}

impl NativeElement for StackNavigatorElement {
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::StackNavigator(Box::new(StackNavigaterNode {
            id: self.id,
            style: self.style.clone(),
            command_reciever: self.command_recv.clone(),
            children: core::mem::replace(&mut self.rendered_children, Vec::new()),
            child_names: core::mem::replace(&mut self.child_names, Vec::new()),
        }))
    }
    fn render(&mut self) {
        // render all the children
        for (name, child) in &mut self.children {
            // render the child
            let mut elem = child.render();

            // keep rendering until a core component is reached
            let component = loop {
                match elem {
                    Err(mut e) => {
                        elem = e.render();
                    }
                    Ok(c) => break c,
                }
            };

            // push child
            self.rendered_children.push(component);
            self.child_names.push(name.clone());
        }
    }
}

impl ElementLike for StackNavigatorElement {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
