use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, TextEditNode};
use crate::style::StyleRef;

pub struct TextEdit {
    style: StyleRef,
}

impl TextEdit {
    pub fn new() -> Self {
        Self {
            style: StyleRef::DEFAULT,
        }
    }

    pub fn with_style<S: Into<StyleRef>>(mut self, style: S) -> Self {
        self.set_style(style);
        return self;
    }

    pub fn set_style<S: Into<StyleRef>>(&mut self, style: S) {
        self.style = style.into();
    }
}

impl NativeElement for TextEdit {
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::TextEdit(Box::new(TextEditNode {
            id: None,
            style: self.style.clone(),
        }))
    }
    fn render(&mut self) {}
}

impl ElementLike for TextEdit {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
