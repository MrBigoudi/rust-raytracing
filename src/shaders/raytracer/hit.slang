implementing raytracer;

import scene.scene; 


public struct Hit {
    public float4 coords; // (b0, b1, b2, t)
    public uint did_hit;
    public uint triangle_index;
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

    // CCW
    if (dot(n, w) >= 0.) {
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
        if(closest_hit.did_hit == 0 || cur_hit.coords.w < closest_hit.coords.w){
            closest_hit = cur_hit;
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
    color += materials[models[hit_triangle.model_index].material_index].ambient;

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