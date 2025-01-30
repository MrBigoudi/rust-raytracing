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
    if(abs(_S19 - bvh_node_0.bounding_box_0.mins_0.x) < 0.05000000074505806)
    {

#line 150
        close_to_x_0 = true;

#line 150
    }
    else
    {

#line 150
        close_to_x_0 = abs(_S19 - bvh_node_0.bounding_box_0.maxs_0.x) < 0.05000000074505806;

#line 150
    }
    float _S20 = hit_point_0.y;

#line 151
    bool close_to_y_0;
    if(abs(_S20 - bvh_node_0.bounding_box_0.mins_0.y) < 0.05000000074505806)
    {

#line 152
        close_to_y_0 = true;

#line 152
    }
    else
    {

#line 152
        close_to_y_0 = abs(_S20 - bvh_node_0.bounding_box_0.maxs_0.y) < 0.05000000074505806;

#line 152
    }
    float _S21 = hit_point_0.z;

#line 153
    bool close_to_z_0;
    if(abs(_S21 - bvh_node_0.bounding_box_0.mins_0.z) < 0.05000000074505806)
    {

#line 154
        close_to_z_0 = true;

#line 154
    }
    else
    {

#line 154
        close_to_z_0 = abs(_S21 - bvh_node_0.bounding_box_0.maxs_0.z) < 0.05000000074505806;

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


#line 56
Ray_0 get_ray_0(vec2 _S27)
{

#line 56
    mat4x4 _S28 = unpackStorage_0(_Camera_0.inv_view_0);

#line 56
    vec4 _S29 = _Camera_0.eye_0;

#line 56
    float _S30 = _Camera_0.plane_width_0;

#line 56
    float _S31 = _Camera_0.plane_height_0;

#line 56
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


#line 161 6
vec3 Triangle_get_world_position_p0_0(Triangle_0 _S33)
{

#line 10 2
    vec4 _S34 = (((_S33.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S33.model_index_0)].model_matrix_0))));
    return _S34.xyz;
}


#line 11
vec3 Triangle_get_world_position_p1_0(Triangle_0 _S35)
{


    vec4 _S36 = (((_S35.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S35.model_index_0)].model_matrix_0))));
    return _S36.xyz;
}


#line 16
vec3 Triangle_get_world_position_p2_0(Triangle_0 _S37)
{


    vec4 _S38 = (((_S37.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S37.model_index_0)].model_matrix_0))));
    return _S38.xyz;
}


