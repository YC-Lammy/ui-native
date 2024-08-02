use std::sync::Arc;

use crate::custom::NativeCustomElement;
use crate::imp::{
    NativeButton, NativeElement, NativeImageView, NativeListView, NativeScrollView,
    NativeStackNavigator, NativeText, NativeTextEdit, NativeTextInput, NativeView,
};
use crate::shadow_tree::NodeID;
use crate::style::StyleRef;

use super::traits::*;

pub enum NativeComponent {
    View(NativeView),
    ImageView(NativeImageView),
    ScrollView(NativeScrollView),
    ListView(NativeListView),

    Button(NativeButton),
    Text(NativeText),
    TextInput(NativeTextInput),
    TextEdit(NativeTextEdit),

    StackNavigator(NativeStackNavigator),

    Custom(Box<dyn NativeCustomElement>),
}

impl NativeComponent {
    pub fn widget(&self) -> &dyn NativeElement {
        match self {
            Self::View(v) => v,
            Self::ImageView(i) => i,
            Self::ScrollView(s) => s,
            Self::ListView(v) => v,
            Self::Button(b) => b,
            Self::Text(t) => t,
            Self::TextInput(t) => t,
            Self::TextEdit(t) => t,
            Self::StackNavigator(s) => s,
            Self::Custom(c) => c.as_native_element(),
        }
    }

    pub fn should_retain(&self) -> bool {
        match self {
            Self::StackNavigator(s) => s.should_retain(),
            _ => false,
        }
    }

    pub fn layout_child(&self, child: &NativeComponent, x: f32, y: f32, width: f32, height: f32) {
        match self {
            Self::View(v) => v.layout_child(child.widget(), x, y, width, height),
            Self::ScrollView(s) => s.layout_child(child.widget(), x, y, width, height),
            Self::StackNavigator(s) => s.layout_child(child.widget(), x, y, width, height),
            _ => todo!(),
        }
    }
}

pub struct NativeNode {
    pub parent: Option<NodeID>,
    pub children: Vec<NodeID>,
    pub component: Arc<NativeComponent>,
    pub style: StyleRef,

    pub layout_style: taffy::Style,
    pub cache: taffy::Cache,
    pub computed_layout: taffy::Layout,
}

impl NativeNode {
    pub fn new(component: Arc<NativeComponent>, style: StyleRef) -> Self {
        let layout_style = style.to_taffy_style();

        Self {
            parent: None,
            children: Vec::new(),
            style: style,
            component: component,

            layout_style: layout_style,
            cache: taffy::Cache::new(),
            computed_layout: taffy::Layout::new(),
        }
    }
    pub fn component(&self) -> &NativeComponent {
        &self.component
    }
}
