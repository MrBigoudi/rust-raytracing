#version 460
#extension GL_EXT_samplerless_texture_functions : require
layout(row_major) uniform;
layout(row_major) buffer;

#line 12 0
layout(rgba32f)
layout(binding = 0)
uniform image2D _Framebuffer_0;


#line 6 1
struct _MatrixStorage_float4x4_ColMajorstd140_0
{
    vec4  data_0[4];
};


#line 10
struct Camera_std140_0
{
    _MatrixStorage_float4x4_ColMajorstd140_0 inv_view_0;
    vec4 eye_0;
    float plane_width_0;
    float plane_height_0;
    float plane_near_0;
};


#line 58 0
layout(binding = 0, set = 2)
layout(std140) uniform _S1
{
    _MatrixStorage_float4x4_ColMajorstd140_0 inv_view_0;
    vec4 eye_0;
    float plane_width_0;
    float plane_height_0;
    float plane_near_0;
}_Camera_0;

#line 75
struct PushConstant_std140_0
{
    uint nb_triangles_0;
    uint is_wireframe_on_0;
    uint bvh_type_0;
    uint should_display_bvh_0;
    uint bvh_depth_to_display_0;
    float current_time_0;
};


#line 79
layout(push_constant)
layout(std140) uniform _S2
{
    uint nb_triangles_0;
    uint is_wireframe_on_0;
    uint bvh_type_0;
    uint should_display_bvh_0;
    uint bvh_depth_to_display_0;
    float current_time_0;
}_PushConstants_0;

#line 7 2
struct Triangle_std430_0
{
    vec4 p0_0;
    vec4 p1_0;
    vec4 p2_0;
    uint model_index_0;
};


#line 18 0
layout(std430, binding = 1) buffer StructuredBuffer_Triangle_std430_t_0 {
    Triangle_std430_0 _data[];
} _Triangles_0;

#line 4 3
struct _MatrixStorage_float4x4_ColMajorstd430_0
{
    vec4  data_1[4];
};


#line 5
struct Model_std430_0
{
    _MatrixStorage_float4x4_ColMajorstd430_0 model_matrix_0;
    uint material_index_0;
};


#line 24 0
layout(std430, binding = 2) buffer StructuredBuffer_Model_std430_t_0 {
    Model_std430_0 _data[];
} _Models_0;

#line 17 4
struct Aabb_std430_0
{
    vec3 mins_0;
    float padding_1_0;
    vec3 maxs_0;
    float padding_2_0;
};



struct BvhNode_std430_0
{
    Aabb_std430_0 bounding_box_0;
    uint triangle_index_0;
    uint left_child_index_0;
    uint right_child_index_0;
    float padding_1_1;
};


#line 44 0
layout(std430, binding = 0, set = 1) buffer StructuredBuffer_BvhNode_std430_t_0 {
    BvhNode_std430_0 _data[];
} _Bvhs_0;

#line 4 5
struct Material_std430_0
{
    vec4 ambient_0;
};


#line 30 0
layout(std430, binding = 3) buffer StructuredBuffer_Material_std430_t_0 {
    Material_std430_0 _data[];
} _Materials_0;

#line 30
mat4x4 unpackStorage_0(_MatrixStorage_float4x4_ColMajorstd140_0 _S3)
{

#line 30
    return mat4x4(_S3.data_0[0][0], _S3.data_0[1][0], _S3.data_0[2][0], _S3.data_0[3][0], _S3.data_0[0][1], _S3.data_0[1][1], _S3.data_0[2][1], _S3.data_0[3][1], _S3.data_0[0][2], _S3.data_0[1][2], _S3.data_0[2][2], _S3.data_0[3][2], _S3.data_0[0][3], _S3.data_0[1][3], _S3.data_0[2][3], _S3.data_0[3][3]);
}


