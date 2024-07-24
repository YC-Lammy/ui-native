use std::sync::Arc;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, TextInputNode};
use  crate::style::{StyleSheet, DEFAULT_STYLESHEET_ARC};





pub struct TextInput{
    style: Arc<StyleSheet>,
    bg_text: Option<String>
}

impl TextInput{
    pub fn new() -> Self{
        Self { 
            style: DEFAULT_STYLESHEET_ARC.clone() ,
            bg_text: None
        }
    }

    pub fn with_background_text<S: Into<String>>(mut self, text: S) -> Self{
        self.set_background_text(text);
        return self
    }

    pub fn set_background_text<S: Into<String>>(&mut self, text: S){
        self.bg_text = Some(text.into())
    }

    pub fn with_style(mut self, style: Arc<StyleSheet>) -> Self{
        self.set_style(style);
        return self
    }

    pub fn set_style(&mut self, style: Arc<StyleSheet>){
        self.style = style;
    }
}

impl NativeElement for TextInput{
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::TextInput(Box::new(TextInputNode{
            id: None,
            style: self.style.clone(),
            background_text: None
        }))
    }
    fn on_state_change(&mut self, _ctx: &crate::Context) {
        
    }
    fn render(&mut self) {
        
    }
}

impl ElementLike for TextInput{
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}