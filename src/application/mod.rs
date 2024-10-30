use core::error::ErrorCode;
use std::time::{Duration, Instant};

use log::{debug, error};
use parameters::ApplicationParameters;
use pipelines::Pipelines;
use scene::Scene;
use vulkan::types::VulkanContext;
use window::init::WindowContext;
use winit::{
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

        debug!("Initializing the vulkan context...");
        let vulkan_context = match VulkanContext::init(&parameters, &window) {
            Ok(vulkan_context) => vulkan_context,
            Err(err) => {
                error!("Failed to initialize the vulkan context: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the scene...");
        let scene = match Scene::init(&vulkan_context) {
            Ok(scene) => scene,
            Err(err) => {
                error!("Failed to initialize the scene: {:?}", err);
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
}
