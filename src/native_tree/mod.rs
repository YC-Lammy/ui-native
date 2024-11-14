use std::collections::HashMap;
use std::sync::Arc;

use crate::custom::NativeCustomElement;
use crate::imp::{
    NativeButton, NativeImageView, NativeListView, NativeScrollView, NativeStackNavigator,
    NativeText, NativeTextEdit, NativeTextInput, NativeView,
};
use crate::shadow_tree::{command::Command, NodeID};

pub(crate) mod context;
pub(crate) mod layout;
pub(crate) mod node;
pub(crate) mod style;
pub(crate) mod traits;
pub(crate) mod update;

use context::Context;
use node::{NativeComponent, NativeNode};
pub(crate) use traits::*;

pub use traits::{AvalableSpace, MeasuredSize};

pub struct NativeTree<'a> {
    nodes: HashMap<NodeID, NativeNode>,
    root: Option<NodeID>,
    context: Option<&'a mut Context<'a>>,
}

unsafe impl<'a> Sync for NativeTree<'a> {}
unsafe impl<'a> Send for NativeTree<'a> {}

impl<'a> NativeTree<'a> {
    /// should only be called from the main thread
    pub(crate) fn get<'b>(ctx: *mut Context<'b>) -> &'static mut Self {
        lazy_static::lazy_static! {
            static ref NATIVE_TREE: NativeTree<'static> = NativeTree {
                nodes: HashMap::new(),
                root: None,
                context: None,
            };
        };

        unsafe {
            let tree = ((&*NATIVE_TREE) as *const NativeTree<'static> as *mut NativeTree<'a>)
                .as_mut()
                .unwrap();

            tree.context = Some((ctx as *mut Context<'a>).as_mut().unwrap());

            return tree;
        }
    }

    pub fn context(&self) -> &mut Context {
        unsafe {
            self.context
                .as_ref()
                .and_then(|s| {
                    (s as *const &mut Context as *const *mut Context<'_>)
                        .read()
                        .as_mut()
                })
                .unwrap()
        }
    }

    pub fn get_root_node(&self) -> Option<Arc<NativeComponent>> {
        let id = self.root?;
        let node = self.nodes.get(&id).expect("invalid node id");
        return Some(node.component.clone());
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

    /// aux function to get known view node
    fn get_image_view(&self, id: NodeID) -> (&NativeNode, &NativeImageView) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::ImageView(v) = node.component.as_ref() {
                    return (node, &v);
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    fn get_scroll_view(&self, id: NodeID) -> (&NativeNode, &NativeScrollView) {
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

    /// aux function to get known text node
    fn get_text_input(&self, id: NodeID) -> (&NativeNode, &NativeTextInput) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::TextInput(t) = node.component.as_ref() {
                    return (&node, &t);
                } else {
                    unreachable!()
                }
            }
            None => unreachable!(),
        }
    }

    /// aux function to get known stack
    fn get_stack_nav(&self, id: NodeID) -> (&NativeNode, &NativeStackNavigator) {
        match self.nodes.get(&id) {
            Some(node) => {
                if let NativeComponent::StackNavigator(t) = node.component.as_ref() {
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
    pub fn execute_commands(&mut self, context: &mut Context, commands: Vec<Command>) {
        for cmd in commands {
            // println!("{:?}", cmd);
            match cmd {
                Command::MountRoot { node } => {
                    self.root = Some(node);
                }
                Command::RemoveNode { node: id } => {
                    let node = self.nodes.remove(&id).expect("invalid node");

                    // should retain if navigator or scroll view is valid
                    if node.component.should_retain(context) {
                        self.nodes.insert(id, node);
                    }
                }
                Command::SetStyle { node, style } => {
                    let node = self.nodes.get_mut(&node).unwrap();

                    // set the layout style
                    node.layout_style = style.to_taffy_style();
                    // set the style
                    node.style = style;
                }
                // button commands
                Command::ButtonCreate { id, style } => {
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::Button(NativeButton::new(context))),
                            style,
                        ),
                    );
                }
                Command::ButtonSetDisabled { id, disabled } => {
                    let (_node, b) = self.get_button(id);
                    b.set_disabled(context, disabled);
                }
                Command::ButtonSetLabelText { id, label } => {
                    let (_node, b) = self.get_button(id);
                    b.set_label(context, label)
                }
                Command::ButtonSetOnClick { id, on_click } => {
                    let (_node, b) = self.get_button(id);
                    b.set_on_click(context, on_click);
                }

                Command::ViewCreate { id, style } => {
                    // create view node
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::View(NativeView::new(context))),
                            style,
                        ),
                    );
                }
                Command::ViewRemoveChild { id, child, index } => {
                    let (_node, view) = self.get_view(id);
                    let elem = self.nodes.get(&child).unwrap();
                    view.remove_child(context, elem.component.widget());

                    let node = self.nodes.get_mut(&id).expect("invalid node id");
                    node.children.remove(index);

                    let child_node = self.nodes.get_mut(&child).expect("invalid node id");
                    child_node.parent = None;
                }
                Command::ViewSetChild { id, child, index } => {
                    let (_node, view) = self.get_view(id);
                    let child_node = self.nodes.get(&child).unwrap();

                    view.insert_child(context, index, child_node.component.widget());

                    let node = self.nodes.get_mut(&id).expect("invalid node");

                    if node.children.len() == index {
                        node.children.push(child);
                    } else {
                        node.children[index] = child;
                    }

                    let child_node = self.nodes.get_mut(&child).expect("invalid node id");
                    child_node.parent = Some(id);
                }

                Command::ImageViewCreate { id, style, src } => {
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::ImageView(NativeImageView::new(
                                context, src,
                            ))),
                            style,
                        ),
                    );
                }
                Command::ImageViewSetSource { id, src } => {
                    let (_node, view) = self.get_image_view(id);

                    view.set_source(context, src);
                }

                Command::ScrollViewCreate { id, style } => {
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::ScrollView(NativeScrollView::new(context))),
                            style,
                        ),
                    );
                }
                Command::ScrollViewRemoveChild { id } => {
                    let (_node, view) = self.get_scroll_view(id);

                    view.remove_child(context);

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
                    view.set_child(context, child_node.component().widget());

                    // get the mutable child node
                    let child_node = self.nodes.get_mut(&id).expect("invalid node id");
                    // set the parent as scroll view
                    child_node.parent = Some(id);

                    // get the mutable node of scroll view
                    let node = self.nodes.get_mut(&id).expect("invalid node id");

                    // register child node
                    match node.children.get_mut(0) {
                        Some(c) => *c = child,
                        None => node.children.push(child.clone()),
                    }
                }

                Command::TextCreate { id, style, text } => {
                    // create text node
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::Text(NativeText::new(context, &text))),
                            style,
                        ),
                    );
                }
                Command::TextSetText { id, text } => {
                    let (_node, t) = self.get_text(id);
                    t.set_text(context, &text);
                }
                Command::TextInputCreate { id, style } => {
                    // create the input node
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::TextInput(NativeTextInput::new(context))),
                            style,
                        ),
                    );
                }
                Command::TextInputSetBGText { id, text } => {
                    let (_node, input) = self.get_text_input(id);
                    input.set_background_text(context, &text);
                }

                Command::TextEditCreate { id, style } => {
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::TextEdit(NativeTextEdit::new(context))),
                            style,
                        ),
                    );
                }

                Command::ListViewCreate {
                    id,
                    style,
                    data,
                    factory,
                } => {
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::ListView(NativeListView::new(
                                context, data, factory,
                            ))),
                            style,
                        ),
                    );
                }

                Command::StackNavigatorCreate {
                    id,
                    style,
                    command_recv,
                } => {
                    self.nodes.insert(
                        id,
                        NativeNode::new(
                            Arc::new(NativeComponent::StackNavigator(NativeStackNavigator::new(
                                context,
                                command_recv,
                            ))),
                            style,
                        ),
                    );
                }
                Command::StackNavigatorAddChild { id, child, name } => {
                    let (_node, nav) = self.get_stack_nav(id);

                    nav.add_child(
                        context,
                        self.nodes.get(&child).unwrap().component.widget(),
                        &name,
                        child,
                    );

                    let node = self.nodes.get_mut(&id).unwrap();
                    node.children.push(child);

                    let child_node = self.nodes.get_mut(&child).unwrap();
                    child_node.parent = Some(id);
                }
                Command::StackNavigatorRemoveChild { id, child, name } => {
                    let (_node, nav) = self.get_stack_nav(id);

                    nav.remove_child(context, &name);

                    let node = self.nodes.get_mut(&id).unwrap();

                    for (i, c) in node.children.iter().enumerate() {
                        if *c == child {
                            node.children.remove(i);
                            break;
                        }
                    }

                    let child_node = self.nodes.get_mut(&child).unwrap();
                    child_node.parent = None;
                }

                Command::CustomCreate {
                    id,
                    style,
                    build_fn,
                } => {
                    // create custom element
                    let custom = build_fn();
                    // create custom node
                    self.nodes.insert(
                        id,
                        NativeNode::new(Arc::new(NativeComponent::Custom(custom)), style),
                    );
                }
                Command::CustomCommitChanges { id, changes } => {
                    let (_node, custom) = self.get_custom(id);
                    custom.commit_custom_changes(changes);
                }
            }
        }
    }
}
