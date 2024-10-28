use camera::Camera;
use material::Material;
use model::Model;
use triangle::Triangle;

pub mod camera;
pub mod material;
pub mod model;
pub mod triangle;

pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    pub camera: Camera,
}

impl Default for Scene{
    fn default() -> Self {
        let (model, triangles) = Model::triangle();
        let models = vec![model];
        let materials = vec![Material::default()];
        let camera = Camera::default();

        Self {
            triangles,
            models,
            materials,
            camera,
        }
    }
}
