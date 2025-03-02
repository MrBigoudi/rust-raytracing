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

    pub fn get_volume(&self) -> f32 {
        let diffs = self.maxs - self.mins;
        diffs.x * diffs.y * diffs.z
    }

    pub fn get_surface_area(&self) -> f32 {
        let diffs = self.maxs - self.mins;
        2. * (diffs.x * diffs.y + diffs.y * diffs.z + diffs.z * diffs.x)
    }

    pub fn get_circumscribed_cube(&self) -> Self {
        let longest_axis = self.get_longest_axis();
        let mut circumscribed_aabb = *self;

        let max_dist = match longest_axis {
            AabbAxis::X => self.get_length_x(),
            AabbAxis::Y => self.get_length_y(),
            AabbAxis::Z => self.get_length_z(),
        };
        let delta = (max_dist - self.get_length_x()) * 0.5;
        circumscribed_aabb.maxs.x += delta;
        circumscribed_aabb.mins.x -= delta;
        let delta = (max_dist - self.get_length_y()) * 0.5;
        circumscribed_aabb.maxs.y += delta;
        circumscribed_aabb.mins.y -= delta;
        let delta = (max_dist - self.get_length_z()) * 0.5;
        circumscribed_aabb.maxs.z += delta;
        circumscribed_aabb.mins.z -= delta;
        circumscribed_aabb
    }

    pub fn merge(aabb_1: &Self, aabb_2: &Self) -> Self {
        let min_x = f32::min(aabb_1.mins.x, aabb_2.mins.x);
        let min_y = f32::min(aabb_1.mins.y, aabb_2.mins.y);
        let min_z = f32::min(aabb_1.mins.z, aabb_2.mins.z);

        let max_x = f32::max(aabb_1.maxs.x, aabb_2.maxs.x);
        let max_y = f32::max(aabb_1.maxs.y, aabb_2.maxs.y);
        let max_z = f32::max(aabb_1.maxs.z, aabb_2.maxs.z);

        Self {
            mins: glam::Vec3 {
                x: min_x,
                y: min_y,
                z: min_z,
            },
            padding_1: 0.,
            maxs: glam::Vec3 {
                x: max_x,
                y: max_y,
                z: max_z,
            },
            padding_2: 0.,
        }
    }

    // Compute the intersection of two AABBs
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min_x = self.mins.x.max(other.mins.x);
        let min_y = self.mins.y.max(other.mins.y);
        let min_z = self.mins.z.max(other.mins.z);

        let max_x = self.maxs.x.min(other.maxs.x);
        let max_y = self.maxs.y.min(other.maxs.y);
        let max_z = self.maxs.z.min(other.maxs.z);

        // Check if there is a valid intersection
        if min_x <= max_x && min_y <= max_y && min_z <= max_z {
            Some(Self {
                mins: glam::Vec3 {
                    x: min_x,
                    y: min_y,
                    z: min_z,
                },
                padding_1: 0.,
                maxs: glam::Vec3 {
                    x: max_x,
                    y: max_y,
                    z: max_z,
                },
                padding_2: 0.,
            })
        } else {
            None
        }
    }

    // Calculate the volume of the union of two AABBs
    pub fn get_union_volume(a: &Self, b: &Self) -> f32 {
        let volume_a = a.get_volume();
        let volume_b = b.get_volume();

        // Compute the intersection volume (if any)
        let intersection_volume = match a.intersection(b) {
            Some(intersection) => intersection.get_volume(),
            None => 0.0,
        };

        // Apply the inclusion-exclusion principle
        volume_a + volume_b - intersection_volume
    }

    pub fn from_points(points: &Vec<glam::Vec3>) -> Self {
        let mut aabb = Aabb::default();
        for point in points {
            aabb.mins.x = f32::min(aabb.mins.x, point.x);
            aabb.mins.y = f32::min(aabb.mins.y, point.y);
            aabb.mins.z = f32::min(aabb.mins.z, point.z);
            aabb.maxs.x = f32::max(aabb.maxs.x, point.x);
            aabb.maxs.y = f32::max(aabb.maxs.y, point.y);
            aabb.maxs.z = f32::max(aabb.maxs.z, point.z);
        }
        aabb
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
