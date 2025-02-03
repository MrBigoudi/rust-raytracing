use std::collections::HashSet;

use log::error;

use crate::application::{
    core::error::ErrorCode,
    scene::{bvh::aabb::AabbAxis, triangle::Triangle, Scene},
};

use super::{aabb::Aabb, Bvh, BvhNode};

#[derive(Debug)]
pub enum SahSetType {
    Final,
    DisjointRight,
    DisjointLeft,
    OverlapRight,
    OverlapLeft,
    SplitRight,
    SplitLeft,
}

impl SahSetType {
    pub fn get_initial_set(aabb: Aabb, triangle: &Triangle, scene: &Scene) -> Self {
        let aabb_longest_axis = aabb.get_longest_axis();
        let half_aabb = (aabb.maxs + aabb.mins) * 0.5;
        let left_max = match aabb_longest_axis {
            AabbAxis::X => triangle.get_max_x(scene) < half_aabb.x,
            AabbAxis::Y => triangle.get_max_y(scene) < half_aabb.y,
            AabbAxis::Z => triangle.get_max_z(scene) < half_aabb.z,
        };
        let left_min = match aabb_longest_axis {
            AabbAxis::X => triangle.get_min_x(scene) > half_aabb.x,
            AabbAxis::Y => triangle.get_min_y(scene) > half_aabb.y,
            AabbAxis::Z => triangle.get_min_z(scene) > half_aabb.z,
        };

        if left_max {
            return Self::DisjointLeft;
        } // Completely on the left
        if left_min {
            return Self::DisjointRight;
        } // Completely on the right

        let triangle_centroid =
            triangle.get_centroid(scene.models[triangle.model_index].model_matrix);
        match aabb_longest_axis {
            AabbAxis::X => {
                if triangle_centroid.x < half_aabb.x {
                    Self::OverlapLeft
                } else {
                    Self::OverlapRight
                }
            }
            AabbAxis::Y => {
                if triangle_centroid.y < half_aabb.y {
                    Self::OverlapLeft
                } else {
                    Self::OverlapRight
                }
            }
            AabbAxis::Z => {
                if triangle_centroid.z < half_aabb.z {
                    Self::OverlapLeft
                } else {
                    Self::OverlapRight
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct BvhTopDownSahNode {
    #[allow(unused)]
    pub set_type: SahSetType,
    pub base: BvhNode,
    pub triangles: Vec<usize>,
    pub is_empty: bool,
}

impl BvhTopDownSahNode {
    pub fn new(set_type: SahSetType) -> Self {
        Self {
            set_type,
            base: BvhNode::default(),
            triangles: Vec::new(),
            is_empty: true,
        }
    }

    fn get_area(set_1: &Self, set_2: &Self) -> f32 {
        if set_1.is_empty {
            if set_2.is_empty {
                0.
            } else {
                set_2.base.bounding_box.get_volume()
            }
        } else if set_2.is_empty {
            set_1.base.bounding_box.get_volume()
        } else {
            Aabb::get_union_volume(&set_1.base.bounding_box, &set_2.base.bounding_box)
        }
    }

    pub fn compute_costs(
        dr: &Self,
        dl: &Self,
        or: &Self,
        ol: &Self,
        sr: &Self,
        sl: &Self,
    ) -> (f32, f32) {
        let area_dl_ol = Self::get_area(dl, ol);
        let area_dl_sl = Self::get_area(dl, sl);
        let area_dr_or = Self::get_area(dr, or);
        let area_dr_sr = Self::get_area(dr, sr);

        let nb_triangles_dl_ol = (dl.triangles.len() + ol.triangles.len()) as f32;
        let nb_triangles_dl_sl = (dl.triangles.len() + sl.triangles.len()) as f32;
        let nb_triangles_dr_or = (dr.triangles.len() + or.triangles.len()) as f32;
        let nb_triangles_dr_sr = (dr.triangles.len() + sr.triangles.len()) as f32;

        let cost_overlap = area_dl_ol * nb_triangles_dl_ol + area_dr_or * nb_triangles_dr_or;
        let cost_split = area_dl_sl * nb_triangles_dl_sl + area_dr_sr * nb_triangles_dr_sr;

        (cost_overlap, cost_split)
    }

    pub fn create_left_child(
        cost_overlap: f32,
        cost_split: f32,
        dl: &Self,
        ol: &Self,
        sl: &Self,
    ) -> Self {
        if cost_overlap < cost_split {
            let left_triangles: Vec<usize> = dl.triangles
                .iter()
                .copied()
                .chain(ol.triangles.iter().copied())
                .collect::<HashSet<_>>() // Deduplicate using HashSet
                .into_iter() // Convert back to an iterator
                .collect() // Collect into a Vec<usize>
            ;
            let triangle_index = left_triangles[0];
            BvhTopDownSahNode {
                set_type: SahSetType::Final,
                triangles: left_triangles,
                base: BvhNode {
                    bounding_box: Aabb::merge(&dl.base.bounding_box, &ol.base.bounding_box),
                    triangle_index: triangle_index as u32,
                    ..Default::default()
                },
                is_empty: false,
            }
        } else {
            let left_triangles: Vec<usize> = dl.triangles
                .iter()
                .copied()
                .chain(sl.triangles.iter().copied())
                .collect::<HashSet<_>>() // Deduplicate using HashSet
                .into_iter() // Convert back to an iterator
                .collect() // Collect into a Vec<usize>
            ;
            let triangle_index = left_triangles[0];
            BvhTopDownSahNode {
                set_type: SahSetType::Final,
                triangles: left_triangles,
                base: BvhNode {
                    bounding_box: Aabb::merge(&dl.base.bounding_box, &sl.base.bounding_box),
                    triangle_index: triangle_index as u32,
                    ..Default::default()
                },
                is_empty: false,
            }
        }
    }

    pub fn create_right_child(
        cost_overlap: f32,
        cost_split: f32,
        dr: &Self,
        or: &Self,
        sr: &Self,
    ) -> Self {
        if cost_overlap < cost_split {
            let right_triangles: Vec<usize> = dr.triangles
                .iter()
                .copied()
                .chain(or.triangles.iter().copied())
                .collect::<HashSet<_>>() // Deduplicate using HashSet
                .into_iter() // Convert back to an iterator
                .collect() // Collect into a Vec<usize>
            ;
            let triangle_index = right_triangles[0];
            BvhTopDownSahNode {
                set_type: SahSetType::Final,
                triangles: right_triangles,
                base: BvhNode {
                    bounding_box: Aabb::merge(&dr.base.bounding_box, &or.base.bounding_box),
                    triangle_index: triangle_index as u32,
                    ..Default::default()
                },
                is_empty: false,
            }
        } else {
            let right_triangles: Vec<usize> = dr.triangles
                .iter()
                .copied()
                .chain(sr.triangles.iter().copied())
                .collect::<HashSet<_>>() // Deduplicate using HashSet
                .into_iter() // Convert back to an iterator
                .collect() // Collect into a Vec<usize>
            ;
            let triangle_index = right_triangles[0];
            BvhTopDownSahNode {
                set_type: SahSetType::Final,
                triangles: right_triangles,
                base: BvhNode {
                    bounding_box: Aabb::merge(&dr.base.bounding_box, &sr.base.bounding_box),
                    triangle_index: triangle_index as u32,
                    ..Default::default()
                },
                is_empty: false,
            }
        }
    }

    /// Returns [DR, DL, OR, OL, SR, SL]
    pub fn partition(
        &self,
        scene: &Scene,
    ) -> Result<
        (
            BvhTopDownSahNode,
            BvhTopDownSahNode,
            BvhTopDownSahNode,
            BvhTopDownSahNode,
            BvhTopDownSahNode,
            BvhTopDownSahNode,
        ),
        ErrorCode,
    > {
        let mut dr = Self::new(SahSetType::DisjointRight);
        let mut dl = Self::new(SahSetType::DisjointLeft);
        let mut or = Self::new(SahSetType::OverlapRight);
        let mut ol = Self::new(SahSetType::OverlapLeft);
        let mut sr = Self::new(SahSetType::SplitRight);
        let mut sl = Self::new(SahSetType::SplitLeft);

        // Get mid point bounding box
        let triangles: Vec<Triangle> = self
            .triangles
            .iter()
            .map(|&index| scene.triangles[index])
            .collect();
        let midpoints = Triangle::get_centroids(&triangles, &scene.models);
        let midpoint_aabb = Aabb::from_points(&midpoints);

        // Partition into 4 sets
        for &triangle_index in &self.triangles {
            let triangle = scene.triangles[triangle_index];
            let model_matrix = scene.models[triangle.model_index].model_matrix;
            let triangle_aabb = Aabb::from_triangle(&triangle, model_matrix);
            match SahSetType::get_initial_set(midpoint_aabb, &triangle, scene) {
                SahSetType::DisjointRight => {
                    dr.triangles.push(triangle_index);
                    dr.is_empty = false;
                    dr.base.bounding_box = Aabb::merge(&dr.base.bounding_box, &triangle_aabb);
                }
                SahSetType::DisjointLeft => {
                    dl.triangles.push(triangle_index);
                    dl.is_empty = false;
                    dl.base.bounding_box = Aabb::merge(&dl.base.bounding_box, &triangle_aabb);
                }
                SahSetType::OverlapRight => {
                    or.triangles.push(triangle_index);
                    sr.triangles.push(triangle_index);
                    sl.triangles.push(triangle_index);
                    or.base.bounding_box = Aabb::merge(&or.base.bounding_box, &triangle_aabb);
                    or.is_empty = false;
                    sr.is_empty = false;
                    sl.is_empty = false;
                }
                SahSetType::OverlapLeft => {
                    ol.triangles.push(triangle_index);
                    sr.triangles.push(triangle_index);
                    sl.triangles.push(triangle_index);
                    ol.base.bounding_box = Aabb::merge(&ol.base.bounding_box, &triangle_aabb);
                    ol.is_empty = false;
                    sr.is_empty = false;
                    sl.is_empty = false;
                }
                _ => {
                    error!("Invalid initial state");
                    return Err(ErrorCode::InitializationFailure);
                }
            }
        }

        // Update the split sets
        let mut sl_aabb = Aabb::merge(&or.base.bounding_box, &ol.base.bounding_box);
        let mut sr_aabb = Aabb::merge(&or.base.bounding_box, &ol.base.bounding_box);
        let half_aabb = 0.5 * (midpoint_aabb.mins + midpoint_aabb.maxs);
        match midpoint_aabb.get_longest_axis() {
            AabbAxis::X => {
                sl_aabb.maxs.x = f32::min(half_aabb.x, sl_aabb.maxs.x);
                sr_aabb.mins.x = f32::max(half_aabb.x, sr_aabb.mins.x);
            }
            AabbAxis::Y => {
                sl_aabb.maxs.y = f32::min(half_aabb.y, sl_aabb.maxs.y);
                sr_aabb.mins.y = f32::max(half_aabb.y, sr_aabb.mins.y);
            }
            AabbAxis::Z => {
                sl_aabb.maxs.z = f32::min(half_aabb.z, sl_aabb.maxs.z);
                sr_aabb.mins.z = f32::max(half_aabb.z, sr_aabb.mins.z);
            }
        }
        sl.base.bounding_box = sl_aabb;
        sr.base.bounding_box = sr_aabb;

        Ok((dr, dl, or, ol, sr, sl))
    }
}

#[derive(Debug)]
pub struct BvhTopDownSah<'a> {
    pub bvh: Vec<BvhTopDownSahNode>,
    pub scene: &'a Scene,
}

impl<'a> BvhTopDownSah<'a> {
    pub fn new(scene: &'a Scene) -> Result<Self, ErrorCode> {
        let aabb = match Aabb::from_scene(&scene.triangles, &scene.models) {
            Ok(aabb) => aabb,
            Err(err) => {
                error!(
                    "Failed to initialize an AABB for the top down sah bvh: {:?}",
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
            padding_1: 0,
        };
        let triangles = (0..scene.triangles.len()).collect::<Vec<usize>>();

        let root_node = BvhTopDownSahNode {
            set_type: SahSetType::Final,
            base: node_base,
            triangles,
            is_empty: false,
        };

        let bvh = vec![root_node];

        Ok(BvhTopDownSah { bvh, scene })
    }

    pub fn add_children(
        &mut self,
        bvh_node_index: usize,
        left_child: BvhTopDownSahNode,
        right_child: BvhTopDownSahNode,
    ) {
        // Update parent index
        let nb_nodes_in_bvh = self.bvh.len();
        let bvh_node = &mut self.bvh[bvh_node_index];
        bvh_node.base.left_child_index = nb_nodes_in_bvh as u32;
        self.bvh.push(left_child);
        let bvh_node = &mut self.bvh[bvh_node_index];
        bvh_node.base.right_child_index = (nb_nodes_in_bvh + 1) as u32;
        self.bvh.push(right_child);
    }

    pub fn get_false_leaves_indices(&self) -> Vec<usize> {
        let mut leaves = Vec::new();
        for (index, node) in self.bvh.iter().enumerate() {
            if node.triangles.len() > 1 && node.base.is_leaf() {
                leaves.push(index);
            }
        }
        leaves
    }

    #[allow(unused)]
    pub fn get_leaves(&self) -> Vec<&BvhTopDownSahNode> {
        let mut leaves = Vec::new();
        for node in &self.bvh {
            if node.triangles.len() == 1 && node.base.is_leaf() {
                leaves.push(node);
            }
        }
        leaves
    }

    pub fn get_bvh(&self) -> Vec<BvhNode> {
        let mut bvh = Vec::new();
        for node in &self.bvh {
            bvh.push(node.base);
        }
        bvh
    }

    fn build_last_two_children(
        &self,
        bvh_node: &BvhTopDownSahNode,
    ) -> (BvhTopDownSahNode, BvhTopDownSahNode) {
        debug_assert!(bvh_node.triangles.len() == 2);
        let left_triangle_index = bvh_node.triangles[0];
        let left_triangle = self.scene.triangles[left_triangle_index];
        let left_model_matrix = self.scene.models[left_triangle.model_index].model_matrix;
        let left_child = BvhTopDownSahNode {
            set_type: SahSetType::Final,
            base: BvhNode {
                bounding_box: Aabb::from_triangle(&left_triangle, left_model_matrix),
                triangle_index: left_triangle_index as u32,
                left_child_index: 0,
                right_child_index: 0,
                padding_1: 0,
            },
            triangles: vec![left_triangle_index],
            is_empty: false,
        };

        let right_triangle_index = bvh_node.triangles[1];
        let right_triangle = self.scene.triangles[right_triangle_index];
        let right_model_matrix = self.scene.models[right_triangle.model_index].model_matrix;
        let right_child = BvhTopDownSahNode {
            set_type: SahSetType::Final,
            base: BvhNode {
                bounding_box: Aabb::from_triangle(&right_triangle, right_model_matrix),
                triangle_index: right_triangle_index as u32,
                left_child_index: 0,
                right_child_index: 0,
                padding_1: 0,
            },
            triangles: vec![right_triangle_index],
            is_empty: false,
        };

        (left_child, right_child)
    }

    pub fn build_children(
        &self,
        bvh_node: &BvhTopDownSahNode,
    ) -> Result<(BvhTopDownSahNode, BvhTopDownSahNode), ErrorCode> {
        // Check if there are only 2 triangles left in the node
        if bvh_node.triangles.len() == 2 {
            Ok(self.build_last_two_children(bvh_node))
        } else {
            // Compute DL, DR, OL, OR, SL, SR
            let (mut dr, mut dl, or, ol, mut sr, mut sl) = match bvh_node.partition(self.scene) {
                Ok(sets) => sets,
                Err(err) => {
                    error!("Failed to partition the sets: {:?}", err);
                    return Err(ErrorCode::Unknown);
                }
            };

            // Handle degenerate cases (due to floating point error ?)
            if sl.triangles.is_empty() && dl.triangles.is_empty() {
                let from_right = dr.triangles[0];
                sl.triangles.push(from_right);
                dr.triangles.swap_remove(0);
            }
            if sr.triangles.is_empty() && dr.triangles.is_empty() {
                let from_left = dl.triangles[0];
                sr.triangles.push(from_left);
                dl.triangles.swap_remove(0);
            }

            // Compute SAH cost CO and CS
            let (cost_overlap, cost_split) =
                BvhTopDownSahNode::compute_costs(&dr, &dl, &or, &ol, &sr, &sl);

            // Create the new left and right children
            let left =
                BvhTopDownSahNode::create_left_child(cost_overlap, cost_split, &dl, &ol, &sl);
            let right =
                BvhTopDownSahNode::create_right_child(cost_overlap, cost_split, &dr, &or, &sr);
            Ok((left, right))
        }
    }
}

impl Bvh for BvhTopDownSah<'_> {
    fn build(scene: &Scene) -> Result<Vec<BvhNode>, ErrorCode> {
        let mut top_down_sah_bvh = match BvhTopDownSah::new(scene) {
            Ok(handler) => handler,
            Err(err) => {
                error!("Failed to initialize the top down sah bvh: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        // While there are not as many leaves as the number of triangles in the scene
        'main: loop {
            let expandable_leaves = top_down_sah_bvh.get_false_leaves_indices();
            if expandable_leaves.is_empty() {
                break 'main;
            }

            // For each of the expandable leaves
            for leaf_index in expandable_leaves {
                let leaf_sah_node = &top_down_sah_bvh.bvh[leaf_index];
                // Build two children
                let (mut left, mut right) = match top_down_sah_bvh.build_children(leaf_sah_node) {
                    Ok(children) => children,
                    Err(err) => {
                        error!(
                            "Failed to build the children in the top down bvh sah: {:?}",
                            err
                        );
                        return Err(ErrorCode::InitializationFailure);
                    }
                };

                if leaf_sah_node.triangles.len() == left.triangles.len() {
                    'fix: for i in 0..left.triangles.len() {
                        if right.triangles.contains(&left.triangles[i]) {
                            left.triangles.swap_remove(i);
                            left.base.triangle_index = left.triangles[0] as u32;
                            break 'fix;
                        }
                    }
                }
                if leaf_sah_node.triangles.len() == right.triangles.len() {
                    'fix: for i in 0..right.triangles.len() {
                        if left.triangles.contains(&right.triangles[i]) {
                            right.triangles.swap_remove(i);
                            right.base.triangle_index = right.triangles[0] as u32;
                            break 'fix;
                        }
                    }
                }

                // Update the old bvh
                top_down_sah_bvh.add_children(leaf_index, left, right);
            }
        }

        let bvh = top_down_sah_bvh.get_bvh();
        Ok(bvh)
    }
}
