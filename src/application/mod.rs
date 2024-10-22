use core::error::ErrorCode;

use window::init::WindowContext;
use winit::{event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::Window};
use log::{debug, error};
use parameters::ApplicationParameters;
use pipelines::Pipelines;
use scene::Scene;
use vulkan::types::VulkanContext;

mod core;
mod parameters;
mod pipelines;
mod scene;
mod vulkan;
mod window;
mod handler;

#[derive(Default)]
pub struct Application<'a> {
    parameters: ApplicationParameters,
    window: Option<Window>,
    vulkan_context: Option<VulkanContext<'a>>,
    scene: Option<Scene>,
    pipelines: Option<Pipelines>,
}

impl Application<'_> {
    fn init(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ErrorCode> {
        debug!("Initializing parameters...");
        let parameters = ApplicationParameters::default();

        debug!("Initializing the window...");
        let window = match WindowContext::init(&parameters, event_loop){
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
        let scene = Scene::default();

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
        let event_loop = match EventLoop::new(){
            Ok(event_loop) => event_loop,
            Err(err) => {
                error!("Failed to initialize the event loop: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = Application::default();
        if let Err(err) = event_loop.run_app(&mut app){
            error!("An error occured during the main event loop: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }
}