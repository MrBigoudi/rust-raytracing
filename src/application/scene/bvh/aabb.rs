use log::error;
use std::fmt::Debug;

use crate::application::{
    core::error::ErrorCode,
    scene::{model::Model, triangle::Triangle},
};

#[derive(Debug, Default)]
pub enum AabbAxis {
    #[default]
    X,
    Y,
    Z,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Aabb {
    pub mins: glam::Vec3,
    pub padding_1: f32,
    pub maxs: glam::Vec3,
    pub padding_2: f32,
}

impl Debug for Aabb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Aabb")
            .field("mins", &self.mins)
            .field("maxs", &self.maxs)
            .finish()
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            mins: f32::INFINITY * glam::Vec3::ONE,
            padding_1: 0.,
            maxs: f32::NEG_INFINITY * glam::Vec3::ONE,
            padding_2: 0.,
        }
    }
}

impl Aabb {
    pub fn get_length_x(&self) -> f32 {
        (self.maxs.x - self.mins.x).abs()
    }

    pub fn get_length_y(&self) -> f32 {
        (self.maxs.y - self.mins.y).abs()
    }

    pub fn get_length_z(&self) -> f32 {
        (self.maxs.z - self.mins.z).abs()
    }

    pub fn get_longest_axis(&self) -> AabbAxis {
        let length_x = self.get_length_x();
        let length_y = self.get_length_y();
        let length_z = self.get_length_z();

        if length_x > length_y {
            if length_x > length_z {
                AabbAxis::X
            } else {
                AabbAxis::Z
            }
        } else if length_y > length_z {
            AabbAxis::Y
        } else {
            AabbAxis::Z
        }
    }

    pub fn from_triangle(triangle: &Triangle, model_matrix: glam::Mat4) -> Self {
        let mut aabb = Aabb::default();
        // Build the AABB in world space
        let p0 = model_matrix * triangle.p0;
        let p1 = model_matrix * triangle.p1;
        let p2 = model_matrix * triangle.p2;

        let min_x = f32::min(f32::min(p0.x, p1.x), p2.x);
        let max_x = f32::max(f32::max(p0.x, p1.x), p2.x);
        let min_y = f32::min(f32::min(p0.y, p1.y), p2.y);
        let max_y = f32::max(f32::max(p0.y, p1.y), p2.y);
        let min_z = f32::min(f32::min(p0.z, p1.z), p2.z);
        let max_z = f32::max(f32::max(p0.z, p1.z), p2.z);

        aabb.mins.x = f32::min(aabb.mins.x, min_x);
        aabb.mins.y = f32::min(aabb.mins.y, min_y);
        aabb.mins.z = f32::min(aabb.mins.z, min_z);

        aabb.maxs.x = f32::max(aabb.maxs.x, max_x);
        aabb.maxs.y = f32::max(aabb.maxs.y, max_y);
        aabb.maxs.z = f32::max(aabb.maxs.z, max_z);

        aabb
    }

    pub fn from_scene(triangles: &[Triangle], models: &[Model]) -> Result<Self, ErrorCode> {
        if triangles.is_empty() {
            error!("Can't create an AABB from an empty scene");
            return Err(ErrorCode::InitializationFailure);
        }

        let mut aabb = Aabb::default();
        for triangle in triangles {
            // Build the AABB in world space
            let model_matrix = models[triangle.model_index].model_matrix;
            let p0 = model_matrix * triangle.p0;
            let p1 = model_matrix * triangle.p1;
            let p2 = model_matrix * triangle.p2;

            let min_x = f32::min(f32::min(p0.x, p1.x), p2.x);
            let max_x = f32::max(f32::max(p0.x, p1.x), p2.x);
            let min_y = f32::min(f32::min(p0.y, p1.y), p2.y);
            let max_y = f32::max(f32::max(p0.y, p1.y), p2.y);
            let min_z = f32::min(f32::min(p0.z, p1.z), p2.z);
            let max_z = f32::max(f32::max(p0.z, p1.z), p2.z);

            aabb.mins.x = f32::min(aabb.mins.x, min_x);
            aabb.mins.y = f32::min(aabb.mins.y, min_y);
            aabb.mins.z = f32::min(aabb.mins.z, min_z);

            aabb.maxs.x = f32::max(aabb.maxs.x, max_x);
            aabb.maxs.y = f32::max(aabb.maxs.y, max_y);
            aabb.maxs.z = f32::max(aabb.maxs.z, max_z);
        }

        Ok(aabb)
    }
}