#line 30
mat4x4 unpackStorage_1(_MatrixStorage_float4x4_ColMajorstd430_0 _S4)
{

#line 30
    return mat4x4(_S4.data_1[0][0], _S4.data_1[1][0], _S4.data_1[2][0], _S4.data_1[3][0], _S4.data_1[0][1], _S4.data_1[1][1], _S4.data_1[2][1], _S4.data_1[3][1], _S4.data_1[0][2], _S4.data_1[1][2], _S4.data_1[2][2], _S4.data_1[3][2], _S4.data_1[0][3], _S4.data_1[1][3], _S4.data_1[2][3], _S4.data_1[3][3]);
}


#line 3 2
struct Triangle_0
{
    vec4 p0_0;
    vec4 p1_0;
    vec4 p2_0;
    uint model_index_0;
};


#line 3
Triangle_0 unpackStorage_2(Triangle_std430_0 _S5)
{

#line 3
    Triangle_0 _S6 = { _S5.p0_0, _S5.p1_0, _S5.p2_0, _S5.model_index_0 };

#line 3
    return _S6;
}


#line 6 6
struct Hit_0
{
    vec4 coords_0;
    uint did_hit_0;
    uint triangle_index_1;
};


#line 11
float Hit_get_distance_0(Hit_0 this_0)
{

#line 12
    return this_0.coords_0.w;
}


#line 13 4
struct Aabb_0
{
    vec3 mins_0;
    float padding_1_0;
    vec3 maxs_0;
    float padding_2_0;
};


#line 13
Aabb_0 unpackStorage_3(Aabb_std430_0 _S7)
{

#line 13
    Aabb_0 _S8 = { _S7.mins_0, _S7.padding_1_0, _S7.maxs_0, _S7.padding_2_0 };

#line 13
    return _S8;
}


#line 20
struct BvhNode_0
{
    Aabb_0 bounding_box_0;
    uint triangle_index_0;
    uint left_child_index_0;
    uint right_child_index_0;
    float padding_1_1;
};


#line 20
BvhNode_0 unpackStorage_4(BvhNode_std430_0 _S9)
{

#line 20
    BvhNode_0 _S10 = { unpackStorage_3(_S9.bounding_box_0), _S9.triangle_index_0, _S9.left_child_index_0, _S9.right_child_index_0, _S9.padding_1_1 };

#line 20
    return _S10;
}


#line 3 7
struct Ray_0
{
    vec4 origin_0;
    vec4 direction_0;
    bool is_shadow_ray_0;
};


