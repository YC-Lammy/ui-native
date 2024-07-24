use std::any::Any;
use std::sync::Arc;

use crate::private::ElementLike;



pub trait ListViewWidgetFactory: Sync + Send{
    type Item;
    fn render_item(&self, index: usize, data: Self::Item) -> Box<dyn ElementLike>;
}

pub trait ListViewDataSource: Sync + Send{
    type Item;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<Self::Item>;
}

pub(crate) struct ListViewDataSourceWrapper{
    get_len: Arc<dyn Fn() -> usize + Send + Sync>,
    get_item: Arc<dyn Fn(usize) -> Option<Box<dyn Any>> + Send + Sync>,
}

impl ListViewDataSourceWrapper{
    pub fn new<D: ListViewDataSource + 'static>(data: D) -> Self{
        let d1 = Arc::new(data);
        let d2 = d1.clone();

        Self {
            get_len: Arc::new(move ||{
                d1.len()
            }),
            get_item: Arc::new(move |index|{
                match d2.get(index){
                    Some(v) => Some(Box::new(v)),
                    None => None
                }
            })
        }
    }

    pub fn len(&self) -> usize{
        return (self.get_len)()
    }

    pub fn get(&self, index: usize) -> Option<Box<dyn Any>>{
        return (self.get_item)(index)
    }
}

pub(crate) struct ListViewWidgetFactoryWrapper{
    render_item: Arc<dyn Fn(usize, Box<dyn Any>) -> Box<dyn ElementLike> + Sync + Send>
}

impl ListViewWidgetFactoryWrapper{
    pub fn new<F: ListViewWidgetFactory + 'static>(factory: F) -> Self{
        return Self{
            render_item: Arc::new(move |index, item|{
                let item = item.downcast::<F::Item>().unwrap();
                let data = *item;
                factory.render_item(index, data)
            })
        }
    }

    pub fn render_item(&self, index: usize, item: Box<dyn Any>) -> Box<dyn ElementLike>{
        return (self.render_item)(index, item)
    }
}