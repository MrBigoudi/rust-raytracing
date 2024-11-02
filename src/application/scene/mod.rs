use std::collections::HashMap;

use camera::{Camera, CameraMovement};
use glam::Vec3;
// use log::error;
use material::Material;
use model::Model;
use triangle::Triangle;
use winit::{
    dpi::LogicalPosition, event::{DeviceId, ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}
};

use super::{core::error::ErrorCode, vulkan::types::VulkanContext, window::key_map::{Key, KeyState}};

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

    pub fn update(&mut self, delta_time: f64, keys: &HashMap<Key, KeyState>) -> Result<(), ErrorCode> {
        // Move the camera
        if keys.contains_key(&Key::W) && (keys.get(&Key::W).unwrap() == &KeyState::Pressed) {
            self.camera.on_keyboard_input(CameraMovement::Forward, delta_time);
        }
        if keys.contains_key(&Key::S) && (keys.get(&Key::S).unwrap() == &KeyState::Pressed) {
            self.camera.on_keyboard_input(CameraMovement::Backward, delta_time);
        }
        if keys.contains_key(&Key::A) && (keys.get(&Key::A).unwrap() == &KeyState::Pressed) {
            self.camera.on_keyboard_input(CameraMovement::Left, delta_time);
        }
        if keys.contains_key(&Key::D) && (keys.get(&Key::D).unwrap() == &KeyState::Pressed) {
            self.camera.on_keyboard_input(CameraMovement::Right, delta_time);
        }
        if keys.contains_key(&Key::Up) && (keys.get(&Key::Up).unwrap() == &KeyState::Pressed) {
            self.camera.on_keyboard_input(CameraMovement::Up, delta_time);
        }
        if keys.contains_key(&Key::Down) && (keys.get(&Key::Down).unwrap() == &KeyState::Pressed) {
            self.camera.on_keyboard_input(CameraMovement::Down, delta_time);
        }
        Ok(())
    }

    pub fn on_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
    ) -> Result<(), ErrorCode> {
        // Handle camera input
        if let KeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state: ElementState::Pressed,
            ..
        } = event
        {
            match key_code {
                KeyCode::KeyM => self.camera.switch_mode(),
                _ => (),
            }
        }
        Ok(())
    }

    pub fn on_mouse_moved(
        &mut self,
        old_position: Option<LogicalPosition<f64>>,
        new_position: LogicalPosition<f64>,
        _delta_time: f64,
    ) -> Result<(), ErrorCode> {
        if let Some(old_position) = old_position {
            let x_offset = (new_position.x - old_position.x) as f32;
            let y_offset = (new_position.y - old_position.y) as f32;
            let should_constrain_pitch = true;
            self.camera.on_mouse_moved(x_offset, y_offset, should_constrain_pitch);
        }
        Ok(())
    }
}
