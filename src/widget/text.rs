use alloc::{boxed::Box, string::ToString};

use crate::{
    private::{ElementLike, NativeElement},
    shadow_tree::component::{CoreComponent, TextNode},
};

pub struct Text {
    /// shadow tree
    tree_node: TextNode,
}

impl Text {
    pub fn new<T: AsRef<str>>(text: T) -> Self {
        Self {
            tree_node: TextNode {
                text: text.as_ref().to_string(),
                ..Default::default()
            },
        }
    }
}

impl NativeElement for Text {
    fn on_state_change(&mut self, _ctx: &crate::Context) {
        // do nothing
    }

    fn core_component(&self) -> crate::shadow_tree::component::CoreComponent {
        CoreComponent::Text(Box::new(self.tree_node.clone()))
    }

    fn render(&mut self) {}
}

impl ElementLike for Text {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
