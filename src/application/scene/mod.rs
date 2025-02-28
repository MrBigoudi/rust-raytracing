use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use bvh::{
    aabb::Aabb, default_top_down::BvhDefaultTopDown, ploc::BvhPloc,
    ploc_parallel::BvhPlocParallel, top_down_sah::BvhTopDownSah, Bvh, BvhNode, BvhType,
};
use camera::{Camera, CameraMovement};
use glam::Vec3;
use log::{error, info, warn};
// use log::error;
use material::Material;
use model::Model;
use rand::Rng;
use triangle::Triangle;
use winit::{
    dpi::LogicalPosition,
    event::{DeviceId, ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{
    core::error::ErrorCode,
    parameters::ApplicationParameters,
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
    pub start_time: u128,
    pub current_time: f32,

    // Bvhs
    pub bvh_type: BvhType,
    pub bvh_last_type: BvhType, // Cheecky way to check if an update happened
    pub bvhs: HashMap<BvhType, Vec<BvhNode>>,
    pub bvhs_build_times: HashMap<BvhType, Duration>,
    pub should_display_bvh: bool,
    pub bvh_depth_to_display: u32,

    duration: f64,
    best_duration: f64,
    worst_duration: f64,
    nb_frames: u32,
    nb_frames_threshold: u32,

    pub best_fps: u32,
    pub worst_fps: u32,
    pub avg_fps: u32,
    pub best_ms: f64,
    pub worst_ms: f64,
    pub avg_ms: f64,
}

pub enum SceneType {
    #[allow(unused)]
    SingleSphere(u32, glam::Vec3, f32, glam::Vec3), // (resolution, position, radius, color)
    #[allow(unused)]
    MultipleSphere(u16, u32, f32, f32, f32, f32), // (nb_spheres, resolution, min_position, max_position, min_radius, max_radius)
    #[allow(unused)]
    MultipleObj(Vec<(PathBuf, glam::Mat4)>),      // ((path to the obj file, model_matrix))
}

impl Scene {
    fn init_scene_skeleton(
        triangles: Vec<Triangle>,
        models: Vec<Model>,
        materials: Vec<Material>,
        camera: Camera,
    ) -> Result<Scene, ErrorCode> {
        let bvh_type = BvhType::default();
        let mut bvhs: HashMap<BvhType, Vec<BvhNode>> = Default::default();
        let _ = bvhs.insert(BvhType::default(), Vec::new());
        let bvhs_build_times: HashMap<BvhType, Duration> = Default::default();

        let start_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(err) => {
                error!("Failed to get the current time: {:?}", err);
                return Err(ErrorCode::AccessFailure);
            }
        };

        Ok(Scene {
            triangles,
            models,
            materials,
            camera,
            is_wireframe_on: false,
            start_time,
            current_time: 0.,
            bvh_type,
            bvh_last_type: bvh_type,
            bvhs,
            bvhs_build_times,
            should_display_bvh: false,
            bvh_depth_to_display: 0,
            duration: 0.,
            best_duration: f64::MAX,
            worst_duration: 0.,
            nb_frames: 0,
            nb_frames_threshold: 30,
            best_fps: 0,
            worst_fps: 0,
            avg_fps: 0,
            best_ms: 0.,
            worst_ms: 0.,
            avg_ms: 0.,
        })
    }

    fn init_camera(
        parameters: &ApplicationParameters,
        position: Vec3,
        speed: f32,
    ) -> Camera {
        let width = parameters.window_width as f32;
        let height = parameters.window_height as f32;
        let aspect_ratio = width / height;
        let mut camera = Camera::new(
            position,
            aspect_ratio,
            50.,
            0.1,
            Vec3::new(0., 1., 0.),
        );
        camera.movement_speed = speed;
        camera
    }

    // Scene with only one centered sphere
    fn init_scene_single_sphere(
        sphere_resolution: u32,
        sphere_position: glam::Vec3,
        sphere_radius: f32,
        sphere_color: glam::Vec3,
        camera: Camera,
    ) -> Result<Scene, ErrorCode> {
        let mut triangles = Vec::new();
        let mut models = Vec::new();
        let mut materials = vec![Material::default()];

        let material = Material::uniform(&sphere_color);

        if let Err(err) = Model::add_sphere(
            sphere_resolution,
            sphere_radius,
            sphere_position,
            Some(material),
            &mut triangles,
            &mut models,
            &mut materials,
        ) {
            error!("Failed to load a new sphere to the scene: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }

        Self::init_scene_skeleton(triangles, models, materials, camera)
    }

    fn init_scene_objs(
        objs: Vec<(PathBuf, glam::Mat4)>,
        camera: Camera,
    ) -> Result<Scene, ErrorCode> {
        let mut triangles = Vec::new();
        let mut models = Vec::new();
        let mut materials = vec![Material::default()];

        for (path, model_matrix) in objs {
            if let Err(err) = Model::add_obj(
                &path,
                false,
                Some(model_matrix),
                &mut triangles,
                &mut models,
                &mut materials,
            ) {
                error!(
                    "Failed to load the object `{:?}' to the scene: {:?}",
                    path, err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        }

        Self::init_scene_skeleton(triangles, models, materials, camera)
    }

    fn init_scene_multi_spheres(
        nb_spheres: u16,
        sphere_resolution: u32,
        min_pos: f32,
        max_pos: f32,
        min_radius: f32,
        max_radius: f32,
        camera: Camera,
    ) -> Result<Scene, ErrorCode> {
        let mut triangles = Vec::new();
        let mut models = Vec::new();
        let mut materials = vec![Material::default()];

        for _ in 0..nb_spheres {
            let mut rng = rand::thread_rng();
            let radius = rng.gen::<f32>() * (max_radius - min_radius) + min_radius;
            let material = Material::random();
            let center = glam::Vec3::new(
                rng.gen::<f32>() * (max_pos - min_pos) + min_pos,
                rng.gen::<f32>() * (max_pos - min_pos) + min_pos,
                rng.gen::<f32>() * (max_pos - min_pos) + min_pos,
            );

            if let Err(err) = Model::add_sphere(
                sphere_resolution,
                radius,
                center,
                Some(material),
                &mut triangles,
                &mut models,
                &mut materials,
            ) {
                error!("Failed to load a new sphere to the scene: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        }

        Self::init_scene_skeleton(triangles, models, materials, camera)
    }

    fn from_scene_type(
        scene_type: SceneType,
        camera: Camera,
    ) -> Result<Scene, ErrorCode> {
        match scene_type {
            SceneType::SingleSphere(resolution, position, radius, color) => {
                Self::init_scene_single_sphere(resolution, position, radius, color, camera)
            }
            SceneType::MultipleSphere(
                nb_spheres,
                resolution,
                min_position,
                max_position,
                min_radius,
                max_radius,
            ) => Self::init_scene_multi_spheres(
                nb_spheres,
                resolution,
                min_position,
                max_position,
                min_radius,
                max_radius,
                camera
            ),
            SceneType::MultipleObj(objs) => Self::init_scene_objs(objs, camera),
        }
    }

    #[allow(unused)]
    fn init_single_sphere() -> SceneType {
        let resolution = 16;
        let position = glam::Vec3::ZERO;
        let radius = 10.;
        let color = glam::Vec3::new(0.1, 0.5, 0.5);
        SceneType::SingleSphere(resolution, position, radius, color)
    }

    #[allow(unused)]
    fn init_multi_spheres() -> SceneType {
        let nb_spheres = 100;
        let resolution = 132;
        let min_position = -50.;
        let max_position = 50.;
        let min_radius = 0.5;
        let max_radius = 5.;
        SceneType::MultipleSphere(
            nb_spheres,
            resolution,
            min_position,
            max_position,
            min_radius,
            max_radius,
        )
    }

    #[allow(unused)]
    fn init_single_obj(path: PathBuf) -> SceneType {
        let mut objs = Vec::new();
        let obj = (path, glam::Mat4::IDENTITY);
        objs.push(obj);
        SceneType::MultipleObj(objs)   
    }

    #[allow(unused)]
    fn init_multi_objs() -> SceneType {
        let mut objs = Vec::new();
        let armadillo = (PathBuf::from("armadillo.obj"), glam::Mat4::IDENTITY);
        objs.push(armadillo);

        let num_objects = 10;
        let radius = 150.;

        for i in 0..num_objects {
            let angle = (i as f32 / num_objects as f32) * std::f32::consts::TAU;
            let x = radius * angle.cos();
            let z = radius * angle.sin();

            // Bunny circle at height 100
            let bunny_height = 100.0;
            let bunny_translation =
                glam::Mat4::from_translation(glam::Vec3::new(x, bunny_height, z));
            let bunny_rotation = glam::Mat4::from_rotation_y(-angle);
            let bunny_scale_factor = glam::Mat4::from_scale(glam::Vec3::splat(100.));
            let bunny_model_matrix = bunny_translation * bunny_rotation * bunny_scale_factor;
            let bunny = (PathBuf::from("stanford-bunny.obj"), bunny_model_matrix);
            objs.push(bunny);

            // Suzanne circle at height 0
            let suzanne_height = 0.0;
            let suzanne_translation =
                glam::Mat4::from_translation(glam::Vec3::new(x, suzanne_height, z));
            let suzanne_rotation = glam::Mat4::from_rotation_y(-angle);
            let suzanne_scale_factor = glam::Mat4::from_scale(glam::Vec3::splat(10.));
            let suzanne_model_matrix =
                suzanne_translation * suzanne_rotation * suzanne_scale_factor;
            let suzanne = (PathBuf::from("suzanne.obj"), suzanne_model_matrix);
            objs.push(suzanne);

            // Teapot circle at height -100
            let teapot_height = -100.0;
            let teapot_translation =
                glam::Mat4::from_translation(glam::Vec3::new(x, teapot_height, z));
            let teapot_rotation = glam::Mat4::from_rotation_y(-angle);
            let teapot_scale_factor = glam::Mat4::from_scale(glam::Vec3::splat(10.));
            let teapot_model_matrix =
                teapot_translation * teapot_rotation * teapot_scale_factor;
            let teapot = (PathBuf::from("teapot.obj"), teapot_model_matrix);
            objs.push(teapot);
        }

        let dragon = (
            PathBuf::from("xyzrgb_dragon.obj"),
            glam::Mat4::from_translation(glam::Vec3::new(-50., -100., 0.)),
        );
        objs.push(dragon);
        SceneType::MultipleObj(objs)
    }

    pub fn init(parameters: &ApplicationParameters) -> Result<Scene, ErrorCode> {
        let scene_type = Self::init_single_obj(PathBuf::from("cube.obj"));
        // let scene_type = Self::init_single_obj(PathBuf::from("armadillo.obj"));
        // let scene_type = Self::init_single_sphere();

        // let camera = Self::init_camera(
        //     parameters, 
        //     Vec3::new(0., 0., -40.), 
        //     100.
        // );

        let camera = Self::init_camera(
            parameters, 
            Vec3::new(0., 0.,-3.), 
            1.
        );

        let mut scene = Self::from_scene_type(scene_type, camera)?;
        // First is the first one to display
        let bvhs_to_build = [
            BvhType::PlocParallel,
        ];

        let displayed_bvh_type = if bvhs_to_build.is_empty() {
            BvhType::None
        } else {
            bvhs_to_build[0]
        };

        scene.bvh_type = displayed_bvh_type;
        scene.bvh_last_type = displayed_bvh_type;

        for bvh_type in bvhs_to_build {
            let time = match scene.init_bvh(bvh_type) {
                Ok(time) => time,
                Err(err) => {
                    error!("Failed to init a bvh: {:?}", err);
                    return Err(ErrorCode::Unknown);
                }
            };
            info!(
                "It took {:?}s to build the `{:?}' bvh",
                time.as_secs_f32(),
                bvh_type
            );
            let _ = scene.bvhs_build_times.insert(bvh_type, time);
        }

        Ok(scene)
    }

    pub fn update(
        &mut self,
        delta_time: f64,
        keys: &HashMap<Key, KeyState>,
    ) -> Result<(), ErrorCode> {
        // Update current time
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => time.as_millis(),
            Err(err) => {
                error!("Failed to get the current time: {:?}", err);
                return Err(ErrorCode::AccessFailure);
            }
        };
        self.current_time = (now - self.start_time) as f32;

        // Delta time computation
        self.nb_frames += 1;
        self.duration += delta_time;
        if delta_time < self.best_duration {
            self.best_duration = delta_time;
        }
        if delta_time > self.worst_duration {
            self.worst_duration = delta_time;
        }
        if self.nb_frames > self.nb_frames_threshold {
            self.best_fps = (1. / self.best_duration) as u32;
            self.worst_fps = (1. / self.worst_duration) as u32;
            self.avg_fps = (self.nb_frames as f64 / self.duration) as u32;
            self.best_ms = 1000. * self.best_duration;
            self.worst_ms = 1000. * self.worst_duration;
            self.avg_ms = 1000. * self.duration / (self.nb_frames as f64);
            self.duration = 0.;
            self.nb_frames = 0;
            self.best_duration = f64::MAX;
            self.worst_duration = 0.;
        }

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
            BvhType::DefaultBottomUp => {
                let start = Instant::now();
                match BvhDefaultTopDown::build(self) {
                    Ok(new_bvh) => {
                        let end = Instant::now();
                        let _ = self.bvhs.insert(BvhType::DefaultBottomUp, new_bvh);
                        Ok(end - start)
                    }
                    Err(err) => {
                        error!("Failed to build the default bottom up bvh: {:?}", err);
                        Err(ErrorCode::Unknown)
                    }
                }
            }
            // BvhType::BottomUpSah => {
            //     let start = Instant::now();
            //     match BvhBottomUpSah::build(self) {
            //         Ok(new_bvh) => {
            //             let end = Instant::now();
            //             let _ = self.bvhs.insert(BvhType::BottomUpSah, new_bvh);
            //             Ok(end - start)
            //         }
            //         Err(err) => {
            //             error!("Failed to build the bottom up sah bvh: {:?}", err);
            //             Err(ErrorCode::Unknown)
            //         }
            //     }
            // }
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
            BvhType::TopDownSah => {
                let start = Instant::now();
                match BvhTopDownSah::build(self) {
                    Ok(new_bvh) => {
                        let end = Instant::now();
                        let _ = self.bvhs.insert(BvhType::TopDownSah, new_bvh);
                        Ok(end - start)
                    }
                    Err(err) => {
                        error!("Failed to build the top down bvh sah: {:?}", err);
                        Err(ErrorCode::Unknown)
                    }
                }
            }
            BvhType::Ploc => {
                let start = Instant::now();
                match BvhPloc::build(self) {
                    Ok(new_bvh) => {
                        let end = Instant::now();
                        let _ = self.bvhs.insert(BvhType::Ploc, new_bvh);
                        Ok(end - start)
                    }
                    Err(err) => {
                        error!("Failed to build the ploc bvh: {:?}", err);
                        Err(ErrorCode::Unknown)
                    }
                }
            }
            BvhType::PlocParallel => {
                let start = Instant::now();
                match BvhPlocParallel::build(self) {
                    Ok(new_bvh) => {
                        let end = Instant::now();
                        let _ = self.bvhs.insert(BvhType::PlocParallel, new_bvh);
                        Ok(end - start)
                    }
                    Err(err) => {
                        error!("Failed to build the ploc bvh in parallel: {:?}", err);
                        Err(ErrorCode::Unknown)
                    }
                }
            }
        }
    }

    pub fn get_aabb(&self) -> Result<Aabb, ErrorCode> {
        Aabb::from_scene(&self.triangles, &self.models)
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

    pub fn get_max_bvh_detph(&self) -> u32 {
        // TODO: access the max bvh depth
        10
    }
}
