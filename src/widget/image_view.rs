use std::sync::Arc;

use crate::image::ImageSource;
use crate::private::{ElementLike, NativeElement};
use crate::shadow_tree::component::{CoreComponent, ImageViewNode};
use crate::style::StyleRef;
use crate::util::Comparable;

pub struct ImageView {
    src: Arc<Comparable<dyn ImageSource>>,
    style: StyleRef,
}

impl ImageView {
    pub fn new<S: ImageSource + 'static>(src: S) -> Self {
        Self {
            src: Arc::new(Comparable {
                hash: Comparable::<dyn ImageSource>::hash(&src),
                data: src,
            }),
            style: StyleRef::DEFAULT,
        }
    }

    pub fn with_source<S: ImageSource + 'static>(mut self, src: S) -> Self {
        self.set_source(src);
        return self;
    }

    pub fn set_source<S: ImageSource + 'static>(&mut self, src: S) {
        self.src = Arc::new(Comparable {
            hash: Comparable::<dyn ImageSource>::hash(&src),
            data: src,
        });
    }

    pub fn with_style<S: Into<StyleRef>>(mut self, style: S) -> Self {
        self.set_style(style);
        return self;
    }

    pub fn set_style<S: Into<StyleRef>>(&mut self, style: S) {
        self.style = style.into();
    }
}

impl NativeElement for ImageView {
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::ImageView(Box::new(ImageViewNode {
            id: None,
            style: self.style.clone(),
            src: self.src.clone(),
        }))
    }
    fn render(&mut self) {}
}

impl ElementLike for ImageView {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
