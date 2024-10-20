use ash::{
    khr::swapchain,
    vk::{
        ColorSpaceKHR, Extent2D, Format, Image, ImageView, PresentModeKHR, SurfaceFormatKHR,
        SwapchainCreateInfoKHR, SwapchainKHR,
    },
};
use log::{error, warn};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub(crate) struct SwapchainHandler {
    pub device: swapchain::Device,
    pub handler: SwapchainKHR,
    pub surface_format: SurfaceFormatKHR,
    pub present_mode: PresentModeKHR,
    pub max_frames_in_flight: u16,
    pub images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    pub extent: Extent2D,
}

impl VulkanContext<'_> {
    pub fn init_swapchain(&mut self) -> Result<(), ErrorCode> {
        let swapchain_device = swapchain::Device::new(self.get_instance()?, self.get_device()?);
        self.swapchain_handler = Some(SwapchainHandler {
            device: swapchain_device,
            handler: SwapchainKHR::default(),
            surface_format: SurfaceFormatKHR::default(),
            present_mode: PresentModeKHR::default(),
            max_frames_in_flight: 0,
            images: Vec::new(),
            image_views: Vec::new(),
            extent: Extent2D::default(),
        });

        if let Err(err) = self.swapchain_create(self.framebuffer_width, self.framebuffer_height) {
            error!("Failed to create the swapchain: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }
        Ok(())
    }

    pub fn clean_swpachain(&mut self) -> Result<(), ErrorCode> {
        if self.swapchain_handler.is_none() {
            return Ok(());
        }
        self.swapchain_destroy_base()?;
        self.swapchain_handler = None;
        Ok(())
    }

    pub fn get_swapchain_handler(&self) -> Result<&SwapchainHandler, ErrorCode> {
        match &self.swapchain_handler {
            Some(swapchain) => Ok(swapchain),
            None => {
                error!("Can't access the vulkan swapchain");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn swapchain_create(&mut self, width: u32, height: u32) -> Result<(), ErrorCode> {
        if let Err(err) = self.swapchain_create_base(width, height) {
            error!("Failed to create the base swapchain: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }
        Ok(())
    }

    pub fn swapchain_recreate(&mut self) -> Result<(), ErrorCode> {
        warn!("Recreating the swapchain...");
        self.device_wait_idle()?;

        // Destroy the old swapchain
        if let Err(err) = self.swapchain_destroy_base() {
            error!(
                "Failed to destroy previous swapchain when recreating a swapchain: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        }

        // Recompute the framebuffer dimensions
        if let Err(err) = self.init_framebuffer_dimensions() {
            error!(
                "Failed to recompute the framebuffer dimensions when recreating a swapchain: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        }

        // Recreate a new swapchain
        let new_width = self.framebuffer_width;
        let new_height = self.framebuffer_height;
        if let Err(err) = self.swapchain_create_base(new_width, new_height) {
            error!("Failed to create a new swapchain: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }

        // Cleanup sync structures
        if let Err(err) = self.clean_frames_sync_structures(){
            error!("Failed to clean the sync structures when recreating the swapchain: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.init_frames_sync_structures(){
            error!("Failed to initialize the sync structures when recreating the swapchain: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        }

        Ok(())
    }

    fn swapchain_destroy_base(&mut self) -> Result<(), ErrorCode> {
        // Only destroy the views, not the images, since those are owned by the swapchain
        for image_view in &self.get_swapchain_handler()?.image_views {
            let device = self.get_device()?;
            unsafe {
                device.destroy_image_view(*image_view, self.get_allocation_callback()?);
            }
        }

        unsafe {
            let device = &self.get_swapchain_handler()?.device;
            let swapchain = self.get_swapchain_handler()?.handler;
            device.destroy_swapchain(swapchain, self.get_allocation_callback()?)
        }

        Ok(())
    }

    fn swapchain_create_base(&mut self, width: u32, height: u32) -> Result<(), ErrorCode> {
        // For triple buffering, so at most writting to 2 frames at a time
        if let Err(err) = self.swapchain_select_max_frames_in_flight(2) {
            error!(
                "Failed to select the number of maximum frames in flight: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }
        // Select the surface format and color space
        if let Err(err) = self.swapchain_select_format_and_color_space(
            Format::B8G8R8A8_SRGB,
            ColorSpaceKHR::SRGB_NONLINEAR,
        ) {
            error!(
                "Failed to select the swapchain format and color space: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }
        // Select the presentation mode
        // if let Err(err) = self.swapchain_select_present_mode(PresentModeKHR::MAILBOX) {
        if let Err(err) = self.swapchain_select_present_mode(PresentModeKHR::FIFO) {
            error!(
                "Failed to select the swapchain presentation mode: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }
        // Select the extent
        if let Err(err) = self.swpachain_select_extent(width, height) {
            error!("Failed to select the swapchain extent: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }
        // Create the swapchain info
        let swapchain_create_info_params = match self.swapchain_get_create_info_parameters() {
            Ok(params) => params,
            Err(err) => {
                error!(
                    "Failed to get all the swapchain create info parameters: {:?}",
                    err
                );
                return Err(ErrorCode::VulkanFailure);
            }
        };
        let swapchain_handler = self.get_swapchain_handler()?;
        let surface = self.get_surface()?;
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .image_format(swapchain_handler.surface_format.format)
            .image_color_space(swapchain_handler.surface_format.color_space)
            .present_mode(swapchain_handler.present_mode)
            .image_extent(swapchain_handler.extent)
            .surface(*surface)
            .min_image_count(swapchain_create_info_params.min_image_count)
            .pre_transform(swapchain_create_info_params.pre_transform)
            .image_array_layers(swapchain_create_info_params.image_array_layers)
            .composite_alpha(swapchain_create_info_params.composite_alpha)
            .clipped(swapchain_create_info_params.clipped)
            .image_usage(swapchain_create_info_params.image_usage)
            .queue_family_indices(&swapchain_create_info_params.queue_family_indices)
            .image_sharing_mode(swapchain_create_info_params.image_sharing_mode);

        // Create the swapchain
        let swapchain = unsafe {
            let swapchain_device = &self.get_swapchain_handler()?.device;
            match swapchain_device
                .create_swapchain(&swapchain_create_info, self.get_allocation_callback()?)
            {
                Ok(swapchain) => swapchain,
                Err(err) => {
                    error!("Failed to create a vulkan swapchain: {:?}", err);
                    return Err(ErrorCode::VulkanFailure);
                }
            }
        };
        self.swapchain_handler.as_mut().unwrap().handler = swapchain;

        // Create the swapchain images
        if let Err(err) = self.init_swapchain_images() {
            error!("Failed to initialize the swapchain images: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }
        if let Err(err) = self.init_swapchain_image_views() {
            error!("Failed to initialize the swapchain image views: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }

        Ok(())
    }
}
