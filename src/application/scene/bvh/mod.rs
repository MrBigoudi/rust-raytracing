use aabb::Aabb;
use std::fmt::Debug;

use crate::application::core::error::ErrorCode;

use super::Scene;

pub mod aabb;
pub mod default_bottom_up;
pub mod default_top_down;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub enum BvhType {
    #[default]
    None = 0,
    DefaultBottomUp = 1,
    DefaultTopDown = 2,
    Ploc = 3,
    Other = 4,
}

#[derive(Default, Clone, Copy)]
pub struct BvhNode {
    pub bounding_box: Aabb,
    // If not leaf then dummy variable
    pub triangle_index: u32,
    // If child_index == 0 then leaf
    pub left_child_index: u32,
    pub right_child_index: u32,
    #[allow(dead_code)]
    pub padding_1: u32,
}

impl Debug for BvhNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BvhNode")
            .field("bounding_box", &self.bounding_box)
            .field("triangle_index", &self.triangle_index)
            .field("left_child_index", &self.left_child_index)
            .field("right_child_index", &self.right_child_index)
            .finish()
    }
}

impl BvhNode {
    pub fn is_leaf(&self) -> bool {
        if self.left_child_index == 0 || self.right_child_index == 0 {
            assert_eq!(self.left_child_index, self.right_child_index);
            true
        } else {
            false
        }
    }

    #[allow(unused)]
    pub fn to_string(bvh: &Vec<BvhNode>) -> String {
        if bvh.is_empty() {
            return "BVH is empty".to_string();
        }
        // Start displaying from the root node at index 0
        Self::to_string_node(bvh, 0, 0)
    }

    #[allow(unused)]
    fn to_string_node(bvh: &Vec<BvhNode>, node_index: u32, depth: u32) -> String {
        let node = &bvh[node_index as usize];
        let indent = "    ".repeat((2 * depth) as usize);
        let mut output = String::new();

        if node.is_leaf() {
            output.push_str(&format!(
                "{}Leaf Node - Index: {}, Triangle Index: {}, Bounding Box: {:?}\n",
                indent, node_index, node.triangle_index, node.bounding_box
            ));
        } else {
            output.push_str(&format!(
                "{}Internal Node - Index: {}, Bounding Box: {:?}\n",
                indent, node_index, node.bounding_box
            ));
            output.push_str(&format!(
                "{}    Left Child Index: {}\n",
                indent, node.left_child_index
            ));
            output.push_str(&format!(
                "{}    Right Child Index: {}\n",
                indent, node.right_child_index
            ));

            // Recursively accumulate the left and right child outputs
            output.push_str(&Self::to_string_node(bvh, node.left_child_index, depth + 1));
            output.push_str(&Self::to_string_node(
                bvh,
                node.right_child_index,
                depth + 1,
            ));
        }

        output
    }
}

pub trait Bvh {
    fn build(scene: &Scene) -> Result<Vec<BvhNode>, ErrorCode>;
}
