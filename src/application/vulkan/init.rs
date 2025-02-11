use log::{debug, error};
use winit::window::Window;

use crate::application::{core::error::ErrorCode, parameters::ApplicationParameters};

use super::types::VulkanContext;

impl VulkanContext<'_> {
    pub fn init(parameters: &ApplicationParameters, window: &Window) -> Result<Self, ErrorCode> {
        let mut context = VulkanContext {
            parameters: parameters.clone(),
            ..Self::default()
        };

        if let Err(err) = context.init_entry() {
            error!("Failed to initialize the vulkan entry: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan entry initialized successfully !");
        }

        if let Err(err) = context.init_allocation_callback() {
            error!("Failed to initialize the vulkan allocator: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan allocator initialized successfully !");
        }

        if let Err(err) = context.init_instance(window) {
            error!("Failed to initialize the vulkan instance: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan instance initialized successfully !");
        }

        #[cfg(debug_assertions)]
        {
            if let Err(err) = context.init_debugger() {
                error!("Failed to initialize the vulkan debugger: {:?}", err);
                return Err(ErrorCode::InitializationFailure);
            } else {
                debug!("Vulkan debugger initialized successfully !");
            }
        }

        if let Err(err) = context.init_surface(window) {
            error!("Failed to initialize the vulkan surface: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan surface initialized successfully !");
        }

        if let Err(err) = context.init_device_requirements() {
            error!(
                "Failed to initialize the vulkan device requirements: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan device requirements initialized successfully !");
        }

        if let Err(err) = context.init_physical_device() {
            error!("Failed to initialize the vulkan physical device: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan physical device initialized successfully !");
        }

        if let Err(err) = context.init_device() {
            error!("Failed to initialize the vulkan logical device: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan logical device initialized successfully !");
        }

        if let Err(err) = context.init_queues() {
            error!(
                "Failed to initialize the vulkan logical device queues: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan logical device queues initialized successfully !");
        }

        if let Err(err) = context.init_framebuffer_dimensions() {
            error!(
                "Failed to initialize the vulkan framebuffer dimensions: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan framebuffer dimensions initialized successfully: (width={:?}, height={:?})!",
                context.framebuffer_width, context.framebuffer_height
            );
        }

        if let Err(err) = context.init_swapchain() {
            error!("Failed to initialize the vulkan swapchain: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan swapchain initialized successfully !");
        }

        if let Err(err) = context.init_commands() {
            error!("Failed to initialize the vulkan commands: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan commands initialized successfully !");
        }

        if let Err(err) = context.init_allocator() {
            error!(
                "Failed to initialize the vulkan memory allocator: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan memory allocator initialized successfully !");
        }

        if let Err(err) = context.init_draw_resources() {
            error!(
                "Failed to initialize the vulkan drawing resources: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan drawing resources initialized successfully !");
        }

        if let Err(err) = context.init_immediate() {
            error!(
                "Failed to initialize the vulkan immediate submit structure: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan immediate submit structure initialized successfully !");
        }

        if let Err(err) = context.init_gui(window) {
            error!("Failed to initialize the vulkan gui: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        } else {
            debug!("Vulkan gui initialized successfully !");
        }

        Ok(context)
    }
}
