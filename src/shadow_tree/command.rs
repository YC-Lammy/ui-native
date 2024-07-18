use std::any::Any;
use std::sync::Arc;

use crate::{custom::NativeCustomElement, style::StyleSheetID};
use crate::private::ElementLike;

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
        id: NodeID,
        style: StyleSheetID
    },

    ViewCreate{
        id: NodeID,
        style: StyleSheetID
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

    CreateImageView(NodeID),
    CreateScrollView(NodeID),

    ///////////////////////////////////////
    /////////   Button commands   /////////
    ///////////////////////////////////////
    
    ButtonCreate{
        id: NodeID,
        style: StyleSheetID,
    },
    ButtonSetLabelText{
        id: NodeID,
        label: String
    },
    ButtonSetOnClick{
        id: NodeID,
        on_click: Option<Arc<dyn Fn() + Send + Sync>>
    },
    ButtonSetDisabled{
        id: NodeID,
        disabled: bool
    },

    /////////////////////////////////////
    /////////   Text commands   /////////
    /////////////////////////////////////

    TextCreate{
        id: NodeID,
        style: StyleSheetID,
        text: String,
    },
    TextSetText{
        id: NodeID,
        text: String,
    },
    TextSetFont{
        id: NodeID,
        font: String
    },

    ///////////////////////////////////////////
    /////////   Text Input commands   /////////
    ///////////////////////////////////////////
    
    CreateTextInput(NodeID),


    /// create or initialise stack navigator
    StackNavigatorCreate{
        id: NodeID,
        style: StyleSheetID,
    },

    /////////////////////////////////////////
    /////////   flatlist commands   /////////
    /////////////////////////////////////////
    
    FlatListCreate{
        id: NodeID,
        style: StyleSheetID,
    },
    FlatListSetGetItem{
        id: NodeID,
        /// clousure to get user item
        get_item: Arc<dyn Fn(usize) -> Box<dyn Any> + Send + Sync>,
    },
    FlatListSetGetLen{
        id: NodeID,
        /// clousure to get len
        get_len: Arc<dyn Fn() -> usize + Send + Sync>,
    },
    FlatListSetRender{
        id: NodeID,
        /// clousure to render item
        render: Arc<dyn Fn(Box<dyn Any>) -> Box<dyn ElementLike> + Send + Sync>
    },

    
    StackNavigatorCommands {
        node: NodeID,
        commands: Vec<NavigatorCommand>,
    },

    CustomCreate{
        id: NodeID,
        build_fn: Arc<dyn Fn() -> Box<dyn NativeCustomElement> + Send + Sync>
    },
    CustomCommitChanges{
        id: NodeID,
        changes: Box<dyn Any + Send + Sync>
    }
}

impl std::fmt::Debug for Command{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let args = match self{
            Self::MountRoot { .. } => format!("mount root"),
            Self::ButtonCreate { .. } => format!("button create"),
            Self::ButtonSetDisabled { .. } => format!("button set disabled"),
            Self::ButtonSetLabelText { .. } => format!("button set label text"),
            Self::ButtonSetOnClick { .. } => format!("button set on click"),
            Self::ViewCreate { .. } => format!("view create"),
            Self::ViewSetChild{ index, .. } => format!("view set child {{ index:{} }}", index),
            Self::ViewRemoveChild { .. } => format!("view add child"),
            Self::TextCreate{ .. } => format!("text create"),
            Self::TextSetFont { .. } => format!("text set font"),
            Self::TextSetText { .. } => format!("text set text"),
            _ => format!("")
        };
        f.write_str(&args)
    }
}