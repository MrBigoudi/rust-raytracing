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


#line 6 6
struct Hit_0
{
    vec4 coords_0;
    uint did_hit_0;
    uint triangle_index_1;
};


#line 11
Hit_0 Hit_x24init_0()
{

#line 11
    Hit_0 _S5;

#line 8
    _S5.did_hit_0 = 0U;
    _S5.triangle_index_1 = 0U;


    _S5.did_hit_0 = 0U;
    _S5.triangle_index_1 = 0U;
    _S5.coords_0 = vec4(0.0);

#line 11
    return _S5;
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
Triangle_0 unpackStorage_2(Triangle_std430_0 _S6)
{

#line 3
    Triangle_0 _S7 = { _S6.p0_0, _S6.p1_0, _S6.p2_0, _S6.model_index_0 };

#line 3
    return _S7;
}


#line 17 6
float Hit_get_distance_0(Hit_0 this_0)
{

#line 18
    return this_0.coords_0.w;
}


#line 167
bool update_closests_hits_0(uint nb_hits_0, Hit_0 new_hit_0, inout Hit_0  closests_hit_0[8])
{
    float _S8 = Hit_get_distance_0(new_hit_0);

    int _S9 = min(8, int(nb_hits_0));

#line 171
    uint index_0 = 0U;

#line 171
    for(;;)
    {

#line 171
        bool _S10;

#line 171
        if(int(index_0) < _S9)
        {

#line 171
            float _S11 = Hit_get_distance_0(closests_hit_0[index_0]);

#line 171
            _S10 = _S11 < _S8;

#line 171
        }
        else
        {

#line 171
            _S10 = false;

#line 171
        }

#line 171
        if(_S10)
        {
        }
        else
        {

#line 171
            break;
        }

#line 172
        index_0 = index_0 + 1U;

#line 172
    }

    if(index_0 < 8U)
    {

#line 175
        uint i_0 = 6U;

#line 175
        for(;;)
        {

#line 175
            if(i_0 > index_0)
            {
            }
            else
            {

#line 175
                break;
            }

#line 176
            closests_hit_0[i_0 + 1U] = closests_hit_0[i_0];

#line 175
            i_0 = i_0 - 1U;

#line 175
        }


        closests_hit_0[index_0] = new_hit_0;
        return true;
    }
    return false;
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
Aabb_0 unpackStorage_3(Aabb_std430_0 _S12)
{

#line 13
    Aabb_0 _S13 = { _S12.mins_0, _S12.padding_1_0, _S12.maxs_0, _S12.padding_2_0 };

#line 13
    return _S13;
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
BvhNode_0 unpackStorage_4(BvhNode_std430_0 _S14)
{

#line 20
    BvhNode_0 _S15 = { unpackStorage_3(_S14.bounding_box_0), _S14.triangle_index_0, _S14.left_child_index_0, _S14.right_child_index_0, _S14.padding_1_1 };

#line 20
    return _S15;
}


#line 3 7
struct Ray_0
{
    vec4 origin_0;
    vec4 direction_0;
    bool is_shadow_ray_0;
};


#line 130 6
int ray_bvh_intersection_0(Ray_0 ray_0, BvhNode_0 bvh_node_0)
{
    vec3 _S16 = ray_0.direction_0.xyz;

#line 132
    vec3 inverse_ray_dir_0 = 1.0 / _S16;
    vec3 _S17 = ray_0.origin_0.xyz;

#line 133
    vec3 t1_0 = (bvh_node_0.bounding_box_0.mins_0 - _S17) * inverse_ray_dir_0;
    vec3 t2_0 = (bvh_node_0.bounding_box_0.maxs_0 - _S17) * inverse_ray_dir_0;

    float _S18 = t1_0.x;

#line 136
    float _S19 = t2_0.x;

#line 136
    float t_min_0 = min(_S18, _S19);
    float t_max_0 = max(_S18, _S19);
    bool close_to_x_0;

#line 138
    if(t_max_0 < 0.0)
    {

#line 138
        close_to_x_0 = true;

#line 138
    }
    else
    {

#line 138
        close_to_x_0 = t_min_0 > t_max_0;

#line 138
    }

#line 138
    if(close_to_x_0)
    {

#line 139
        return 0;
    }
    float _S20 = t1_0.y;

#line 141
    float _S21 = t2_0.y;

#line 141
    float t_min_1 = max(t_min_0, min(_S20, _S21));
    float t_max_1 = min(t_max_0, max(_S20, _S21));
    if(t_max_1 < 0.0)
    {

#line 143
        close_to_x_0 = true;

#line 143
    }
    else
    {

#line 143
        close_to_x_0 = t_min_1 > t_max_1;

#line 143
    }

#line 143
    if(close_to_x_0)
    {

#line 144
        return 0;
    }
    float _S22 = t1_0.z;

#line 146
    float _S23 = t2_0.z;

#line 146
    float t_min_2 = max(t_min_1, min(_S22, _S23));
    float t_max_2 = min(t_max_1, max(_S22, _S23));
    if(t_max_2 < 0.0)
    {

#line 148
        close_to_x_0 = true;

#line 148
    }
    else
    {

#line 148
        close_to_x_0 = t_min_2 > t_max_2;

#line 148
    }

#line 148
    if(close_to_x_0)
    {

#line 149
        return 0;
    }



    vec3 hit_point_0 = _S17 + _S16 * t_min_2;
    float _S24 = hit_point_0.x;
    if(abs(_S24 - bvh_node_0.bounding_box_0.mins_0.x) < 0.20000000298023224)
    {

#line 156
        close_to_x_0 = true;

#line 156
    }
    else
    {

#line 156
        close_to_x_0 = abs(_S24 - bvh_node_0.bounding_box_0.maxs_0.x) < 0.20000000298023224;

#line 156
    }
    float _S25 = hit_point_0.y;

#line 157
    bool close_to_y_0;
    if(abs(_S25 - bvh_node_0.bounding_box_0.mins_0.y) < 0.20000000298023224)
    {

#line 158
        close_to_y_0 = true;

#line 158
    }
    else
    {

#line 158
        close_to_y_0 = abs(_S25 - bvh_node_0.bounding_box_0.maxs_0.y) < 0.20000000298023224;

#line 158
    }
    float _S26 = hit_point_0.z;

#line 159
    bool close_to_z_0;
    if(abs(_S26 - bvh_node_0.bounding_box_0.mins_0.z) < 0.20000000298023224)
    {

#line 160
        close_to_z_0 = true;

#line 160
    }
    else
    {

#line 160
        close_to_z_0 = abs(_S26 - bvh_node_0.bounding_box_0.maxs_0.z) < 0.20000000298023224;

#line 160
    }
    bool _S27;

#line 161
    if(close_to_x_0)
    {

#line 161
        _S27 = close_to_y_0;

#line 161
    }
    else
    {

#line 161
        _S27 = false;

#line 161
    }

#line 161
    if(_S27)
    {

#line 161
        close_to_x_0 = true;

#line 161
    }
    else
    {

#line 161
        if(close_to_x_0)
        {

#line 161
            close_to_x_0 = close_to_z_0;

#line 161
        }
        else
        {

#line 161
            close_to_x_0 = false;

#line 161
        }

#line 161
    }

#line 161
    if(close_to_x_0)
    {

#line 161
        close_to_x_0 = true;

#line 161
    }
    else
    {

#line 161
        if(close_to_y_0)
        {

#line 161
            close_to_x_0 = close_to_z_0;

#line 161
        }
        else
        {

#line 161
            close_to_x_0 = false;

#line 161
        }

#line 161
    }

#line 161
    if(close_to_x_0)
    {

#line 162
        return 1;
    }
    return 2;
}


#line 29 4
bool BvhNode_is_leaf_0(BvhNode_0 this_1)
{

#line 30
    bool _S28;

#line 30
    if(this_1.left_child_index_0 == 0U)
    {

#line 30
        _S28 = this_1.right_child_index_0 == 0U;

#line 30
    }
    else
    {

#line 30
        _S28 = false;

#line 30
    }

#line 30
    return _S28;
}


#line 66 8
struct DirectionalLight_0
{
    vec3 direction_1;
    vec3 color_0;
    float intensity_0;
};


#line 71
DirectionalLight_0 DirectionalLight_x24init_0()
{

#line 71
    DirectionalLight_0 _S29;
    _S29.color_0 = vec3(1.0, 1.0, 1.0);
    _S29.intensity_0 = 1.0;
    _S29.direction_1 = normalize(vec3(1.0, 0.0, 0.0));

#line 71
    return _S29;
}


#line 94
struct PointLight_0
{
    vec3 origin_1;
    vec3 color_1;
    float intensity_1;
};


#line 99
PointLight_0 PointLight_x24init_0()
{

#line 99
    PointLight_0 _S30;
    _S30.color_1 = vec3(1.0, 1.0, 1.0);
    _S30.intensity_1 = 1.0;
    _S30.origin_1 = vec3(0.0, 0.0, 0.0);

#line 99
    return _S30;
}




vec3 PointLight_get_direction_0(PointLight_0 this_2, vec3 hit_position_0)
{

#line 106
    return normalize(this_2.origin_1 - hit_position_0);
}


#line 117
float PointLight_get_max_distance_0(PointLight_0 this_3, vec3 hit_position_1)
{

#line 118
    return length(this_3.origin_1 - hit_position_1);
}


#line 109
vec3 PointLight_get_color_0(PointLight_0 this_4)
{

#line 110
    return this_4.color_1;
}

float PointLight_get_intensity_0(PointLight_0 this_5)
{

#line 114
    return this_5.intensity_1;
}


#line 3 5
struct Material_0
{
    vec4 ambient_0;
};


#line 3
Material_0 unpackStorage_5(Material_std430_0 _S31)
{

#line 3
    Material_0 _S32 = { _S31.ambient_0 };

#line 3
    return _S32;
}


#line 21 6
vec3 Hit_get_barycentric_coordinates_0(Hit_0 this_6)
{

#line 22
    return vec3(this_6.coords_0.x, this_6.coords_0.y, this_6.coords_0.z);
}


#line 62
Ray_0 get_ray_0(vec2 _S33)
{

#line 62
    mat4x4 _S34 = unpackStorage_0(_Camera_0.inv_view_0);

#line 62
    vec4 _S35 = _Camera_0.eye_0;

#line 62
    float _S36 = _Camera_0.plane_width_0;

#line 62
    float _S37 = _Camera_0.plane_height_0;

#line 62
    float _S38 = _Camera_0.plane_near_0;

#line 11 7
    vec3 pos_view_space_0 = vec3(_S33 - 0.5, 1.0) * vec3(_S36, - _S37, _S38);
    vec4 pos_world_space_0 = (((vec4(pos_view_space_0, 1.0)) * (_S34)));

    Ray_0 ray_1;
    ray_1.origin_0 = _S35;
    ray_1.direction_0 = normalize(pos_world_space_0 - ray_1.origin_0);
    ray_1.direction_0[3] = 0.0;
    ray_1.is_shadow_ray_0 = false;
    return ray_1;
}


#line 283 6
vec3 Triangle_get_world_position_p0_0(Triangle_0 _S39)
{

#line 10 2
    vec4 _S40 = (((_S39.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S39.model_index_0)].model_matrix_0))));
    return _S40.xyz;
}


