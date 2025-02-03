use crate::application::{core::error::ErrorCode, scene::Scene};

use super::{Bvh, BvhNode};

#[derive(Clone, Copy)]
pub struct BvhBottomUpSahNode {
    pub node: BvhNode,
    pub is_available: bool,
}

impl BvhBottomUpSahNode {
    pub fn new(node: BvhNode) -> Self {
        BvhBottomUpSahNode {
            node,
            is_available: true,
        }
    }
}

#[derive(Clone)]
pub struct BvhBottomUpSah<'a> {
    pub bvh: Vec<BvhBottomUpSahNode>,
    pub scene: &'a Scene,
}

impl<'a> BvhBottomUpSah<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self {
            bvh: vec![BvhBottomUpSahNode {
                node: BvhNode::default(),
                is_available: false,
            }], // Add a dummy element
            scene,
        }
    }

    pub fn nb_available(&self) -> u32 {
        let mut nb_available = 0;
        for node in &self.bvh {
            if node.is_available {
                nb_available += 1;
            }
        }
        nb_available
    }

    pub fn get_bvh(&self) -> Vec<BvhNode> {
        let mut nodes = Vec::new();
        let nb_nodes = self.bvh.len();
        nodes.push(self.bvh[nb_nodes - 1].node);
        for index in 1..(nb_nodes - 1) {
            nodes.push(self.bvh[index].node);
        }
        nodes
    }

    pub fn create_leaves(&mut self) {
        let nb_triangles = self.scene.triangles.len();
        for index in 0..nb_triangles {
            let triangle = self.scene.triangles[index];
            let model = self.scene.models[triangle.model_index];
            let leaf = BvhNode::from_triangle(&triangle, &model, index as u32);
            self.bvh.push(BvhBottomUpSahNode::new(leaf));
        }
    }
}

impl Bvh for BvhBottomUpSah<'_> {
    fn build(scene: &Scene) -> Result<Vec<BvhNode>, ErrorCode> {
        let mut bvh_bottom_up = BvhBottomUpSah::new(scene);

        // Create leaves
        bvh_bottom_up.create_leaves();

        // Until there is only one node
        while bvh_bottom_up.nb_available() > 1 {
            let mut best_sah = f32::MAX;
            let mut best_candidates = (0, 1);

            // For each node
            let nb_node = bvh_bottom_up.bvh.len();
            for i in 1..nb_node {
                let cur_node = bvh_bottom_up.bvh[i];
                if !cur_node.is_available {
                    continue;
                }
                // For each other node
                for j in 1..nb_node {
                    if i == j {
                        continue;
                    }
                    let test_node = bvh_bottom_up.bvh[j];
                    if !test_node.is_available {
                        continue;
                    }

                    // Find the best fusion
                    let new_node = BvhNode::merge_bottom_up(
                        &cur_node.node,
                        &test_node.node,
                        i as u32,
                        j as u32,
                    );

                    bvh_bottom_up.bvh.push(BvhBottomUpSahNode::new(new_node));
                    let new_sah = new_node.get_sah_cost(&bvh_bottom_up.get_bvh(), 1., 1.);

                    if new_sah < best_sah {
                        best_sah = new_sah;
                        best_candidates = (i, j);
                    }

                    bvh_bottom_up.bvh.pop().unwrap();
                }
            }
            // Fuse the two nodes
            let left_index = best_candidates.0;
            let left_node = bvh_bottom_up.bvh[left_index].node;
            let right_index = best_candidates.1;
            let right_node = bvh_bottom_up.bvh[right_index].node;
            let new_node = BvhNode::merge_bottom_up(
                &left_node,
                &right_node,
                left_index as u32,
                right_index as u32,
            );
            bvh_bottom_up.bvh.push(BvhBottomUpSahNode::new(new_node));

            // Update the available nodes
            bvh_bottom_up.bvh[left_index].is_available = false;
            bvh_bottom_up.bvh[right_index].is_available = false;
        }

        Ok(bvh_bottom_up.get_bvh())
    }
}
