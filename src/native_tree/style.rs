use crate::shadow_tree::NodeID;
use crate::style::{Style, StyleNode, StyleRef};

use super::NativeTree;

impl NativeTree {
    pub fn compute_style(&self) {
        if let Some(root) = self.root {
            // compute style for node and its children
            self.compute_style_node(
                StyleNode {
                    parent: None,
                    style: StyleRef::Style(&Style::DEFAULT),
                },
                root,
            );
        }
    }

    pub fn compute_style_node(&self, parent_style: StyleNode, node_id: NodeID) {
        let node = self.nodes.get(&node_id).expect("invalid node id");

        for child in &node.children {
            self.compute_style_node(
                StyleNode {
                    parent: Some(&parent_style),
                    style: node.style.clone(),
                },
                *child,
            );
        }
    }
}
