use std::sync::Arc;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, ScrollViewNode};
use crate::style::{StyleSheet, DEFAULT_STYLESHEET_ARC};

pub struct ScrollView{
    style: Arc<StyleSheet>,
    child: Option<Box<dyn ElementLike>>,
    rendered_child: Option<CoreComponent>
}

impl ScrollView{
    pub fn new() -> Self{
        Self { 
            style: DEFAULT_STYLESHEET_ARC.clone(), 
            child: None, 
            rendered_child: None 
        }
    }

    pub fn with_child(mut self, child: impl ElementLike) -> Self{
        self.child = Some(Box::new(child));
        return self
    }

    pub fn set_child(&mut self, child: impl ElementLike){
        self.child = Some(Box::new(child));
    }

    pub fn with_style(mut self, style: Arc<StyleSheet>) -> Self{
        self.set_style(style);
        return self;
    }

    pub fn set_style(&mut self, style: Arc<StyleSheet>){
        self.style = style;
    }
}

impl NativeElement for ScrollView{
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::ScrollView(Box::new(ScrollViewNode{
            id: None,
            style: self.style.clone(),
            child: self.rendered_child.take()
        }))
    }
    fn on_state_change(&mut self, _ctx: &crate::Context) {

    }
    fn render(&mut self) {
        if let Some(child) = &mut self.child{
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

            self.rendered_child = Some(comp)
        }
    }
}

impl ElementLike for ScrollView{
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}

