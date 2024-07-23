use std::collections::HashMap;
use std::sync::Arc;

use taffy::{Size, TaffyTree};

use crate::custom::NativeCustomElement;
use crate::imp::{NativeButton, NativeElement, NativeFlatList, NativeText, NativeView, NativeImageView, NativeScrollView};
use crate::shadow_tree::{command::Command, NodeID};
use crate::style::StyleSheet;

pub mod traits;

pub use traits::*;

pub enum NativeComponent {
    View(NativeView),
    ImageView(NativeImageView),
    ScrollView(NativeScrollView),
    Button(NativeButton),
    Text(NativeText),
    TextInput(),
    TextEdit(),
    StackNavigator(),

    FlatList(NativeFlatList),
    Custom(Box<dyn NativeCustomElement>),
}

impl NativeComponent {
    pub fn widget(&self) -> &dyn NativeElement {
        match self {
            Self::View(v) => v,
            Self::ImageView(i) => i,
            Self::ScrollView(s) => s,
            Self::Button(b) => b,
            Self::Text(t) => t,
            Self::FlatList(f) => f,
            Self::Custom(c) => c.as_native_element(),
            _ => todo!(),
        }
    }

    pub fn is_scroll_view(&self) -> bool{
        match self{
            Self::ScrollView(_) => true,
            _ => false
        }
    }

    pub fn set_width(&self, width: f32) {
        match self {
            Self::View(v) => v.set_width(width),
            Self::ImageView(i) => i.set_width(width),
            Self::ScrollView(s) => s.set_width(width),
            Self::Button(b) => b.set_width(width),
            Self::Text(t) => t.set_width(width),
            Self::FlatList(f) => f.set_width(width as _),
            Self::Custom(c) => c.set_custom_width(width),
            _ => todo!(),
        }
    }

    pub fn set_height(&self, height: f32) {
        match self {
            Self::View(v) => v.set_height(height as _),
            Self::ImageView(i) => i.set_height(height),
            Self::ScrollView(s) => s.set_height(height),
            Self::Button(b) => b.set_height(height),
            Self::Text(t) => t.set_height(height as _),
            Self::FlatList(f) => f.set_height(height as _),
            Self::Custom(c) => c.set_custom_height(height),
            _ => todo!(),
        }
    }

    pub fn set_child_position(&self, child: &NativeComponent, x: f32, y: f32) {
        match self {
            Self::View(v) => v.set_child_position(child.widget(), x, y),
            Self::ScrollView(_) => {}
            _ => todo!(),
        }
    }

    pub fn set_visible(&self, visible: bool) {
        match self {
            Self::View(v) => v.set_visible(visible),
            Self::Button(b) => b.set_visible(visible),
            Self::Text(t) => t.set_visible(visible),
            _ => todo!(),
        }
    }
}

/// node context is used for temporary storing component for measure function
struct NodeContext{
    id: NodeID,
    comp: Arc<NativeComponent>
}

fn measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    _id: taffy::NodeId,
    node_context: Option<&mut NodeContext>,
    _style: &taffy::Style,
) -> taffy::Size<f32> {
    // return the known dimension
    if let taffy::Size {
        width: Some(width),
        height: Some(height),
    } = known_dimensions
    {
        return taffy::Size { width, height };
    }

    // measure the actual size
    match node_context {
        // should not happen
        None => Size::ZERO,
        Some(ctx) => match ctx.comp.as_ref() {
            // text widget
            NativeComponent::Text(t) => {
                let mut width = t.get_width() as f32;
                let mut height = t.get_height() as f32;

                if let taffy::AvailableSpace::Definite(w) = available_space.width {
                    width = width.min(w);
                }

                if let taffy::AvailableSpace::Definite(h) = available_space.height {
                    height = height.min(h);
                }

                Size {
                    width: width,
                    height: height,
                }
            }
            // text edit
            NativeComponent::TextEdit() => todo!(),
            // image view
            NativeComponent::ImageView(i) => {
                let mut width = i.get_width() as f32;
                let mut height = i.get_height() as f32;

                if let taffy::AvailableSpace::Definite(w) = available_space.width {
                    if width > w{
                        width = w;
                        i.set_width(w);
                    }
                }

                if let taffy::AvailableSpace::Definite(h) = available_space.height {
                    if height > h{
                        height = h;
                        i.set_height(h);
                    }
                }

                Size {
                    width: width,
                    height: height,
                }
            },
            NativeComponent::Custom(c) => {
                if let Some((width, height)) = c.measure_custom_size() {
                    Size { width, height }
                } else {
                    Size::ZERO
                }
            }
            _ => Size::ZERO,
        },
    }
}

pub struct NativeNode {
    parent: Option<NodeID>,
    children: Vec<NodeID>,
    component: Arc<NativeComponent>,
    style: Arc<StyleSheet>
}

