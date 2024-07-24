use std::sync::Arc;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, TextNode};
use crate::style::{StyleSheet, DEFAULT_STYLESHEET_ARC};

pub struct Text {
    /// shadow tree
    text: String,
    style: Arc<StyleSheet>
}

impl Text {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
            style: DEFAULT_STYLESHEET_ARC.clone()
        }
    }

    pub fn with_style(mut self, style: Arc<StyleSheet>) -> Self{
        self.set_style(style);
        return self
    }

    pub fn set_style(&mut self, style: Arc<StyleSheet>){
        self.style = style;
    }
}

impl NativeElement for Text {
    fn on_state_change(&mut self, _ctx: &crate::Context) {
        // do nothing
    }

    fn core_component(&mut self) -> crate::shadow_tree::component::CoreComponent {
        CoreComponent::Text(Box::new(TextNode{
            id: None,
            style: self.style.clone(),
            text: self.text.clone()
        }))
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
