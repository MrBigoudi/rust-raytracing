use core::error::ErrorCode;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use log::{debug, error, warn};
use parameters::ApplicationParameters;
use pipelines::Pipelines;
use scene::Scene;
use vulkan::types::VulkanContext;
use window::{
    init::WindowContext,
    key_map::{Key, KeyState},
};
use winit::{
    dpi::LogicalPosition,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

mod core;
mod handler;
mod parameters;
mod pipelines;
mod scene;
mod vulkan;
mod window;

pub struct Application<'a> {
    parameters: ApplicationParameters,
    window: Option<Window>,
    vulkan_context: Option<VulkanContext<'a>>,
    scene: Option<Scene>,
    pipelines: Option<Pipelines>,
    last_frame: Instant,
    delta_time: Duration,
    keys: HashMap<Key, KeyState>,
    mouse_position: Option<LogicalPosition<f64>>,
}

impl Default for Application<'_> {
    fn default() -> Self {
        Self {
            parameters: Default::default(),
            window: Default::default(),
            vulkan_context: Default::default(),
            scene: Default::default(),
            pipelines: Default::default(),
            last_frame: Instant::now(),
            delta_time: Default::default(),
            keys: Default::default(),
            mouse_position: None,
        }
    }
}

impl Application<'_> {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErrorCode> {
        debug!("Initializing parameters...");
        let parameters = ApplicationParameters::default();

        debug!("Initializing the window...");
        let window = match WindowContext::init(&parameters, event_loop) {
            Ok(window) => window,
            Err(err) => {
                error!("Failed to initialize the application window: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the scene...");
        let scene = match Scene::init(&parameters) {
            Ok(scene) => scene,
            Err(err) => {
                error!("Failed to initialize the scene: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the vulkan context...");
        let vulkan_context = match VulkanContext::init(&parameters, &window) {
            Ok(vulkan_context) => vulkan_context,
            Err(err) => {
                error!("Failed to initialize the vulkan context: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the pipelines...");
        let pipelines = match Pipelines::init(&vulkan_context, &scene) {
            Ok(pipelines) => pipelines,
            Err(err) => {
                error!("Failed to initialize the pipelines: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        self.parameters = parameters;
        self.window = Some(window);
        self.vulkan_context = Some(vulkan_context);
        self.scene = Some(scene);
        self.pipelines = Some(pipelines);

        Ok(())
    }

    pub fn run() -> Result<(), ErrorCode> {
        debug!("Initializing the event loop...");
        let event_loop = match EventLoop::new() {
            Ok(event_loop) => event_loop,
            Err(err) => {
                error!("Failed to initialize the event loop: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = Application::default();
        if let Err(err) = event_loop.run_app(&mut app) {
            error!("An error occured during the main event loop: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), ErrorCode> {
        // Update delta time
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;

        // Update the scene
        if let Some(ref mut scene) = &mut self.scene {
            if let Err(err) = scene.update(self.delta_time.as_secs_f64(), &self.keys) {
                error!(
                    "Failed to update the scene when updating the application: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
            // Update the bvh
            if scene.bvh_last_type != scene.bvh_type {
                if let Some(ref mut pipelines) = &mut self.pipelines {
                    if let Some(vulkan_context) = &self.vulkan_context {
                        if let Err(err) = pipelines
                            .raytracing_pipeline
                            .update_bvhs_buffer(vulkan_context, scene)
                        {
                            error!(
                                "Failed to update the bvhs buffer in the raytracing pipeline when updating the application: {:?}",
                                err
                            );
                            return Err(ErrorCode::Unknown);
                        }
                    } else {
                        warn!("The vulkan context is not initialized correctly...");
                    }
                } else {
                    warn!("The pipelines are not initialized correctly...");
                }
                scene.bvh_last_type = scene.bvh_type;
            }
        } else {
            warn!("The scene is not initialized correctly...");
        }
        Ok(())
    }
}