#line 124 6
int ray_bvh_intersection_0(Ray_0 ray_0, BvhNode_0 bvh_node_0)
{
    vec3 _S11 = ray_0.direction_0.xyz;

#line 126
    vec3 inverse_ray_dir_0 = 1.0 / _S11;
    vec3 _S12 = ray_0.origin_0.xyz;

#line 127
    vec3 t1_0 = (bvh_node_0.bounding_box_0.mins_0 - _S12) * inverse_ray_dir_0;
    vec3 t2_0 = (bvh_node_0.bounding_box_0.maxs_0 - _S12) * inverse_ray_dir_0;

    float _S13 = t1_0.x;

#line 130
    float _S14 = t2_0.x;

#line 130
    float t_min_0 = min(_S13, _S14);
    float t_max_0 = max(_S13, _S14);
    bool close_to_x_0;

#line 132
    if(t_max_0 < 0.0)
    {

#line 132
        close_to_x_0 = true;

#line 132
    }
    else
    {

#line 132
        close_to_x_0 = t_min_0 > t_max_0;

#line 132
    }

#line 132
    if(close_to_x_0)
    {

#line 133
        return 0;
    }
    float _S15 = t1_0.y;

#line 135
    float _S16 = t2_0.y;

#line 135
    float t_min_1 = max(t_min_0, min(_S15, _S16));
    float t_max_1 = min(t_max_0, max(_S15, _S16));
    if(t_max_1 < 0.0)
    {

#line 137
        close_to_x_0 = true;

#line 137
    }
    else
    {

#line 137
        close_to_x_0 = t_min_1 > t_max_1;

#line 137
    }

#line 137
    if(close_to_x_0)
    {

#line 138
        return 0;
    }
    float _S17 = t1_0.z;

#line 140
    float _S18 = t2_0.z;

#line 140
    float t_min_2 = max(t_min_1, min(_S17, _S18));
    float t_max_2 = min(t_max_1, max(_S17, _S18));
    if(t_max_2 < 0.0)
    {

#line 142
        close_to_x_0 = true;

#line 142
    }
    else
    {

#line 142
        close_to_x_0 = t_min_2 > t_max_2;

#line 142
    }

#line 142
    if(close_to_x_0)
    {

#line 143
        return 0;
    }



    vec3 hit_point_0 = _S12 + _S11 * t_min_2;
    float _S19 = hit_point_0.x;
    if(abs(_S19 - bvh_node_0.bounding_box_0.mins_0.x) < 0.20000000298023224)
    {

#line 150
        close_to_x_0 = true;

#line 150
    }
    else
    {

#line 150
        close_to_x_0 = abs(_S19 - bvh_node_0.bounding_box_0.maxs_0.x) < 0.20000000298023224;

#line 150
    }
    float _S20 = hit_point_0.y;

#line 151
    bool close_to_y_0;
    if(abs(_S20 - bvh_node_0.bounding_box_0.mins_0.y) < 0.20000000298023224)
    {

#line 152
        close_to_y_0 = true;

#line 152
    }
    else
    {

#line 152
        close_to_y_0 = abs(_S20 - bvh_node_0.bounding_box_0.maxs_0.y) < 0.20000000298023224;

#line 152
    }
    float _S21 = hit_point_0.z;

#line 153
    bool close_to_z_0;
    if(abs(_S21 - bvh_node_0.bounding_box_0.mins_0.z) < 0.20000000298023224)
    {

#line 154
        close_to_z_0 = true;

#line 154
    }
    else
    {

#line 154
        close_to_z_0 = abs(_S21 - bvh_node_0.bounding_box_0.maxs_0.z) < 0.20000000298023224;

#line 154
    }
    bool _S22;

#line 155
    if(close_to_x_0)
    {

#line 155
        _S22 = close_to_y_0;

#line 155
    }
    else
    {

#line 155
        _S22 = false;

#line 155
    }

#line 155
    if(_S22)
    {

#line 155
        close_to_x_0 = true;

#line 155
    }
    else
    {

#line 155
        if(close_to_x_0)
        {

#line 155
            close_to_x_0 = close_to_z_0;

#line 155
        }
        else
        {

#line 155
            close_to_x_0 = false;

#line 155
        }

#line 155
    }

#line 155
    if(close_to_x_0)
    {

#line 155
        close_to_x_0 = true;

#line 155
    }
    else
    {

#line 155
        if(close_to_y_0)
        {

#line 155
            close_to_x_0 = close_to_z_0;

#line 155
        }
        else
        {

#line 155
            close_to_x_0 = false;

#line 155
        }

#line 155
    }

#line 155
    if(close_to_x_0)
    {

#line 156
        return 1;
    }
    return 2;
}


#line 29 4
bool BvhNode_is_leaf_0(BvhNode_0 this_1)
{

#line 30
    bool _S23;

#line 30
    if(this_1.left_child_index_0 == 0U)
    {

#line 30
        _S23 = this_1.right_child_index_0 == 0U;

#line 30
    }
    else
    {

#line 30
        _S23 = false;

#line 30
    }

#line 30
    return _S23;
}


#line 64 8
struct DirectionalLight_0
{
    vec3 direction_1;
    vec3 color_0;
    float intensity_0;
};


#line 69
DirectionalLight_0 DirectionalLight_x24init_0()
{

#line 69
    DirectionalLight_0 _S24;
    _S24.color_0 = vec3(1.0, 1.0, 1.0);
    _S24.intensity_0 = 1.0;
    _S24.direction_1 = normalize(vec3(1.0, 0.0, 0.0));

#line 69
    return _S24;
}


#line 92
struct PointLight_0
{
    vec3 origin_1;
    vec3 color_1;
    float intensity_1;
};


