use std::cmp::{max, min};

use log::error;

use crate::application::{
    core::error::ErrorCode,
    scene::{triangle::Triangle, Scene},
};

use super::{aabb::Aabb, Bvh, BvhNode};

#[derive(Debug)]
pub struct BvhPloc {
    nb_triangles: usize,
    pub clusters: Vec<Option<BvhNode>>,
    pub parents: Vec<Option<usize>>,
    pub is_leaf: Vec<bool>,
    pub left_children: Vec<Option<usize>>,
    pub right_children: Vec<Option<usize>>,
    pub triangle_indices: Vec<usize>,
}

#[derive(Debug)]
pub struct PlocParameters {
    nb_triangles: usize,
    pub search_radius: u32,
    pub nb_total_clusters: usize,
    pub iteration: usize,
    pub morton_codes: Vec<u32>,
    pub c_in: Vec<Option<usize>>,
    pub c_out: Vec<Option<usize>>,
    pub nearest_neighbor_indices: Vec<usize>,
    pub prefix_scan: Vec<usize>,
}

fn expand_bits(value: u32) -> u32 {
    #[allow(clippy::let_and_return)]
    let value = value.wrapping_mul(0x00010001u32) & 0xFF0000FFu32;
    #[allow(clippy::let_and_return)]
    let value = value.wrapping_mul(0x00000101u32) & 0x0F00F00Fu32;
    #[allow(clippy::let_and_return)]
    let value = value.wrapping_mul(0x00000011u32) & 0xC30C30C3u32;
    #[allow(clippy::let_and_return)]
    let value = value.wrapping_mul(0x00000005u32) & 0x49249249u32;
    value
}

fn get_morton_code(point: &glam::Vec3) -> u32 {
    let x = (point.x * 1024.0).clamp(0., 1023.0);
    let y = (point.y * 1024.0).clamp(0., 1023.0);
    let z = (point.z * 1024.0).clamp(0., 1023.0);

    let xx = expand_bits(x as u32);
    let yy = expand_bits(y as u32);
    let zz = expand_bits(z as u32);

    (xx << 2) | (yy << 1) | zz
}

impl BvhPloc {
    pub fn get_triangle_indices(nb_triangles: usize) -> Vec<usize> {
        (0..nb_triangles).collect()
    }

    pub fn new(scene: &Scene) -> Self {
        let nb_triangles = scene.triangles.len();
        let triangle_indices = Self::get_triangle_indices(nb_triangles);
        let clusters = vec![None; 2 * nb_triangles - 1];
        let is_leaf = vec![false; 2 * nb_triangles - 1];
        let parents = vec![None; 2 * nb_triangles - 1];
        let left_children = vec![None; 2 * nb_triangles - 1];
        let right_children = vec![None; 2 * nb_triangles - 1];
        Self {
            nb_triangles,
            clusters,
            parents,
            is_leaf,
            left_children,
            right_children,
            triangle_indices,
        }
    }

    fn get_bvh_node_recursive(
        &self,
        final_bvh: &mut Vec<BvhNode>,
        cur_node_index: usize,
    ) -> Result<(), ErrorCode> {
        let cur_node = match self.clusters[cur_node_index]{
            Some(cur_node) => cur_node,
            None => return Err(ErrorCode::AccessFailure)
        };
        let position = final_bvh.len();
        final_bvh.push(cur_node);
        if !self.is_leaf[cur_node_index] {
            let left_child = match self.left_children[cur_node_index]{
                Some(child) => child,
                None => return Err(ErrorCode::AccessFailure)
            };
            final_bvh[position].left_child_index = final_bvh.len() as u32;
            self.get_bvh_node_recursive(final_bvh, left_child)?;
            let right_child = match self.right_children[cur_node_index]{
                Some(child) => child,
                None => return Err(ErrorCode::AccessFailure)
            };
            final_bvh[position].right_child_index = final_bvh.len() as u32;
            self.get_bvh_node_recursive(final_bvh, right_child)?;
        }
        Ok(())
    }

    pub fn get_bvh(&self) -> Result<Vec<BvhNode>, ErrorCode> {
        let mut final_bvh = Vec::new();
        let root_node_index = 2 * self.nb_triangles - 2;
        self.get_bvh_node_recursive(&mut final_bvh, root_node_index)?;
        Ok(final_bvh)
    }

    pub fn final_update(&mut self, ploc_parameters: &mut PlocParameters) -> Result<(), ErrorCode> {
        if ploc_parameters.c_in[ploc_parameters.iteration - 1].is_some() {
            ploc_parameters.iteration =
                ploc_parameters.prefix_scan[ploc_parameters.iteration - 1] + 1;
        } else {
            ploc_parameters.iteration = ploc_parameters.prefix_scan[ploc_parameters.iteration - 1];
        }
        std::mem::swap(&mut ploc_parameters.c_in, &mut ploc_parameters.c_out);
        Ok(())
    }

