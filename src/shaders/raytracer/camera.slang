implementing raytracer;

#define IDENTITY_MATRIX float4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)

public struct Camera {
    public float4x4 inv_view;
    public float4 eye;
    public float plane_width;
    public float plane_height;
    public float plane_near;

    public __init() {
        eye = float4(0.f, 0.f, -1.f, 1.f); // Position the camera at the origin

        // Inverse matrices
        inv_view = IDENTITY_MATRIX; // Since view is identity, inv_view is also identity

        // Plane dimensions
        plane_width = 2.f;
        plane_height = 2.f;
        plane_near = 0.1f;
    }
};

