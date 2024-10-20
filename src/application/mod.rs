use core::error::ErrorCode;

use ::winit::event::{Event, WindowEvent};
use log::{debug, error, warn};
use parameters::ApplicationParameters;
use pipelines::Pipelines;
use scene::Scene;
use vulkan::types::VulkanContext;

mod core;
mod parameters;
mod pipelines;
mod scene;
mod vulkan;
mod winit;

pub struct Application;

impl Application {
    /// Render
    fn render(vulkan_context: &mut VulkanContext, pipelines: &Pipelines) -> Result<(), ErrorCode> {
        if let Err(err) = vulkan_context.draw(pipelines) {
            error!("The vulkan context failed to draw stuff: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }
        Ok(())
    }

    /// Init, run, and clean the application
    pub fn run() -> Result<(), ErrorCode> {
        // Initialize the application
        debug!("Initializing parameters...");
        let parameters = ApplicationParameters::default();

        debug!("Initializing the event loop...");
        let event_loop = match Self::init_event_loop() {
            Ok(event_loop) => event_loop,
            Err(err) => {
                error!("Failed to initialize the event loop: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the window...");
        let window = match Self::init_window(&parameters, &event_loop) {
            Ok(event_loop) => event_loop,
            Err(err) => {
                error!("Failed to initialize the event loop: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the vulkan context...");
        let mut vulkan_context = match VulkanContext::init(&parameters, &window) {
            Ok(vulkan_context) => vulkan_context,
            Err(err) => {
                error!("Failed to initialize the vulkan context: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        debug!("Initializing the pipelines...");
        let pipelines = match Pipelines::init(&vulkan_context, &Scene::default()) {
            Ok(pipelines) => pipelines,
            Err(err) => {
                error!("Failed to initialize the pipelines: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            }
        };

        // Main loop
        if let Err(err) = event_loop.run(move |event, elwt| {
            match event {
                Event::LoopExiting => {
                    if let Err(err) = pipelines.clean(&vulkan_context) {
                        panic!("Failed to clean the pipelines: {:?}", err);
                    } else {
                        debug!("Pipelines cleaned successfully !");
                    }
                    if let Err(err) = vulkan_context.clean() {
                        panic!("Failed to clean the vulkan context: {:?}", err);
                    } else {
                        debug!("Vulkan context cleaned successfully !");
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    elwt.exit();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_physical_size),
                    ..
                } => {
                    vulkan_context.parameters.window_width = new_physical_size.width as u16;
                    vulkan_context.parameters.window_height = new_physical_size.height as u16;
                    warn!("The window has been resized...");
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    if let Err(err) = Application::render(&mut vulkan_context, &pipelines) {
                        panic!("Failed to render the application: {:?}", err);
                    }
                }
                _ => (),
            };
            // if let Err(err) = Application::input_handler(&event, elwt) {
            //     error!("Failed to handle inputs in the application: {:?}", err);
            //     panic!();
            // }
        }) {
            error!("An error occured during the main loop: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }
}
