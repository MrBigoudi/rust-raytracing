implementing scene;

public struct Triangle {
    public float4 p0;
    public float4 p1;
    public float4 p2;
    public uint model_index;

    public func get_normal(models: RWStructuredBuffer<Model>)->float3 {
        float3 p0 = mul(models[model_index].model_matrix, p0).xyz;
        float3 p1 = mul(models[model_index].model_matrix, p1).xyz;
        float3 p2 = mul(models[model_index].model_matrix, p2).xyz;

        float3 e0 = p1 - p0;
        float3 e1 = p2 - p0;
        float3 tmp = cross(e1, e0);
        float3 n = normalize(tmp);
        return n;
    }
};

