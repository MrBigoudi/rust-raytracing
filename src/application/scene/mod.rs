use camera::Camera;
use material::Material;
use model::Model;
use triangle::Triangle;

pub mod camera;
pub mod material;
pub mod model;
pub mod triangle;

#[derive(Default)]
pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    pub camera: Option<Camera>,
}
