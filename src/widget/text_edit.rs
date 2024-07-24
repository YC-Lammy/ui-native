use std::sync::Arc;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, TextEditNode};
use crate::style::{StyleSheet, DEFAULT_STYLESHEET_ARC};


pub struct TextEdit{
    style: Arc<StyleSheet>
}

impl TextEdit{
    pub fn new() -> Self{
        Self { style: DEFAULT_STYLESHEET_ARC.clone() }
    }
}

impl NativeElement for TextEdit{
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::TextEdit(Box::new(TextEditNode{
            id: None,
            style: self.style.clone()
        }))
    }
    fn on_state_change(&mut self, _ctx: &crate::Context) {
        
    }
    fn render(&mut self) {
        
    }
}

impl ElementLike for TextEdit{
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}