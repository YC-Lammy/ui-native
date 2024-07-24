use std::{any::Any, sync::Arc};

use crate::custom::CustomElementWrapper;
use  crate::private::ElementLike;
use crate::style::StyleSheet;

use super::NodeID;

#[derive(Default, Clone)]
pub struct ViewNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub children: Vec<CoreComponent>,
}

#[derive(Debug, Default, Clone)]
pub struct ImageViewNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub src: (),
}

#[derive(Default, Clone)]
pub struct ScrollViewNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub child: Option<CoreComponent>,

}

#[derive(Default, Clone)]
pub struct ButtonNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub title: String,
    pub disabled: bool,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
}

#[derive(Debug, Default, Clone)]
pub struct TextNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub text: String,
}

#[derive(Debug, Default, Clone)]
pub struct TextInputNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub background_text: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct TextEditNode{
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>
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
    pub style: Arc<StyleSheet>,
    pub commands: Vec<NavigatorCommand>,
    /// the component of each screen
    pub children: Vec<CoreComponent>,
    /// describe the stack screen
    pub screen_options: Vec<StackNavigaterScreenOptions>,
}

#[derive(Clone)]
pub struct FlatListNode {
    pub id: Option<NodeID>,
    pub style: Arc<StyleSheet>,
    pub get_item: Arc<dyn Fn(usize) -> Box<dyn Any> + Send + Sync>,
    pub get_len: Arc<dyn Fn() -> usize + Sync + Send>,
    pub render: Arc<dyn Fn(Box<dyn Any>) -> Box<dyn ElementLike> + Sync + Send>,
}

#[derive(Clone)]
pub struct CustomNode {
    pub id: Option<NodeID>,
    pub(crate) wrapper: CustomElementWrapper,
}

#[derive(Debug)]
pub enum CoreComponentType {
    View,
    ImageView,
    ScrollView,
    Button,
    Text,
    TextInput,
    TextEdit,
    StackNavigator,
    FlatList,
    Custom,
}

#[derive(Clone)]
pub enum CoreComponent {
    View(Box<ViewNode>),
    ImageView(Box<ImageViewNode>),
    ScrollView(Box<ScrollViewNode>),
    Button(Box<ButtonNode>),
    Text(Box<TextNode>),
    TextInput(Box<TextInputNode>),
    TextEdit(Box<TextEditNode>),
    StackNavigator(Box<StackNavigaterNode>),

    FlatList(Box<FlatListNode>),

    Custom(Box<CustomNode>),
}

impl CoreComponent {
    pub fn ty(&self) -> CoreComponentType {
        match self {
            Self::View(_) => CoreComponentType::View,
            Self::ImageView(_) => CoreComponentType::ImageView,
            Self::ScrollView(_) => CoreComponentType::ScrollView,
            Self::Button(_) => CoreComponentType::Button,
            Self::Text(_) => CoreComponentType::Text,
            Self::TextInput(_) => CoreComponentType::TextInput,
            Self::TextEdit(_) => CoreComponentType::TextEdit,
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
            Self::TextEdit(t) => t.id,
            Self::StackNavigator(n) => Some(n.id),
            Self::FlatList(f) => f.id,
            Self::Custom(c) => c.id,
        }
    }

    pub fn child_mut(&mut self) -> &mut [CoreComponent] {
        match self {
            Self::View(v) => &mut v.children,
            Self::ScrollView(v) => match &mut v.child{
                Some(c) => core::slice::from_mut(c),
                None => &mut []
            },
            Self::StackNavigator(s) => &mut s.children,
            Self::Custom(c) => c.wrapper.children_mut(),
            Self::ImageView(_)
            | Self::Button(_)
            | Self::Text(_)
            | Self::TextInput(_)
            | Self::TextEdit(_)
            | Self::FlatList(_) => &mut [],
        }
    }

    pub fn child(&self) -> &[CoreComponent] {
        match self {
            Self::View(v) => &v.children,
            Self::ScrollView(v) => match &v.child{
                Some(c) => core::slice::from_ref(c),
                None => &mut []
            },
            Self::StackNavigator(s) => &s.children,
            Self::Custom(c) => c.wrapper.children(),
            Self::ImageView(_)
            | Self::Button(_)
            | Self::Text(_)
            | Self::TextInput(_)
            | Self::TextEdit(_)
            | Self::FlatList(_) => &mut [],
        }
    }
}
