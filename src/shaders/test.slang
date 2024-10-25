/// PUSH CONSTANTS
[[vk::push_constant]]
cbuffer _PushConstants {
    float4 data0;
    float4 data1;
    float4 data2;
    float4 data3;
}

//////////// FRAMEBUFFER
static const int DESCRIPTOR_SET_FRAMEBUFFER = 0;
static const int DESCRIPTOR_BINDING_FRAMEBUFFER = 0;
[[vk::binding(DESCRIPTOR_BINDING_FRAMEBUFFER, DESCRIPTOR_SET_FRAMEBUFFER)]]
RWTexture2D<float4> _Framebuffer;

//////////// DISPATCH COMPUTE
static const int THREAD_GROUP_SIZE_X = 16;
static const int THREAD_GROUP_SIZE_Y = 16;
static const int THREAD_GROUP_SIZE_Z = 1;

[shader("compute")]
[numthreads(THREAD_GROUP_SIZE_X, THREAD_GROUP_SIZE_Y, THREAD_GROUP_SIZE_Z)]
void main(uint3 thread_id: SV_DispatchThreadID) {
    uint image_width = 0;
    uint image_height = 0;
    _Framebuffer.GetDimensions(image_width, image_height);
    if (thread_id.x < image_width && thread_id.y < image_height) {
        if (thread_id.x % THREAD_GROUP_SIZE_X != 0 && thread_id.y % THREAD_GROUP_SIZE_Y != 0) {
            float4 color = float4(0.f, 0.f, 0.f, 1.f);
            color.x = float(thread_id.x) / image_width;
            color.y = float(thread_id.y) / image_height;
            _Framebuffer[thread_id.xy] = color;
        }
    }
}