use std::any::Any;
use std::sync::Arc;

use crossbeam_channel::Receiver;

use crate::image::ImageSource;
use crate::style::StyleRef;
use crate::util::Comparable;
use crate::widget::flatlist::ListViewWidgetFactoryWrapper;
use crate::{custom::NativeCustomElement, widget::flatlist::ListViewDataSourceWrapper};

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

    SetStyle {
        node: NodeID,
        style: StyleRef,
    },

    ViewCreate {
        id: NodeID,
        style: StyleRef,
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

    ImageViewCreate {
        id: NodeID,
        style: StyleRef,
        src: Arc<Comparable<dyn ImageSource>>,
    },
    ImageViewSetSource {
        id: NodeID,
        src: Arc<Comparable<dyn ImageSource>>,
    },

    ScrollViewCreate {
        id: NodeID,
        style: StyleRef,
    },
    ScrollViewRemoveChild {
        id: NodeID,
    },
    ScrollViewSetChild {
        id: NodeID,
        child: NodeID,
    },

    ///////////////////////////////////////
    /////////   Button commands   /////////
    ///////////////////////////////////////
    ButtonCreate {
        id: NodeID,
        style: StyleRef,
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
        style: StyleRef,
        text: String,
    },
    TextSetText {
        id: NodeID,
        text: String,
    },

    ///////////////////////////////////////////
    /////////   Text Input commands   /////////
    ///////////////////////////////////////////
    TextInputCreate {
        id: NodeID,
        style: StyleRef,
    },
    TextInputSetBGText {
        id: NodeID,
        text: String,
    },

    TextEditCreate {
        id: NodeID,
        style: StyleRef,
    },

    /////////////////////////////////////////
    /////////   flatlist commands   /////////
    /////////////////////////////////////////
    ListViewCreate {
        id: NodeID,
        style: StyleRef,
        data: Arc<ListViewDataSourceWrapper>,
        factory: Arc<ListViewWidgetFactoryWrapper>,
    },

    //////////////////////////////////////////
    /////////   stack nav commands   /////////
    //////////////////////////////////////////
    /// create or initialise stack navigator
    StackNavigatorCreate {
        id: NodeID,
        style: StyleRef,
        command_recv: Receiver<NavigatorCommand>,
    },
    StackNavigatorAddChild {
        id: NodeID,
        child: NodeID,
        name: String,
    },
    StackNavigatorRemoveChild {
        id: NodeID,
        child: NodeID,
        name: String,
    },
    CustomCreate {
        id: NodeID,
        style: StyleRef,
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
            Self::TextSetText { .. } => format!("text set text"),
            _ => format!(""),
        };
        f.write_str(&args)
    }
}