#line 11
vec3 Triangle_get_world_position_p1_0(Triangle_0 _S41)
{


    vec4 _S42 = (((_S41.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S41.model_index_0)].model_matrix_0))));
    return _S42.xyz;
}


#line 16
vec3 Triangle_get_world_position_p2_0(Triangle_0 _S43)
{


    vec4 _S44 = (((_S43.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S43.model_index_0)].model_matrix_0))));
    return _S44.xyz;
}


#line 21
Hit_0 ray_triangle_intersection_0(Ray_0 _S45, uint _S46)
{

#line 68 6
    Hit_0 _S47 = Hit_x24init_0();

#line 68
    Hit_0 hit_0 = _S47;

    Triangle_0 _S48 = unpackStorage_2(_Triangles_0._data[uint(_S46)]);

    vec3 _S49 = Triangle_get_world_position_p0_0(_S48);
    vec3 _S50 = Triangle_get_world_position_p1_0(_S48);
    vec3 _S51 = Triangle_get_world_position_p2_0(_S48);

    vec3 e0_0 = _S50 - _S49;
    vec3 e1_0 = _S51 - _S49;

    vec3 w_0 = _S45.direction_0.xyz;
    vec3 o_0 = _S45.origin_0.xyz;

    vec3 tmp_0 = cross(e0_0, e1_0);
    if(length(tmp_0) == 0.0)
    {

#line 84
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    vec3 n_0 = normalize(tmp_0);
    vec3 q_0 = cross(w_0, e1_0);
    float a_0 = dot(e0_0, q_0);

    bool _S52;

#line 92
    if(!_S45.is_shadow_ray_0)
    {

#line 92
        _S52 = dot(n_0, w_0) >= 0.0;

#line 92
    }
    else
    {

#line 92
        _S52 = false;

#line 92
    }

#line 92
    if(_S52)
    {

#line 93
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }


    if(abs(a_0) < 9.99999997475242708e-07)
    {

#line 99
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    vec3 s_0 = (o_0 - _S49) / a_0;
    vec3 r_0 = cross(s_0, e0_0);

    float b0_0 = dot(s_0, q_0);
    float b1_0 = dot(r_0, w_0);
    float b2_0 = 1.0 - b0_0 - b1_0;
    if(b0_0 < 0.0)
    {

#line 109
        _S52 = true;

#line 109
    }
    else
    {

#line 109
        _S52 = b1_0 < 0.0;

#line 109
    }

#line 109
    if(_S52)
    {

#line 109
        _S52 = true;

#line 109
    }
    else
    {

#line 109
        _S52 = b2_0 < 0.0;

#line 109
    }

#line 109
    if(_S52)
    {

#line 110
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    float t_0 = dot(e1_0, r_0);
    if(t_0 < 0.00009999999747379)
    {

#line 115
        _S52 = true;

#line 115
    }
    else
    {

#line 115
        _S52 = t_0 > 1.0e+06;

#line 115
    }

#line 115
    if(_S52)
    {

#line 116
        hit_0.did_hit_0 = 0U;
        return hit_0;
    }

    hit_0.coords_0[0] = b2_0;
    hit_0.coords_0[1] = b0_0;
    hit_0.coords_0[2] = b1_0;
    hit_0.coords_0[3] = t_0;
    hit_0.did_hit_0 = 1U;
    hit_0.triangle_index_1 = _S46;

    return hit_0;
}


#line 127
void get_closests_hit_0(Ray_0 _S53, uint _S54, inout Hit_0  _S55[8], float _S56)
{

#line 193
    uint i_1 = 0U;

#line 193
    uint nb_hits_1 = 0U;

#line 193
    for(;;)
    {

#line 193
        if(i_1 < _S54)
        {
        }
        else
        {

#line 193
            break;
        }

#line 194
        Hit_0 _S57 = ray_triangle_intersection_0(_S53, i_1);
        if(_S57.did_hit_0 == 0U)
        {

#line 195
            i_1 = i_1 + 1U;

#line 195
            continue;
        }

#line 196
        float _S58 = Hit_get_distance_0(_S57);
        if(_S58 > _S56)
        {

#line 197
            i_1 = i_1 + 1U;

#line 197
            continue;
        }

#line 198
        bool _S59 = update_closests_hits_0(nb_hits_1, _S57, _S55);

#line 198
        uint nb_hits_2;

#line 198
        if(_S59)
        {

#line 199
            uint _S60 = nb_hits_1 + 1U;
            if(_S53.is_shadow_ray_0)
            {

#line 201
                return;
            }

#line 201
            nb_hits_2 = _S60;

#line 201
        }
        else
        {

#line 201
            nb_hits_2 = nb_hits_1;

#line 201
        }

#line 201
        nb_hits_1 = nb_hits_2;

#line 201
        i_1 = i_1 + 1U;

#line 201
    }



    return;
}


#line 224
void get_closests_hit_bvh_0(Ray_0 _S61, inout Hit_0  _S62[8], inout vec4 _S63, bool _S64, uint _S65, float _S66)
{

#line 224
    uint  stack_0[32];



    stack_0[0U] = 0U;

#line 225
    uint  depth_stack_0[32];



    depth_stack_0[0U] = 0U;

#line 245
    const vec4 bvh_color_0 = vec4(0.0, 0.80000001192092896, 0.0, 0.80000001192092896);



    const vec4 bvh_edge_color_0 = vec4(0.0, 1.0, 0.0, 0.80000001192092896);

#line 249
    uint stack_index_0 = 1U;

#line 249
    uint nb_hits_3 = 0U;

#line 249
    for(;;)
    {

#line 232
        if(stack_index_0 > 0U)
        {
        }
        else
        {

#line 232
            break;
        }
        uint stack_index_1 = stack_index_0 - 1U;
        uint current_node_index_0 = stack_0[stack_index_1];
        BvhNode_0 _S67 = unpackStorage_4(_Bvhs_0._data[uint(current_node_index_0)]);
        uint current_depth_0 = depth_stack_0[stack_index_1];

        int intersection_type_0 = ray_bvh_intersection_0(_S61, _S67);
        if(intersection_type_0 != 0)
        {
            bool _S68;

#line 242
            if(_S64)
            {

#line 242
                _S68 = _S65 == current_depth_0;

#line 242
            }
            else
            {

#line 242
                _S68 = false;

#line 242
            }

#line 242
            if(_S68)
            {

#line 243
                switch(intersection_type_0)
                {
                case 2:
                    {

#line 246
                        _S63 = bvh_color_0;
                        break;
                    }
                case 1:
                    {

#line 250
                        _S63 = bvh_edge_color_0;
                        break;
                    }
                default:
                    {

#line 253
                        break;
                    }
                }

#line 242
            }

#line 258
            bool _S69 = BvhNode_is_leaf_0(_S67);

#line 258
            uint stack_index_2;

#line 258
            uint nb_hits_4;

#line 258
            if(_S69)
            {

#line 259
                Hit_0 _S70 = ray_triangle_intersection_0(_S61, _S67.triangle_index_0);
                if(_S70.did_hit_0 == 0U)
                {

#line 260
                    stack_index_0 = stack_index_1;

#line 260
                    continue;
                }

#line 261
                float _S71 = Hit_get_distance_0(_S70);
                if(_S71 > _S66)
                {

#line 262
                    stack_index_0 = stack_index_1;

#line 262
                    continue;
                }

#line 263
                bool _S72 = update_closests_hits_0(nb_hits_3, _S70, _S62);

#line 263
                if(_S72)
                {

#line 264
                    uint _S73 = nb_hits_3 + 1U;
                    if(_S61.is_shadow_ray_0)
                    {

#line 266
                        return;
                    }

#line 266
                    stack_index_2 = _S73;

#line 266
                }
                else
                {

#line 266
                    stack_index_2 = nb_hits_3;

#line 266
                }

#line 266
                uint _S74 = stack_index_2;

#line 266
                stack_index_2 = stack_index_1;

#line 266
                nb_hits_4 = _S74;

#line 266
            }
            else
            {


                stack_0[stack_index_1] = _S67.left_child_index_0;
                uint _S75 = current_depth_0 + 1U;

#line 272
                depth_stack_0[stack_index_1] = _S75;
                uint stack_index_3 = stack_index_1 + 1U;
                stack_0[stack_index_3] = _S67.right_child_index_0;
                depth_stack_0[stack_index_3] = _S75;
                stack_index_2 = stack_index_3 + 1U;

#line 276
                nb_hits_4 = nb_hits_3;

#line 276
            }

#line 276
            stack_index_0 = stack_index_2;

#line 276
            nb_hits_3 = nb_hits_4;

#line 276
        }
        else
        {

#line 276
            stack_index_0 = stack_index_1;

#line 276
        }

#line 276
    }

#line 281
    return;
}


#line 281
vec3 Hit_get_world_position_0(Hit_0 _S76)
{

#line 29
    Triangle_0 _S77 = unpackStorage_2(_Triangles_0._data[uint(_S76.triangle_index_1)]);
    vec3 _S78 = Hit_get_barycentric_coordinates_0(_S76);
    vec3 _S79 = Triangle_get_world_position_p0_0(_S77);
    vec3 _S80 = Triangle_get_world_position_p1_0(_S77);
    vec3 _S81 = Triangle_get_world_position_p2_0(_S77);

    return _S78.x * _S79 + _S78.y * _S80 + _S78.z * _S81;
}


#line 35
vec3 Triangle_get_normal_0(Triangle_0 _S82)
{

#line 25 2
    vec3 p0_1 = (((_S82.p0_0) * (unpackStorage_1(_Models_0._data[uint(_S82.model_index_0)].model_matrix_0)))).xyz;
    vec3 p1_1 = (((_S82.p1_0) * (unpackStorage_1(_Models_0._data[uint(_S82.model_index_0)].model_matrix_0)))).xyz;
    vec3 p2_1 = (((_S82.p2_0) * (unpackStorage_1(_Models_0._data[uint(_S82.model_index_0)].model_matrix_0)))).xyz;

    vec3 e0_1 = p1_1 - p0_1;
    vec3 e1_1 = p2_1 - p0_1;
    vec3 tmp_1 = cross(e0_1, e1_1);
    vec3 n_1 = normalize(tmp_1);
    return n_1;
}


#line 33
vec3 Hit_get_world_norm_0(Hit_0 _S83)
{

#line 42 6
    Triangle_0 _S84 = unpackStorage_2(_Triangles_0._data[uint(_S83.triangle_index_1)]);

    vec3 _S85 = Triangle_get_normal_0(_S84);

    vec3 _S86 = Hit_get_barycentric_coordinates_0(_S83);
    return normalize(_S86.x * _S85 + _S86.y * _S85 + _S86.z * _S85);
}


#line 47
vec3 Hit_get_ambient_0(Hit_0 _S87)
{

#line 55
    vec3 _S88 = Hit_get_barycentric_coordinates_0(_S87);
    Material_0 _S89 = unpackStorage_5(_Materials_0._data[uint(_Models_0._data[uint(_Triangles_0._data[uint(_S87.triangle_index_1)].model_index_0)].material_index_0)]);
    vec3 _S90 = vec3(_S89.ambient_0.x, _S89.ambient_0.y, _S89.ambient_0.z);
    return _S88.x * _S90 + _S88.y * _S90 + _S88.z * _S90;
}


#line 58
vec3 Lights_shade_0(PointLight_0 _S91, Hit_0  _S92[8], int _S93, uint _S94, uint _S95)
{

#line 21 8
    switch(_S93)
    {
    case 0:
        {

#line 23
            vec3 _S96 = Hit_get_world_position_0(_S92[0]);
            float _S97 = PointLight_get_max_distance_0(_S91, _S96);

            Ray_0 shadow_ray_0;
            shadow_ray_0.origin_0 = vec4(_S96, 1.0);
            vec3 _S98 = PointLight_get_direction_0(_S91, _S96);

#line 28
            shadow_ray_0.direction_0 = vec4(_S98, 0.0);
            shadow_ray_0.is_shadow_ray_0 = true;

            Hit_0  closests_hit_1[8];
            closests_hit_1[0].did_hit_0 = 0U;


            if(int(_S94) == 0)
            {

#line 36
                get_closests_hit_0(shadow_ray_0, _S95, closests_hit_1, _S97);

#line 35
            }
            else
            {
                vec4 bvh_color_dummy_0 = vec4(0.0);
                get_closests_hit_bvh_0(shadow_ray_0, closests_hit_1, bvh_color_dummy_0, false, 0U, _S97);

#line 35
            }

#line 41
            if(closests_hit_1[0].did_hit_0 == 0U)
            {

#line 42
                vec3 _S99 = Hit_get_world_norm_0(_S92[0]);
                float _S100 = clamp(dot(_S99, shadow_ray_0.direction_0.xyz), 0.0, 1.0);
                float _S101 = PointLight_get_intensity_0(_S91);

#line 44
                float _S102 = _S100 * _S101;

#line 44
                vec3 _S103 = PointLight_get_color_0(_S91);

#line 44
                vec3 _S104 = _S102 * _S103;

#line 44
                vec3 _S105 = Hit_get_ambient_0(_S92[0]);

#line 44
                return _S104 * _S105;
            }
            else
            {

#line 46
                return vec3(0.0, 0.0, 0.0);
            }

#line 46
        }
    case 1:
        {


            return vec3(0.0);
        }
    default:
        {

#line 54
            return vec3(0.0);
        }
    }

#line 54
}


#line 54
void get_color_0(PointLight_0 _S106, uint _S107, uint _S108, Hit_0  _S109[8], inout vec4 _S110, bool _S111)
{

#line 295 6
    if(_S109[0].did_hit_0 == 0U)
    {

#line 295
        return;
    }
    vec3 _S112 = Lights_shade_0(_S106, _S109, 0, _S107, _S108);

#line 308
    _S110 = vec4(_S112, 1.0);


    if(_S111)
    {

        const vec4 wireframe_edges_color_0 = vec4(0.0, 0.0, 0.0, 1.0);
        bool _S113;
        if(_S109[0].coords_0.x < 0.01999999955296516)
        {

#line 316
            _S113 = true;

#line 316
        }
        else
        {

#line 316
            _S113 = _S109[0].coords_0.y < 0.01999999955296516;

#line 316
        }
        if(_S113)
        {

#line 317
            _S113 = true;

#line 317
        }
        else
        {

#line 317
            _S113 = _S109[0].coords_0.z < 0.01999999955296516;

#line 317
        }

#line 315
        if(_S113)
        {

            _S110 = wireframe_edges_color_0;

#line 315
        }

#line 311
    }

#line 321
    return;
}


#line 96 0
layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;
void main()
{

#line 97
    uvec2 texel_coord_0 = gl_GlobalInvocationID.xy;

    const uvec2 _S114 = uvec2(0U, 0U);

#line 99
    uvec2 image_size_0 = _S114;
    ((image_size_0[0]) = imageSize((_Framebuffer_0)).x), ((image_size_0[1]) = imageSize((_Framebuffer_0)).y);

    const vec2 _S115 = vec2(0.0);

#line 102
    vec2 pixel_position_0 = _S115;
    float _S116 = float(texel_coord_0.x) / float(image_size_0.x);

#line 103
    pixel_position_0[0] = _S116;
    float _S117 = float(texel_coord_0.y) / float(image_size_0.y);

#line 104
    pixel_position_0[1] = _S117;
    bool _S118;

#line 105
    if(pixel_position_0.x >= 1.0)
    {

#line 105
        _S118 = true;

#line 105
    }
    else
    {

#line 105
        _S118 = pixel_position_0.x < 0.0;

#line 105
    }
    if(_S118)
    {

#line 106
        _S118 = true;

#line 106
    }
    else
    {

#line 106
        _S118 = pixel_position_0.y >= 1.0;

#line 106
    }

#line 106
    if(_S118)
    {

#line 106
        _S118 = true;

#line 106
    }
    else
    {

#line 106
        _S118 = pixel_position_0.y < 0.0;

#line 106
    }

#line 105
    if(_S118)
    {
        return;
    }

    Ray_0 _S119 = get_ray_0(pixel_position_0);


    const vec4 _S120 = vec4(0.0, 0.0, 0.0, 0.0);

#line 113
    vec4 bvh_color_1 = _S120;
    Hit_0 _S121 = { _S120, 0U, 0U };

#line 114
    Hit_0  _S122[8] = { _S121, _S121, _S121, _S121, _S121, _S121, _S121, _S121 };

#line 114
    Hit_0  closests_hit_2[8];

#line 114
    closests_hit_2[0] = _S121;

#line 114
    closests_hit_2[1] = _S121;

#line 114
    closests_hit_2[2] = _S121;

#line 114
    closests_hit_2[3] = _S121;

#line 114
    closests_hit_2[4] = _S121;

#line 114
    closests_hit_2[5] = _S121;

#line 114
    closests_hit_2[6] = _S121;

#line 114
    closests_hit_2[7] = _S121;

#line 144
    const vec3 _S123 = vec3(0.0, 0.0, 0.0);


    const vec4 _S124 = vec4(0.5, 0.69999998807907104, 1.0, 1.0);

#line 168
    ivec2 _S125 = ivec2(texel_coord_0);

#line 168
    uint i_2 = 0U;

#line 168
    for(;;)
    {

#line 115
        if(i_2 < 8U)
        {
        }
        else
        {

#line 115
            break;
        }

#line 116
        Hit_0 _S126 = Hit_x24init_0();

#line 116
        closests_hit_2[i_2] = _S126;

#line 115
        uint i_3 = i_2 + 1U;

#line 115
        i_2 = i_3;

#line 115
    }



    if(int(_PushConstants_0.bvh_type_0) == 0)
    {

#line 120
        get_closests_hit_0(_S119, _PushConstants_0.nb_triangles_0, closests_hit_2, 3.4028234663852886e+38);

#line 119
    }
    else
    {
        bool should_display_bvh_1 = _PushConstants_0.should_display_bvh_0 != 0U;
        get_closests_hit_bvh_0(_S119, closests_hit_2, bvh_color_1, should_display_bvh_1, _PushConstants_0.bvh_depth_to_display_0, 3.4028234663852886e+38);

#line 119
    }

#line 130
    DirectionalLight_0 _S127 = DirectionalLight_x24init_0();



    float sun_angle_xz_0 = _PushConstants_0.current_time_0 * 0.00009999999747379;

#line 142
    PointLight_0 _S128 = PointLight_x24init_0();

#line 142
    PointLight_0 point_light_0 = _S128;
    point_light_0.intensity_1 = 0.5;
    point_light_0.origin_1 = _S123;

    float factor_0 = 0.5 * pixel_position_0.y + 1.0;
    vec4 _S129 = vec4(1.0 - factor_0) + factor_0 * _S124;

#line 147
    vec4 color_2 = _S129;

    get_color_0(point_light_0, _PushConstants_0.bvh_type_0, _PushConstants_0.nb_triangles_0, closests_hit_2, color_2, _PushConstants_0.is_wireframe_on_0 != 0U);

#line 164
    float _S130 = bvh_color_1.w;
    vec4 _S131 = _S130 * bvh_color_1 + (1.0 - _S130) * color_2;

#line 165
    color_2 = _S131;
    color_2[3] = 1.0;

    imageStore((_Framebuffer_0), (_S125), color_2);
    return;
}

