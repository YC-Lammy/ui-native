use std::sync::Arc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, ViewNode};
use crate::style::{StyleSheet, DEFAULT_STYLESHEET_ARC};
use crate::Context;

pub struct View {
    style: Arc<StyleSheet>,
    children: Vec<Box<dyn ElementLike>>,
    rendered_children: Vec<CoreComponent>
}

impl View {
    pub fn new() -> Self {
        Self {
            style: DEFAULT_STYLESHEET_ARC.clone(),
            children: Vec::new(),
            rendered_children: Vec::new()
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

    pub fn with_style(mut self, style: Arc<StyleSheet>) -> Self{
        self.set_style(style);
        return self
    }

    pub fn set_style(&mut self, style: Arc<StyleSheet>){
        self.style = style;
    }
}

impl NativeElement for View {
    fn on_state_change(&mut self, ctx: &Context) {
        for child in &mut self.children {
            child.on_state_change(ctx);
        }
    }

    fn core_component(&mut self) -> CoreComponent {
        let children = core::mem::replace(&mut self.rendered_children, Vec::new());

        CoreComponent::View(Box::new(
            ViewNode{
                id: None,
                style: self.style.clone(),
                children: children
            }
        ))
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

            self.rendered_children.push(comp);
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
