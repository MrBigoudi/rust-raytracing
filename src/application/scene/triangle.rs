use glam::Vec4;

use super::{bvh::aabb::Aabb, model::Model};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Triangle {
    pub p0: Vec4,
    pub p1: Vec4,
    pub p2: Vec4,
    pub model_index: usize,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            // CCW
            p0: Vec4::from_array([-1., 0., 0., 1.]),
            p1: Vec4::from_array([1., 0., 0., 1.]),
            p2: Vec4::from_array([0., 1., 0., 1.]),
            model_index: 0,
        }
    }
}

impl Triangle {
    pub fn get_centroid(&self, model_matrix: glam::Mat4) -> glam::Vec3 {
        let p0 = model_matrix * self.p0;
        let p1 = model_matrix * self.p1;
        let p2 = model_matrix * self.p2;

        let p0 = glam::Vec3::new(p0.x, p0.y, p0.y);
        let p1 = glam::Vec3::new(p1.x, p1.y, p1.y);
        let p2 = glam::Vec3::new(p2.x, p2.y, p2.y);

        (0.33333) * (p0 + p1 + p2)
    }

    pub fn get_centroids(triangles: &[Triangle], models: &[Model]) -> Vec<glam::Vec3> {
        triangles
            .iter()
            .map(|t| t.get_centroid(models[t.model_index].model_matrix))
            .collect::<Vec<glam::Vec3>>()
    }

    pub fn get_normalized_centroids(
        triangles: &[Triangle],
        models: &[Model],
        circumscribed_cube: &Aabb,
    ) -> Vec<glam::Vec3> {
        let centroids = Self::get_centroids(triangles, models);
        let cube_length = circumscribed_cube.get_length_x();
        centroids
            .iter()
            .map(|c| (*c - circumscribed_cube.mins) / cube_length)
            .collect::<Vec<glam::Vec3>>()
    }
}
