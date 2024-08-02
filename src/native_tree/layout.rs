use taffy::Size;

use crate::custom::NativeCustomElement;
use crate::imp::{
    NativeButton, NativeImageView, NativeListView, NativeText, NativeTextEdit, NativeTextInput,
};
use crate::shadow_tree::NodeID;
use crate::style::StyleRef;

use super::traits::*;
use super::{NativeComponent, NativeTree};

impl NativeTree {
    /// recompute the layout of the tree
    pub fn compute_layout(&mut self, width: f64, height: f64) {
        let root = match self.root {
            Some(r) => r,
            None => return,
        };

        taffy::compute_root_layout(
            self,
            taffy::NodeId::new(root.0),
            taffy::Size {
                width: taffy::AvailableSpace::Definite(width as f32),
                height: taffy::AvailableSpace::Definite(height as f32),
            },
        );

        // assign layout to nodes
        self.assign_layout(root);
    }

    fn assign_layout(&self, id: NodeID) -> &taffy::Layout {
        // get the node
        let node = self.nodes.get(&id).expect("invalid id");

        for child in &node.children {
            let child_layout = self.assign_layout(*child);
            let child_node = self.nodes.get(child).expect("invalid id");

            // layout the child
            node.component.layout_child(
                &child_node.component,
                child_layout.location.x,
                child_layout.location.y,
                child_layout.size.width,
                child_layout.size.height,
            );
        }
        return &node.computed_layout;
    }
}

// workaround for the child iterator
pub enum ChildIdIter<'a> {
    Normal { iter: core::slice::Iter<'a, NodeID> },
    Navigator { gotten: bool, page: NodeID },
    None,
}

impl<'a> Iterator for ChildIdIter<'a> {
    type Item = taffy::NodeId;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Normal { iter } => match iter.next() {
                Some(id) => Some(taffy::NodeId::new(id.0)),
                None => None,
            },
            Self::Navigator { gotten, page } => {
                if !*gotten {
                    *gotten = true;
                    Some(taffy::NodeId::new(page.0))
                } else {
                    None
                }
            }
            Self::None => None,
        }
    }
}

impl taffy::traits::TraversePartialTree for NativeTree {
    type ChildIter<'a> = ChildIdIter<'a>;

    fn child_ids(&self, parent_node_id: taffy::NodeId) -> Self::ChildIter<'_> {
        let parent_id: u64 = unsafe { core::mem::transmute(parent_node_id) };
        let parent = self.nodes.get(&NodeID(parent_id)).expect("invalid id");

        // we handle navigator differently since only one page is shown at a time
        match parent.component.as_ref() {
            NativeComponent::StackNavigator(n) => {
                if let Some(p) = n.visible_child() {
                    ChildIdIter::Navigator {
                        gotten: false,
                        page: p,
                    }
                } else {
                    ChildIdIter::None
                }
            }
            _ => ChildIdIter::Normal {
                iter: parent.children.iter(),
            },
        }
    }
    fn child_count(&self, parent_node_id: taffy::NodeId) -> usize {
        let parent_id: u64 = unsafe { core::mem::transmute(parent_node_id) };
        let parent = self.nodes.get(&NodeID(parent_id)).expect("invalid id");

        // navigators have either one or none child
        match parent.component.as_ref() {
            NativeComponent::StackNavigator(n) => {
                if n.visible_child().is_some() {
                    return 1;
                }
                return 0;
            }
            _ => parent.children.len(),
        }
    }
    fn get_child_id(&self, parent_node_id: taffy::NodeId, child_index: usize) -> taffy::NodeId {
        let parent_id: u64 = unsafe { core::mem::transmute(parent_node_id) };
        let parent = self.nodes.get(&NodeID(parent_id)).expect("invalid id");

        match parent.children.get(child_index) {
            Some(id) => taffy::NodeId::new(id.0),
            None => panic!("invalid child index"),
        }
    }
}

impl taffy::traits::TraverseTree for NativeTree {}