#line 21
Hit_0 ray_triangle_intersection_0(Ray_0 _S39, uint _S40)
{

#line 62 6
    Hit_0 hit_0;

    Triangle_0 _S41 = unpackStorage_2(_Triangles_0._data[uint(_S40)]);

    vec3 _S42 = Triangle_get_world_position_p0_0(_S41);
    vec3 _S43 = Triangle_get_world_position_p1_0(_S41);
    vec3 _S44 = Triangle_get_world_position_p2_0(_S41);

    vec3 e0_0 = _S43 - _S42;
    vec3 e1_0 = _S44 - _S42;

    vec3 w_0 = _S39.direction_0.xyz;
    vec3 o_0 = _S39.origin_0.xyz;

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

    bool _S45;

#line 86
    if(!_S39.is_shadow_ray_0)
    {

#line 86
        _S45 = dot(n_0, w_0) >= 0.0;

#line 86
    }
    else
    {

#line 86
        _S45 = false;

#line 86
    }

#line 86
    if(_S45)
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

    vec3 s_0 = (o_0 - _S42) / a_0;
    vec3 r_0 = cross(s_0, e0_0);

    float b0_0 = dot(s_0, q_0);
    float b1_0 = dot(r_0, w_0);
    float b2_0 = 1.0 - b0_0 - b1_0;
    if(b0_0 < 0.0)
    {

#line 103
        _S45 = true;

#line 103
    }
    else
    {

#line 103
        _S45 = b1_0 < 0.0;

#line 103
    }

#line 103
    if(_S45)
    {

#line 103
        _S45 = true;

#line 103
    }
    else
    {

#line 103
        _S45 = b2_0 < 0.0;

#line 103
    }

#line 103
    if(_S45)
    {

#line 104
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    float t_0 = dot(e1_0, r_0);
    if(t_0 < 0.00009999999747379)
    {

#line 109
        _S45 = true;

#line 109
    }
    else
    {

#line 109
        _S45 = t_0 > 1.0e+06;

#line 109
    }

#line 109
    if(_S45)
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
    hit_0.triangle_index_1 = _S40;

    return hit_0;
}


#line 121
void get_closest_hit_0(Ray_0 _S46, uint _S47, inout Hit_0 _S48)
{

#line 168
    uint i_0 = 0U;

#line 168
    for(;;)
    {

#line 168
        if(i_0 < _S47)
        {
        }
        else
        {

#line 168
            break;
        }

#line 169
        Hit_0 _S49 = ray_triangle_intersection_0(_S46, i_0);
        if(_S49.did_hit_0 == 0U)
        {

#line 170
            i_0 = i_0 + 1U;

#line 170
            continue;
        }

#line 171
        bool _S50;

#line 171
        if(_S48.did_hit_0 == 0U)
        {

#line 171
            _S50 = true;

#line 171
        }
        else
        {

#line 171
            float _S51 = Hit_get_distance_0(_S49);

#line 171
            float _S52 = Hit_get_distance_0(_S48);

#line 171
            _S50 = _S51 < _S52;

#line 171
        }

#line 171
        if(_S50)
        {

#line 172
            _S48 = _S49;
            if(_S46.is_shadow_ray_0)
            {

#line 174
                return;
            }

#line 171
        }

#line 168
        i_0 = i_0 + 1U;

#line 168
    }

#line 178
    return;
}


#line 195
void get_closest_hit_bvh_0(Ray_0 _S53, inout Hit_0 _S54, inout vec4 _S55, bool _S56, uint _S57)
{

#line 195
    uint  stack_0[64];



    stack_0[0U] = 0U;

#line 196
    uint  depth_stack_0[64];



    depth_stack_0[0U] = 0U;

#line 216
    const vec4 bvh_color_0 = vec4(0.5, 0.0, 0.5, 0.10000000149011612);



    const vec4 bvh_edge_color_0 = vec4(0.69999998807907104, 0.0, 0.69999998807907104, 0.10000000149011612);

#line 220
    uint stack_index_0 = 1U;

#line 220
    for(;;)
    {

#line 203
        if(stack_index_0 > 0U)
        {
        }
        else
        {

#line 203
            break;
        }
        uint stack_index_1 = stack_index_0 - 1U;
        uint current_node_index_0 = stack_0[stack_index_1];
        BvhNode_0 _S58 = unpackStorage_4(_Bvhs_0._data[uint(current_node_index_0)]);
        uint current_depth_0 = depth_stack_0[stack_index_1];

        int intersection_type_0 = ray_bvh_intersection_0(_S53, _S58);
        if(intersection_type_0 != 0)
        {
            bool _S59;

#line 213
            if(_S56)
            {

#line 213
                _S59 = _S57 == current_depth_0;

#line 213
            }
            else
            {

#line 213
                _S59 = false;

#line 213
            }

#line 213
            if(_S59)
            {

#line 214
                switch(intersection_type_0)
                {
                case 2:
                    {

#line 217
                        _S55 = bvh_color_0;
                        break;
                    }
                case 1:
                    {

#line 221
                        _S55 = bvh_edge_color_0;
                        break;
                    }
                default:
                    {

#line 224
                        break;
                    }
                }

#line 213
            }

#line 229
            bool _S60 = BvhNode_is_leaf_0(_S58);

#line 229
            uint stack_index_2;

#line 229
            if(_S60)
            {

#line 230
                Hit_0 _S61 = ray_triangle_intersection_0(_S53, _S58.triangle_index_0);
                if(_S61.did_hit_0 == 0U)
                {

#line 231
                    stack_index_0 = stack_index_1;

#line 231
                    continue;
                }

#line 232
                bool _S62;

#line 232
                if(_S54.did_hit_0 == 0U)
                {

#line 232
                    _S62 = true;

#line 232
                }
                else
                {

#line 232
                    float _S63 = Hit_get_distance_0(_S61);

#line 232
                    float _S64 = Hit_get_distance_0(_S54);

#line 232
                    _S62 = _S63 < _S64;

#line 232
                }

#line 232
                if(_S62)
                {

#line 233
                    _S54 = _S61;
                    if(_S53.is_shadow_ray_0)
                    {

#line 235
                        return;
                    }

#line 232
                }

#line 232
                stack_index_2 = stack_index_1;

#line 232
            }
            else
            {

#line 240
                stack_0[stack_index_1] = _S58.left_child_index_0;
                uint _S65 = current_depth_0 + 1U;

#line 241
                depth_stack_0[stack_index_1] = _S65;
                uint stack_index_3 = stack_index_1 + 1U;
                stack_0[stack_index_3] = _S58.right_child_index_0;
                depth_stack_0[stack_index_3] = _S65;
                stack_index_2 = stack_index_3 + 1U;

#line 245
            }

#line 245
            stack_index_0 = stack_index_2;

#line 245
        }
        else
        {

#line 245
            stack_index_0 = stack_index_1;

#line 245
        }

#line 245
    }

#line 250
    return;
}


#line 250
vec3 Hit_get_world_position_0(Hit_0 _S66)
{

#line 23
    Triangle_0 _S67 = unpackStorage_2(_Triangles_0._data[uint(_S66.triangle_index_1)]);
    vec3 _S68 = Hit_get_barycentric_coordinates_0(_S66);
    vec3 _S69 = Triangle_get_world_position_p0_0(_S67);
    vec3 _S70 = Triangle_get_world_position_p1_0(_S67);
    vec3 _S71 = Triangle_get_world_position_p2_0(_S67);

    return _S68.x * _S69 + _S68.y * _S70 + _S68.z * _S71;
}


#line 29
vec3 Triangle_get_normal_0(Triangle_0 _S72)
{

#line 25 2
    vec3 p0_1 = (((_S72.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S72.model_index_0)].model_matrix_0)))).xyz;
    vec3 p1_1 = (((_S72.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S72.model_index_0)].model_matrix_0)))).xyz;
    vec3 p2_1 = (((_S72.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S72.model_index_0)].model_matrix_0)))).xyz;

    vec3 e0_1 = p1_1 - p0_1;
    vec3 e1_1 = p2_1 - p0_1;
    vec3 tmp_1 = cross(e0_1, e1_1);
    vec3 n_1 = normalize(tmp_1);
    return n_1;
}