    pub fn compaction(
        &mut self,
        ploc_parameters: &mut PlocParameters,
        index: usize,
    ) -> Result<(), ErrorCode> {
        if let Some(ci) = ploc_parameters.c_in[index] {
            let new_index = ploc_parameters.prefix_scan[index];
            ploc_parameters.c_out[new_index] = Some(ci);
        }

        Ok(())
    }

    pub fn prefix_scan(&mut self, ploc_parameters: &mut PlocParameters) -> Result<(), ErrorCode> {
        // Hillis Steele Scan
        let n = ploc_parameters.iteration;

        // Init the output array
        for i in 1..n {
            ploc_parameters.prefix_scan[i] = if ploc_parameters.c_in[i - 1].is_some() {
                1
            } else {
                0
            };
        }

        // Up phase
        let mut temp = vec![0; n];
        let mut step = 1;
        while step < n {
            #[allow(clippy::needless_range_loop)]
            for i in step..n {
                temp[i] = ploc_parameters.prefix_scan[i] + ploc_parameters.prefix_scan[i - step];
            }
            #[allow(clippy::needless_range_loop)]
            for i in step..n {
                ploc_parameters.prefix_scan[i] = temp[i];
                temp[i] = 0;
            }
            step *= 2;
        }

        Ok(())
    }

    pub fn merge_nodes(node1: &BvhNode, node2: &BvhNode) -> BvhNode {
        let new_aabb = Aabb::merge(&node1.bounding_box, &node2.bounding_box);
        BvhNode {
            bounding_box: new_aabb,
            ..Default::default()
        }
    }

    pub fn merging(
        &mut self,
        ploc_parameters: &mut PlocParameters,
        index: usize,
    ) -> Result<(), ErrorCode> {
        let neighbor_index = ploc_parameters.nearest_neighbor_indices[index];
        // If nearest neighbors of two clusters mutually correspond
        if ploc_parameters.nearest_neighbor_indices[neighbor_index] == index {
            // To avoid conflicts, only meging on the lower index
            if index < neighbor_index {
                // For global clusters arrays
                let ci = match ploc_parameters.c_in[index]{
                    Some(c_cin) => c_cin,
                    None => return Err(ErrorCode::AccessFailure)
                };
                let ci_neighbor = match ploc_parameters.c_in[neighbor_index]{
                    Some(c_cin) => c_cin,
                    None => return Err(ErrorCode::AccessFailure)
                };

                // Update new clusters
                let node = match self.clusters[ci]{
                    Some(node) => node,
                    None => return Err(ErrorCode::AccessFailure)
                };
                let node_neighbor = match self.clusters[ci_neighbor]{
                    Some(node) => node,
                    None => return Err(ErrorCode::AccessFailure)
                };
                let merged_node = Self::merge_nodes(&node, &node_neighbor);

                let new_cluster_index = ploc_parameters.nb_total_clusters;
                ploc_parameters.nb_total_clusters += 1;

                self.clusters[new_cluster_index] = Some(merged_node);
                self.left_children[new_cluster_index] = Some(ci);
                self.right_children[new_cluster_index] = Some(ci_neighbor);
                self.parents[ci] = Some(new_cluster_index);
                self.parents[ci_neighbor] = Some(new_cluster_index);

                // Mark merged cluster as invalid
                ploc_parameters.c_in[neighbor_index] = None;
                ploc_parameters.c_in[index] = Some(new_cluster_index);
            }
        }
        Ok(())
    }

    pub fn nearest_neighbor_search(
        &mut self,
        ploc_parameters: &mut PlocParameters,
        index: usize,
    ) -> Result<(), ErrorCode> {
        let current_c_in = match ploc_parameters.c_in[index]{
            Some(c_cin) => c_cin,
            None => return Err(ErrorCode::AccessFailure)
        };
        let current_index_cluster = match self.clusters[current_c_in]{
            Some(node) => node,
            None => return Err(ErrorCode::AccessFailure)
        };
        let start_index = max(0, index as i32 - ploc_parameters.search_radius as i32) as usize;
        let end_index = min(
            ploc_parameters.iteration,
            1 + index + ploc_parameters.search_radius as usize,
        );

        let mut min_dist = f32::INFINITY;
        for j in start_index..end_index {
            if j == index {
                continue;
            }
            let j_c_in = match ploc_parameters.c_in[j]{
                Some(c_cin) => c_cin,
                None => return Err(ErrorCode::AccessFailure)
            };
            let j_index_cluster = match self.clusters[j_c_in]{
                Some(node) => node,
                None => return Err(ErrorCode::AccessFailure)
            };

            let new_aabb = Aabb::merge(
                &current_index_cluster.bounding_box,
                &j_index_cluster.bounding_box,
            );
            let cur_dist = new_aabb.get_surface_area();

            // Update neighbour if needed
            if cur_dist < min_dist {
                min_dist = cur_dist;
                ploc_parameters.nearest_neighbor_indices[index] = j;
            }
        }
        Ok(())
    }
}