#line 97
PointLight_0 PointLight_x24init_0()
{

#line 97
    PointLight_0 _S25;
    _S25.color_1 = vec3(1.0, 1.0, 1.0);
    _S25.intensity_1 = 1.0;
    _S25.origin_1 = vec3(0.0, 0.0, 0.0);

#line 97
    return _S25;
}




vec3 PointLight_get_direction_0(PointLight_0 this_2, vec3 hit_position_0)
{

#line 104
    return normalize(this_2.origin_1 - hit_position_0);
}


#line 115
float PointLight_get_max_distance_0(PointLight_0 this_3, vec3 hit_position_1)
{

#line 116
    return length(this_3.origin_1 - hit_position_1);
}


#line 107
vec3 PointLight_get_color_0(PointLight_0 this_4)
{

#line 108
    return this_4.color_1;
}

float PointLight_get_intensity_0(PointLight_0 this_5)
{

#line 112
    return this_5.intensity_1;
}


#line 3 5
struct Material_0
{
    vec4 ambient_0;
};


#line 3
Material_0 unpackStorage_5(Material_std430_0 _S26)
{

#line 3
    Material_0 _S27 = { _S26.ambient_0 };

#line 3
    return _S27;
}


#line 15 6
vec3 Hit_get_barycentric_coordinates_0(Hit_0 this_6)
{

#line 16
    return vec3(this_6.coords_0.x, this_6.coords_0.y, this_6.coords_0.z);
}


#line 56
Ray_0 get_ray_0(vec2 _S28)
{

#line 56
    mat4x4 _S29 = unpackStorage_0(_Camera_0.inv_view_0);

#line 56
    vec4 _S30 = _Camera_0.eye_0;

#line 56
    float _S31 = _Camera_0.plane_width_0;

#line 56
    float _S32 = _Camera_0.plane_height_0;

#line 56
    float _S33 = _Camera_0.plane_near_0;

#line 11 7
    vec3 pos_view_space_0 = vec3(_S28 - 0.5, 1.0) * vec3(_S31, - _S32, _S33);
    vec4 pos_world_space_0 = (((vec4(pos_view_space_0, 1.0)) * (_S29)));

    Ray_0 ray_1;
    ray_1.origin_0 = _S30;
    ray_1.direction_0 = normalize(pos_world_space_0 - ray_1.origin_0);
    ray_1.direction_0[3] = 0.0;
    ray_1.is_shadow_ray_0 = false;
    return ray_1;
}


#line 161 6
vec3 Triangle_get_world_position_p0_0(Triangle_0 _S34)
{

#line 10 2
    vec4 _S35 = (((_S34.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S34.model_index_0)].model_matrix_0))));
    return _S35.xyz;
}


#line 11
vec3 Triangle_get_world_position_p1_0(Triangle_0 _S36)
{


    vec4 _S37 = (((_S36.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S36.model_index_0)].model_matrix_0))));
    return _S37.xyz;
}


#line 16
vec3 Triangle_get_world_position_p2_0(Triangle_0 _S38)
{


    vec4 _S39 = (((_S38.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S38.model_index_0)].model_matrix_0))));
    return _S39.xyz;
}


