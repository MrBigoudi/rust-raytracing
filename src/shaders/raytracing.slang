import scene.scene; 
import raytracer.raytracer;

///////////////////////////////////////////////////////
//////////////////////// SET 0 ////////////////////////
///////////////////////////////////////////////////////

//////////// FRAMEBUFFER
static const int DESCRIPTOR_SET_FRAMEBUFFER = 0;
static const int DESCRIPTOR_BINDING_FRAMEBUFFER = 0;
[[vk::binding(DESCRIPTOR_BINDING_FRAMEBUFFER, DESCRIPTOR_SET_FRAMEBUFFER)]]
RWTexture2D<float4> _Framebuffer;

//////////// TRIANGLES
static const int DESCRIPTOR_SET_TRIANGLES = 0;
static const int DESCRIPTOR_BINDING_TRIANGLES = 1;
[[vk::binding(DESCRIPTOR_BINDING_TRIANGLES, DESCRIPTOR_SET_TRIANGLES)]]
RWStructuredBuffer<Triangle> _Triangles;

//////////// MODELS
static const int DESCRIPTOR_SET_MODELS = 0;
static const int DESCRIPTOR_BINDING_MODELS = 2;
[[vk::binding(DESCRIPTOR_BINDING_MODELS, DESCRIPTOR_SET_MODELS)]]
RWStructuredBuffer<Model> _Models;

//////////// MATERIALS
static const int DESCRIPTOR_SET_MATERIALS = 0;
static const int DESCRIPTOR_BINDING_MATERIALS = 3;
[[vk::binding(DESCRIPTOR_BINDING_MATERIALS, DESCRIPTOR_SET_MATERIALS)]]
RWStructuredBuffer<Material> _Materials;





///////////////////////////////////////////////////////
//////////////////////// SET 1 ////////////////////////
///////////////////////////////////////////////////////

//////////// BVHS
static const int DESCRIPTOR_SET_BVHS = 1;
static const int DESCRIPTOR_BINDING_BVHS = 0;
[[vk::binding(DESCRIPTOR_BINDING_BVHS, DESCRIPTOR_SET_BVHS)]]
RWStructuredBuffer<BvhNode> _Bvhs;





///////////////////////////////////////////////////////
//////////////////////// SET 2 ////////////////////////
///////////////////////////////////////////////////////

//////////// CAMERA
static const int DESCRIPTOR_SET_CAMERA_UBO = 2;
static const int DESCRIPTOR_BINDING_CAMERA_UBO = 0;
[[vk::binding(DESCRIPTOR_BINDING_CAMERA_UBO, DESCRIPTOR_SET_CAMERA_UBO)]]
ConstantBuffer<Camera> _Camera;





///////////////////////////////////////////////////////
//////////////////////// CSTS /////////////////////////
///////////////////////////////////////////////////////

//////////// PUSH CONSTANTS
struct PushConstant {
    uint nb_triangles;
    uint is_wireframe_on;
    uint bvh_type;
    uint should_display_bvh;
    uint bvh_depth_to_display;
    float current_time;
}

[[vk::push_constant]]
ConstantBuffer<PushConstant> _PushConstants;





///////////////////////////////////////////////////////
//////////////////////// MAIN /////////////////////////
///////////////////////////////////////////////////////

//////////// DISPATCH COMPUTE
static const int THREAD_GROUP_SIZE_X = 32;
static const int THREAD_GROUP_SIZE_Y = 32;
static const int THREAD_GROUP_SIZE_Z = 1;

[shader("compute")]
[numthreads(THREAD_GROUP_SIZE_X, THREAD_GROUP_SIZE_Y, THREAD_GROUP_SIZE_Z)]
void main(uint3 thread_id: SV_DispatchThreadID) {
    uint2 texel_coord = thread_id.xy;

    uint2 image_size = uint2(0, 0);
    _Framebuffer.GetDimensions(image_size.x, image_size.y);

    float2 pixel_position = float2(0.f);
    pixel_position.x = float(texel_coord.x) / float(image_size.x);
    pixel_position.y = float(texel_coord.y) / float(image_size.y);
    if (pixel_position.x >= 1. || pixel_position.x < 0 
        || pixel_position.y >= 1. || pixel_position.y < 0) {
        return;
    }

    Ray ray = get_ray(pixel_position, _Camera);
    // Ray ray = get_ray(pixel_position, Camera());

    float4 bvh_color = float4(0.f, 0.f, 0.f, 0.f);
    Hit closest_hit;
    closest_hit.did_hit = 0;
    if (_PushConstants.bvh_type == BvhType::None) {
        get_closest_hit(ray, _PushConstants.nb_triangles, closest_hit, _Triangles, _Models);
    } else {
        bool should_display_bvh = (_PushConstants.should_display_bvh != 0);
        get_closest_hit_bvh(_Bvhs, ray, closest_hit, _Triangles, _Models,
            bvh_color, should_display_bvh, _PushConstants.bvh_depth_to_display
        );
    }

    // Adjust light direction
    DirectionalLight sun;
    // Orbit arround center
    float sun_speed_xz = 0.001f;
    float sun_speed_y = sun_speed_xz / 3.f;
    float sun_angle_xz = _PushConstants.current_time * sun_speed_xz;
    float sun_angle_y = _PushConstants.current_time * sun_speed_y;
    // sun.direction = normalize(float3(cos(sun_angle_xz), sin(sun_angle_y), sin(sun_angle_xz)));
    sun.direction = normalize(float3(
        cos(sun_angle_xz), 
        0.f, 
        sin(sun_angle_xz)
    ));
    
    float4 color = float4(0.);
    
    get_color(
        sun, 
        _Bvhs,
        _PushConstants.bvh_type,
        _PushConstants.nb_triangles,
        closest_hit,
        color,
        _Triangles, 
        _Models, 
        _Materials, 
        (_PushConstants.is_wireframe_on != 0)
    );

    // Alpha blending assuming there is no transparent material in the scene
    let alpha = bvh_color.w;
    color = alpha * bvh_color + (1 - alpha) * color;
    color.w = 1.f;

    _Framebuffer[texel_coord] = color;
}