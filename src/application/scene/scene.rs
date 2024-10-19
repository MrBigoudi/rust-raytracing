use super::{camera::Camera, material::Material, model::Model, triangle::Triangle};

#[derive(Default)]
pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    pub camera: Option<Camera>,
}