#line 21
Hit_0 ray_triangle_intersection_0(Ray_0 _S40, uint _S41)
{

#line 62 6
    Hit_0 hit_0;

    Triangle_0 _S42 = unpackStorage_2(_Triangles_0._data[uint(_S41)]);

    vec3 _S43 = Triangle_get_world_position_p0_0(_S42);
    vec3 _S44 = Triangle_get_world_position_p1_0(_S42);
    vec3 _S45 = Triangle_get_world_position_p2_0(_S42);

    vec3 e0_0 = _S44 - _S43;
    vec3 e1_0 = _S45 - _S43;

    vec3 w_0 = _S40.direction_0.xyz;
    vec3 o_0 = _S40.origin_0.xyz;

    vec3 tmp_0 = cross(e0_0, e1_0);
    if(length(tmp_0) == 0.0)
    {

#line 78
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    vec3 n_0 = normalize(tmp_0);
    vec3 q_0 = cross(w_0, e1_0);
    float a_0 = dot(e0_0, q_0);

    bool _S46;

#line 86
    if(!_S40.is_shadow_ray_0)
    {

#line 86
        _S46 = dot(n_0, w_0) >= 0.0;

#line 86
    }
    else
    {

#line 86
        _S46 = false;

#line 86
    }

#line 86
    if(_S46)
    {

#line 87
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }


    if(abs(a_0) < 9.99999997475242708e-07)
    {

#line 93
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    vec3 s_0 = (o_0 - _S43) / a_0;
    vec3 r_0 = cross(s_0, e0_0);

    float b0_0 = dot(s_0, q_0);
    float b1_0 = dot(r_0, w_0);
    float b2_0 = 1.0 - b0_0 - b1_0;
    if(b0_0 < 0.0)
    {

#line 103
        _S46 = true;

#line 103
    }
    else
    {

#line 103
        _S46 = b1_0 < 0.0;

#line 103
    }

#line 103
    if(_S46)
    {

#line 103
        _S46 = true;

#line 103
    }
    else
    {

#line 103
        _S46 = b2_0 < 0.0;

#line 103
    }

#line 103
    if(_S46)
    {

#line 104
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    float t_0 = dot(e1_0, r_0);
    if(t_0 < 0.00009999999747379)
    {

#line 109
        _S46 = true;

#line 109
    }
    else
    {

#line 109
        _S46 = t_0 > 1.0e+06;

#line 109
    }

#line 109
    if(_S46)
    {

#line 110
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    hit_0.coords_0[0] = b2_0;
    hit_0.coords_0[1] = b0_0;
    hit_0.coords_0[2] = b1_0;
    hit_0.coords_0[3] = t_0;
    hit_0.did_hit_0 = 1U;
    hit_0.triangle_index_1 = _S41;

    return hit_0;
}


#line 121
void get_closest_hit_0(Ray_0 _S47, uint _S48, inout Hit_0 _S49, float _S50)
{

#line 170
    uint i_0 = 0U;

#line 170
    for(;;)
    {

#line 170
        if(i_0 < _S48)
        {
        }
        else
        {

#line 170
            break;
        }

#line 171
        Hit_0 _S51 = ray_triangle_intersection_0(_S47, i_0);
        if(_S51.did_hit_0 == 0U)
        {

#line 172
            i_0 = i_0 + 1U;

#line 172
            continue;
        }

#line 173
        float _S52 = Hit_get_distance_0(_S51);
        if(_S52 > _S50)
        {

#line 174
            i_0 = i_0 + 1U;

#line 174
            continue;
        }

#line 175
        bool _S53;

#line 175
        if(_S49.did_hit_0 == 0U)
        {

#line 175
            _S53 = true;

#line 175
        }
        else
        {

#line 175
            _S53 = _S52 < 0.0;

#line 175
        }

#line 175
        if(_S53)
        {

#line 176
            _S49 = _S51;
            if(_S47.is_shadow_ray_0)
            {

#line 178
                return;
            }

#line 175
        }

#line 170
        i_0 = i_0 + 1U;

#line 170
    }

#line 182
    return;
}


#line 201
void get_closest_hit_bvh_0(Ray_0 _S54, inout Hit_0 _S55, inout vec4 _S56, bool _S57, uint _S58, float _S59)
{

#line 201
    uint  stack_0[32];



    stack_0[0U] = 0U;

#line 202
    uint  depth_stack_0[32];



    depth_stack_0[0U] = 0U;

#line 222
    const vec4 bvh_color_0 = vec4(0.0, 0.80000001192092896, 0.0, 0.80000001192092896);



    const vec4 bvh_edge_color_0 = vec4(0.0, 1.0, 0.0, 0.80000001192092896);

#line 226
    uint stack_index_0 = 1U;

#line 226
    for(;;)
    {

#line 209
        if(stack_index_0 > 0U)
        {
        }
        else
        {

#line 209
            break;
        }
        uint stack_index_1 = stack_index_0 - 1U;
        uint current_node_index_0 = stack_0[stack_index_1];
        BvhNode_0 _S60 = unpackStorage_4(_Bvhs_0._data[uint(current_node_index_0)]);
        uint current_depth_0 = depth_stack_0[stack_index_1];

        int intersection_type_0 = ray_bvh_intersection_0(_S54, _S60);
        if(intersection_type_0 != 0)
        {
            bool _S61;

#line 219
            if(_S57)
            {

#line 219
                _S61 = _S58 == current_depth_0;

#line 219
            }
            else
            {

#line 219
                _S61 = false;

#line 219
            }

#line 219
            if(_S61)
            {

#line 220
                switch(intersection_type_0)
                {
                case 2:
                    {

#line 223
                        _S56 = bvh_color_0;
                        break;
                    }
                case 1:
                    {

#line 227
                        _S56 = bvh_edge_color_0;
                        break;
                    }
                default:
                    {

#line 230
                        break;
                    }
                }

#line 219
            }

#line 235
            bool _S62 = BvhNode_is_leaf_0(_S60);

#line 235
            uint stack_index_2;

#line 235
            if(_S62)
            {

#line 236
                Hit_0 _S63 = ray_triangle_intersection_0(_S54, _S60.triangle_index_0);
                if(_S63.did_hit_0 == 0U)
                {

#line 237
                    stack_index_0 = stack_index_1;

#line 237
                    continue;
                }

#line 238
                float _S64 = Hit_get_distance_0(_S63);
                if(_S64 > _S59)
                {

#line 239
                    stack_index_0 = stack_index_1;

#line 239
                    continue;
                }

#line 240
                bool _S65;

#line 240
                if(_S55.did_hit_0 == 0U)
                {

#line 240
                    _S65 = true;

#line 240
                }
                else
                {

#line 240
                    _S65 = _S64 < 0.0;

#line 240
                }

#line 240
                if(_S65)
                {

#line 241
                    _S55 = _S63;
                    if(_S54.is_shadow_ray_0)
                    {

#line 243
                        return;
                    }

#line 240
                }

#line 240
                stack_index_2 = stack_index_1;

#line 240
            }
            else
            {

#line 248
                stack_0[stack_index_1] = _S60.left_child_index_0;
                uint _S66 = current_depth_0 + 1U;

#line 249
                depth_stack_0[stack_index_1] = _S66;
                uint stack_index_3 = stack_index_1 + 1U;
                stack_0[stack_index_3] = _S60.right_child_index_0;
                depth_stack_0[stack_index_3] = _S66;
                stack_index_2 = stack_index_3 + 1U;

#line 253
            }

#line 253
            stack_index_0 = stack_index_2;

#line 253
        }
        else
        {

#line 253
            stack_index_0 = stack_index_1;

#line 253
        }

#line 253
    }

#line 258
    return;
}


#line 258
vec3 Hit_get_world_position_0(Hit_0 _S67)
{

#line 23
    Triangle_0 _S68 = unpackStorage_2(_Triangles_0._data[uint(_S67.triangle_index_1)]);
    vec3 _S69 = Hit_get_barycentric_coordinates_0(_S67);
    vec3 _S70 = Triangle_get_world_position_p0_0(_S68);
    vec3 _S71 = Triangle_get_world_position_p1_0(_S68);
    vec3 _S72 = Triangle_get_world_position_p2_0(_S68);

    return _S69.x * _S70 + _S69.y * _S71 + _S69.z * _S72;
}


#line 29
vec3 Triangle_get_normal_0(Triangle_0 _S73)
{

#line 25 2
    vec3 p0_1 = (((_S73.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S73.model_index_0)].model_matrix_0)))).xyz;
    vec3 p1_1 = (((_S73.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S73.model_index_0)].model_matrix_0)))).xyz;
    vec3 p2_1 = (((_S73.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S73.model_index_0)].model_matrix_0)))).xyz;

    vec3 e0_1 = p1_1 - p0_1;
    vec3 e1_1 = p2_1 - p0_1;
    vec3 tmp_1 = cross(e0_1, e1_1);
    vec3 n_1 = normalize(tmp_1);
    return n_1;
}


