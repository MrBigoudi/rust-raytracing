implementing raytracer;

import scene.scene; 


public struct Hit {
    public float4 coords; // (b0, b1, b2, t)
    public uint did_hit;
    public uint triangle_index;

    public func get_distance()->float {
        return coords.w;
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

    float3 p0 = mul(models[triangle.model_index].model_matrix, triangle.p0).xyz;
    float3 p1 = mul(models[triangle.model_index].model_matrix, triangle.p1).xyz;
    float3 p2 = mul(models[triangle.model_index].model_matrix, triangle.p2).xyz;

    float3 e0 = p1 - p0;
    float3 e1 = p2 - p0;

    float3 w = ray.direction.xyz;
    float3 o = ray.origin.xyz;

    float3 tmp = cross(e1, e0);
    if (length(tmp) == 0.f) {
        hit.did_hit = 0;
        return hit;
    }

    float3 n = normalize(tmp);
    float3 q = cross(w, e1);
    float a = dot(e0, q);

    if (dot(n, w) <= 0.) {
        hit.did_hit = 0;
        return hit;
    }
    float epsilon = 1e-4;
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
    if (t < 1e-3 || t > 1e6) {
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

    // TODO: Check if border
    float threshold = 0.05;
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

public func get_closest_hit(
    ray: Ray,
    nb_triangles: uint,
    inout closest_hit: Hit,
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    ){
    for(uint i=0; i<nb_triangles; i++){
        Hit cur_hit = ray_triangle_intersection(ray, i, triangles, models);
        if(cur_hit.did_hit == 0) continue;
        if(closest_hit.did_hit == 0 || cur_hit.get_distance() < closest_hit.get_distance()){
            closest_hit = cur_hit;
        }
    }
}

public func get_closest_hit_bvh(
    bvh: RWStructuredBuffer<BvhNode>,
    ray: Ray,
    inout closest_hit: Hit,
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    inout color: float4,
    should_display_bvh: bool,
    bvh_depth_to_display: uint,
    ) {

    // Create a stack for the node indices
    // At most, the stack size is the number of entries in the bvh
    // const uint STACK_SIZE = 2*nb_triangles - 1;
    const uint STACK_SIZE = 1024;
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
                        float4 bvh_color = float4(0.5f, 0.f, 0.5f, 0.1f);
                        color = bvh_color;
                        break;
                    case BvhNodeIntersection::Edge:
                        float4 bvh_edge_color = float4(0.7f, 0.f, 0.7f, 0.1f);
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
                if (closest_hit.did_hit == 0 || cur_hit.get_distance() < closest_hit.get_distance()) {
                    closest_hit = cur_hit;
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
    hit: Hit,
    inout color: float4,
    triangles: RWStructuredBuffer<Triangle>,
    models: RWStructuredBuffer<Model>,
    materials: RWStructuredBuffer<Material>,
    is_wireframe_on: bool,
    ){
    if(hit.did_hit == 0) return;
    Triangle hit_triangle = triangles[hit.triangle_index];
    // color = materials[models[hit_triangle.model_index].material_index].ambient;
    // color = float4(0., dot(hit_triangle.get_normal(models), float3(1.,0.,0.)), 0., 1.);
    // color = float4(0., hit_triangle.p0.y, 0., 1.);
    color = float4(hit_triangle.get_normal(models), 1.);

    // Wireframe color
    if (is_wireframe_on) {
        // Check distance to edges
        float wireframe_edges_width = 0.02;
        float4 wireframe_edges_color = float4(0.f, 0.f, 0.f, 1.f);
            if (hit.coords.x < wireframe_edges_width
            || hit.coords.y < wireframe_edges_width
            || hit.coords.z < wireframe_edges_width) {
            color = wireframe_edges_color;
        }
    }
}

