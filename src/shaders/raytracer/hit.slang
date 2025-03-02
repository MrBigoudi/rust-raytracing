implementing raytracer;

import scene.scene;


public struct Hit {
    public float4 coords; // (b0, b1, b2, t)
    public uint did_hit = 0;
    public uint triangle_index = 0;

    public __init() {
        did_hit = 0;
        triangle_index = 0;
        coords = float4(0.);
    }

    public func get_distance()->float {
        return coords.w;
    }

    public func get_barycentric_coordinates()->float3 {
        return float3(coords.x, coords.y, coords.z);
    }

    public func get_world_position(
        triangles: RWStructuredBuffer<Triangle>,
        models: RWStructuredBuffer<Model>,
    ) -> float3 {
        let triangle = triangles[triangle_index];
        let b = get_barycentric_coordinates();
        float3 p0_world = triangle.get_world_position_p0(models);
        float3 p1_world = triangle.get_world_position_p1(models);
        float3 p2_world = triangle.get_world_position_p2(models);

        return (b.x * p0_world) + (b.y * p1_world) + (b.z * p2_world);
    }

    public func get_world_norm(
        triangles: RWStructuredBuffer<Triangle>,
        models: RWStructuredBuffer<Model>,
    ) ->float3 {
        let triangle = triangles[triangle_index];
        // No vertex normals
        let n = triangle.get_normal(models);

        let b = get_barycentric_coordinates();
        return normalize((b.x * n) + (b.y * n) + (b.z * n));
    }

    public func get_ambient(
        triangles: RWStructuredBuffer<Triangle>,
        models: RWStructuredBuffer<Model>,
        materials: RWStructuredBuffer<Material>,
    ) ->float3 {
        let b = get_barycentric_coordinates();
        let material = materials[models[triangles[triangle_index].model_index].material_index];
        let ambient = float3(material.ambient.x, material.ambient.y, material.ambient.z);
        return (b.x * ambient) + (b.y * ambient) + (b.z * ambient);
    }
};

public func ray_triangle_intersection(
    ray: Ray,
    triangle_index: uint,
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    ) -> Hit{
    Hit hit;

    Triangle triangle = triangles[triangle_index];

    float3 p0 = triangle.get_world_position_p0(models);
    float3 p1 = triangle.get_world_position_p1(models);
    float3 p2 = triangle.get_world_position_p2(models);

    float3 e0 = p1 - p0;
    float3 e1 = p2 - p0;

    float3 w = ray.direction.xyz;
    float3 o = ray.origin.xyz;

    float3 tmp = cross(e0, e1);
    if (length(tmp) == 0.) {
        hit.did_hit = 0;
        return hit;
    }

    float3 n = normalize(tmp);
    float3 q = cross(w, e1);
    float a = dot(e0, q);

    if(!ray.is_shadow_ray && dot(n, w) >= 0.) {
        hit.did_hit = 0;
        return hit;
    }

    float epsilon = 1e-6;
    if (abs(a) < epsilon) {
        hit.did_hit = 0;
        return hit;
    }

    float3 s = (o - p0) / a;
    float3 r = cross(s, e0);

    float b0 = dot(s, q);
    float b1 = dot(r, w);
    float b2 = 1 - b0 - b1;
    if (b0 < 0 || b1 < 0 || b2 < 0) {
        hit.did_hit = 0;
        return hit;
    }

    float t = dot(e1, r);
    if (t < 1e-4 || t > 1e6) {
        hit.did_hit = 0;
        return hit;
    }

    hit.coords.x = b2;
    hit.coords.y = b0;
    hit.coords.z = b1;
    hit.coords.w = t;
    hit.did_hit = 1;
    hit.triangle_index = triangle_index;

    return hit;
}

public func ray_bvh_intersection(ray: Ray, bvh_node: BvhNode) -> BvhNodeIntersection {
    // Check intersection with X,Y,Z-slabs
    float3 inverse_ray_dir = 1.f / ray.direction.xyz;
    float3 t1 = (bvh_node.bounding_box.mins - ray.origin.xyz) * inverse_ray_dir;
    float3 t2 = (bvh_node.bounding_box.maxs - ray.origin.xyz) * inverse_ray_dir;
    // Check for early exit
    float t_min = min(t1.x, t2.x);
    float t_max = max(t1.x, t2.x);
    if (t_max < 0.f || t_min > t_max) {
        return BvhNodeIntersection::None;
    }
    t_min = max(t_min, min(t1.y, t2.y));
    t_max = min(t_max, max(t1.y, t2.y));
    if (t_max < 0.f || t_min > t_max) {
        return BvhNodeIntersection::None;
    }
    t_min = max(t_min, min(t1.z, t2.z));
    t_max = min(t_max, max(t1.z, t2.z));
    if (t_max < 0.f || t_min > t_max) {
        return BvhNodeIntersection::None;
    }

    // Check if border
    float threshold = 0.2;
    float3 hit_point = ray.origin.xyz + ray.direction.xyz * t_min;
    bool close_to_x = (abs(hit_point.x - bvh_node.bounding_box.mins.x) < threshold)
            || (abs(hit_point.x - bvh_node.bounding_box.maxs.x) < threshold);
    bool close_to_y = (abs(hit_point.y - bvh_node.bounding_box.mins.y) < threshold)
            || (abs(hit_point.y - bvh_node.bounding_box.maxs.y) < threshold);
    bool close_to_z = (abs(hit_point.z - bvh_node.bounding_box.mins.z) < threshold)
            || (abs(hit_point.z - bvh_node.bounding_box.maxs.z) < threshold);
    if ((close_to_x && close_to_y) || (close_to_x && close_to_z) || (close_to_y && close_to_z)) {
        return BvhNodeIntersection::Edge;
    }
    return BvhNodeIntersection::NotEdge;
}

