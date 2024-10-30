use camera::{Camera, CameraMovement};
use glam::Vec3;
// use log::error;
use material::Material;
use model::Model;
use triangle::Triangle;
use winit::{
    event::{DeviceId, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{core::error::ErrorCode, vulkan::types::VulkanContext};

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

impl Scene {
    pub fn init(vulkan_context: &VulkanContext) -> Result<Scene, ErrorCode> {
        let width = vulkan_context.draw_extent.width as f32;
        let height = vulkan_context.draw_extent.height as f32;
        let aspect_ratio = width / height;
        // error!("aspect ratio: {:?}, width: {:?}, height: {:?}", aspect_ratio, width, height);
        let camera = Camera::new(
            Vec3::new(0., 0., -5.),
            aspect_ratio,
            50.,
            0.1,
            100.,
            Vec3::new(0., 1., 0.),
        );

        let (model, triangles) = Model::triangle();
        let models = vec![model];
        let materials = vec![Material::default()];
        Ok(Scene {
            triangles,
            models,
            materials,
            camera,
        })
    }

    pub fn on_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
        delta_time: f64,
    ) -> Result<(), ErrorCode> {
        // Handle camera input
        if let KeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state: ElementState::Pressed,
            ..
        } = event
        {
            match key_code {
                KeyCode::KeyW => self
                    .camera
                    .on_keyboard_input(CameraMovement::Forward, delta_time),
                KeyCode::KeyS => self
                    .camera
                    .on_keyboard_input(CameraMovement::Backward, delta_time),
                KeyCode::KeyA => self
                    .camera
                    .on_keyboard_input(CameraMovement::Left, delta_time),
                KeyCode::KeyD => self
                    .camera
                    .on_keyboard_input(CameraMovement::Right, delta_time),
                KeyCode::ArrowUp => self
                    .camera
                    .on_keyboard_input(CameraMovement::Up, delta_time),
                KeyCode::ArrowDown => self
                    .camera
                    .on_keyboard_input(CameraMovement::Down, delta_time),
                _ => (),
            }
        }
        Ok(())
    }
}
