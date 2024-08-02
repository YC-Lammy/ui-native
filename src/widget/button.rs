use std::sync::Arc;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{ButtonNode, CoreComponent};

pub struct Button {
    tree_node: ButtonNode,
}

impl Button {
    pub fn new() -> Self {
        Self {
            tree_node: Default::default(),
        }
    }

    pub fn with_label<S: Into<String>>(mut self, title: S) -> Self {
        self.tree_node.title = title.into();
        return self;
    }

    pub fn set_label<S: Into<String>>(&mut self, label: S) {
        self.tree_node.title = label.into();
    }

    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.tree_node.disabled = disabled;
        return self;
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.tree_node.on_click = Some(Arc::new(callback));
        return self;
    }

    pub fn set_on_click<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.tree_node.on_click = Some(Arc::new(callback))
    }
}

impl NativeElement for Button {
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::Button(Box::new(self.tree_node.clone()))
    }
    fn render(&mut self) {
        // do nothing
    }
}

impl ElementLike for Button {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
