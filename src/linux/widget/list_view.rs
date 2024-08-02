use std::sync::Arc;

use glib::object::Cast;
use gtk4::prelude::*;

use crate::native_tree::{MeasuredSize, NativeListViewImp, NativeStyledElement};
use crate::style::{
    BorderStyle, Colour, FontStyle, FontWeight, PointEvents, TextAlign, TextDecorationLine,
    TextTransform,
};
use crate::widget::flatlist::{ListViewDataSourceWrapper, ListViewWidgetFactoryWrapper};

use super::list_item_widget::GtkNativeListItemWidget;
use super::list_model::{GtkNativeListItem, GtkNativeListModel};

pub struct NativeListView {
    scroll: gtk4::ScrolledWindow,
    list: gtk4::ListView,
}

impl super::NativeElement for NativeListView {
    fn as_gtk4_widget(&self) -> &gtk4::Widget {
        self.scroll.as_ref()
    }
}

impl NativeListViewImp for NativeListView {
    fn new(
        data: Arc<ListViewDataSourceWrapper>,
        render: Arc<ListViewWidgetFactoryWrapper>,
    ) -> Self {
        // create a list model that wraps around user data source
        let model = GtkNativeListModel::new(data);

        let factory = gtk4::SignalListItemFactory::new();

        factory.connect_setup(move |_factory, list_item| {
            list_item.set_child(Some(&GtkNativeListItemWidget::new()))
        });

        // connect widget binding
        factory.connect_bind(move |_factory, list_item| {
            // get the list item
            if let Some(item) = list_item.item() {
                // cast item as our own item
                let item = item
                    .dynamic_cast::<GtkNativeListItem>()
                    .expect("expect native item");

                // get the position in list
                let pos = item.get_position();

                // get the user data from item
                if let Some(data) = item.get_data() {
                    // render the components using factory
                    let mut tree = render.render_item(pos as _, data);

                    // render the component tree
                    let mut component = loop {
                        match tree.render() {
                            Ok(comp) => break comp,
                            Err(r) => tree = r,
                        }
                    };

                    // get the native tree
                    let native_tree = crate::native_tree::NativeTree::get();

                    // check if list item already has widget
                    if let Some(list_item_widget) = list_item.child() {
                        // cast the widget as our own wrapper
                        let list_item_widget = list_item_widget
                            .dynamic_cast::<GtkNativeListItemWidget>()
                            .expect("expect list item widget");

                        // get the old shadow tree
                        let old_comp = list_item_widget.take_core_component();

                        // encode tree into commands
                        let commands = crate::shadow_tree::commit::commit_tree(
                            &mut component,
                            old_comp.as_ref(),
                        );

                        // run the tree
                        native_tree.execute_commands(commands);

                        // recalculate layout
                        native_tree.compute_layout(
                            list_item_widget.width() as f64,
                            list_item_widget.height() as f64,
                        );
                        // recalculate style
                        native_tree.compute_style();

                        // get the root widget
                        let widget = native_tree.get_root_node().expect("expecting node");

                        // set the child
                        list_item_widget.set_or_replace_child(widget.widget().as_gtk4_widget());
                        list_item_widget.set_core_component(component);
                    } else {
                        // encode tree into commands
                        let commands =
                            crate::shadow_tree::commit::commit_tree(&mut component, None);

                        // run the tree
                        native_tree.execute_commands(commands);

                        // get the root widget
                        let widget = native_tree.get_root_node().expect("expecting node");

                        // create a new widget wrapper
                        let w = GtkNativeListItemWidget::new();

                        w.set_core_component(component);
                        w.set_or_replace_child(widget.widget().as_gtk4_widget());

                        // set the child
                        list_item.set_child(Some(&w));
                    }
                }
            };
        });

        let view = gtk4::ListView::new(Some(gtk4::NoSelection::new(Some(model))), Some(factory));

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_child(Some(&view));
        scroll.set_hscrollbar_policy(gtk4::PolicyType::Never);

        return Self { scroll, list: view };
    }
}

impl NativeStyledElement for NativeListView {
    fn measure(&self, known_width: Option<f32>, known_height: Option<f32>) -> MeasuredSize {
        // measure width
        let (min_width, natural_width, _, _) = self.list.measure(
            gtk4::Orientation::Horizontal,
            known_height.map(|i| i as i32).unwrap_or(-1),
        );
        // measure height
        let (min_height, natural_height, _, _) = self.list.measure(
            gtk4::Orientation::Vertical,
            known_width.map(|i| i as i32).unwrap_or(-1),
        );

        return MeasuredSize {
            min_width: min_width as f32,
            natural_width: natural_width as f32,
            min_height: min_height as f32,
            natural_height: natural_height as f32,
        };
    }
    fn set_visible(&self, visible: bool) {
        self.list.set_visible(visible)
    }
    fn set_backface_visible(&self, _visible: bool) {}
    fn set_colour(&self, _colour: Colour) {}
    fn set_background_colour(&self, _colour: Colour) {}

    fn set_border_top_width(&self, width: f32) {
        self.list.style_context().border().set_top(width as i16);
    }
    fn set_border_bottom_width(&self, width: f32) {
        self.list.style_context().border().set_bottom(width as i16);
    }
    fn set_border_left_width(&self, width: f32) {
        self.list.style_context().border().set_left(width as i16);
    }
    fn set_border_right_width(&self, width: f32) {
        self.list.style_context().border().set_right(width as i16);
    }

    fn set_border_top_left_radius(&self, _radius: f32) {}
    fn set_border_top_right_radius(&self, _radius: f32) {}
    fn set_border_bottom_left_radius(&self, _radius: f32) {}
    fn set_border_bottom_right_radius(&self, _radius: f32) {}

    fn set_border_top_colour(&self, _colour: Colour) {}
    fn set_border_bottom_colour(&self, _colour: Colour) {}
    fn set_border_left_colour(&self, _colour: Colour) {}
    fn set_border_right_colour(&self, _colour: Colour) {}

    fn set_border_style(&self, _style: BorderStyle) {}

    fn set_opacity(&self, opacity: f32) {
        self.list.set_opacity(opacity as f64);
    }
    fn set_points_event(&self, _event: PointEvents) {}

    fn set_font_size(&self, _size: f32) {}
    fn set_font_style(&self, _style: FontStyle) {}
    fn set_font_weight(&self, _weight: FontWeight) {}

    fn set_letter_spacing(&self, _spacing: f32) {}
    fn set_line_height(&self, _height: f32) {}

    fn set_text_align(&self, _align: TextAlign) {}
    fn set_text_decloration_line(&self, _line: TextDecorationLine) {}
    fn set_text_decloration_colour(&self, _colour: Colour) {}
    fn set_text_shadow_colour(&self, _colour: Colour) {}
    fn set_text_shadow_radius(&self, _radius: f32) {}
    fn set_text_transform(&self, _transform: TextTransform) {}
}
