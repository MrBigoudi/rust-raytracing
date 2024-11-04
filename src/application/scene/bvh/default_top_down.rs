use log::{error, info};

use crate::application::{
    core::error::ErrorCode,
    scene::{bvh::aabb::AabbAxis, triangle::Triangle, Scene},
};

use super::{aabb::Aabb, Bvh, BvhNode};

#[derive(Debug)]
pub struct BvhDefaultTopDownNode {
    pub base: BvhNode,
    pub triangles: Vec<usize>,
}

#[derive(Debug)]
pub struct BvhDefaultTopDown<'a> {
    pub bvh: Vec<BvhDefaultTopDownNode>,
    pub scene: &'a Scene,
}

impl<'a> BvhDefaultTopDown<'a> {
    pub fn new(scene: &'a Scene) -> Result<Self, ErrorCode> {
        let aabb = match Aabb::from_scene(&scene.triangles, &scene.models) {
            Ok(aabb) => aabb,
            Err(err) => {
                error!(
                    "Failed to initialize an AABB for the default top down bvh: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let node_base = BvhNode {
            bounding_box: aabb,
            triangle_index: 0,
            left_child_index: 0,
            right_child_index: 0,
        };
        let triangles = (0..scene.triangles.len()).collect::<Vec<usize>>();

        let root_node = BvhDefaultTopDownNode {
            base: node_base,
            triangles,
        };

        let bvh = vec![root_node];

        Ok(BvhDefaultTopDown { bvh, scene })
    }

    fn get_leaves(&self) -> Vec<&BvhDefaultTopDownNode> {
        let mut leaves = Vec::new();
        for node in &self.bvh {
            if node.triangles.len() == 1 && node.base.is_leaf() {
                leaves.push(node);
            }
        }
        leaves
    }

    pub fn get_false_leaves_indices(&self) -> Vec<usize> {
        let mut leaves = Vec::new();
        for (index, node) in self.bvh.iter().enumerate() {
            if node.triangles.len() != 1 && node.base.is_leaf() {
                leaves.push(index);
            }
        }
        leaves
    }

    pub fn is_complete(&self) -> bool {
        self.get_leaves().len() == self.scene.triangles.len()
    }

    pub fn add_children(
        &mut self,
        bvh_node_index: usize,
        left_child: BvhDefaultTopDownNode,
        right_child: BvhDefaultTopDownNode,
    ) {
        // Update parent index
        let nb_nodes_in_bvh = self.bvh.len();
        let bvh_node = &mut self.bvh[bvh_node_index];
        bvh_node.base.left_child_index = nb_nodes_in_bvh;
        self.bvh.push(left_child);
        let bvh_node = &mut self.bvh[bvh_node_index];
        bvh_node.base.right_child_index = nb_nodes_in_bvh + 1;
        self.bvh.push(right_child);
    }

    fn build_last_two_children(&self, bvh_node_index: usize) -> [BvhDefaultTopDownNode; 2] {
        let bvh_node = &self.bvh[bvh_node_index];
        assert!(bvh_node.triangles.len() == 2);
        let left_triangle_index = bvh_node.triangles[0];
        let left_triangle = self.scene.triangles[left_triangle_index];
        let left_model_matrix = self.scene.models[left_triangle.model_index].model_matrix;
        let left_child = BvhDefaultTopDownNode {
            base: BvhNode {
                bounding_box: Aabb::from_triangle(&left_triangle, left_model_matrix),
                triangle_index: left_triangle_index,
                left_child_index: 0,
                right_child_index: 0,
            },
            triangles: vec![left_triangle_index],
        };

        let right_triangle_index = bvh_node.triangles[1];
        let right_triangle = self.scene.triangles[right_triangle_index];
        let right_model_matrix = self.scene.models[right_triangle.model_index].model_matrix;
        let right_child = BvhDefaultTopDownNode {
            base: BvhNode {
                bounding_box: Aabb::from_triangle(&right_triangle, right_model_matrix),
                triangle_index: right_triangle_index,
                left_child_index: 0,
                right_child_index: 0,
            },
            triangles: vec![right_triangle_index],
        };

        [left_child, right_child]
    }

    /// Build two children from a given node
    pub fn build_children(
        &self,
        bvh_node_index: usize,
    ) -> Result<[BvhDefaultTopDownNode; 2], ErrorCode> {
        let bvh_node = &self.bvh[bvh_node_index];
        // Check if the node has only two elements
        if bvh_node.triangles.len() == 2 {
            return Ok(self.build_last_two_children(bvh_node_index));
        }

        // Get the longest axis in the AABB of the current node
        let aabb = &bvh_node.base.bounding_box;
        let longest_axis = aabb.get_longest_axis();

        let is_on_the_right = |triangle: &Triangle| -> bool {
            let model_matrix = self.scene.models[triangle.model_index].model_matrix;
            let centroid = triangle.get_centroid(model_matrix);
            info!(
                "aabb: {:?}, triangle: {:?}, centroid: {:?}",
                aabb, triangle, centroid
            );
            // Check if the centroid is greater than half of the length
            // of the current bounding volume in the bounding volume's biggest direction
            match longest_axis {
                AabbAxis::X => centroid.x > ((aabb.maxs.x + aabb.mins.x) / 2.),
                AabbAxis::Y => centroid.y > ((aabb.maxs.y + aabb.mins.y) / 2.),
                AabbAxis::Z => centroid.z > ((aabb.maxs.z + aabb.mins.z) / 2.),
            }
        };

        let mut left_triangles = Vec::new();
        let mut right_triangles = Vec::new();
        let mut left_triangles_indices = Vec::new();
        let mut right_triangles_indices = Vec::new();

        'fill_tri_loop: for (i, triangle_index) in bvh_node.triangles.iter().enumerate() {
            let triangle = &self.scene.triangles[*triangle_index];
            // For the last element, check if one of the list is empty
            if i == (bvh_node.triangles.len() - 1) {
                if left_triangles.is_empty() {
                    left_triangles.push(*triangle);
                    left_triangles_indices.push(*triangle_index);
                    break 'fill_tri_loop;
                }
                if right_triangles.is_empty() {
                    right_triangles.push(*triangle);
                    right_triangles_indices.push(*triangle_index);
                    break 'fill_tri_loop;
                }
            }

            if is_on_the_right(triangle) {
                right_triangles.push(*triangle);
                right_triangles_indices.push(*triangle_index);
            } else {
                left_triangles.push(*triangle);
                left_triangles_indices.push(*triangle_index);
            }
        }

        // An internal node must have at least one triangle in each of its children
        assert!(!left_triangles.is_empty());
        assert!(!right_triangles.is_empty());
        assert!(!left_triangles_indices.is_empty());
        assert!(!right_triangles_indices.is_empty());

        let right_aabb = match Aabb::from_scene(&right_triangles, &self.scene.models) {
            Ok(aabb) => aabb,
            Err(err) => {
                error!("Failed to create the AABB for the left child in the default top down bvh: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let left_aabb = match Aabb::from_scene(&left_triangles, &self.scene.models) {
            Ok(aabb) => aabb,
            Err(err) => {
                error!("Failed to create the AABB for the left child in the default top down bvh: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let left_child = BvhDefaultTopDownNode {
            base: BvhNode {
                bounding_box: left_aabb,
                ..Default::default()
            },
            triangles: left_triangles_indices,
        };

        let right_child = BvhDefaultTopDownNode {
            base: BvhNode {
                bounding_box: right_aabb,
                ..Default::default()
            },
            triangles: right_triangles_indices,
        };

        Ok([left_child, right_child])
    }

    pub fn get_bvh(&self) -> Vec<BvhNode> {
        let mut bvh = Vec::new();
        for node in &self.bvh {
            bvh.push(node.base);
        }
        bvh
    }
}

impl Bvh for BvhDefaultTopDown<'_> {
    fn build(scene: &Scene) -> Result<Vec<BvhNode>, ErrorCode> {
        // Init the first bounding box
        let mut handler = match BvhDefaultTopDown::new(scene) {
            Ok(handler) => handler,
            Err(err) => {
                error!("Failed to initialize the default top down bvh: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        // While there are triangles in the same node
        while !handler.is_complete() {
            // Get all the current leaves that can still be extended
            let extendable_leaves = handler.get_false_leaves_indices();
            for leaf_index in extendable_leaves {
                // Build two children
                let (left_child, right_child) = match handler.build_children(leaf_index) {
                    Ok([left_child, right_child]) => (left_child, right_child),
                    Err(err) => {
                        error!(
                            "Failed to build the children in the default top down bvh: {:?}",
                            err
                        );
                        return Err(ErrorCode::InitializationFailure);
                    }
                };
                // Update the old bvh
                handler.add_children(leaf_index, left_child, right_child);
            }
        }

        Ok(handler.get_bvh())
    }
}
