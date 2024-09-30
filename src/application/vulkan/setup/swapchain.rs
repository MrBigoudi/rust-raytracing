use ash::vk::{PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR};
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

#[derive(Default, Debug)]
pub(crate) struct SwapchainSupportDetails {
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
    pub(crate) fn query_swapchain_support(
        &self,
        physical_device: &PhysicalDevice,
    ) -> Result<SwapchainSupportDetails, ErrorCode> {
        let surface_capabilities = match unsafe {
            self.get_surface_loader()?
                .get_physical_device_surface_capabilities(*physical_device, *(self.get_surface()?))
        } {
            Ok(capabilities) => capabilities,
            Err(err) => {
                error!(
                    "Failed to get the physical device surface capabilities: {:?}",
                    err
                );
                return Err(ErrorCode::VulkanFailure);
            }
        };

        let surface_formats = match unsafe {
            self.get_surface_loader()?
                .get_physical_device_surface_formats(*physical_device, *(self.get_surface()?))
        } {
            Ok(formats) => formats,
            Err(err) => {
                error!("Failed to get the physical device formats: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };

        let surface_present_modes = match unsafe {
            self.get_surface_loader()?
                .get_physical_device_surface_present_modes(*physical_device, *(self.get_surface()?))
        } {
            Ok(present_modes) => present_modes,
            Err(err) => {
                error!("Failed to get the physical device present modes: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };

        Ok(SwapchainSupportDetails {
            capabilities: surface_capabilities,
            formats: surface_formats,
            present_modes: surface_present_modes,
        })
    }
}