#line 33
vec3 Hit_get_world_norm_0(Hit_0 _S74)
{

#line 36 6
    Triangle_0 _S75 = unpackStorage_2(_Triangles_0._data[uint(_S74.triangle_index_1)]);

    vec3 _S76 = Triangle_get_normal_0(_S75);

    vec3 _S77 = Hit_get_barycentric_coordinates_0(_S74);
    return normalize(_S77.x * _S76 + _S77.y * _S76 + _S77.z * _S76);
}


#line 41
vec3 Hit_get_ambient_0(Hit_0 _S78)
{

#line 49
    vec3 _S79 = Hit_get_barycentric_coordinates_0(_S78);
    Material_0 _S80 = unpackStorage_5(_Materials_0._data[uint(_Models_0._data[uint(_Triangles_0._data[uint(_S78.triangle_index_1)].model_index_0)].material_index_0)]);
    vec3 _S81 = vec3(_S80.ambient_0.x, _S80.ambient_0.y, _S80.ambient_0.z);
    return _S79.x * _S81 + _S79.y * _S81 + _S79.z * _S81;
}


#line 52
vec3 Lights_shade_0(PointLight_0 _S82, Hit_0 _S83, int _S84, uint _S85, uint _S86)
{

#line 20 8
    switch(_S84)
    {
    case 0:
        {

#line 22
            vec3 _S87 = Hit_get_world_position_0(_S83);
            float _S88 = PointLight_get_max_distance_0(_S82, _S87);

            Ray_0 shadow_ray_0;
            shadow_ray_0.origin_0 = vec4(_S87, 1.0);
            vec3 _S89 = PointLight_get_direction_0(_S82, _S87);

#line 27
            shadow_ray_0.direction_0 = vec4(_S89, 0.0);
            shadow_ray_0.is_shadow_ray_0 = true;
            Hit_0 closest_hit_0;
            closest_hit_0.did_hit_0 = 0U;


            if(int(_S85) == 0)
            {

#line 34
                get_closest_hit_0(shadow_ray_0, _S86, closest_hit_0, _S88);

#line 33
            }
            else
            {
                vec4 bvh_color_dummy_0 = vec4(0.0);
                get_closest_hit_bvh_0(shadow_ray_0, closest_hit_0, bvh_color_dummy_0, false, 0U, _S88);

#line 33
            }

#line 39
            if(closest_hit_0.did_hit_0 == 0U)
            {

#line 40
                vec3 _S90 = Hit_get_world_norm_0(_S83);
                float _S91 = clamp(dot(_S90, shadow_ray_0.direction_0.xyz), 0.0, 1.0);
                float _S92 = PointLight_get_intensity_0(_S82);

#line 42
                float _S93 = _S91 * _S92;

#line 42
                vec3 _S94 = PointLight_get_color_0(_S82);

#line 42
                vec3 _S95 = _S93 * _S94;

#line 42
                vec3 _S96 = Hit_get_ambient_0(_S83);

#line 42
                return _S95 * _S96;
            }
            else
            {

#line 44
                return vec3(0.0, 0.0, 0.0);
            }

#line 44
        }
    case 1:
        {


            return vec3(0.0);
        }
    default:
        {

#line 52
            return vec3(0.0);
        }
    }

#line 52
}


