use std::any::Any;

use glib::subclass::types::ObjectSubclassExt;

mod imp_model {
    use core::any::Any;
    use std::cell::RefCell;

    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use glib::types::StaticType;
    use gtk4::{gio::ListModel, subclass::prelude::ListModelImpl};

    #[derive(Default)]
    pub struct GtkNativeListModel {
        pub(super) get_item: RefCell<Option<Box<dyn Fn(usize) -> Option<Box<dyn Any>>>>>,
        pub(super) get_len: RefCell<Option<Box<dyn Fn() -> usize>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkNativeListModel {
        const NAME: &'static str = "GtkNativeListModel";
        type Type = super::GtkNativeListModel;
        type Interfaces = (ListModel,);
    }

    impl ObjectImpl for GtkNativeListModel {}

    impl ListModelImpl for GtkNativeListModel {
        fn item_type(&self) -> glib::Type {
            super::GtkNativeListItem::static_type()
        }
        fn item(&self, position: u32) -> Option<glib::Object> {
            let inner = self.get_item.borrow();

            let value = match inner.as_ref() {
                Some(g) => g(position as usize)?,
                None => return None,
            };

            return Some(super::GtkNativeListItem::new(position, value).into());
        }
        fn n_items(&self) -> u32 {
            let g = self.get_len.borrow();
            match g.as_ref() {
                Some(l) => l() as u32,
                None => 0,
            }
        }
    }
}

glib::wrapper! {
    pub struct GtkNativeListModel(ObjectSubclass<imp_model::GtkNativeListModel>)
    @implements gtk4::gio::ListModel;
}

impl GtkNativeListModel {
    pub fn new(
        get_item: Box<dyn Fn(usize) -> Option<Box<dyn Any>>>,
        get_len: Box<dyn Fn() -> usize>,
    ) -> Self {
        let obj = glib::Object::new();

        let model = imp_model::GtkNativeListModel::from_obj(&obj);
        model.get_item.borrow_mut().replace(get_item);
        model.get_len.borrow_mut().replace(get_len);

        return obj;
    }
}

mod imp_item {
    use std::any::Any;
    use std::cell::RefCell;

    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};

    #[derive(Default)]
    pub struct GtkNativeListItem {
        pub(super) data: RefCell<Option<Box<dyn Any>>>,
        pub(super) index: RefCell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkNativeListItem {
        const NAME: &'static str = "GtkNativeListItem";
        type Type = super::GtkNativeListItem;
    }

    impl ObjectImpl for GtkNativeListItem {}

    impl GtkNativeListItem{
        pub fn get(&self) -> Option<Box<dyn Any>>{
            self.data.replace(None)
        }

        pub fn get_pos(&self) -> u32{
            self.index.clone().into_inner()
        }
    }
}

glib::wrapper! {
    pub struct GtkNativeListItem(ObjectSubclass<imp_item::GtkNativeListItem>);
}
impl GtkNativeListItem {
    pub fn new(index: u32, value: Box<dyn Any>) -> Self {
        let obj = glib::Object::new();

        let item = imp_item::GtkNativeListItem::from_obj(&obj);
        item.data.borrow_mut().replace(value);

        item.index.replace(index);

        return obj;
    }

    pub fn get_data(&self) -> Option<Box<dyn Any>>{
        let item = imp_item::GtkNativeListItem::from_obj(self);
        item.get()
    }

    pub fn get_position(&self) -> u32{
        let item = imp_item::GtkNativeListItem::from_obj(self);
        item.get_pos()
    }
}
