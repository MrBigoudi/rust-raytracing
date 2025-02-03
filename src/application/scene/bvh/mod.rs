use aabb::Aabb;
use std::fmt::Debug;

use crate::application::core::error::ErrorCode;

use super::{model::Model, triangle::Triangle, Scene};

pub mod aabb;
pub mod bottom_up_sah;
pub mod default_bottom_up;
pub mod default_top_down;
pub mod ploc;
pub mod ploc_parallel;
pub mod top_down_sah;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub enum BvhType {
    #[default]
    None = 0,
    DefaultTopDown = 1,
    DefaultBottomUp = 2,
    BottomUpSah = 3,
    TopDownSah = 4,
    Ploc = 5,
    PlocParallel = 6,
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
        writeln!(f)?;
        writeln!(f, "bounding_box: {:?}", self.bounding_box)?;
        writeln!(f, "triangle_index: {}", self.triangle_index)?;
        writeln!(f, "left_child_index: {}", self.left_child_index)?;
        writeln!(f, "right_child_index: {}", self.right_child_index)?;
        Ok(())
    }
}

impl BvhNode {
    pub fn from_triangle(triangle: &Triangle, model: &Model, index: u32) -> Self {
        let aabb = Aabb::from_triangle(triangle, model.model_matrix);
        Self {
            bounding_box: aabb,
            triangle_index: index,
            left_child_index: 0,
            right_child_index: 0,
            padding_1: 0,
        }
    }

    pub fn is_leaf(&self) -> bool {
        if self.left_child_index == 0 || self.right_child_index == 0 {
            assert_eq!(self.left_child_index, self.right_child_index);
            true
        } else {
            false
        }
    }

    pub fn merge_bottom_up(
        left_node: &BvhNode,
        right_node: &BvhNode,
        left_index: u32,
        right_index: u32,
    ) -> BvhNode {
        let bounding_box = Aabb::merge(&left_node.bounding_box, &right_node.bounding_box);
        BvhNode {
            bounding_box,
            triangle_index: 0,
            left_child_index: left_index,
            right_child_index: right_index,
            padding_1: 0,
        }
    }

    pub fn get_sah_cost(
        &self,
        bvh: &Vec<BvhNode>,
        cost_traverse_internal: f32,
        cost_triangle_intersection: f32,
    ) -> f32 {
        if self.is_leaf() {
            cost_triangle_intersection
        } else {
            let area = self.bounding_box.get_volume();

            let left = bvh[self.left_child_index as usize];
            let right = bvh[self.right_child_index as usize];

            let area_left = left.bounding_box.get_volume();
            let factor_left = area_left / area;
            let cost_left =
                left.get_sah_cost(bvh, cost_traverse_internal, cost_triangle_intersection);

            let area_right = right.bounding_box.get_volume();
            let factor_right = area_right / area;
            let cost_right =
                right.get_sah_cost(bvh, cost_traverse_internal, cost_triangle_intersection);

            cost_traverse_internal + factor_left * cost_left + factor_right * cost_right
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
