/*
   Work around for keeping record of the shadow tree in ListView.
*/

use glib::subclass::types::FromObject;
use gtk4::prelude::*;

use crate::shadow_tree::component::CoreComponent;

mod imp {
    use std::cell::RefCell;

    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::subclass::{box_::BoxImpl, widget::WidgetImpl};

    use crate::shadow_tree::component::CoreComponent;

    #[derive(Default)]
    pub struct GtkNativeListItemWidget {
        pub shadow_tree: RefCell<Option<CoreComponent>>,
        pub child: RefCell<Option<gtk4::Widget>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GtkNativeListItemWidget {
        const NAME: &'static str = "GtkNativeListItemWidget";
        type Type = super::GtkNativeListItemWidget;
        type Interfaces = ();
        type ParentType = gtk4::Box;
    }

    impl ObjectImpl for GtkNativeListItemWidget {}

    impl WidgetImpl for GtkNativeListItemWidget {}
    impl BoxImpl for GtkNativeListItemWidget {}
}

glib::wrapper! {
    pub struct GtkNativeListItemWidget(ObjectSubclass<imp::GtkNativeListItemWidget>)
    @extends gtk4::Box, gtk4::Widget;
}

impl GtkNativeListItemWidget {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();

        obj.set_hexpand(true);

        return obj;
    }

    pub fn take_core_component(&self) -> Option<CoreComponent> {
        let w = imp::GtkNativeListItemWidget::from_object(self);
        w.shadow_tree.borrow_mut().take()
    }

    pub fn set_core_component(&self, comp: CoreComponent) {
        let w = imp::GtkNativeListItemWidget::from_object(self);
        w.shadow_tree.borrow_mut().replace(comp);
    }

    pub fn set_or_replace_child(&self, other: &gtk4::Widget) {
        let w = imp::GtkNativeListItemWidget::from_object(self);

        let mut lock = w.child.borrow_mut();

        if lock.is_none() {
            lock.replace(other.clone());
            self.append(other);
            return;
        }

        let c = lock.as_ref().unwrap();

        // if both widget is the same, return immediately
        if c == other {
            return;
        }

        // remove the old widget
        self.remove(c);
        // replace the widget reference
        lock.replace(other.clone());

        // drop borrow lock
        drop(lock);

        // set child as the new widget
        self.append(other);
    }
}
