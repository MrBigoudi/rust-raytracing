use ash::vk::{PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

#[derive(Default, Debug)]
pub struct SwapchainSupportDetails {
    #[allow(dead_code)]
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}

impl SwapchainSupportDetails {
    pub fn is_complete(&self) -> bool {
        !self.formats.is_empty() && !self.present_modes.is_empty()
    }
}

impl VulkanContext<'_> {
    pub fn get_swapchain_support_details(&self) -> Result<SwapchainSupportDetails, ErrorCode> {
        self.query_swapchain_support(self.get_physical_device()?)
    }

    pub fn get_swapchain_support_surface_capabilities(
        &self,
    ) -> Result<SurfaceCapabilitiesKHR, ErrorCode> {
        self.query_swapchain_support_surface_capabilities(self.get_physical_device()?)
    }

    pub fn get_swapchain_support_surface_formats(
        &self,
    ) -> Result<Vec<SurfaceFormatKHR>, ErrorCode> {
        self.query_swapchain_support_surface_formats(self.get_physical_device()?)
    }

    pub fn get_swapchain_support_surface_present_modes(
        &self,
    ) -> Result<Vec<PresentModeKHR>, ErrorCode> {
        self.query_swapchain_support_surface_present_modes(self.get_physical_device()?)
    }

    fn query_swapchain_support_surface_capabilities(
        &self,
        physical_device: &PhysicalDevice,
    ) -> Result<SurfaceCapabilitiesKHR, ErrorCode> {
        match unsafe {
            self.get_surface_loader()?
                .get_physical_device_surface_capabilities(*physical_device, *(self.get_surface()?))
        } {
            Ok(capabilities) => Ok(capabilities),
            Err(err) => {
                error!(
                    "Failed to get the physical device surface capabilities: {:?}",
                    err
                );
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    fn query_swapchain_support_surface_formats(
        &self,
        physical_device: &PhysicalDevice,
    ) -> Result<Vec<SurfaceFormatKHR>, ErrorCode> {
        match unsafe {
            self.get_surface_loader()?
                .get_physical_device_surface_formats(*physical_device, *(self.get_surface()?))
        } {
            Ok(formats) => Ok(formats),
            Err(err) => {
                error!("Failed to get the physical device formats: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    fn query_swapchain_support_surface_present_modes(
        &self,
        physical_device: &PhysicalDevice,
    ) -> Result<Vec<PresentModeKHR>, ErrorCode> {
        match unsafe {
            self.get_surface_loader()?
                .get_physical_device_surface_present_modes(*physical_device, *(self.get_surface()?))
        } {
            Ok(present_modes) => Ok(present_modes),
            Err(err) => {
                error!("Failed to get the physical device present modes: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    pub fn query_swapchain_support(
        &self,
        physical_device: &PhysicalDevice,
    ) -> Result<SwapchainSupportDetails, ErrorCode> {
        let surface_capabilities =
            self.query_swapchain_support_surface_capabilities(physical_device)?;
        let surface_formats = self.query_swapchain_support_surface_formats(physical_device)?;
        let surface_present_modes =
            self.query_swapchain_support_surface_present_modes(physical_device)?;
        Ok(SwapchainSupportDetails {
            capabilities: surface_capabilities,
            formats: surface_formats,
            present_modes: surface_present_modes,
        })
    }
}
