use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, TextNode};
use crate::style::StyleRef;

pub struct Text {
    /// shadow tree
    text: String,
    style: StyleRef,
}

impl Text {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self {
            text: text.into(),
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

impl NativeElement for Text {
    fn core_component(&mut self) -> crate::shadow_tree::component::CoreComponent {
        CoreComponent::Text(Box::new(TextNode {
            id: None,
            style: self.style.clone(),
            text: self.text.clone(),
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