#line 52
void get_color_0(PointLight_0 _S97, uint _S98, uint _S99, Hit_0 _S100, inout vec4 _S101, bool _S102)
{

#line 272 6
    if(_S100.did_hit_0 == 0U)
    {

#line 272
        return;
    }
    vec3 _S103 = Lights_shade_0(_S97, _S100, 0, _S98, _S99);

#line 285
    _S101 = vec4(_S103, 1.0);


    if(_S102)
    {

        const vec4 wireframe_edges_color_0 = vec4(0.0, 0.0, 0.0, 1.0);
        bool _S104;
        if(_S100.coords_0.x < 0.01999999955296516)
        {

#line 293
            _S104 = true;

#line 293
        }
        else
        {

#line 293
            _S104 = _S100.coords_0.y < 0.01999999955296516;

#line 293
        }
        if(_S104)
        {

#line 294
            _S104 = true;

#line 294
        }
        else
        {

#line 294
            _S104 = _S100.coords_0.z < 0.01999999955296516;

#line 294
        }

#line 292
        if(_S104)
        {

            _S101 = wireframe_edges_color_0;

#line 292
        }

#line 288
    }

#line 298
    return;
}


#line 96 0
layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;
void main()
{

#line 97
    uvec2 texel_coord_0 = gl_GlobalInvocationID.xy;

    const uvec2 _S105 = uvec2(0U, 0U);

#line 99
    uvec2 image_size_0 = _S105;
    ((image_size_0[0]) = imageSize((_Framebuffer_0)).x), ((image_size_0[1]) = imageSize((_Framebuffer_0)).y);

    const vec2 _S106 = vec2(0.0);

#line 102
    vec2 pixel_position_0 = _S106;
    float _S107 = float(texel_coord_0.x) / float(image_size_0.x);

#line 103
    pixel_position_0[0] = _S107;
    float _S108 = float(texel_coord_0.y) / float(image_size_0.y);

#line 104
    pixel_position_0[1] = _S108;
    bool _S109;

#line 105
    if(pixel_position_0.x >= 1.0)
    {

#line 105
        _S109 = true;

#line 105
    }
    else
    {

#line 105
        _S109 = pixel_position_0.x < 0.0;

#line 105
    }
    if(_S109)
    {

#line 106
        _S109 = true;

#line 106
    }
    else
    {

#line 106
        _S109 = pixel_position_0.y >= 1.0;

#line 106
    }

#line 106
    if(_S109)
    {

#line 106
        _S109 = true;

#line 106
    }
    else
    {

#line 106
        _S109 = pixel_position_0.y < 0.0;

#line 106
    }

#line 105
    if(_S109)
    {
        return;
    }

    Ray_0 _S110 = get_ray_0(pixel_position_0);


    const vec4 _S111 = vec4(0.0, 0.0, 0.0, 0.0);

#line 113
    vec4 bvh_color_1 = _S111;
    Hit_0 closest_hit_1;
    closest_hit_1.did_hit_0 = 0U;
    if(int(_PushConstants_0.bvh_type_0) == 0)
    {

#line 117
        get_closest_hit_0(_S110, _PushConstants_0.nb_triangles_0, closest_hit_1, 3.4028234663852886e+38);

#line 116
    }
    else
    {
        bool should_display_bvh_1 = _PushConstants_0.should_display_bvh_0 != 0U;
        get_closest_hit_bvh_0(_S110, closest_hit_1, bvh_color_1, should_display_bvh_1, _PushConstants_0.bvh_depth_to_display_0, 3.4028234663852886e+38);

#line 116
    }

#line 127
    DirectionalLight_0 _S112 = DirectionalLight_x24init_0();



    float sun_angle_xz_0 = _PushConstants_0.current_time_0 * 0.00009999999747379;

#line 139
    PointLight_0 _S113 = PointLight_x24init_0();

#line 139
    PointLight_0 point_light_0 = _S113;
    point_light_0.intensity_1 = 0.5;
    const vec3 _S114 = vec3(0.0, 0.0, 0.0);

#line 141
    point_light_0.origin_1 = _S114;

    float factor_0 = 0.5 * pixel_position_0.y + 1.0;
    vec4 _S115 = vec4(1.0 - factor_0) + factor_0 * vec4(0.5, 0.69999998807907104, 1.0, 1.0);

#line 144
    vec4 color_2 = _S115;

    get_color_0(point_light_0, _PushConstants_0.bvh_type_0, _PushConstants_0.nb_triangles_0, closest_hit_1, color_2, _PushConstants_0.is_wireframe_on_0 != 0U);

#line 161
    float _S116 = bvh_color_1.w;
    vec4 _S117 = _S116 * bvh_color_1 + (1.0 - _S116) * color_2;

#line 162
    color_2 = _S117;
    color_2[3] = 1.0;

    imageStore((_Framebuffer_0), (ivec2(texel_coord_0)), color_2);
    return;
}