impl PlocParameters {
    pub fn get_morton_codes(scene: &Scene) -> Result<Vec<u32>, ErrorCode> {
        let scene_aabb = match scene.get_aabb() {
            Ok(aabb) => aabb,
            Err(err) => {
                error!(
                    "Failed to get the scene AABB for the ploc algorithm: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };
        let circumscribed_cube = Aabb::get_circumscribed_cube(&scene_aabb);
        let triangles_centroids = Triangle::get_normalized_centroids(
            &scene.triangles,
            &scene.models,
            &circumscribed_cube,
        );

        Ok(triangles_centroids
            .iter()
            .map(get_morton_code)
            .collect::<Vec<u32>>())
    }

    pub fn sort(triangle_indices: &mut [usize], morton_codes: &mut [u32]) {
        // Put them in tuple
        let mut indices_codes = Vec::new();
        assert!(triangle_indices.len() == morton_codes.len());
        for i in 0..triangle_indices.len() {
            indices_codes.push((morton_codes[i], triangle_indices[i]));
        }
        indices_codes.sort();
        // Un-tuple them
        for i in 0..triangle_indices.len() {
            morton_codes[i] = indices_codes[i].0;
            triangle_indices[i] = indices_codes[i].1;
        }
    }

    pub fn preprocessing(&mut self, bvh_ploc: &mut BvhPloc, scene: &Scene) {
        Self::sort(&mut bvh_ploc.triangle_indices, &mut self.morton_codes);
        // Create leaf nodes
        for i in 0..self.nb_triangles {
            let triangle_index = bvh_ploc.triangle_indices[i];
            let cur_triangle = scene.triangles[triangle_index];
            let cur_model = scene.models[cur_triangle.model_index];

            let bounding_box = Aabb::from_triangle(&cur_triangle, cur_model.model_matrix);
            let leaf_cluster = BvhNode {
                bounding_box,
                triangle_index: triangle_index as u32,
                left_child_index: 0,
                right_child_index: 0,
                padding_1: 0,
            };

            bvh_ploc.clusters[i] = Some(leaf_cluster);
            bvh_ploc.is_leaf[i] = true;
            self.c_in[i] = Some(i);
            self.c_out[i] = None;
        }
        self.iteration = self.nb_triangles;
        self.nb_total_clusters = self.nb_triangles;
    }

    pub fn new(scene: &Scene) -> Result<Self, ErrorCode> {
        let nb_triangles = scene.triangles.len();
        let search_radius = 16;
        let nb_total_clusters = 0;
        let iteration = 0;
        let c_in = vec![None; nb_triangles];
        let c_out = vec![None; nb_triangles];
        let nearest_neighbor_indices = vec![0; nb_triangles];
        let prefix_scan = vec![0; nb_triangles];
        let morton_codes = Self::get_morton_codes(scene)?;
        Ok(Self {
            nb_triangles,
            search_radius,
            nb_total_clusters,
            iteration,
            morton_codes,
            c_in,
            c_out,
            nearest_neighbor_indices,
            prefix_scan,
        })
    }
}

impl Bvh for BvhPloc {
    fn build(scene: &Scene) -> Result<Vec<BvhNode>, ErrorCode> {
        let mut bvh_ploc = BvhPloc::new(scene);
        let mut ploc_parameters = PlocParameters::new(scene)?;

        // Preprocessing
        ploc_parameters.preprocessing(&mut bvh_ploc, scene);

        // Ploc main loop algorithm
        while ploc_parameters.iteration > 1 {
            // Nearest Neighbor search
            for index in 0..ploc_parameters.iteration {
                if let Err(err) = bvh_ploc.nearest_neighbor_search(&mut ploc_parameters, index) {
                    error!("Failed to do the nearest neighbor search phase in the ploc algorithm: {:?}", err);
                    return Err(ErrorCode::Unknown);
                }
            }

            // Merging
            for index in 0..ploc_parameters.iteration {
                if let Err(err) = bvh_ploc.merging(&mut ploc_parameters, index) {
                    error!(
                        "Failed to do the merging phase in the ploc algorithm: {:?}",
                        err
                    );
                    return Err(ErrorCode::Unknown);
                }
            }

            // Prefix scan
            if let Err(err) = bvh_ploc.prefix_scan(&mut ploc_parameters) {
                error!(
                    "Failed to do the prefix scan phase in the ploc algorithm: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }

            // Compaction
            for index in 0..ploc_parameters.iteration {
                if let Err(err) = bvh_ploc.compaction(&mut ploc_parameters, index) {
                    error!(
                        "Failed to do the compaction phase in the ploc algorithm: {:?}",
                        err
                    );
                    return Err(ErrorCode::Unknown);
                }
            }

            // Update
            if let Err(err) = bvh_ploc.final_update(&mut ploc_parameters) {
                error!(
                    "Failed to do the final update phase in the ploc algorithm: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        }

        // Get the bvh to send to the GPU
        match bvh_ploc.get_bvh() {
            Ok(bvh) => Ok(bvh),
            Err(err) => {
                error!(
                    "Failed to build the final bvh structure in the ploc algorithm: {:?}",
                    err
                );
                Err(ErrorCode::Unknown)
            }
        }
    }
}