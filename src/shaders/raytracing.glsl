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

#line 74
struct PushConstant_std140_0
{
    uint nb_triangles_0;
    uint is_wireframe_on_0;
    uint bvh_type_0;
    uint should_display_bvh_0;
    uint bvh_depth_to_display_0;
};


#line 78
layout(push_constant)
layout(std140) uniform _S2
{
    uint nb_triangles_0;
    uint is_wireframe_on_0;
    uint bvh_type_0;
    uint should_display_bvh_0;
    uint bvh_depth_to_display_0;
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


#line 120 6
int ray_bvh_intersection_0(Ray_0 ray_0, BvhNode_0 bvh_node_0)
{
    vec3 _S11 = ray_0.direction_0.xyz;

#line 122
    vec3 inverse_ray_dir_0 = 1.0 / _S11;
    vec3 _S12 = ray_0.origin_0.xyz;

#line 123
    vec3 t1_0 = (bvh_node_0.bounding_box_0.mins_0 - _S12) * inverse_ray_dir_0;
    vec3 t2_0 = (bvh_node_0.bounding_box_0.maxs_0 - _S12) * inverse_ray_dir_0;

    float _S13 = t1_0.x;

#line 126
    float _S14 = t2_0.x;

#line 126
    float t_min_0 = min(_S13, _S14);
    float t_max_0 = max(_S13, _S14);
    bool close_to_x_0;

#line 128
    if(t_max_0 < 0.0)
    {

#line 128
        close_to_x_0 = true;

#line 128
    }
    else
    {

#line 128
        close_to_x_0 = t_min_0 > t_max_0;

#line 128
    }

#line 128
    if(close_to_x_0)
    {

#line 129
        return 0;
    }
    float _S15 = t1_0.y;

#line 131
    float _S16 = t2_0.y;

#line 131
    float t_min_1 = max(t_min_0, min(_S15, _S16));
    float t_max_1 = min(t_max_0, max(_S15, _S16));
    if(t_max_1 < 0.0)
    {

#line 133
        close_to_x_0 = true;

#line 133
    }
    else
    {

#line 133
        close_to_x_0 = t_min_1 > t_max_1;

#line 133
    }

#line 133
    if(close_to_x_0)
    {

#line 134
        return 0;
    }
    float _S17 = t1_0.z;

#line 136
    float _S18 = t2_0.z;

#line 136
    float t_min_2 = max(t_min_1, min(_S17, _S18));
    float t_max_2 = min(t_max_1, max(_S17, _S18));
    if(t_max_2 < 0.0)
    {

#line 138
        close_to_x_0 = true;

#line 138
    }
    else
    {

#line 138
        close_to_x_0 = t_min_2 > t_max_2;

#line 138
    }

#line 138
    if(close_to_x_0)
    {

#line 139
        return 0;
    }



    vec3 hit_point_0 = _S12 + _S11 * t_min_2;
    float _S19 = hit_point_0.x;
    if(abs(_S19 - bvh_node_0.bounding_box_0.mins_0.x) < 0.05000000074505806)
    {

#line 146
        close_to_x_0 = true;

#line 146
    }
    else
    {

#line 146
        close_to_x_0 = abs(_S19 - bvh_node_0.bounding_box_0.maxs_0.x) < 0.05000000074505806;

#line 146
    }
    float _S20 = hit_point_0.y;

#line 147
    bool close_to_y_0;
    if(abs(_S20 - bvh_node_0.bounding_box_0.mins_0.y) < 0.05000000074505806)
    {

#line 148
        close_to_y_0 = true;

#line 148
    }
    else
    {

#line 148
        close_to_y_0 = abs(_S20 - bvh_node_0.bounding_box_0.maxs_0.y) < 0.05000000074505806;

#line 148
    }
    float _S21 = hit_point_0.z;

#line 149
    bool close_to_z_0;
    if(abs(_S21 - bvh_node_0.bounding_box_0.mins_0.z) < 0.05000000074505806)
    {

#line 150
        close_to_z_0 = true;

#line 150
    }
    else
    {

#line 150
        close_to_z_0 = abs(_S21 - bvh_node_0.bounding_box_0.maxs_0.z) < 0.05000000074505806;

#line 150
    }
    bool _S22;

#line 151
    if(close_to_x_0)
    {

#line 151
        _S22 = close_to_y_0;

#line 151
    }
    else
    {

#line 151
        _S22 = false;

#line 151
    }

#line 151
    if(_S22)
    {

#line 151
        close_to_x_0 = true;

#line 151
    }
    else
    {

#line 151
        if(close_to_x_0)
        {

#line 151
            close_to_x_0 = close_to_z_0;

#line 151
        }
        else
        {

#line 151
            close_to_x_0 = false;

#line 151
        }

#line 151
    }

#line 151
    if(close_to_x_0)
    {

#line 151
        close_to_x_0 = true;

#line 151
    }
    else
    {

#line 151
        if(close_to_y_0)
        {

#line 151
            close_to_x_0 = close_to_z_0;

#line 151
        }
        else
        {

#line 151
            close_to_x_0 = false;

#line 151
        }

#line 151
    }

#line 151
    if(close_to_x_0)
    {

#line 152
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


#line 7 8
struct DirectionalLight_0
{
    vec3 color_0;
    float intensity_0;
    vec3 direction_1;
};


#line 12
DirectionalLight_0 DirectionalLight_x24init_0()
{

#line 12
    DirectionalLight_0 _S24;
    _S24.color_0 = vec3(1.0, 1.0, 1.0);
    _S24.intensity_0 = 1.0;
    _S24.direction_1 = normalize(vec3(1.0, 0.0, 0.0));

#line 12
    return _S24;
}


#line 3 5
struct Material_0
{
    vec4 ambient_0;
};


#line 3
Material_0 unpackStorage_5(Material_std430_0 _S25)
{

#line 3
    Material_0 _S26 = { _S25.ambient_0 };

#line 3
    return _S26;
}


#line 15 6
vec3 Hit_get_barycentric_coordinates_0(Hit_0 this_2)
{

#line 16
    return vec3(this_2.coords_0.x, this_2.coords_0.y, this_2.coords_0.z);
}


#line 53
Ray_0 get_ray_0(vec2 _S27)
{

#line 53
    mat4x4 _S28 = unpackStorage_0(_Camera_0.inv_view_0);

#line 53
    vec4 _S29 = _Camera_0.eye_0;

#line 53
    float _S30 = _Camera_0.plane_width_0;

#line 53
    float _S31 = _Camera_0.plane_height_0;

#line 53
    float _S32 = _Camera_0.plane_near_0;

#line 11 7
    vec3 pos_view_space_0 = vec3(_S27 - 0.5, 1.0) * vec3(_S30, - _S31, _S32);
    vec4 pos_world_space_0 = (((vec4(pos_view_space_0, 1.0)) * (_S28)));

    Ray_0 ray_1;
    ray_1.origin_0 = _S29;
    ray_1.direction_0 = normalize(pos_world_space_0 - ray_1.origin_0);
    ray_1.direction_0[3] = 0.0;
    ray_1.is_shadow_ray_0 = false;
    return ray_1;
}


#line 157 6
Hit_0 ray_triangle_intersection_0(Ray_0 _S33, uint _S34)
{

#line 59
    Hit_0 hit_0;

    Triangle_0 _S35 = unpackStorage_2(_Triangles_0._data[uint(_S34)]);

    vec3 p0_1 = (((_S35.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S35.model_index_0)].model_matrix_0)))).xyz;
    vec3 p1_1 = (((_S35.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S35.model_index_0)].model_matrix_0)))).xyz;
    vec3 p2_1 = (((_S35.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S35.model_index_0)].model_matrix_0)))).xyz;

    vec3 e0_0 = p1_1 - p0_1;
    vec3 e1_0 = p2_1 - p0_1;

    vec3 w_0 = _S33.direction_0.xyz;
    vec3 o_0 = _S33.origin_0.xyz;

    vec3 tmp_0 = cross(e0_0, e1_0);
    if(length(tmp_0) == 0.0)
    {

#line 75
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    vec3 n_0 = normalize(tmp_0);
    vec3 q_0 = cross(w_0, e1_0);
    float a_0 = dot(e0_0, q_0);

    bool _S36;

#line 83
    if(!_S33.is_shadow_ray_0)
    {

#line 83
        _S36 = dot(n_0, w_0) >= 0.0;

#line 83
    }
    else
    {

#line 83
        _S36 = false;

#line 83
    }

#line 83
    if(_S36)
    {

#line 84
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    if(abs(a_0) < 0.00100000004749745)
    {

#line 89
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    vec3 s_0 = (o_0 - p0_1) / a_0;
    vec3 r_0 = cross(s_0, e0_0);

    float b0_0 = dot(s_0, q_0);
    float b1_0 = dot(r_0, w_0);
    float b2_0 = 1.0 - b0_0 - b1_0;
    if(b0_0 < 0.0)
    {

#line 99
        _S36 = true;

#line 99
    }
    else
    {

#line 99
        _S36 = b1_0 < 0.0;

#line 99
    }

#line 99
    if(_S36)
    {

#line 99
        _S36 = true;

#line 99
    }
    else
    {

#line 99
        _S36 = b2_0 < 0.0;

#line 99
    }

#line 99
    if(_S36)
    {

#line 100
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    float t_0 = dot(e1_0, r_0);
    if(t_0 < 0.00000999999974738)
    {

#line 105
        _S36 = true;

#line 105
    }
    else
    {

#line 105
        _S36 = t_0 > 1.0e+06;

#line 105
    }

#line 105
    if(_S36)
    {

#line 106
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    hit_0.coords_0[0] = b0_0;
    hit_0.coords_0[1] = b1_0;
    hit_0.coords_0[2] = b2_0;
    hit_0.coords_0[3] = t_0;
    hit_0.did_hit_0 = 1U;
    hit_0.triangle_index_1 = _S34;

    return hit_0;
}


#line 117
void get_closest_hit_0(Ray_0 _S37, uint _S38, inout Hit_0 _S39)
{

#line 164
    uint i_0 = 0U;

#line 164
    for(;;)
    {

#line 164
        if(i_0 < _S38)
        {
        }
        else
        {

#line 164
            break;
        }

#line 165
        Hit_0 _S40 = ray_triangle_intersection_0(_S37, i_0);
        if(_S40.did_hit_0 == 0U)
        {

#line 166
            i_0 = i_0 + 1U;

#line 166
            continue;
        }

#line 167
        bool _S41;

#line 167
        if(_S39.did_hit_0 == 0U)
        {

#line 167
            _S41 = true;

#line 167
        }
        else
        {

#line 167
            float _S42 = Hit_get_distance_0(_S40);

#line 167
            float _S43 = Hit_get_distance_0(_S39);

#line 167
            _S41 = _S42 < _S43;

#line 167
        }

#line 167
        if(_S41)
        {

#line 168
            _S39 = _S40;

#line 167
        }

#line 164
        i_0 = i_0 + 1U;

#line 164
    }

#line 171
    return;
}


#line 188
void get_closest_hit_bvh_0(Ray_0 _S44, inout Hit_0 _S45, inout vec4 _S46, bool _S47, uint _S48)
{

#line 188
    uint  stack_0[2048];



    stack_0[0U] = 0U;

#line 189
    uint  depth_stack_0[2048];



    depth_stack_0[0U] = 0U;

#line 209
    const vec4 bvh_color_0 = vec4(0.5, 0.0, 0.5, 0.10000000149011612);



    const vec4 bvh_edge_color_0 = vec4(0.69999998807907104, 0.0, 0.69999998807907104, 0.10000000149011612);

#line 213
    uint stack_index_0 = 1U;

#line 213
    for(;;)
    {

#line 196
        if(stack_index_0 > 0U)
        {
        }
        else
        {

#line 196
            break;
        }
        uint stack_index_1 = stack_index_0 - 1U;
        uint current_node_index_0 = stack_0[stack_index_1];
        BvhNode_0 _S49 = unpackStorage_4(_Bvhs_0._data[uint(current_node_index_0)]);
        uint current_depth_0 = depth_stack_0[stack_index_1];

        int intersection_type_0 = ray_bvh_intersection_0(_S44, _S49);
        if(intersection_type_0 != 0)
        {
            bool _S50;

#line 206
            if(_S47)
            {

#line 206
                _S50 = _S48 == current_depth_0;

#line 206
            }
            else
            {

#line 206
                _S50 = false;

#line 206
            }

#line 206
            if(_S50)
            {

#line 207
                switch(intersection_type_0)
                {
                case 2:
                    {

#line 210
                        _S46 = bvh_color_0;
                        break;
                    }
                case 1:
                    {

#line 214
                        _S46 = bvh_edge_color_0;
                        break;
                    }
                default:
                    {

#line 217
                        break;
                    }
                }

#line 206
            }

#line 222
            bool _S51 = BvhNode_is_leaf_0(_S49);

#line 222
            uint stack_index_2;

#line 222
            if(_S51)
            {

#line 223
                Hit_0 _S52 = ray_triangle_intersection_0(_S44, _S49.triangle_index_0);
                if(_S52.did_hit_0 == 0U)
                {

#line 224
                    stack_index_0 = stack_index_1;

#line 224
                    continue;
                }

#line 225
                bool _S53;

#line 225
                if(_S45.did_hit_0 == 0U)
                {

#line 225
                    _S53 = true;

#line 225
                }
                else
                {

#line 225
                    float _S54 = Hit_get_distance_0(_S52);

#line 225
                    float _S55 = Hit_get_distance_0(_S45);

#line 225
                    _S53 = _S54 < _S55;

#line 225
                }

#line 225
                if(_S53)
                {

#line 226
                    _S45 = _S52;
                    if(_S44.is_shadow_ray_0)
                    {

#line 228
                        return;
                    }

#line 225
                }

#line 225
                stack_index_2 = stack_index_1;

#line 225
            }
            else
            {

#line 233
                stack_0[stack_index_1] = _S49.left_child_index_0;
                uint _S56 = current_depth_0 + 1U;

#line 234
                depth_stack_0[stack_index_1] = _S56;
                uint stack_index_3 = stack_index_1 + 1U;
                stack_0[stack_index_3] = _S49.right_child_index_0;
                depth_stack_0[stack_index_3] = _S56;
                stack_index_2 = stack_index_3 + 1U;

#line 238
            }

#line 238
            stack_index_0 = stack_index_2;

#line 238
        }
        else
        {

#line 238
            stack_index_0 = stack_index_1;

#line 238
        }

#line 238
    }

#line 243
    return;
}


#line 243
vec3 Hit_get_ambient_0(Hit_0 _S57)
{

#line 46
    vec3 _S58 = Hit_get_barycentric_coordinates_0(_S57);
    Material_0 _S59 = unpackStorage_5(_Materials_0._data[uint(_Models_0._data[uint(_Triangles_0._data[uint(_S57.triangle_index_1)].model_index_0)].material_index_0)]);
    vec3 _S60 = vec3(_S59.ambient_0.x, _S59.ambient_0.y, _S59.ambient_0.z);
    return _S58.x * _S60 + _S58.y * _S60 + _S58.z * _S60;
}


#line 49
vec3 DirectionalLight_shade_0(DirectionalLight_0 _S61, Hit_0 _S62, int _S63, uint _S64, uint _S65)
{

#line 29 8
    vec3 _S66 = Hit_get_ambient_0(_S62);

#line 29
    return _S66;
}


#line 29
void get_color_0(DirectionalLight_0 _S67, uint _S68, uint _S69, Hit_0 _S70, inout vec4 _S71, bool _S72)
{

#line 257 6
    if(_S70.did_hit_0 == 0U)
    {

#line 257
        return;
    }
    vec3 _S73 = DirectionalLight_shade_0(_S67, _S70, 0, _S68, _S69);

#line 269
    _S71 = vec4(_S73, 1.0);

#line 278
    if(_S72)
    {

        const vec4 wireframe_edges_color_0 = vec4(0.0, 0.0, 0.0, 1.0);
        bool _S74;
        if(_S70.coords_0.x < 0.01999999955296516)
        {

#line 283
            _S74 = true;

#line 283
        }
        else
        {

#line 283
            _S74 = _S70.coords_0.y < 0.01999999955296516;

#line 283
        }
        if(_S74)
        {

#line 284
            _S74 = true;

#line 284
        }
        else
        {

#line 284
            _S74 = _S70.coords_0.z < 0.01999999955296516;

#line 284
        }

#line 282
        if(_S74)
        {

            _S71 = wireframe_edges_color_0;

#line 282
        }

#line 278
    }

#line 288
    return;
}


#line 95 0
layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;
void main()
{

#line 96
    uvec2 texel_coord_0 = gl_GlobalInvocationID.xy;

    const uvec2 _S75 = uvec2(0U, 0U);

#line 98
    uvec2 image_size_0 = _S75;
    ((image_size_0[0]) = imageSize((_Framebuffer_0)).x), ((image_size_0[1]) = imageSize((_Framebuffer_0)).y);

    const vec2 _S76 = vec2(0.0);

#line 101
    vec2 pixel_position_0 = _S76;
    float _S77 = float(texel_coord_0.x) / float(image_size_0.x);

#line 102
    pixel_position_0[0] = _S77;
    float _S78 = float(texel_coord_0.y) / float(image_size_0.y);

#line 103
    pixel_position_0[1] = _S78;
    bool _S79;

#line 104
    if(pixel_position_0.x >= 1.0)
    {

#line 104
        _S79 = true;

#line 104
    }
    else
    {

#line 104
        _S79 = pixel_position_0.x < 0.0;

#line 104
    }
    if(_S79)
    {

#line 105
        _S79 = true;

#line 105
    }
    else
    {

#line 105
        _S79 = pixel_position_0.y >= 1.0;

#line 105
    }

#line 105
    if(_S79)
    {

#line 105
        _S79 = true;

#line 105
    }
    else
    {

#line 105
        _S79 = pixel_position_0.y < 0.0;

#line 105
    }

#line 104
    if(_S79)
    {
        return;
    }

    Ray_0 _S80 = get_ray_0(pixel_position_0);


    const vec4 _S81 = vec4(0.0, 0.0, 0.0, 0.0);

#line 112
    vec4 bvh_color_1 = _S81;
    Hit_0 closest_hit_0;
    closest_hit_0.did_hit_0 = 0U;
    if(int(_PushConstants_0.bvh_type_0) == 0)
    {

#line 116
        get_closest_hit_0(_S80, _PushConstants_0.nb_triangles_0, closest_hit_0);

#line 115
    }
    else
    {
        bool should_display_bvh_1 = _PushConstants_0.should_display_bvh_0 != 0U;
        get_closest_hit_bvh_0(_S80, closest_hit_0, bvh_color_1, should_display_bvh_1, _PushConstants_0.bvh_depth_to_display_0);

#line 115
    }

#line 124
    DirectionalLight_0 sun_0 = DirectionalLight_x24init_0();
    const vec4 _S82 = vec4(0.0);

#line 125
    vec4 color_1 = _S82;
    get_color_0(sun_0, _PushConstants_0.bvh_type_0, _PushConstants_0.nb_triangles_0, closest_hit_0, color_1, _PushConstants_0.is_wireframe_on_0 != 0U);

#line 140
    float _S83 = bvh_color_1.w;
    vec4 _S84 = _S83 * bvh_color_1 + (1.0 - _S83) * color_1;

#line 141
    color_1 = _S84;
    color_1[3] = 1.0;

    imageStore((_Framebuffer_0), (ivec2(texel_coord_0)), color_1);
    return;
}