func update_closests_hits(nb_hits: uint, new_hit: Hit, inout closests_hit: Hit[8])->bool {
    uint index = 0;
    let new_dist = new_hit.get_distance();

    while (index < min(8, nb_hits) && closests_hit[index].get_distance() < new_dist) {
        index++;
    }
    if (index < 8) {
        for (uint i = 8 - 2; i > index; i--) {
            closests_hit[i + 1] = closests_hit[i];
        }
        closests_hit[index] = new_hit;
        return true;
    }
    return false;
}

public func get_closests_hit(
    ray: Ray,
    nb_triangles: uint,
    inout closests_hit: Hit[8],
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    max_dist: float, 
    ) {
    uint nb_hits = 0;
    for(uint i=0; i<nb_triangles; i++){
        Hit cur_hit = ray_triangle_intersection(ray, i, triangles, models);
        if (cur_hit.did_hit == 0) continue;
        let cur_dist = cur_hit.get_distance();
        if (cur_dist > max_dist) continue;
        if(update_closests_hits(nb_hits, cur_hit, closests_hit)){
            nb_hits++;
            if (ray.is_shadow_ray) {
                return;
            }
        }
    }
}

public func get_closests_hit_bvh(
    bvh: RWStructuredBuffer<BvhNode>,
    ray: Ray,
    inout closests_hit: Hit[8],
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    inout color: float4,
    should_display_bvh: bool,
    bvh_depth_to_display: uint,
    max_dist: float, 
    ) {
    uint nb_hits = 0;

    // Create a stack for the node indices
    // At most, the stack size is the number of entries in the bvh
    // const uint STACK_SIZE = 2*nb_triangles - 1;
    const uint STACK_SIZE = 32;
    uint stack[STACK_SIZE];
    uint depth_stack[STACK_SIZE];
    uint stack_index = 0;
    // Push the root node onto the stack
    stack[stack_index] = 0;
    depth_stack[stack_index] = 0;
    stack_index++;
    // Iterate while the stack is not empty
    while (stack_index > 0) {
        // Pop the current node
        stack_index--;
        uint current_node_index = stack[stack_index];
        BvhNode current_node = bvh[current_node_index];
        uint current_depth = depth_stack[stack_index];
        // Check if the ray intersects the current BVH node's bounding box
        BvhNodeIntersection intersection_type = ray_bvh_intersection(ray, current_node);
        if (intersection_type != BvhNodeIntersection::None) {
            // Update the color if needed
            if (should_display_bvh && (bvh_depth_to_display == current_depth)) {
                switch(intersection_type){
                    case BvhNodeIntersection::NotEdge:
                        float4 bvh_color = float4(0.f, 0.8f, 0.f, 0.8f);
                        color = bvh_color;
                        break;
                    case BvhNodeIntersection::Edge:
                        float4 bvh_edge_color = float4(0.f, 1.f, 0.f, 0.8f);
                        color = bvh_edge_color;
                        break;
                    default:
                        break;
                }
            }

            // Check if the current node is a leaf
            if (current_node.is_leaf()) {
                Hit cur_hit = ray_triangle_intersection(ray, current_node.triangle_index, triangles, models);
                if (cur_hit.did_hit == 0) continue;
                let cur_dist = cur_hit.get_distance();
                if (cur_dist > max_dist) continue;
                if (update_closests_hits(nb_hits, cur_hit, closests_hit)) {
                    nb_hits++;
                    if (ray.is_shadow_ray) {
                        return;
                    }
                }
            } else {
                // Push the children onto the stack
                stack[stack_index] = current_node.left_child_index;
                depth_stack[stack_index] = current_depth + 1;
                stack_index++;
                stack[stack_index] = current_node.right_child_index;
                depth_stack[stack_index] = current_depth + 1;
                stack_index++;
            }
        }
    }

}

public func get_color(
    light: Light,
    bvh: RWStructuredBuffer<BvhNode>,
    bvh_type: uint,
    nb_triangles: uint,
    Hit[8] closests_hit,
    inout color: float4,
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    materials: RWStructuredBuffer<Material>,
    is_wireframe_on: bool,
    ) {
    if (closests_hit[0].did_hit == 0) return;

    let color_tmp = Lights.shade(
        light,
        closests_hit,
        BRDF::Lambert,
        triangles,
        models,
        materials,
        bvh,
        bvh_type,
        nb_triangles,
    );
    color = float4(color_tmp, 1.);

    // Wireframe color
    if (is_wireframe_on) {
        // Check distance to edges
        float wireframe_edges_width = 0.02;
        float4 wireframe_edges_color = float4(0.f, 0.f, 0.f, 1.f);
        if (closests_hit[0].coords.x < wireframe_edges_width
            || closests_hit[0].coords.y < wireframe_edges_width
            || closests_hit[0].coords.z < wireframe_edges_width) {
            color = wireframe_edges_color;
        }
    }
}

