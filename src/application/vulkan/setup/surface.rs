use ash::{khr::surface, vk::SurfaceKHR};
use log::error;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl VulkanContext<'_> {
    pub fn get_surface_loader(&self) -> Result<&surface::Instance, ErrorCode> {
        match &self.surface_loader {
            Some(surface) => Ok(surface),
            None => {
                error!("Can't access the vulkan surface loader");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn get_surface(&self) -> Result<&SurfaceKHR, ErrorCode> {
        match &self.surface {
            Some(surface) => Ok(surface),
            None => {
                error!("Can't access the vulkan surface");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn init_surface(&mut self, window: &Window) -> Result<(), ErrorCode> {
        // init the loader
        let surface_loader = surface::Instance::new(self.get_entry()?, self.get_instance()?);

        // init the platform specific surface
        let surface = match unsafe {
            ash_window::create_surface(
                self.get_entry()?,
                self.get_instance()?,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
        } {
            Ok(surface) => surface,
            Err(err) => {
                error!("Failed to create the vulkan surface: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };

        self.surface_loader = Some(surface_loader);
        self.surface = Some(surface);

        Ok(())
    }

    pub fn clean_surface(&mut self) -> Result<(), ErrorCode> {
        unsafe {
            self.get_surface_loader()?
                .destroy_surface(*self.get_surface()?, self.get_allocator()?);
        }
        self.surface = None;
        self.surface_loader = None;
        Ok(())
    }
}
