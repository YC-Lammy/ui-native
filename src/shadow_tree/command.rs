use std::any::Any;
use std::sync::Arc;

use crate::private::ElementLike;
use crate::custom::NativeCustomElement;
use crate::style::StyleSheet;

use super::{component::NavigatorCommand, NodeID};

pub enum Command {
    /// mount node as root
    MountRoot {
        node: NodeID,
    },
    /// remove a node
    RemoveNode {
        node: NodeID,
    },

    SetStyle{
        node: NodeID,
        style: Arc<StyleSheet>
    },

    ViewCreate {
        id: NodeID,
        style: Arc<StyleSheet>,
    },
    /// add or replace child at index
    ViewSetChild {
        id: NodeID,
        child: NodeID,
        index: usize,
    },
    /// remove child at index
    ViewRemoveChild {
        id: NodeID,
        child: NodeID,
        index: usize,
    },

    ImageViewCreate{
        id: NodeID,
        style: Arc<StyleSheet>,
    },

    ScrollViewCreate{
        id: NodeID,
        style: Arc<StyleSheet>,
    },
    ScrollViewRemoveChild{
        id: NodeID
    },
    ScrollViewSetChild{
        id: NodeID,
        child: NodeID
    },

    ///////////////////////////////////////
    /////////   Button commands   /////////
    ///////////////////////////////////////
    ButtonCreate {
        id: NodeID,
        style: Arc<StyleSheet>,
    },
    ButtonSetLabelText {
        id: NodeID,
        label: String,
    },
    ButtonSetOnClick {
        id: NodeID,
        on_click: Option<Arc<dyn Fn() + Send + Sync>>,
    },
    ButtonSetDisabled {
        id: NodeID,
        disabled: bool,
    },

    /////////////////////////////////////
    /////////   Text commands   /////////
    /////////////////////////////////////
    TextCreate {
        id: NodeID,
        style: Arc<StyleSheet>,
        text: String,
    },
    TextSetText {
        id: NodeID,
        text: String,
    },
    TextSetFont {
        id: NodeID,
        font: String,
    },

    ///////////////////////////////////////////
    /////////   Text Input commands   /////////
    ///////////////////////////////////////////
    CreateTextInput(NodeID),

    

    /////////////////////////////////////////
    /////////   flatlist commands   /////////
    /////////////////////////////////////////
    FlatListCreate {
        id: NodeID,
        style: Arc<StyleSheet>,
    },
    FlatListSetGetItem {
        id: NodeID,
        /// clousure to get user item
        get_item: Arc<dyn Fn(usize) -> Box<dyn Any> + Send + Sync>,
    },
    FlatListSetGetLen {
        id: NodeID,
        /// clousure to get len
        get_len: Arc<dyn Fn() -> usize + Send + Sync>,
    },
    FlatListSetRender {
        id: NodeID,
        /// clousure to render item
        render: Arc<dyn Fn(Box<dyn Any>) -> Box<dyn ElementLike> + Send + Sync>,
    },
    
    /// create or initialise stack navigator
    StackNavigatorCreate {
        id: NodeID,
        style: Arc<StyleSheet>,
    },
    StackNavigatorCommands {
        node: NodeID,
        commands: Vec<NavigatorCommand>,
    },

    CustomCreate {
        id: NodeID,
        style: Arc<StyleSheet>,
        build_fn: Arc<dyn Fn() -> Box<dyn NativeCustomElement> + Send + Sync>,
    },
    CustomCommitChanges {
        id: NodeID,
        changes: Box<dyn Any + Send + Sync>,
    },
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = match self {
            Self::MountRoot { .. } => format!("mount root"),
            Self::ButtonCreate { .. } => format!("button create"),
            Self::ButtonSetDisabled { .. } => format!("button set disabled"),
            Self::ButtonSetLabelText { .. } => format!("button set label text"),
            Self::ButtonSetOnClick { .. } => format!("button set on click"),
            Self::ViewCreate { .. } => format!("view create"),
            Self::ViewSetChild { index, .. } => format!("view set child {{ index:{} }}", index),
            Self::ViewRemoveChild { .. } => format!("view add child"),
            Self::TextCreate { .. } => format!("text create"),
            Self::TextSetFont { .. } => format!("text set font"),
            Self::TextSetText { .. } => format!("text set text"),
            _ => format!(""),
        };
        f.write_str(&args)
    }
}