#line 33
vec3 Hit_get_world_norm_0(Hit_0 _S73)
{

#line 36 6
    Triangle_0 _S74 = unpackStorage_2(_Triangles_0._data[uint(_S73.triangle_index_1)]);

    vec3 _S75 = Triangle_get_normal_0(_S74);

    vec3 _S76 = Hit_get_barycentric_coordinates_0(_S73);
    return normalize(_S76.x * _S75 + _S76.y * _S75 + _S76.z * _S75);
}


#line 41
vec3 Hit_get_ambient_0(Hit_0 _S77)
{

#line 49
    vec3 _S78 = Hit_get_barycentric_coordinates_0(_S77);
    Material_0 _S79 = unpackStorage_5(_Materials_0._data[uint(_Models_0._data[uint(_Triangles_0._data[uint(_S77.triangle_index_1)].model_index_0)].material_index_0)]);
    vec3 _S80 = vec3(_S79.ambient_0.x, _S79.ambient_0.y, _S79.ambient_0.z);
    return _S78.x * _S80 + _S78.y * _S80 + _S78.z * _S80;
}


#line 52
vec3 DirectionalLight_shade_0(DirectionalLight_0 _S81, Hit_0 _S82, int _S83, uint _S84, uint _S85)
{

#line 28 8
    switch(_S83)
    {
    case 0:
        {

#line 30
            vec3 _S86 = Hit_get_world_position_0(_S82);
            Ray_0 shadow_ray_0;
            shadow_ray_0.origin_0 = vec4(_S86, 1.0);
            vec3 _S87 = - _S81.direction_1;

#line 33
            shadow_ray_0.direction_0 = vec4(_S87, 0.0);
            shadow_ray_0.is_shadow_ray_0 = true;
            Hit_0 closest_hit_0;
            closest_hit_0.did_hit_0 = 0U;

            if(int(_S84) == 0)
            {

#line 39
                get_closest_hit_0(shadow_ray_0, _S85, closest_hit_0);

#line 38
            }
            else
            {
                vec4 bvh_color_dummy_0 = vec4(0.0);
                get_closest_hit_bvh_0(shadow_ray_0, closest_hit_0, bvh_color_dummy_0, false, 0U);

#line 38
            }

#line 44
            if(closest_hit_0.did_hit_0 == 0U)
            {

#line 45
                vec3 _S88 = Hit_get_world_norm_0(_S82);
                float _S89 = clamp(dot(_S88, _S87), 0.0, 1.0);
                vec3 _S90 = _S89 * _S81.intensity_0 * _S81.color_0;

#line 47
                vec3 _S91 = Hit_get_ambient_0(_S82);

#line 47
                return _S90 * _S91;
            }
            else
            {

#line 49
                return vec3(0.0, 0.0, 0.0);
            }

#line 49
        }
    default:
        {

            return vec3(0.0);
        }
    }

#line 53
}