impl taffy::LayoutPartialTree for NativeTree {
    fn get_style(&self, node_id: taffy::NodeId) -> &taffy::Style {
        let id = unsafe { NodeID(core::mem::transmute(node_id)) };
        let node = self.nodes.get(&id).expect("invalid id");

        return &node.layout_style;
    }
    fn get_cache_mut(&mut self, node_id: taffy::NodeId) -> &mut taffy::Cache {
        let id = unsafe { NodeID(core::mem::transmute(node_id)) };
        let node = self.nodes.get_mut(&id).expect("invalid id");

        return &mut node.cache;
    }
    fn set_unrounded_layout(&mut self, node_id: taffy::NodeId, layout: &taffy::Layout) {
        let id = unsafe { NodeID(core::mem::transmute(node_id)) };
        let node = self.nodes.get_mut(&id).expect("invalid id");

        node.computed_layout = layout.clone();
    }
    fn compute_child_layout(
        &mut self,
        node_id: taffy::NodeId,
        inputs: taffy::LayoutInput,
    ) -> taffy::LayoutOutput {
        // If RunMode is PerformHiddenLayout then this indicates that an ancestor node is `Display::None`
        // and thus that we should lay out this node using hidden layout regardless of it's own display style.
        if inputs.run_mode == taffy::RunMode::PerformHiddenLayout {
            return taffy::compute_hidden_layout(self, node_id);
        }

        // We run the following wrapped in "compute_cached_layout", which will check the cache for an entry matching the node and inputs and:
        //   - Return that entry if exists
        //   - Else call the passed closure (below) to compute the result
        //
        // If there was no cache match and a new result needs to be computed then that result will be added to the cache
        return taffy::compute_cached_layout(self, node_id, inputs, |tree, node_id, inputs| {
            let id = unsafe { NodeID(core::mem::transmute(node_id)) };
            let node = tree.nodes.get_mut(&id).expect("invalid id");

            let display_mode = node.layout_style.display;
            let has_children = node.children.len() > 0;

            // Dispatch to a layout algorithm based on the node's display style and whether the node has children or not.
            match (display_mode, has_children) {
                (taffy::Display::None, _) => taffy::compute_hidden_layout(tree, node_id),
                (taffy::Display::Block, true) => taffy::compute_block_layout(tree, node_id, inputs),
                (taffy::Display::Flex, true) => {
                    taffy::compute_flexbox_layout(tree, node_id, inputs)
                }
                (taffy::Display::Grid, true) => taffy::compute_grid_layout(tree, node_id, inputs),
                (_, false) => {
                    // compute leaf layout
                    taffy::compute_leaf_layout(
                        inputs,
                        &node.layout_style,
                        |known_dimensions, available_space| {
                            measuring_function(
                                &node.component,
                                &node.style,
                                known_dimensions,
                                available_space,
                            )
                        },
                    )
                }
            }
        });
    }
}

impl taffy::traits::PrintTree for NativeTree {
    fn get_debug_label(&self, node_id: taffy::NodeId) -> &'static str {
        let id = unsafe { NodeID(core::mem::transmute(node_id)) };
        let node = self.nodes.get(&id).expect("invalid id");

        match node.component.as_ref() {
            NativeComponent::Button(_) => "button",
            NativeComponent::Custom(_) => "custom",
            NativeComponent::View(_) => "view",
            NativeComponent::ImageView(_) => "image view",
            NativeComponent::ListView(_) => "list view",
            NativeComponent::ScrollView(_) => "scroll view",
            NativeComponent::StackNavigator(_) => "stack navigator",
            NativeComponent::Text(_) => "text",
            NativeComponent::TextInput(_) => "text input",
            NativeComponent::TextEdit(_) => "text edit",
        }
    }
    fn get_final_layout(&self, node_id: taffy::NodeId) -> &taffy::Layout {
        let id = unsafe { NodeID(core::mem::transmute(node_id)) };
        let node = self.nodes.get(&id).expect("invalid id");

        &node.computed_layout
    }
}

fn list_view_measuring_function(
    list_view: &NativeListView,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = list_view.measure(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}

fn measuring_function(
    component: &NativeComponent,
    style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    match component {
        NativeComponent::Button(b) => {
            button_measuring_function(b, style, known_dimensions, available_space)
        }
        NativeComponent::Custom(c) => {
            custom_measuring_function(c.as_ref(), style, known_dimensions, available_space)
        }
        NativeComponent::ImageView(i) => {
            image_view_measuring_function(i, style, known_dimensions, available_space)
        }
        NativeComponent::ListView(l) => {
            list_view_measuring_function(l, style, known_dimensions, available_space)
        }
        NativeComponent::ScrollView(_) => Size::ZERO,
        NativeComponent::View(_) => Size::ZERO,
        NativeComponent::StackNavigator(_) => Size::ZERO,
        NativeComponent::Text(t) => {
            text_measuring_function(t, style, known_dimensions, available_space)
        }
        NativeComponent::TextInput(t) => {
            text_input_measuring_function(t, style, known_dimensions, available_space)
        }
        NativeComponent::TextEdit(t) => {
            text_edit_measuring_function(t, style, known_dimensions, available_space)
        }
    }
}

fn button_measuring_function(
    button: &NativeButton,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = button.measure(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}

fn image_view_measuring_function(
    image_view: &NativeImageView,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = image_view.measure(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}

fn text_measuring_function(
    text: &NativeText,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = text.measure(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}

fn text_input_measuring_function(
    text_input: &NativeTextInput,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = text_input.measure(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}

fn text_edit_measuring_function(
    text_edit: &NativeTextEdit,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = text_edit.measure(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}

fn custom_measuring_function(
    custom: &dyn NativeCustomElement,
    _style: &StyleRef,
    known_dimensions: Size<Option<f32>>,
    available_space: Size<taffy::AvailableSpace>,
) -> Size<f32> {
    let known_width = known_dimensions.width;
    let known_height = known_dimensions.height;

    if let (Some(w), Some(h)) = (known_width, known_height) {
        return Size {
            width: w,
            height: h,
        };
    }

    let measured = custom.measure_custom_size(known_width, known_height);

    let width = match known_width {
        Some(w) => w,
        None => match available_space.width {
            taffy::AvailableSpace::MinContent => measured.min_width,
            taffy::AvailableSpace::MaxContent => measured.natural_width,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    let height = match known_height {
        Some(h) => h,
        None => match available_space.height {
            taffy::AvailableSpace::MinContent => measured.min_height,
            taffy::AvailableSpace::MaxContent => measured.natural_height,
            taffy::AvailableSpace::Definite(d) => d,
        },
    };

    return Size { width, height };
}
