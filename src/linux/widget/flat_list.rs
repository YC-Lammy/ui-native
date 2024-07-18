use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use glib::object::Cast;
use gtk4::prelude::*;

use crate::private::ElementLike;

use super::list_model::{
    GtkNativeListItem,
    GtkNativeListModel
};

pub struct NativeFlatList{
    list: gtk4::ListView
}

impl NativeFlatList{
    pub fn new(
        get_item: Box<dyn Fn(usize) -> Box<dyn Any>>, 
        get_len: Box<dyn Fn() -> usize>,
        render: Box<dyn Fn(Box<dyn Any>) -> Box<dyn ElementLike>>
    ){
        let model = GtkNativeListModel::new(
            Box::new(move |idx|{
                Some(get_item(idx))
            }), 
            get_len
        );

        let factory = gtk4::SignalListItemFactory::new();

        let factory_old_components = Rc::new(RefCell::new(HashMap::new()));

        let factory_old_components_cloned = factory_old_components.clone();

        factory.connect_bind(move |_factory, list_item|{
            if let Some(item) = list_item.item(){
                let mut factory_old_components = factory_old_components_cloned.borrow_mut();

                let item = item.dynamic_cast::<GtkNativeListItem>().expect("expect native item");

                let pos = item.get_position();
                // get the user data from item
                if let Some(data) = item.get_data(){
                    let data = data.downcast().expect("invalid data type");

                    // get the rendering tree
                    let mut tree = render(*data);

                    let mut component = loop{
                        match tree.render(){
                            Ok(comp) => break comp,
                            Err(r) => tree = r,
                        }
                    };

                    let old = factory_old_components.get(&pos);

                    // encode tree into commands
                    let commands = crate::shadow_tree::commit::commit_tree(&mut component, old);

                    // get the native tree
                    let native_tree = crate::native_tree::NativeTree::get();

                    // run the tree
                    native_tree.execute_commands(commands);

                    let comp = native_tree.get_root_node().expect("expecting node");

                    list_item.set_child(
                        Some(comp.component().widget().as_gtk4_widget())
                    );

                    factory_old_components.insert(pos, component);
                }
                
            };
        });

        factory.connect_unbind(move |_factory, list_item|{
            // remove the component to free space
            factory_old_components.borrow_mut().remove(&list_item.position());
        });

        gtk4::ListView::new(
            Some(gtk4::NoSelection::new(Some(model))), 
            Some(factory)
        );
    }

    pub fn set_width(&self, width: i32){
        self.list.set_width_request(width as _);
    }

    pub fn set_height(&self, height: i32){
        self.list.set_height_request(height as _);
    }
}

impl super::NativeElement for NativeFlatList{
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.list.as_ref()
    }
}