impl NativeNode {
    pub fn component(&self) -> &NativeComponent {
        &self.component
    }
}

pub struct NativeTree {
    nodes: HashMap<NodeID, NativeNode>,
    root: Option<NodeID>,
}

unsafe impl Sync for NativeTree {}
unsafe impl Send for NativeTree {}

impl NativeTree {
    /// should only be called from the main thread
    pub(crate) fn get() -> &'static mut Self {
        lazy_static::lazy_static! {
            static ref NATIVE_TREE: NativeTree = NativeTree {
                nodes: HashMap::new(),
                root: None
            };
        };
        unsafe {
            ((&*NATIVE_TREE) as *const NativeTree as *mut NativeTree)
                .as_mut()
                .unwrap()
        }
    }

    pub fn get_root_node(&self) -> Option<Arc<NativeComponent>> {
        let id = self.root?;
        let node = self.nodes.get(&id).expect("invalid node id");
        return Some(node.component.clone());
    }

    /// recompute the layout of the tree
    pub fn compute_layout(&mut self, width: f64, height: f64) {
        let mut stretch = TaffyTree::<NodeContext>::new();

        let root = match self.root {
            Some(r) => r,
            None => return,
        };

        // create the root node and its children
        let root_node = self.create_layout_node(&mut stretch, root);

        // compute the layout
        stretch
            .compute_layout_with_measure(
                root_node,
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(width as _),
                    height: taffy::AvailableSpace::Definite(height as _),
                },
                measure_function,
            )
            .expect("");

        // assign layout to nodes
        self.assign_layout(&stretch, root_node);
    }

    fn create_layout_node(
        &mut self,
        stretch: &mut TaffyTree<NodeContext>,
        node_id: NodeID,
    ) -> taffy::NodeId {
        let node = self.nodes.get(&node_id).expect("invalid node id");

        // get the layout
        let mut style: taffy::Style = node.style.to_taffy_style();

        if node.component.is_scroll_view(){
            style.overflow = taffy::Point{
                x: taffy::Overflow::Scroll,
                y: taffy::Overflow::Scroll
            };
        }

        let comp = node.component.clone();

        // create the parent node
        let id = stretch
            .new_leaf_with_context(style, NodeContext{id: node_id, comp})
            .expect("Stretch");

        let children = node.children.clone();

        // add child to node
        for child_id in children {
            // create the layout
            let child_id = self.create_layout_node(stretch, child_id);
            // add child
            stretch
                .add_child(id, child_id)
                .expect("failed to add child");
        }

        return id;
    }

    fn assign_layout<'a>(
        &mut self,
        stretch: &'a TaffyTree<NodeContext>,
        node: taffy::NodeId,
    ) -> &'a taffy::Layout {

        let context = stretch.get_node_context(node).unwrap();

        let layout = stretch.layout(node).expect("");

        context.comp.set_width(layout.size.width);
        context.comp.set_height(layout.size.height);

        let mut i = 0;
        loop{
            let child = match stretch.child_at_index(node, i){
                Ok(c) => c,
                Err(_) => break
            };

            let child_layout = self.assign_layout(stretch, child);

            // get the child context
            let child_context = stretch.get_node_context(child).unwrap();
            // set the child position
            context.comp.set_child_position(
                &child_context.comp,
                 child_layout.location.x, 
                 child_layout.location.y
                );

            i += 1;
        }
        return layout;
    }

    /// aux function to get known button node
    fn get_button(&self, id: NodeID) -> (&NativeNode, &NativeButton) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::Button(b) = node.component.as_ref() {
                    return (&node, &b);
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    /// aux function to get known view node
    fn get_view(&self, id: NodeID) -> (&NativeNode, &NativeView) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::View(v) = node.component.as_ref() {
                    return (node, &v);
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    fn get_scroll_view(&self, id: NodeID) -> (&NativeNode, &NativeScrollView){
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::ScrollView(v) = node.component.as_ref() {
                    return (&node, &v);
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    /// aux function to get known text node
    fn get_text(&self, id: NodeID) -> (&NativeNode, &NativeText) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::Text(t) = node.component.as_ref() {
                    return (&node, &t);
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    /// aux function to get known custom node
    fn get_custom(&self, id: NodeID) -> (&NativeNode, &dyn NativeCustomElement) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::Custom(c) = node.component.as_ref() {
                    return (&node, c.as_ref());
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    /// runs the command on the native tree
    pub fn execute_commands(&mut self, commands: Vec<Command>) {
        for cmd in commands {
            // println!("{:?}", cmd);
            match cmd {
                Command::MountRoot { node } => {
                    self.root = Some(node);
                }
                Command::RemoveNode { node } => {
                    self.nodes.remove(&node);
                }
                Command::SetStyle { node, style } => {
                    let node = self.nodes.get_mut(&node).unwrap();
                    // set all the fields as updated
                    style.set_owned_as_modified();
                    // set the style
                    node.style = style;
                }
                // button commands
                Command::ButtonCreate { id, style} => {
                    self.nodes.insert(
                        id,
                        NativeNode {
                            parent: None,
                            children: Vec::new(),
                            component: Arc::new(NativeComponent::Button(NativeButton::new())),
                            style
                        },
                    );
                }
                Command::ButtonSetDisabled { id, disabled } => {
                    let (_node, b) = self.get_button(id);
                    b.set_disabled(disabled);
                }
                Command::ButtonSetLabelText { id, label } => {
                    let (_node, b) = self.get_button(id);
                    b.set_label(label)
                }
                Command::ButtonSetOnClick { id, on_click } => {
                    let (_node, b) = self.get_button(id);
                    b.set_on_click(on_click);
                }

                Command::ViewCreate { id, style} => {
                    // create view node
                    self.nodes.insert(
                        id,
                        NativeNode {
                            parent: None,
                            children: Vec::new(),
                            component: Arc::new(NativeComponent::View(NativeView::new())),
                            style
                        },
                    );
                }
                Command::ViewRemoveChild { id, child, index } => {
                    let (_node, view) = self.get_view(id);
                    let elem = self.nodes.get(&child).unwrap();
                    view.remove_child(elem.component.widget());

                    let node = self.nodes.get_mut(&id).expect("invalid node id");
                    node.children.remove(index);

                    let child_node = self.nodes.get_mut(&child).expect("invalid node id");
                    child_node.parent = None;
                }
                Command::ViewSetChild { id, child, index } => {
                    let (_node, view) = self.get_view(id);
                    let child_node = self.nodes.get(&child).unwrap();

                    view.insert_child(index, child_node.component.widget());

                    let node = self.nodes.get_mut(&id).expect("invalid node");

                    if node.children.len() == index {
                        node.children.push(child);
                    } else {
                        node.children[index] = child;
                    }

                    let child_node = self.nodes.get_mut(&child).expect("invalid node id");
                    child_node.parent = Some(id);
                }

                Command::ImageViewCreate { id, style} => {
                    self.nodes.insert(
                        id, 
                        NativeNode{
                            parent: None,
                            children: Vec::new(),
                            component: Arc::new(NativeComponent::ImageView(NativeImageView::new())),
                            style
                        }
                    );
                },

                Command::ScrollViewCreate { id, style} => {
                    self.nodes.insert(
                        id, 
                        NativeNode { 
                            parent: None,
                            children: Vec::new(),
                            component: Arc::new(NativeComponent::ScrollView(NativeScrollView::new())), 
                            style
                        }
                    );
                },
                Command::ScrollViewRemoveChild { id } => {
                    let (_node, view) = self.get_scroll_view(id);

                    view.remove_child();

                    let node = self.nodes.get_mut(&id).expect("invalid node id");
                    node.children.clear();

                    // get the mutable child node
                    let child_node = self.nodes.get_mut(&id).expect("invalid node id");
                    child_node.parent = None;
                }
                Command::ScrollViewSetChild { id, child } => {
                    let (_node, view) = self.get_scroll_view(id);

                    // get the child node
                    let child_node = self.nodes.get(&id).expect("invalid node id");

                    // set the child into view
                    view.set_child(child_node.component().widget());
                    
                    // get the mutable child node
                    let child_node = self.nodes.get_mut(&id).expect("invalid node id");
                    // set the parent as scroll view
                    child_node.parent = Some(id);
                    
                    // get the mutable node of scroll view
                    let node = self.nodes.get_mut(&id).expect("invalid node id");
                    
                    // register child node
                    match node.children.get_mut(0){
                        Some(c) => *c = child,
                        None => node.children.push(child.clone()),
                    }
                }

                Command::TextCreate { id, style, text } => {
                    // create text node
                    self.nodes.insert(
                        id,
                        NativeNode {
                            parent: None,
                            children: Vec::new(),
                            component: Arc::new(NativeComponent::Text(NativeText::new(&text))),
                            style
                        },
                    );
                }
                Command::TextSetText { id, text } => {
                    let (_node, t) = self.get_text(id);
                    t.set_text(&text);
                }
                Command::TextSetFont { id, font } => {
                    let (_node, t) = self.get_text(id);
                    t.set_font(&font);
                }

                Command::CustomCreate { id, style, build_fn } => {
                    // create custom element
                    let custom = build_fn();
                    // create custom node
                    self.nodes.insert(
                        id,
                        NativeNode {
                            parent: None,
                            children: Vec::new(),
                            component: Arc::new(NativeComponent::Custom(custom)),
                            style
                        },
                    );
                }
                Command::CustomCommitChanges { id, changes } => {
                    let (_node, custom) = self.get_custom(id);
                    custom.commit_custom_changes(changes);
                }

                _ => todo!(),
            }
        }
    }
}
