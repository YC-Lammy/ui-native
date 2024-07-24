use std::sync::Arc;

use taffy::{Size, TaffyTree};

use crate::shadow_tree::NodeID;

use super::{NativeComponent, NativeTree};
use super::traits::*;

/// node context is used for temporary storing component for measure function
struct NodeContext{
    comp: Arc<NativeComponent>
}

fn measure_function(
    known_dimensions: taffy::Size<Option<f32>>,
    available_space: taffy::Size<taffy::AvailableSpace>,
    _id: taffy::NodeId,
    node_context: Option<&mut NodeContext>,
    _style: &taffy::Style,
) -> taffy::Size<f32> {
    // return the known dimension
    if let taffy::Size {
        width: Some(width),
        height: Some(height),
    } = known_dimensions
    {
        return taffy::Size { width, height };
    }

    // measure the actual size
    match node_context {
        // should not happen
        None => Size::ZERO,
        Some(ctx) => match ctx.comp.as_ref() {
            // text widget
            NativeComponent::Text(t) => {
                let mut width = t.get_width() as f32;
                let mut height = t.get_height() as f32;

                if let taffy::AvailableSpace::Definite(w) = available_space.width {
                    width = width.min(w);
                }

                if let taffy::AvailableSpace::Definite(h) = available_space.height {
                    height = height.min(h);
                }

                Size {
                    width: width,
                    height: height,
                }
            }
            // text edit
            NativeComponent::TextEdit() => todo!(),
            // image view
            NativeComponent::ImageView(i) => {
                let mut width = i.get_width() as f32;
                let mut height = i.get_height() as f32;

                if let taffy::AvailableSpace::Definite(w) = available_space.width {
                    if width > w{
                        width = w;
                        i.set_width(w);
                    }
                }

                if let taffy::AvailableSpace::Definite(h) = available_space.height {
                    if height > h{
                        height = h;
                        i.set_height(h);
                    }
                }

                Size {
                    width: width,
                    height: height,
                }
            },
            NativeComponent::Button(b) => {
                let width = b.get_width();
                let height = b.get_height();

                Size{
                    width,
                    height
                }
            }
            NativeComponent::Custom(c) => {
                if let Some((width, height)) = c.measure_custom_size() {
                    Size { width, height }
                } else {
                    Size::ZERO
                }
            }
            _ => Size::ZERO,
        },
    }
}


impl NativeTree{

    /// recompute the layout of the tree
    pub fn compute_layout(&mut self, width: f64, height: f64) {
        let mut stretch = TaffyTree::<NodeContext>::new();

        let root = match self.root {
            Some(r) => r,
            None => return,
        };

        // create the root node and its children
        let root_node = self.create_layout_node(&mut stretch, root);

        // compute the layout
        stretch
            .compute_layout_with_measure(
                root_node,
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(width as _),
                    height: taffy::AvailableSpace::Definite(height as _),
                },
                measure_function,
            )
            .expect("");

        // assign layout to nodes
        self.assign_layout(&stretch, root_node);
    }

    fn create_layout_node(
        &mut self,
        stretch: &mut TaffyTree<NodeContext>,
        node_id: NodeID,
    ) -> taffy::NodeId {
        let node = self.nodes.get(&node_id).expect("invalid node id");

        // get the layout
        let mut style: taffy::Style = node.style.to_taffy_style();

        if node.component.is_scroll_view(){
            style.overflow = taffy::Point{
                x: taffy::Overflow::Scroll,
                y: taffy::Overflow::Scroll
            };
        }

        let comp = node.component.clone();

        // create the parent node
        let id = stretch
            .new_leaf_with_context(style, NodeContext{comp})
            .expect("Stretch");

        let children = node.children.clone();

        // add child to node
        for child_id in children {
            // create the layout
            let child_id = self.create_layout_node(stretch, child_id);
            // add child
            stretch
                .add_child(id, child_id)
                .expect("failed to add child");
        }

        return id;
    }

    fn assign_layout<'a>(
        &mut self,
        stretch: &'a TaffyTree<NodeContext>,
        node: taffy::NodeId,
    ) -> &'a taffy::Layout {

        let context = stretch.get_node_context(node).unwrap();

        let layout = stretch.layout(node).expect("");

        context.comp.set_width(layout.size.width);
        context.comp.set_height(layout.size.height);

        let mut i = 0;
        loop{
            let child = match stretch.child_at_index(node, i){
                Ok(c) => c,
                Err(_) => break
            };

            let child_layout = self.assign_layout(stretch, child);

            // get the child context
            let child_context = stretch.get_node_context(child).unwrap();
            // set the child position
            context.comp.set_child_position(
                &child_context.comp,
                 child_layout.location.x, 
                 child_layout.location.y
                );

            i += 1;
        }
        return layout;
    }
}