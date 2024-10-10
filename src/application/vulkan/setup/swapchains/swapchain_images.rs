use ash::vk::{ImageAspectFlags, ImageSubresourceRange, ImageViewCreateInfo, ImageViewType};
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl VulkanContext<'_> {
    pub fn init_swapchain_images(&mut self) -> Result<(), ErrorCode> {
        let swapchain = self.swapchain_handler.as_mut().unwrap();
        swapchain.images = unsafe {
            let swapchain_device = &swapchain.device;
            match swapchain_device.get_swapchain_images(swapchain.handler) {
                Ok(images) => images,
                Err(err) => {
                    error!("Failed to get vulkan swapchain images: {:?}", err);
                    return Err(ErrorCode::VulkanFailure);
                }
            }
        };

        Ok(())
    }

    pub fn init_swapchain_image_views(&mut self) -> Result<(), ErrorCode> {
        let swapchain = self.get_swapchain_handler()?;
        let mut new_image_views = Vec::new();
        for image in &swapchain.images {
            let subresource_range = ImageSubresourceRange::default()
                .aspect_mask(ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1) // No mipmap for the swapchain image
                .base_array_layer(0)
                .layer_count(1); // Only one layer unless stereographic 3D app
            let image_view_info = ImageViewCreateInfo::default()
                .image(*image)
                .format(swapchain.surface_format.format)
                .view_type(ImageViewType::TYPE_2D) // Image are 2D textures
                .subresource_range(subresource_range);

            let new_image_view = unsafe {
                let device = self.get_device()?;
                match device.create_image_view(&image_view_info, self.get_allocator()?) {
                    Ok(image_views) => image_views,
                    Err(err) => {
                        error!(
                            "Failed to create new vulkan swapchain image views: {:?}",
                            err
                        );
                        return Err(ErrorCode::VulkanFailure);
                    }
                }
            };

            new_image_views.push(new_image_view);
        }

        {
            let swapchain = self.swapchain_handler.as_mut().unwrap();
            swapchain.image_views = new_image_views;
        }

        Ok(())
    }
}
