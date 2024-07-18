use std::{any::Any, sync::Arc};

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{custom::CustomElementWrapper, private::ElementLike, style::StyleSheetID};

use super::NodeID;

#[derive(Default, Clone)]
pub struct ViewNode {
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub children: Vec<CoreComponent>,
}

#[derive(Debug, Default, Clone)]
pub struct ImageViewNode {
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub src: (),
}

#[derive(Default, Clone)]
pub struct ScrollViewNode {
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub children: Vec<CoreComponent>,
}

#[derive(Default, Clone)]
pub struct ButtonNode {
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub title: String,
    pub disabled: bool,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
}

#[derive(Debug, Default, Clone)]
pub struct TextNode {
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub text: String,
}

#[derive(Debug, Default, Clone)]
pub struct TextInputNode {
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub background_text: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum NavigatorCommand {
    Goback,
    Goto(usize),
}

#[derive(Debug, Clone, Copy)]
pub struct StackNavigaterScreenOptions {}

impl Default for StackNavigaterScreenOptions {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Default, Clone)]
pub struct StackNavigaterNode {
    pub id: NodeID,
    pub style: StyleSheetID,
    pub commands: Vec<NavigatorCommand>,
    /// the component of each screen
    pub children: Vec<CoreComponent>,
    /// describe the stack screen
    pub screen_options: Vec<StackNavigaterScreenOptions>,
}

#[derive(Clone)]
pub struct FlatListNode{
    pub id: Option<NodeID>,
    pub style: StyleSheetID,
    pub get_item: Arc<dyn Fn(usize) -> Box<dyn Any> + Send + Sync>,
    pub get_len: Arc<dyn Fn() -> usize + Sync + Send>,
    pub render: Arc<dyn Fn(Box<dyn Any>) -> Box<dyn ElementLike> + Sync + Send>
}

#[derive(Clone)]
pub struct CustomNode{
    pub id: Option<NodeID>,
    pub wrapper: CustomElementWrapper
}

#[derive(Debug)]
pub enum CoreComponentType {
    View,
    ImageView,
    ScrollView,
    Button,
    Text,
    TextInput,
    StackNavigator,
    FlatList,
    Custom
}

#[derive(Clone)]
pub enum CoreComponent {
    View(Box<ViewNode>),
    ImageView(Box<ImageViewNode>),
    ScrollView(Box<ScrollViewNode>),
    Button(Box<ButtonNode>),
    Text(Box<TextNode>),
    TextInput(Box<TextInputNode>),
    StackNavigator(Box<StackNavigaterNode>),

    FlatList(Box<FlatListNode>),

    Custom(Box<CustomNode>)
}

impl CoreComponent {
    pub fn ty(&self) -> CoreComponentType{
        match self{
            Self::View(_) => CoreComponentType::View,
            Self::ImageView(_) => CoreComponentType::ImageView,
            Self::ScrollView(_) => CoreComponentType::ScrollView,
            Self::Button(_) => CoreComponentType::Button,
            Self::Text(_) => CoreComponentType::Text,
            Self::TextInput(_) => CoreComponentType::TextInput,
            Self::FlatList(_) => CoreComponentType::FlatList,
            Self::StackNavigator(_) => CoreComponentType::StackNavigator,
            Self::Custom(_) => CoreComponentType::Custom,
        }
    }
    pub fn id(&self) -> Option<NodeID> {
        match self {
            Self::View(v) => v.id,
            Self::ImageView(v) => v.id,
            Self::ScrollView(v) => v.id,
            Self::Button(b) => b.id,
            Self::Text(t) => t.id,
            Self::TextInput(t) => t.id,
            Self::StackNavigator(n) => Some(n.id),
            Self::FlatList(f) => f.id,
            Self::Custom(c) => c.id,
        }
    }

    pub fn child_mut(&mut self) -> &mut [CoreComponent] {
        match self {
            Self::View(v) => &mut v.children,
            Self::ScrollView(v) => &mut v.children,
            Self::StackNavigator(s) => &mut s.children,
            Self::Custom(c) => c.wrapper.children_mut(),
            Self::ImageView(_) | Self::Button(_) | Self::Text(_) | Self::TextInput(_) | Self::FlatList(_) => &mut [],
        }
    }

    pub fn child(&self) -> &[CoreComponent] {
        match self {
            Self::View(v) => &v.children,
            Self::ScrollView(v) => &v.children,
            Self::StackNavigator(s) => &s.children,
            Self::Custom(c) => c.wrapper.children(),
            Self::ImageView(_) | Self::Button(_) | Self::Text(_) | Self::TextInput(_) | Self::FlatList(_) => &mut [],
        }
    }
}
