implementing scene;

// Values should match the rust implementation
// see src/application/scene/bvh/mod.rs
public enum BvhType {
    None = 0,
    DefaultBottomUp = 1,
    DefaultTopDown = 2,
    Ploc = 3,
    Other = 4,
}

public struct Aabb {
    public float3 mins;
    public float padding_1;
    public float3 maxs;
    public float padding_2;
}

public struct BvhNode {
    public Aabb bounding_box;
    // If not leaf then dummy variable
    public uint triangle_index;
    // If child_index == 0 then leaf
    public uint left_child_index;
    public uint right_child_index;
    public float padding_1;
}

public enum BvhNodeIntersection {
    None,
    Edge,
    NotEdge,
}

public func is_leaf(bvh_node: BvhNode)->bool {
    return (bvh_node.left_child_index == 0) && (bvh_node.right_child_index == 0);
}