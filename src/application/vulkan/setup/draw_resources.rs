use ash::vk::{
    Extent3D, Format, Image, ImageAspectFlags, ImageCreateInfo, ImageSubresourceRange, ImageTiling,
    ImageType, ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, MemoryPropertyFlags,
    SampleCountFlags,
};
use log::error;
use vk_mem::{Alloc, Allocation, AllocationCreateInfo, MemoryUsage};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub struct AllocatedImage {
    pub image: Image,
    pub image_view: ImageView,
    pub image_extent: Extent3D,
    #[allow(dead_code)]
    pub image_format: Format,
    pub allocation: Allocation,
}

impl VulkanContext<'_> {
    fn init_draw_image(&mut self) -> Result<(), ErrorCode> {
        // This is 16 bit floats for all 4 channels, and will use 64 bits per pixel
        let image_format = Format::R16G16B16A16_SFLOAT;

        // Match the window size
        let image_extent = Extent3D::default()
            .width(self.framebuffer_width)
            .height(self.framebuffer_height)
            .depth(1);

        // In vulkan, all images and buffers must fill a UsageFlags with what they will be used for
        // This allows the driver to perform optimizations in the background depending on what that buffer or image
        // is going to do later. We want TransferSRC and TransferDST so that we can copy from and into the image,
        // Storage because thats the “compute shader can write to it”
        // layout, and Color Attachment so that we can use graphics pipelines to draw geometry into it
        let image_usages = ImageUsageFlags::default()
            | ImageUsageFlags::TRANSFER_SRC
            | ImageUsageFlags::TRANSFER_DST
            | ImageUsageFlags::STORAGE
            | ImageUsageFlags::COLOR_ATTACHMENT;

        let image_create_info = ImageCreateInfo::default()
            .image_type(ImageType::TYPE_2D)
            .format(image_format)
            .extent(image_extent)
            .mip_levels(1)
            .array_layers(1)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(image_usages);

        let allocation_info = AllocationCreateInfo {
            // For the draw image, we prefer to allocate it from gpu local memory
            usage: MemoryUsage::AutoPreferDevice,
            required_flags: MemoryPropertyFlags::DEVICE_LOCAL,
            ..Default::default()
        };

        // Allocate and create the image
        let allocator = self.allocator.as_ref().unwrap();
        let allocator = allocator.allocator.lock().unwrap();
        let (image, allocation) =
            match unsafe { allocator.create_image(&image_create_info, &allocation_info) } {
                Ok((image, allocation)) => (image, allocation),
                Err(err) => {
                    error!("Failed to allocate a vulkan image: {:?}", err);
                    return Err(ErrorCode::VulkanFailure);
                }
            };

        // Build the image view
        let image_view_create_info = ImageViewCreateInfo::default()
            .view_type(ImageViewType::TYPE_2D)
            .image(image)
            .format(image_format)
            .subresource_range(ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        let device = self.get_device()?;
        let image_view = match unsafe {
            device.create_image_view(&image_view_create_info, self.get_allocation_callback()?)
        } {
            Ok(image_view) => image_view,
            Err(err) => {
                error!("Failed to allocate a vulkan image view: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };

        self.draw_image = Some(AllocatedImage {
            image,
            image_view,
            image_extent,
            image_format,
            allocation,
        });

        Ok(())
    }

    pub fn init_draw_resources(&mut self) -> Result<(), ErrorCode> {
        if let Err(err) = self.init_draw_image() {
            error!("Failed to initialize the draw image: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }

        Ok(())
    }

    pub fn get_draw_image(&self) -> Result<&AllocatedImage, ErrorCode> {
        match &self.draw_image {
            Some(image) => Ok(image),
            None => {
                error!("Can't access the vulkan draw image");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn clean_draw_resources(&mut self) -> Result<(), ErrorCode> {
        if self.draw_image.is_none() {
            return Ok(());
        };
        unsafe {
            self.get_device()?.destroy_image_view(
                self.get_draw_image()?.image_view,
                self.get_allocation_callback()?,
            );
        }

        let allocator = self.allocator.as_ref().unwrap();
        let allocator = allocator.allocator.lock().unwrap();
        unsafe {
            allocator.destroy_image(
                self.get_draw_image()?.image,
                &mut self.draw_image.as_mut().unwrap().allocation,
            )
        };

        Ok(())
    }
}
