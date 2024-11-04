use std::{
    collections::HashMap,
    path::Path,
    time::{Duration, Instant},
};

use bvh::{default_top_down::BvhDefaultTopDown, Bvh, BvhNode, BvhType};
use camera::{Camera, CameraMovement};
use glam::Vec3;
use log::{error, info, warn};
// use log::error;
use material::Material;
use model::Model;
use triangle::Triangle;
use winit::{
    dpi::LogicalPosition,
    event::{DeviceId, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{
    core::error::ErrorCode,
    vulkan::types::VulkanContext,
    window::key_map::{Key, KeyState},
};

pub mod bvh;
pub mod camera;
pub mod material;
pub mod model;
pub mod triangle;

#[derive(Debug)]
pub struct Scene {
    pub triangles: Vec<Triangle>,
    pub models: Vec<Model>,
    pub materials: Vec<Material>,
    pub camera: Camera,
    pub is_wireframe_on: bool,
    pub bvh_type: BvhType,
    pub bvhs: HashMap<BvhType, Vec<BvhNode>>,
}

impl Scene {
    // TODO: Change the initial scene
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

        let mut triangles = Vec::new();
        let mut models = Vec::new();
        let mut materials = vec![Material::default()];
        if let Err(err) = Model::add_obj(
            Path::new("cube.obj"),
            false,
            &mut triangles,
            &mut models,
            &mut materials,
        ) {
            error!("Failed to load a new object to the scene: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }

        // panic!("nb tri: {:?}, nb mod: {:?}, nb mat: {:?}", triangles.len(), models.len(), materials.len());
        let mut bvhs: HashMap<BvhType, Vec<BvhNode>> = Default::default();
        let _ = bvhs.insert(BvhType::None, Vec::new());

        let mut scene = Scene {
            triangles,
            models,
            materials,
            camera,
            is_wireframe_on: false,
            bvh_type: BvhType::default(),
            bvhs,
        };

        // TODO: Init the bvhs
        let time = match scene.init_bvh(BvhType::DefaultTopDown) {
            Ok(time) => time,
            Err(err) => {
                error!("Failed to init a bvh: {:?}", err);
                return Err(ErrorCode::Unknown);
            }
        };
        info!(
            "It took {:?}s to build the default top down bvh",
            time.as_secs_f32()
        );
        info!(
            "Top Down BVH:\n{}",
            BvhNode::to_string(scene.bvhs.get(&BvhType::DefaultTopDown).unwrap())
        );
        Ok(scene)
    }

    pub fn update(
        &mut self,
        delta_time: f64,
        keys: &HashMap<Key, KeyState>,
    ) -> Result<(), ErrorCode> {
        // Move the camera
        if keys.contains_key(&Key::W) && (keys.get(&Key::W).unwrap() == &KeyState::Pressed) {
            self.camera
                .on_keyboard_input(CameraMovement::Forward, delta_time);
        }
        if keys.contains_key(&Key::S) && (keys.get(&Key::S).unwrap() == &KeyState::Pressed) {
            self.camera
                .on_keyboard_input(CameraMovement::Backward, delta_time);
        }
        if keys.contains_key(&Key::A) && (keys.get(&Key::A).unwrap() == &KeyState::Pressed) {
            self.camera
                .on_keyboard_input(CameraMovement::Left, delta_time);
        }
        if keys.contains_key(&Key::D) && (keys.get(&Key::D).unwrap() == &KeyState::Pressed) {
            self.camera
                .on_keyboard_input(CameraMovement::Right, delta_time);
        }
        if keys.contains_key(&Key::Up) && (keys.get(&Key::Up).unwrap() == &KeyState::Pressed) {
            self.camera
                .on_keyboard_input(CameraMovement::Up, delta_time);
        }
        if keys.contains_key(&Key::Down) && (keys.get(&Key::Down).unwrap() == &KeyState::Pressed) {
            self.camera
                .on_keyboard_input(CameraMovement::Down, delta_time);
        }
        Ok(())
    }

    /// Build a bvh and return the time in seconds it took to build it
    fn init_bvh(&mut self, bvh_type: BvhType) -> Result<Duration, ErrorCode> {
        match bvh_type {
            BvhType::None => {
                warn!("No bvh need to be build...");
                Ok(Duration::default())
            }
            BvhType::DefaultBottomUp => todo!("Bottom up bvh"),
            BvhType::DefaultTopDown => {
                let start = Instant::now();
                match BvhDefaultTopDown::build(self) {
                    Ok(new_bvh) => {
                        let end = Instant::now();
                        let _ = self.bvhs.insert(BvhType::DefaultTopDown, new_bvh);
                        Ok(end - start)
                    }
                    Err(err) => {
                        error!("Failed to build the default top down bvh: {:?}", err);
                        Err(ErrorCode::Unknown)
                    }
                }
            }
            BvhType::Ploc => todo!("Ploc bvh"),
            BvhType::Other => todo!("Other bvh"),
        }
    }

    pub fn get_bvh(&self) -> Result<&Vec<BvhNode>, ErrorCode> {
        if !self.bvhs.contains_key(&self.bvh_type) {
            error!(
                "The bvh for the current bvh type `{:?}' has not been initialized",
                self.bvh_type
            );
            Err(ErrorCode::InitializationFailure)
        } else {
            Ok(self.bvhs.get(&self.bvh_type).unwrap())
        }
    }

    #[allow(clippy::collapsible_match)]
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
            #[allow(clippy::single_match)]
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
            self.camera
                .on_mouse_moved(x_offset, y_offset, should_constrain_pitch);
        }
        Ok(())
    }
}
