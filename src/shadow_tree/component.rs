use std::sync::Arc;

use crossbeam_channel::Receiver;

use crate::custom::CustomElementWrapper;
use crate::image::ImageSource;
use crate::style::StyleRef;
use crate::util::Comparable;
use crate::widget::flatlist::{ListViewDataSourceWrapper, ListViewWidgetFactoryWrapper};

use super::NodeID;

#[derive(Default, Clone)]
pub struct ViewNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
    pub children: Vec<CoreComponent>,
}

#[derive(Clone)]
pub struct ImageViewNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
    pub src: Arc<Comparable<dyn ImageSource>>,
}

#[derive(Default, Clone)]
pub struct ScrollViewNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
    pub child: Option<CoreComponent>,
}

#[derive(Default, Clone)]
pub struct ButtonNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
    pub title: String,
    pub disabled: bool,
    pub on_click: Option<Arc<dyn Fn() + Send + Sync + 'static>>,
}

#[derive(Debug, Default, Clone)]
pub struct TextNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
    pub text: String,
}

#[derive(Debug, Default, Clone)]
pub struct TextInputNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
    pub background_text: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct TextEditNode {
    pub id: Option<NodeID>,
    pub style: StyleRef,
}

#[derive(Debug, Clone)]
pub enum NavigatorCommand {
    Goback,
    Push(String),
    Goto(String),
}

#[derive(Clone)]
pub struct StackNavigaterNode {
    pub id: NodeID,
    pub style: StyleRef,
    pub command_reciever: Receiver<NavigatorCommand>,
    /// the component of each screen
    pub children: Vec<CoreComponent>,
    /// describe the stack screen
    pub child_names: Vec<String>,
}

#[derive(Clone)]
pub struct ListViewNode {
    pub(crate) id: Option<NodeID>,
    pub(crate) style: StyleRef,
    pub(crate) data: Arc<ListViewDataSourceWrapper>,
    pub(crate) factory: Arc<ListViewWidgetFactoryWrapper>,
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
    ListView,
    Button,
    Text,
    TextInput,
    TextEdit,
    StackNavigator,
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

    ListView(Box<ListViewNode>),

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
            Self::ListView(_) => CoreComponentType::ListView,
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
            Self::ListView(f) => f.id,
            Self::Custom(c) => c.id,
        }
    }

    pub fn child_mut(&mut self) -> &mut [CoreComponent] {
        match self {
            Self::View(v) => &mut v.children,
            Self::ScrollView(v) => match &mut v.child {
                Some(c) => core::slice::from_mut(c),
                None => &mut [],
            },
            Self::StackNavigator(s) => &mut s.children,
            Self::Custom(c) => c.wrapper.children_mut(),
            Self::ImageView(_)
            | Self::Button(_)
            | Self::Text(_)
            | Self::TextInput(_)
            | Self::TextEdit(_)
            | Self::ListView(_) => &mut [],
        }
    }

    pub fn child(&self) -> &[CoreComponent] {
        match self {
            Self::View(v) => &v.children,
            Self::ScrollView(v) => match &v.child {
                Some(c) => core::slice::from_ref(c),
                None => &mut [],
            },
            Self::StackNavigator(s) => &s.children,
            Self::Custom(c) => c.wrapper.children(),
            Self::ImageView(_)
            | Self::Button(_)
            | Self::Text(_)
            | Self::TextInput(_)
            | Self::TextEdit(_)
            | Self::ListView(_) => &mut [],
        }
    }
}
