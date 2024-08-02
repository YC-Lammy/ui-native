use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, ScrollViewNode};
use crate::style::StyleRef;

pub struct ScrollView {
    style: StyleRef,
    child: Option<Box<dyn ElementLike>>,
    rendered_child: Option<CoreComponent>,
}

impl ScrollView {
    pub fn new() -> Self {
        Self {
            style: StyleRef::DEFAULT,
            child: None,
            rendered_child: None,
        }
    }

    pub fn with_child(mut self, child: impl ElementLike) -> Self {
        self.child = Some(Box::new(child));
        return self;
    }

    pub fn set_child(&mut self, child: impl ElementLike) {
        self.child = Some(Box::new(child));
    }

    pub fn with_style<S: Into<StyleRef>>(mut self, style: S) -> Self {
        self.set_style(style);
        return self;
    }

    pub fn set_style<S: Into<StyleRef>>(&mut self, style: S) {
        self.style = style.into();
    }
}

impl NativeElement for ScrollView {
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::ScrollView(Box::new(ScrollViewNode {
            id: None,
            style: self.style.clone(),
            child: self.rendered_child.take(),
        }))
    }
    fn render(&mut self) {
        if let Some(child) = &mut self.child {
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

impl ElementLike for ScrollView {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
