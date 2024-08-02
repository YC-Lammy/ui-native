use std::any::Any;
use std::sync::Arc;

use crate::{
    private::{ElementLike, NativeElement},
    shadow_tree::component::{CoreComponent, ListViewNode},
    style::StyleRef,
};

pub trait ListViewDataSource: Sync + Send {
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<Self::Item>;
}

#[derive(Clone)]
pub(crate) struct ListViewDataSourceWrapper {
    get_len: Arc<dyn Fn() -> usize + Send + Sync>,
    get_item: Arc<dyn Fn(usize) -> Option<Box<dyn Any>> + Send + Sync>,
}

impl ListViewDataSourceWrapper {
    pub fn new<D: ListViewDataSource + 'static>(data: D) -> Self {
        let d1 = Arc::new(data);
        let d2 = d1.clone();

        Self {
            get_len: Arc::new(move || d1.len()),
            get_item: Arc::new(move |index| match d2.get(index) {
                Some(v) => Some(Box::new(v)),
                None => None,
            }),
        }
    }

    pub fn len(&self) -> usize {
        return (self.get_len)();
    }

    pub fn get(&self, index: usize) -> Option<Box<dyn Any>> {
        return (self.get_item)(index);
    }
}

pub trait ListViewWidgetFactory: Sync + Send {
    type Item;
    fn render_item(&self, index: usize, data: Self::Item) -> crate::ElementLike;
}

impl<I, E> ListViewWidgetFactory for dyn Fn(I) -> E + Send + Sync
where
    E: ElementLike,
{
    type Item = I;
    fn render_item(&self, _index: usize, data: Self::Item) -> Box<dyn ElementLike> {
        Box::new((self)(data))
    }
}

impl<I, E> ListViewWidgetFactory for dyn Fn(usize, I) -> E + Send + Sync
where
    E: ElementLike,
{
    type Item = I;
    fn render_item(&self, index: usize, data: Self::Item) -> Box<dyn ElementLike> {
        Box::new((self)(index, data))
    }
}

#[derive(Clone)]
pub(crate) struct ListViewWidgetFactoryWrapper {
    render_item: Arc<dyn Fn(usize, Box<dyn Any>) -> Box<dyn ElementLike> + Sync + Send>,
}

impl ListViewWidgetFactoryWrapper {
    pub fn new<F: ListViewWidgetFactory + 'static>(factory: F) -> Self {
        return Self {
            render_item: Arc::new(move |index, item| {
                let item = item.downcast::<F::Item>().unwrap();
                let data = *item;
                factory.render_item(index, data)
            }),
        };
    }

    pub fn render_item(&self, index: usize, item: Box<dyn Any>) -> Box<dyn ElementLike> {
        return (self.render_item)(index, item);
    }
}

pub struct ListView {
    style: StyleRef,

    data: Arc<ListViewDataSourceWrapper>,
    factory: Arc<ListViewWidgetFactoryWrapper>,
}

impl ListView {
    pub fn new<D, F>(data: D, factory: F) -> Self
    where
        D: ListViewDataSource + 'static,
        F: ListViewWidgetFactory + 'static,
    {
        Self {
            style: StyleRef::DEFAULT,
            data: Arc::new(ListViewDataSourceWrapper::new(data)),
            factory: Arc::new(ListViewWidgetFactoryWrapper::new(factory)),
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

impl NativeElement for ListView {
    fn core_component(&mut self) -> CoreComponent {
        CoreComponent::ListView(Box::new(ListViewNode {
            id: None,
            style: self.style.clone(),
            data: self.data.clone(),
            factory: self.factory.clone(),
        }))
    }
    fn render(&mut self) {}
}

impl ElementLike for ListView {
    fn as_native(&mut self) -> Option<&mut dyn NativeElement> {
        Some(self)
    }
    fn as_element(&mut self) -> Option<&mut dyn crate::Element> {
        None
    }
}
