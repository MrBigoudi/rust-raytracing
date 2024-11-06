implementing raytracer;

public struct Ray {
    public float4 origin;
    public float4 direction;
};

public func get_ray(pos: float2, camera: Camera) -> Ray { // pos between 0 and 1
    // pos.x and pos.y are initially in [0;1]
    float3 pos_view_space = float3(pos - 0.5f, 1.f) * float3(camera.plane_width, -camera.plane_height, camera.plane_near);
    float4 pos_world_space = mul(camera.inv_view, float4(pos_view_space, 1.f));

    Ray ray;
    ray.origin = camera.eye;
    ray.direction = normalize(pos_world_space - ray.origin);
    ray.direction.w = 0.;
    return ray;
}