use std::{cmp::{max, min}, sync::Mutex};

use rayon::prelude::*;

use log::error;

use crate::application::{
    core::error::ErrorCode,
    scene::{bvh::ploc::{BvhPloc, PlocParameters}, Scene},
};

use super::{aabb::Aabb, Bvh, BvhNode};

pub struct BvhPlocParallel;

fn create_leaf_nodes(
    index: usize,
    scene: &Scene,
    triangle_indices: &[usize],
    mut_cluster: &mut Option<BvhNode>,
    mut_is_leaf: &mut bool,
    mut_c_in: &mut Option<usize>,
) {
    let triangle_index = triangle_indices[index];
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

    *mut_cluster = Some(leaf_cluster);
    *mut_is_leaf = true;
    *mut_c_in = Some(index);
}

fn nearest_neighbor_search(
    index: usize,
    iteration: usize,
    search_radius: u32,
    c_in: &[Option<usize>],
    clusters: &[Option<BvhNode>],
    mut_nearest_neighbor_index: &mut usize,
) -> Result<(), ErrorCode> {
    let current_c_in = match c_in[index]{
        Some(c_cin) => c_cin,
        None => return Err(ErrorCode::AccessFailure)
    };
    let current_index_cluster = match clusters[current_c_in]{
        Some(node) => node,
        None => return Err(ErrorCode::AccessFailure)
    };
    let start_index = max(0, index as i32 - search_radius as i32) as usize;
    let end_index = min(
        iteration,
        1 + index + search_radius as usize,
    );

    let mut min_dist = f32::INFINITY;
    for j in start_index..end_index {
        if j == index {
            continue;
        }
        let j_c_in = match c_in[j]{
            Some(c_cin) => c_cin,
            None => return Err(ErrorCode::AccessFailure)
        };
        let j_index_cluster = match clusters[j_c_in]{
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
            *mut_nearest_neighbor_index = j;
        }
    }
    Ok(())
}

fn merging(
    index: usize,
    nb_total_clusters: &Mutex<&mut usize>,
    nearest_neighbor_indices: &[usize],
    mut_clusters: &Mutex<&mut Vec<Option<BvhNode>>>,
    mut_left_children: &Mutex<&mut Vec<Option<usize>>>,
    mut_right_children: &Mutex<&mut Vec<Option<usize>>>,
    mut_parents: &Mutex<&mut Vec<Option<usize>>>,
    mut_c_in: &Mutex<&mut Vec<Option<usize>>>,
) -> Result<(), ErrorCode> {
    let neighbor_index = nearest_neighbor_indices[index];
    // If nearest neighbors of two clusters mutually correspond
    if nearest_neighbor_indices[neighbor_index] == index {
        // To avoid conflicts, only meging on the lower index
        if index < neighbor_index {
            // For global clusters arrays
            let ci = match mut_c_in.lock().unwrap()[index]{
                Some(c_cin) => c_cin,
                None => return Err(ErrorCode::AccessFailure)
            };
            let ci_neighbor = match mut_c_in.lock().unwrap()[neighbor_index]{
                Some(c_cin) => c_cin,
                None => return Err(ErrorCode::AccessFailure)
            };

            // Update new clusters
            let node = match mut_clusters.lock().unwrap()[ci]{
                Some(node) => node,
                None => return Err(ErrorCode::AccessFailure)
            };
            let node_neighbor = match mut_clusters.lock().unwrap()[ci_neighbor]{
                Some(node) => node,
                None => return Err(ErrorCode::AccessFailure)
            };
            let merged_node = BvhPloc::merge_nodes(&node, &node_neighbor);

            let mut nb_clusters = nb_total_clusters.lock().unwrap();
            let new_cluster_index: usize = nb_clusters.clone();
            **nb_clusters += 1;

            mut_clusters.lock().unwrap()[new_cluster_index] = Some(merged_node);
            mut_left_children.lock().unwrap()[new_cluster_index] = Some(ci);
            mut_right_children.lock().unwrap()[new_cluster_index] = Some(ci_neighbor);
            mut_parents.lock().unwrap()[ci] = Some(new_cluster_index);
            mut_parents.lock().unwrap()[ci_neighbor] = Some(new_cluster_index);

            // Mark merged cluster as invalid
            mut_c_in.lock().unwrap()[neighbor_index] = None;
            mut_c_in.lock().unwrap()[index] = Some(new_cluster_index);
        }
    }
    Ok(())
}

