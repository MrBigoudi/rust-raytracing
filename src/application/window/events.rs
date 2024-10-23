use log::{debug, error, warn};
use winit::dpi::PhysicalSize;

use crate::application::{core::error::ErrorCode, Application};

impl Application<'_> {
    pub fn on_exit(&mut self) -> Result<(), ErrorCode> {
        if let Some(vulkan_context) = &mut self.vulkan_context {
            if let Some(pipelines) = &mut self.pipelines {
                if let Err(err) = pipelines.clean(vulkan_context) {
                    error!("Failed to clean the pipelines: {:?}", err);
                    return Err(ErrorCode::CleaningFailure);
                } else {
                    debug!("Pipelines cleaned successfully !");
                }
            } else {
                warn!("The pipelines are not initialized correctly...")
            }

            if let Err(err) = vulkan_context.clean() {
                error!("Failed to clean the vulkan context: {:?}", err);
                return Err(ErrorCode::CleaningFailure);
            } else {
                debug!("Vulkan context cleaned successfully !");
            }
        } else {
            warn!("The vulkan context is not initialized correctly...");
        }

        Ok(())
    }

    pub fn on_resize(&mut self, new_physical_size: PhysicalSize<u32>) -> Result<(), ErrorCode> {
        self.parameters.window_width = new_physical_size.width as u16;
        self.parameters.window_height = new_physical_size.height as u16;
        if let Some(vulkan_context) = &mut self.vulkan_context {
            vulkan_context.parameters.window_width = new_physical_size.width as u16;
            vulkan_context.parameters.window_height = new_physical_size.height as u16;
        } else {
            warn!("The vulkan context is not initialized correctly...");
        }
        Ok(())
    }

    pub fn on_redraw(&mut self) -> Result<(), ErrorCode> {
        if let Some(vulkan_context) = &mut self.vulkan_context {
            if let Some(pipelines) = &self.pipelines {
                if let Some(window) = &self.window {
                    if let Err(err) = vulkan_context.draw(pipelines, window) {
                        error!("The vulkan context failed to draw stuff: {:?}", err);
                        return Err(ErrorCode::VulkanFailure);
                    }
                } else {
                    warn!("The window is not initialized correctly...")
                }
            } else {
                warn!("The pipelines are not initialized correctly...")
            }
        } else {
            warn!("The vulkan context is not initialized correctly...");
        }
        Ok(())
    }
}
