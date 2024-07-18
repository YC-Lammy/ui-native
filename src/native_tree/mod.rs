use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use taffy::{Size, TaffyTree};

use crate::custom::NativeCustomElement;
use crate::imp::{NativeElement, NativeButton, NativeText, NativeView, NativeFlatList};
use crate::shadow_tree::{command::Command, NodeID};
use crate::style::StyleSheetID;

pub mod traits;

pub use traits::*;

pub enum NativeComponent {
    View(NativeView),
    ImageView(),
    ScrollView(),
    Button(NativeButton),
    Text(NativeText),
    TextInput(),
    TextEdit(),
    StackNavigator(),

    FlatList(NativeFlatList),
    Custom(Box<dyn NativeCustomElement>)
}

impl NativeComponent {
    pub fn widget(&self) -> &dyn NativeElement {
        match self {
            Self::View(v) => v,
            Self::Text(t) => t,
            Self::FlatList(f) => f,
            Self::Custom(c) => c.as_native_element(),
            _ => todo!(),
        }
    }

    pub fn set_width(&self, width: f32) {
        match self {
            Self::View(v) => v.set_width(width as _),
            Self::Button(b) => b.set_width(width),
            Self::Text(t) => t.set_width(width as _),
            Self::FlatList(f) => f.set_width(width as _),
            Self::Custom(c) => c.set_custom_width(width),
            _ => todo!(),
        }
    }

    pub fn set_height(&self, height: f32) {
        match self {
            Self::View(v) => v.set_height(height as _),
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
            _ => todo!(),
        }
    }

    pub fn set_visible(&self, visible: bool) {

    }
}

/// node context is used for temporary storing component for measure function
struct NodeContext(Arc<NativeComponent>);

fn measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    _id: taffy::NodeId,
    node_context: Option<&mut NodeContext>,
    _style: &taffy::Style,
) -> taffy::Size<f32>{
    // return the known dimension
    if let taffy::Size { width: Some(width), height: Some(height) } = known_dimensions {
        return taffy::Size { width, height };
    }

    // measure the actual size
    match node_context{
        // should not happen
        None => Size::ZERO,
        Some(ctx) => match ctx.0.as_ref(){
            // text widget
            NativeComponent::Text(t) => {
                let mut width = t.get_width() as f32;
                let mut height = t.get_height() as f32;

                if let taffy::AvailableSpace::Definite(w) = available_space.width{
                    width = width.min(w);
                }

                if let taffy::AvailableSpace::Definite(h) = available_space.height{
                    height = height.min(h);
                }

                Size { 
                    width: width, 
                    height: height
                }
            },
            // text edit
            NativeComponent::TextEdit() => todo!(),
            // image view
            NativeComponent::ImageView() => todo!(),
            NativeComponent::Custom(c) => {
                if let Some((width, height)) = c.measure_custom_size(){
                    Size{
                        width, height
                    }
                } else{
                    Size::ZERO
                }
            }
            _ => Size::ZERO,
        }
    }
}

pub struct NativeNode {
    component: Arc<NativeComponent>,
    children: RefCell<Vec<Arc<NativeNode>>>,

    tmp_node: RefCell<taffy::NodeId>,
    style: StyleSheetID,
}

impl NativeNode {
    pub fn component(&self) -> &NativeComponent {
        &self.component
    }
}

pub struct NativeTree {
    nodes: HashMap<NodeID, Arc<NativeNode>>,
    root: Option<NodeID>
}

unsafe impl Sync for NativeTree{}
unsafe impl Send for NativeTree{}