fn prefix_scan_parallel(
    iteration: usize,
    c_in: &[Option<usize>],
    prefix_scan: &mut [usize],
) {
    (
        (1..iteration),
        &mut prefix_scan[1..iteration]
    ).into_par_iter()
        .for_each(|(index, prefix)|{
            *prefix = if c_in[index-1].is_some() { 1 } else { 0 };
        }
    );

    let mut temp = vec![0; iteration];
    let mut step = 1;
    while step < iteration {
        (
            (step..iteration),
            &mut temp[step..iteration]
        ).into_par_iter()
            .for_each(|(index, temp)|{
                *temp = prefix_scan[index] + prefix_scan[index-step]
            }
        );

        (
            &mut temp[step..iteration],
            &mut prefix_scan[step..iteration]
        ).into_par_iter()
            .for_each(|(temp, prefix)|{
                *prefix = temp.clone();
                *temp = 0;
            }
        );
        step *= 2;
    }
}

fn compaction(
    index: usize,
    c_in: &[Option<usize>],
    prefix_scan: &[usize],
    mut_c_out: &Mutex<&mut Vec<Option<usize>>>,
) {
    if let Some(ci) = c_in[index] {
        let new_index = prefix_scan[index];
        let mut mut_c_out = mut_c_out.lock().unwrap();
        mut_c_out[new_index] = Some(ci)
    }
}

fn get_bvh_node_recursive(
    final_bvh: &mut Vec<BvhNode>,
    cur_node_index: usize,
    clusters: &[Option<BvhNode>],
    left_children: &[Option<usize>],
    right_children: &[Option<usize>],
    is_leaf: &[bool],
) -> Result<(), ErrorCode> {
    let cur_node = match clusters[cur_node_index]{
        Some(cur_node) => cur_node,
        None => return Err(ErrorCode::AccessFailure)
    };
    let position = final_bvh.len();
    final_bvh.push(cur_node);
    if !is_leaf[cur_node_index] {
        let left_child = match left_children[cur_node_index]{
            Some(child) => child,
            None => return Err(ErrorCode::AccessFailure)
        };
        final_bvh[position].left_child_index = final_bvh.len() as u32;
        get_bvh_node_recursive(final_bvh, left_child, clusters, left_children, right_children, is_leaf)?;
        let right_child = match right_children[cur_node_index]{
            Some(child) => child,
            None => return Err(ErrorCode::AccessFailure)
        };
        final_bvh[position].right_child_index = final_bvh.len() as u32;
        get_bvh_node_recursive(final_bvh, right_child, clusters, left_children,right_children, is_leaf)?;
    }
    Ok(())
}

fn get_bvh(
    nb_triangles: usize,
    clusters: &[Option<BvhNode>],
    left_children: &[Option<usize>],
    right_children: &[Option<usize>],
    is_leaf: &[bool],
) -> Result<Vec<BvhNode>, ErrorCode> {
    let mut final_bvh = Vec::new();
    let root_node_index = 2 * nb_triangles - 2;
    get_bvh_node_recursive(&mut final_bvh, root_node_index, clusters, left_children, right_children, is_leaf)?;
    Ok(final_bvh)
}