#line 53
void get_color_0(DirectionalLight_0 _S92, uint _S93, uint _S94, Hit_0 _S95, inout vec4 _S96, bool _S97)
{

#line 264 6
    if(_S95.did_hit_0 == 0U)
    {

#line 264
        return;
    }
    vec3 _S98 = DirectionalLight_shade_0(_S92, _S95, 0, _S93, _S94);

#line 276
    _S96 = vec4(_S98, 1.0);


    if(_S97)
    {

        const vec4 wireframe_edges_color_0 = vec4(0.0, 0.0, 0.0, 1.0);
        bool _S99;
        if(_S95.coords_0.x < 0.01999999955296516)
        {

#line 284
            _S99 = true;

#line 284
        }
        else
        {

#line 284
            _S99 = _S95.coords_0.y < 0.01999999955296516;

#line 284
        }
        if(_S99)
        {

#line 285
            _S99 = true;

#line 285
        }
        else
        {

#line 285
            _S99 = _S95.coords_0.z < 0.01999999955296516;

#line 285
        }

#line 283
        if(_S99)
        {

            _S96 = wireframe_edges_color_0;

#line 283
        }

#line 279
    }

#line 289
    return;
}


#line 96 0
layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;
void main()
{

#line 97
    uvec2 texel_coord_0 = gl_GlobalInvocationID.xy;

    const uvec2 _S100 = uvec2(0U, 0U);

#line 99
    uvec2 image_size_0 = _S100;
    ((image_size_0[0]) = imageSize((_Framebuffer_0)).x), ((image_size_0[1]) = imageSize((_Framebuffer_0)).y);

    const vec2 _S101 = vec2(0.0);

#line 102
    vec2 pixel_position_0 = _S101;
    float _S102 = float(texel_coord_0.x) / float(image_size_0.x);

#line 103
    pixel_position_0[0] = _S102;
    float _S103 = float(texel_coord_0.y) / float(image_size_0.y);

#line 104
    pixel_position_0[1] = _S103;
    bool _S104;

#line 105
    if(pixel_position_0.x >= 1.0)
    {

#line 105
        _S104 = true;

#line 105
    }
    else
    {

#line 105
        _S104 = pixel_position_0.x < 0.0;

#line 105
    }
    if(_S104)
    {

#line 106
        _S104 = true;

#line 106
    }
    else
    {

#line 106
        _S104 = pixel_position_0.y >= 1.0;

#line 106
    }

#line 106
    if(_S104)
    {

#line 106
        _S104 = true;

#line 106
    }
    else
    {

#line 106
        _S104 = pixel_position_0.y < 0.0;

#line 106
    }

#line 105
    if(_S104)
    {
        return;
    }

    Ray_0 _S105 = get_ray_0(pixel_position_0);


    const vec4 _S106 = vec4(0.0, 0.0, 0.0, 0.0);

#line 113
    vec4 bvh_color_1 = _S106;
    Hit_0 closest_hit_1;
    closest_hit_1.did_hit_0 = 0U;
    if(int(_PushConstants_0.bvh_type_0) == 0)
    {

#line 117
        get_closest_hit_0(_S105, _PushConstants_0.nb_triangles_0, closest_hit_1);

#line 116
    }
    else
    {
        bool should_display_bvh_1 = _PushConstants_0.should_display_bvh_0 != 0U;
        get_closest_hit_bvh_0(_S105, closest_hit_1, bvh_color_1, should_display_bvh_1, _PushConstants_0.bvh_depth_to_display_0);

#line 116
    }

#line 126
    DirectionalLight_0 _S107 = DirectionalLight_x24init_0();

#line 126
    DirectionalLight_0 sun_0 = _S107;



    float sun_angle_xz_0 = _PushConstants_0.current_time_0 * 0.00100000004749745;
    float sun_angle_y_0 = _PushConstants_0.current_time_0 * 0.00033333335886709;
    vec3 _S108 = normalize(vec3(cos(sun_angle_xz_0), sin(sun_angle_y_0), sin(sun_angle_xz_0)));

#line 132
    sun_0.direction_1 = _S108;

    const vec4 _S109 = vec4(0.0);

#line 134
    vec4 color_1 = _S109;
    get_color_0(sun_0, _PushConstants_0.bvh_type_0, _PushConstants_0.nb_triangles_0, closest_hit_1, color_1, _PushConstants_0.is_wireframe_on_0 != 0U);

#line 149
    float _S110 = bvh_color_1.w;
    vec4 _S111 = _S110 * bvh_color_1 + (1.0 - _S110) * color_1;

#line 150
    color_1 = _S111;
    color_1[3] = 1.0;

    imageStore((_Framebuffer_0), (ivec2(texel_coord_0)), color_1);
    return;
}

