use log::{debug, error};

use crate::application::core::error::ErrorCode;

use super::types::VulkanContext;

impl VulkanContext<'_> {
    pub fn clean(&mut self) -> Result<(), ErrorCode> {
        self.device_wait_idle().unwrap();

        if let Err(err) = self.clean_gui() {
            error!("Failed to shutdown the vulkan gui: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan gui cleaned successfully !");
        }

        if let Err(err) = self.clean_immediate() {
            error!("Failed to shutdown the vulkan immediate submit structures: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan immediate submit structures cleaned successfully !");
        }

        if let Err(err) = self.clean_draw_resources() {
            error!("Failed to shutdown the vulkan drawing resources: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan drawing resources cleaned successfully !");
        }

        if let Err(err) = self.clean_allocator() {
            error!("Failed to shutdown the vulkan memory allocator: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan memory allocator cleaned successfully !");
        }

        if let Err(err) = self.clean_commands() {
            error!("Failed to shutdown the vulkan commands: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan commands cleaned successfully !");
        }

        if let Err(err) = self.clean_swpachain() {
            error!("Failed to shutdown the vulkan swapchain: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan swapchain cleaned successfully !");
        }

        if let Err(err) = self.clean_queues() {
            error!(
                "Failed to shutdown the vulkan logical device queues: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan logical device queues cleaned successfully !");
        }

        if let Err(err) = self.clean_device() {
            error!("Failed to shutdown the vulkan logical device: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan logical device cleaned successfully !");
        }

        if let Err(err) = self.clean_physical_device() {
            error!("Failed to shutdown the vulkan physical device: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan physical device cleaned successfully !");
        }

        if let Err(err) = self.clean_device_requirements() {
            error!(
                "Failed to shutdown the vulkan device requirements: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan device requirements cleaned successfully !");
        }

        if let Err(err) = self.clean_surface() {
            error!("Failed to shutdown the vulkan surface: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan surface cleaned successfully !");
        }

        #[cfg(debug_assertions)]
        {
            if let Err(err) = self.clean_debugger() {
                error!("Failed to clean the vulkan debugger: {:?}", err);
                return Err(ErrorCode::CleaningFailure);
            } else {
                debug!("Vulkan debugger cleaned successfully !");
            }
        }

        if let Err(err) = self.clean_instance() {
            error!("Failed to clean the vulkan instance: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan instance cleaned successfully !");
        }

        if let Err(err) = self.clean_allocation_callback() {
            error!("Failed to clean the vulkan allocator: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan allocator cleaned successfully !");
        }

        if let Err(err) = self.clean_entry() {
            error!("Failed to clean the vulkan entry: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        } else {
            debug!("Vulkan entry cleaned successfully !");
        }

        Ok(())
    }
}
