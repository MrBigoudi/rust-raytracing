implementing scene;

public struct Triangle {
    public float4 p0;
    public float4 p1;
    public float4 p2;
    public uint model_index;

    public func get_world_position_p0(models: RWStructuredBuffer<Model>)->float3 {
        let p0 = mul(models[model_index].model_matrix, p0);
        return p0.xyz / p0.w;
    }

    public func get_world_position_p1(models: RWStructuredBuffer<Model>)->float3 {
        let p1 = mul(models[model_index].model_matrix, p1);
        return p1.xyz / p1.w;
    }

    public func get_world_position_p2(models: RWStructuredBuffer<Model>)->float3 {
        let p2 = mul(models[model_index].model_matrix, p2);
        return p2.xyz / p2.w;
    }

    public func get_normal(models: RWStructuredBuffer<Model>) -> float3 {
        float3 p0 = mul(models[model_index].model_matrix, p0).xyz;
        float3 p1 = mul(models[model_index].model_matrix, p1).xyz;
        float3 p2 = mul(models[model_index].model_matrix, p2).xyz;

        float3 e0 = p1 - p0;
        float3 e1 = p2 - p0;
        float3 tmp = cross(e0, e1);
        float3 n = normalize(tmp);
        return n;
    }
};