impl NativeTree {
    /// should only be called from the main thread
    pub(crate) fn get() -> &'static mut Self {
        lazy_static::lazy_static!{
            static ref NATIVE_TREE: NativeTree = NativeTree {
                nodes: HashMap::new(),
                root: None
            };
        };
        unsafe{
            ((&*NATIVE_TREE) as *const NativeTree as *mut NativeTree).as_mut().unwrap()
        }
    }

    pub fn get_root_node(&self) -> Option<Arc<NativeNode>> {
        let id = self.root?;
        let node = self.nodes.get(&id).expect("invalid node id");
        return Some(node.clone())
    }

    /// recompute the layout of the tree
    pub fn compute_layout(&mut self, width: f64, height: f64) {
        let mut stretch = TaffyTree::<NodeContext>::new();

        let root = match self.get_root_node() {
            Some(r) => r,
            None => return,
        };

        // create the root node and its children
        let root_node = self.create_layout_node(&mut stretch, &root);

        // compute the layout
        stretch
            .compute_layout_with_measure(
                root_node,
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(width as _),
                    height: taffy::AvailableSpace::Definite(height as _),
                },
                measure_function
            )
            .expect("");

        // assign layout to nodes
        self.assign_layout(&stretch, &root);
    }

    fn create_layout_node(
        &mut self,
        stretch: &mut TaffyTree<NodeContext>,
        node: &NativeNode,
    ) -> taffy::NodeId {
        // get the layout
        let layout = Default::default();

        // create the parent node
        let id = stretch.new_leaf_with_context(
            layout,
            NodeContext(node.component.clone())
        ).expect("Stretch");

        *node.tmp_node.borrow_mut() = id;
        
        // add child to node
        for child in node.children.borrow().iter() {
            let child_id = self.create_layout_node(stretch, &child);
            stretch.add_child(id, child_id).expect("failed to add child");
        }

        return id
    }

    fn assign_layout<'a>(
        &mut self,
        stretch: &'a TaffyTree<NodeContext>,
        node: &NativeNode,
    ) -> &'a taffy::Layout {
        let layout = stretch.layout(*node.tmp_node.borrow()).expect("");

        node.component.set_width(layout.size.width);
        node.component.set_height(layout.size.height);

        for child in node.children.borrow().iter() {
            let child_layout = self.assign_layout(stretch, &child);
            node.component.set_child_position(
                &child.component,
                child_layout.location.x - layout.location.x,
                child_layout.location.y - layout.location.y,
            );
        }
        return layout;
    }

    /// aux function to get known button node
    fn get_button(&self, id: NodeID) -> (&NativeNode, &NativeButton){
        match self.nodes.get(&id){
            Some(node) => {
                if let NativeComponent::Button(b) = node.component.as_ref(){
                    return (&node, &b)
                } else{
                    unreachable!()
                }
            }
            None => unreachable!()
        }
    }

    /// aux function to get known view node
    fn get_view(&self, id: NodeID) -> (&NativeNode, &NativeView){
        match self.nodes.get(&id){
            Some(node) => {
                if let NativeComponent::View(v) = node.component.as_ref(){
                    return (&node, &v)
                } else{
                    unreachable!()
                }
            }
            None => unreachable!()
        }
    }

    /// aux function to get known text node
    fn get_text(&self, id: NodeID) -> (&NativeNode, &NativeText){
        match self.nodes.get(&id){
            Some(node) => {
                if let NativeComponent::Text(t) = node.component.as_ref(){
                    return (&node, &t)
                } else{
                    unreachable!()
                }
            }
            None => unreachable!()
        }
    }

    /// aux function to get known custom node
    fn get_custom(&self, id: NodeID) -> (&NativeNode, &dyn NativeCustomElement){
        match self.nodes.get(&id){
            Some(node) => {
                if let NativeComponent::Custom(c) = node.component.as_ref(){
                    return (&node, c.as_ref())
                } else{
                    unreachable!()
                }
            }
            None => unreachable!()
        }
    }

    /// runs the command on the native tree
    pub fn execute_commands(&mut self, commands: Vec<Command>) {
        for cmd in commands{
            // println!("{:?}", cmd);
            match cmd{
                Command::MountRoot { node } => {
                    self.root = Some(node);
                }
                Command::RemoveNode { node } => {
                    self.nodes.remove(&node);
                }
                // button commands
                Command::ButtonCreate { 
                    id, 
                    style,
                } => {
                    self.nodes.insert(
                        id, 
                        Arc::new(NativeNode{
                            component: Arc::new(
                                NativeComponent::Button(
                                    NativeButton::new()
                                )
                            ),
                            children: RefCell::new(Vec::new()),
                            tmp_node: RefCell::new(taffy::NodeId::new(0)),
                            style: style,
                        })
                    );
                },
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

                Command::ViewCreate { id, style } => {
                    // create view node
                    self.nodes.insert(
                        id, 
                        Arc::new(NativeNode{
                            component: Arc::new(
                                NativeComponent::View(NativeView::new())
                            ),
                            children: RefCell::new(Vec::new()),
                            tmp_node: RefCell::new(taffy::NodeId::new(0)),
                            style: style
                        })
                    );
                }
                Command::ViewRemoveChild { id, child, index } => {
                    let(node, view) = self.get_view(id);
                    let elem =  self.nodes.get(&child).unwrap();
                    view.remove_child(
                        elem.component.widget()
                    );
                    node.children.borrow_mut().remove(index);
                }
                Command::ViewSetChild { id, child, index } => {
                    let (node, view) = self.get_view(id);
                    let child =  self.nodes.get(&child).unwrap();

                    view.insert_child(index, child.component.widget());
                    let mut children = node.children.borrow_mut();

                    if children.len() == index{
                        children.push(child.clone());
                    } else{
                        children[index] = child.clone();
                    }
                }

                Command::TextCreate { id, style, text } => {
                    // create text node
                    self.nodes.insert(
                        id, 
                        Arc::new(NativeNode{
                            component: Arc::new(
                                NativeComponent::Text(NativeText::new(&text))
                            ),
                            children: RefCell::new(Vec::new()),
                            tmp_node: RefCell::new(taffy::NodeId::new(0)),
                            style: style
                        })
                    );
                },
                Command::TextSetText { id, text } => {
                    let (_node, t) = self.get_text(id);
                    t.set_text(&text);
                },
                Command::TextSetFont { id, font } => {
                    let (_node, t) = self.get_text(id);
                    t.set_font(&font);
                }

                Command::CustomCreate { id, build_fn } => {
                    // create custom element
                    let custom = build_fn();
                    // create custom node
                    self.nodes.insert(
                        id, 
                        Arc::new(NativeNode{
                            component: Arc::new(
                                NativeComponent::Custom(custom)
                            ),
                            children: RefCell::new(Vec::new()),
                            tmp_node: RefCell::new(taffy::NodeId::new(0)),
                            style: StyleSheetID::default()
                        })
                    );
                },
                Command::CustomCommitChanges { id, changes } => {
                    let (_node, custom) = self.get_custom(id);
                    custom.commit_custom_changes(changes);
                }

                _ => todo!()
            }
        }
    }
}
