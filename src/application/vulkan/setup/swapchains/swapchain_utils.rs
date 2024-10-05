use std::cmp::{max, min};

use ash::vk::{ColorSpaceKHR, Format, PresentModeKHR, SurfaceFormatKHR};
use log::debug;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl VulkanContext<'_> {
    pub fn swapchain_select_max_frames_in_flight(&mut self, nb_frames: u16) -> Result<(), ErrorCode> {
        let swapchain = self.swapchain_handler.as_mut().unwrap();
        swapchain.max_frames_in_flight = nb_frames;
        Ok(())
    }

    pub fn swapchain_select_format_and_color_space(
        &mut self,
        prefered_format: Format,
        prefered_color_space: ColorSpaceKHR,
    ) -> Result<(), ErrorCode> {
        let supported_formats = &self.get_swapchain_support_surface_formats()?;
        let mut selected_format: Option<SurfaceFormatKHR> = None;
        'get_prefered_format_loop: for format in supported_formats {
            if format.format == prefered_format && format.color_space == prefered_color_space {
                selected_format = Some(*format);
                break 'get_prefered_format_loop;
            }
        }
        let swapchain = self.swapchain_handler.as_mut().unwrap();
        match selected_format {
            Some(format) => swapchain.surface_format = format,
            None => swapchain.surface_format = supported_formats[0],
        }
        debug!("The swapchain format is: {:?}", swapchain.surface_format);
        Ok(())
    }

    pub fn swapchain_select_present_mode(
        &mut self,
        prefered_mode: PresentModeKHR,
    ) -> Result<(), ErrorCode> {
        let supported_present_modes = &self.get_swapchain_support_surface_present_modes()?;
        let mut selected_present_mode: Option<PresentModeKHR> = None;
        'get_prefered_present_mode_loop: for present_mode in supported_present_modes {
            if *present_mode == prefered_mode {
                selected_present_mode = Some(*present_mode);
                break 'get_prefered_present_mode_loop;
            }
        }
        let swapchain = self.swapchain_handler.as_mut().unwrap();
        match selected_present_mode {
            Some(present_mode) => swapchain.present_mode = present_mode,
            None => swapchain.present_mode = PresentModeKHR::FIFO,
        }
        debug!(
            "The swapchain present mode is: {:?}",
            swapchain.present_mode
        );
        Ok(())
    }

    pub fn swpachain_select_extent(&mut self, width: u32, height: u32) -> Result<(), ErrorCode> {
        let supported_capabilities = &self.get_swapchain_support_surface_capabilities()?;
        let swapchain = self.swapchain_handler.as_mut().unwrap();
        // Max value means to not match the resolution of the window
        if supported_capabilities.current_extent.width != u32::MAX
            && supported_capabilities.current_extent.height != u32::MAX
        {
            swapchain.extent = supported_capabilities.current_extent;
        } else {
            // Clamp the given extent to the allowed value
            let min_extent = supported_capabilities.min_image_extent;
            let max_extent = supported_capabilities.max_image_extent;
            swapchain.extent.width = min(max_extent.width, max(min_extent.width, width));
            swapchain.extent.height = min(max_extent.height, max(min_extent.height, height));
        }
        debug!("The swapchain extent is: {:?}", swapchain.extent);
        Ok(())
    }
}
