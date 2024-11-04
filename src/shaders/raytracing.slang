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
[[vk::push_constant]]
cbuffer _PushConstants {
    uint _NbTriangles;
    uint _IsWireframeOn;
    uint _BvhType;
}





///////////////////////////////////////////////////////
//////////////////////// MAIN /////////////////////////
///////////////////////////////////////////////////////

//////////// DISPATCH COMPUTE
static const int THREAD_GROUP_SIZE_X = 32;
static const int THREAD_GROUP_SIZE_Y = 32;
static const int THREAD_GROUP_SIZE_Z = 1;

[shader("compute")]
[numthreads(THREAD_GROUP_SIZE_X, THREAD_GROUP_SIZE_Y, THREAD_GROUP_SIZE_Z)]
void main(uint3 thread_id : SV_DispatchThreadID) {
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

    // No bvh
    Hit closest_hit;
    closest_hit.did_hit = 0;
    get_closest_hit(ray, _NbTriangles, closest_hit, _Triangles, _Models);

    float4 color = float4(0.f, 0.f, 0.f, 1.f);
    if (_BvhType != BvhType::None) {
        color *= (_Bvhs[0].triangle_index + 1);
    }
    get_color(closest_hit, color, _Triangles, _Models, _Materials, (_IsWireframeOn != 0));

    _Framebuffer[texel_coord] = color;
}