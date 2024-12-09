implementing raytracer;

public enum BRDF {
    Lambert,
}

public struct DirectionalLight {
    public float3 color;
    public float intensity;
    public float3 direction;

    public __init() {
        color = float3(1., 1., 1.);
        intensity = 1.;
        direction = normalize(float3(1., 0., 0.));
    }

    public func shade(
        incoming_hit: Hit,
        brdf_type: BRDF,
        triangles: RWStructuredBuffer<Triangle>,
        models: RWStructuredBuffer<Model>,
        materials: RWStructuredBuffer<Material>,
        bvh: RWStructuredBuffer<BvhNode>,
        bvh_type: uint,
        nb_triangles: uint,
    ) ->float3 {
        // Don't do nothing
        return incoming_hit.get_ambient(triangles, models, materials);
        switch (brdf_type) {
            case BRDF.Lambert:
                let hit_position = incoming_hit.get_world_position(triangles, models);
                // if (dot(hit_normal, direction) > 0) {
                //     return float3(0.);
                // }
                Ray shadow_ray;
                shadow_ray.origin = float4(hit_position, 0.);
                shadow_ray.direction = float4(-direction, 0.);
                shadow_ray.is_shadow_ray = true;
                Hit closest_hit;
                closest_hit.did_hit = 0;
                if (bvh_type == BvhType::None) {
                    get_closest_hit(shadow_ray, nb_triangles, closest_hit, triangles, models);
                } else {
                    float4 bvh_color_dummy = float4(0.);
                    get_closest_hit_bvh(bvh, shadow_ray, closest_hit, triangles, models, bvh_color_dummy, false, 0);
                }
                if (closest_hit.did_hit == 0) { // Not in shadow
                    let hit_normal = incoming_hit.get_world_norm(triangles, models);
                    let diffuse_factor = clamp(dot(hit_normal, direction), 0., 1.);
                    return diffuse_factor * intensity * color * incoming_hit.get_ambient(triangles, models, materials);
                } else { // In shadow
                    return float3(0.);
                }
            default:
                // TODO: GGX brdf
                return float3(0.);
        }
    }
}