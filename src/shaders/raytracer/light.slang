implementing raytracer;

public enum BRDF {
    Lambert,
    GGX,
}

public struct Lights {
    static public func shade(
        light: Light,
        closests_hit: Hit[8],
        brdf_type: BRDF,
        triangles: RWStructuredBuffer<Triangle>,
        models: RWStructuredBuffer<Model>,
        materials: RWStructuredBuffer<Material>,
        bvh: RWStructuredBuffer<BvhNode>,
        bvh_type: uint,
        nb_triangles: uint,
    ) ->float3 {
        let closest_hit = closests_hit[0];
        switch (brdf_type) {
        case BRDF.Lambert:
            let hit_position = closest_hit.get_world_position(triangles, models);
                let max_dist = light.get_max_distance(hit_position);

                Ray shadow_ray;
                shadow_ray.origin = float4(hit_position, 1.);
                shadow_ray.direction = float4(light.get_direction(hit_position), 0.);
                shadow_ray.is_shadow_ray = true;

                Hit[8] closests_hit;
                closests_hit[0].did_hit = 0;
                
                // Shadow rays
                if (bvh_type == BvhType::None) {
                    get_closests_hit(shadow_ray, nb_triangles, closests_hit, triangles, models, max_dist);
                } else {
                    float4 bvh_color_dummy = float4(0.);
                    get_closests_hit_bvh(bvh, shadow_ray, closests_hit, triangles, models, bvh_color_dummy, false, 0, max_dist);
                }
                if (closests_hit[0].did_hit == 0) { // Not in shadow
                    let hit_normal = closest_hit.get_world_norm(triangles, models);
                    let diffuse_factor = clamp(dot(hit_normal, shadow_ray.direction.xyz), 0., 1.);
                    return diffuse_factor * light.get_intensity() * light.get_color() * closest_hit.get_ambient(triangles, models, materials);
                } else { // In shadow
                    return float3(0., 0., 0.);
                }

            // TODO: GGX brdf
            case BRDF.GGX:
                return float3(0.);

            default:
                return float3(0.);
        }
    }
}

public interface Light {
    public func get_direction(hit_position: float3)->float3;
    public func get_max_distance(hit_position: float3)->float;
    public func get_color() -> float3;
    public func get_intensity() -> float;
}

public struct DirectionalLight : Light {
    public float3 direction;
    public float3 color;
    public float intensity;

    public __init() {
        color = float3(1., 1., 1.);
        intensity = 1.;
        direction = normalize(float3(1., 0., 0.));
    }

    public func get_direction(hit_position: float3) -> float3 {
        return normalize(-direction);
    }

    public func get_color() -> float3 {
        return color;
    }

    public func get_intensity() -> float {
        return intensity;
    }

    public func get_max_distance(hit_position: float3) -> float {
        return float.maxValue;
    }
}

public struct PointLight : Light {
    public float3 origin;
    public float3 color;
    public float intensity;

    public __init() {
        color = float3(1., 1., 1.);
        intensity = 1.;
        origin = float3(0., 0., 0.);
    }

    public func get_direction(hit_position: float3) -> float3 {
        return normalize(origin - hit_position);
    }

    public func get_color() -> float3 {
        return color;
    }

    public func get_intensity()->float {
        return intensity;
    }

    public func get_max_distance(hit_position: float3)->float {
        return length(origin-hit_position);
    }
}