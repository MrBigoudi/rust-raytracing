use std::cmp::min;

use ash::vk::{CompositeAlphaFlagsKHR, ImageUsageFlags, SharingMode, SurfaceTransformFlagsKHR};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub struct SwapchainCreateInfoParameters {
    pub min_image_count: u32,
    pub pre_transform: SurfaceTransformFlagsKHR,
    pub image_array_layers: u32,
    pub composite_alpha: CompositeAlphaFlagsKHR,
    pub clipped: bool,
    pub image_usage: ImageUsageFlags,
    pub queue_family_indices: Vec<u32>,
    pub image_sharing_mode: SharingMode,
}

impl VulkanContext<'_> {
    pub fn swapchain_get_image_count(&self) -> Result<u32, ErrorCode> {
        let supported_capabilities = &self.get_swapchain_support_surface_capabilities()?;
        let image_count = supported_capabilities.min_image_count + 1;
        if supported_capabilities.max_image_count > 0 {
            Ok(min(image_count, supported_capabilities.max_image_count))
        } else {
            Ok(image_count)
        }
    }

    fn swapchain_get_queue_families_and_image_sharing_mode(
        &self,
    ) -> Result<(Vec<u32>, SharingMode), ErrorCode> {
        let queues = self.get_queues()?;
        let graphics_queue_index = queues.graphics_family_index.unwrap() as u32;
        let present_queue_index = queues.present_family_index.unwrap() as u32;
        // If the graphics queue family is different from the presentation queue we draw on the images
        // in the swap chain from the graphics queue and then submit them on the presentation queue
        if graphics_queue_index != present_queue_index {
            Ok((
                vec![graphics_queue_index, present_queue_index],
                // Images can be used across multiple queue families without explicit ownership transfers
                // Slower but easier to handle
                SharingMode::CONCURRENT,
            ))
        } else {
            Ok((
                Vec::new(),
                // Image is owned by one queue family at a time and ownership must be explicitly transferred
                // before using it in another queue family. This option offers the best performance
                SharingMode::EXCLUSIVE,
            ))
        }
    }

    pub fn swapchain_get_create_info_parameters(
        &self,
    ) -> Result<SwapchainCreateInfoParameters, ErrorCode> {
        // Get the number of images in the swapchain
        let min_image_count = self.swapchain_get_image_count()?;
        // The transform that should be applied to images in the swap chain
        let pre_transform = self
            .get_swapchain_support_surface_capabilities()?
            .current_transform;
        // The amount of layers each image consists of. This is always 1 unless developing a stereoscopic 3D application
        let image_array_layers = 1;
        // Specifies if the alpha channel should be used for blending with other windows
        // in the window system. Almost always want to simply ignore the alpha channel
        let composite_alpha = CompositeAlphaFlagsKHR::OPAQUE;
        // If true, that means we don't care about the color of pixels that are obscured
        // for example because another window is in front of them
        let clipped = true;
        // Specifies operations the images will be used for. Usually used as color attachment to render directly to them
        // Possible to render to another image first and do post-processing (then use VK_IMAGE_USAGE_TRANSFER_DST_BIT instead
        // and use a memory operation to transfer the rendered image to a swap chain image)
        let image_usage = ImageUsageFlags::COLOR_ATTACHMENT;
        // Specifies how to handle swap chain images that will be used across multiple queue families
        let (queue_family_indices, image_sharing_mode) =
            self.swapchain_get_queue_families_and_image_sharing_mode()?;

        Ok(SwapchainCreateInfoParameters {
            min_image_count,
            pre_transform,
            image_array_layers,
            composite_alpha,
            clipped,
            image_usage,
            queue_family_indices,
            image_sharing_mode,
        })
    }
}