impl Bvh for BvhPlocParallel {
    fn build(scene: &Scene) -> Result<Vec<BvhNode>, ErrorCode> {
        let nb_triangles = scene.triangles.len();
        let search_radius: u32 = 16;

        // Init the variables
        let mut nb_total_clusters = nb_triangles;
        let mut iteration = nb_triangles;

        // Init the arrays
        let mut triangle_indices: Vec<usize> = BvhPloc::get_triangle_indices(nb_triangles);
        let mut clusters: Vec<Option<BvhNode>> = vec![None; 2 * nb_triangles - 1];
        let mut is_leaf: Vec<bool> = vec![false; 2 * nb_triangles - 1];
        let mut parents: Vec<Option<usize>> = vec![None; 2 * nb_triangles - 1];
        let mut left_children: Vec<Option<usize>> = vec![None; 2 * nb_triangles - 1];
        let mut right_children: Vec<Option<usize>> = vec![None; 2 * nb_triangles - 1];

        let mut c_in: Vec<Option<usize>> = vec![None; nb_triangles];
        let mut c_out: Vec<Option<usize>> = vec![None; nb_triangles];
        let mut nearest_neighbor_indices: Vec<usize> = vec![0; nb_triangles];
        let mut prefix_scan: Vec<usize> = vec![0; nb_triangles];
        let mut morton_codes: Vec<u32> = PlocParameters::get_morton_codes(scene)?;

        // Preprocessing
        PlocParameters::sort(&mut triangle_indices, &mut morton_codes);
        // Create leaf nodes
        (
            (0..nb_triangles),
            &mut clusters[0..nb_triangles],
            &mut is_leaf[0..nb_triangles],
            &mut c_in
        ).into_par_iter()
            .for_each(|(index, cluster, is_leaf, c_in)|{
                create_leaf_nodes(index, scene, &triangle_indices, cluster, is_leaf, c_in);
            }
        );

        while iteration > 1 {
            // Nearest neighbor search (in parallel)
            (
                (0..iteration),
                &mut nearest_neighbor_indices[0..iteration]
            ).into_par_iter()
                .try_for_each(|(index, nearest_neighbor_index)|{
                    if let Err(err) = nearest_neighbor_search(index, iteration, search_radius, &c_in, &clusters, nearest_neighbor_index){
                        error!("Failed to do the nearest neighbor search phase in the parallel ploc algorithm: {:?}", err);
                        return Err(ErrorCode::Unknown);
                    };
                    Ok(())
                }
            )?;

            // Merging (in parallel)
            let mutex_clusters = Mutex::new(&mut clusters);
            let mutex_left_children = Mutex::new(&mut left_children);
            let mutex_right_children = Mutex::new(&mut right_children);
            let mutex_parents = Mutex::new(&mut parents);
            let mutex_c_in = Mutex::new(&mut c_in);
            let mutex_nb_total_clusters = Mutex::new(&mut nb_total_clusters);
            
            (0..iteration)
                .into_par_iter()
                .try_for_each(|index|{
                    if let Err(err) = 
                        merging(
                            index,
                            &mutex_nb_total_clusters,
                            &nearest_neighbor_indices,
                            &mutex_clusters,
                            &mutex_left_children,
                            &mutex_right_children,
                            &mutex_parents,
                            &mutex_c_in
                        )
                    {
                        error!("Failed to do the merging phase in the parallel ploc algorithm: {:?}", err);
                        return Err(ErrorCode::Unknown);
                    };
                    Ok(())
                }
            )?;

            // Prefix Scan (in parallel)
            prefix_scan_parallel(iteration, &c_in, &mut prefix_scan);

            // Compaction (in parallel)
            let mutex_c_out = Mutex::new(&mut c_out);
            (0..iteration).into_par_iter().for_each(|index| {
                compaction(index, &c_in, &prefix_scan, &mutex_c_out);
            });

            // Final update (in one thread)
            if c_in[iteration-1].is_some(){
                iteration = prefix_scan[iteration-1]+1;
            } else {
                iteration = prefix_scan[iteration-1];
            }
            std::mem::swap(&mut c_in, &mut c_out);
        }

        // Get the bvh to send to the GPU
        match get_bvh(
            nb_triangles,
            &clusters,
            &left_children,
            &right_children,
            &is_leaf
        ) {
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