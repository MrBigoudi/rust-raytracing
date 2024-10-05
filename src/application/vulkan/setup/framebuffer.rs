use std::cmp::{max, min};

use crate::application::{
    core::error::ErrorCode, parameters::ApplicationParameters, vulkan::types::VulkanContext,
};

impl VulkanContext<'_> {
    pub fn init_framebuffer_dimensions(
        &mut self,
        parameters: &ApplicationParameters,
    ) -> Result<(), ErrorCode> {
        let width = parameters.window_width as u32;
        let height = parameters.window_height as u32;
        // Clamp framebuffer to swapchain surface capacity
        let swapchain_support_max_extent = self
            .get_swapchain_support_details()?
            .capabilities
            .max_image_extent;
        let swapchain_support_min_extent = self
            .get_swapchain_support_details()?
            .capabilities
            .min_image_extent;
        self.framebuffer_width = min(
            swapchain_support_max_extent.width,
            max(swapchain_support_min_extent.width, width),
        );
        self.framebuffer_height = min(
            swapchain_support_max_extent.height,
            max(swapchain_support_min_extent.height, height),
        );
        Ok(())
    }
}