use std::ffi::CStr;

use ash::vk::{
    PhysicalDeviceFeatures, PhysicalDeviceVulkan12Features, PhysicalDeviceVulkan13Features,
};
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub struct DeviceRequirements<'a> {
    pub does_require_graphics_queue: bool,
    pub does_require_present_queue: bool,
    pub does_require_compute_queue: bool,
    pub does_require_transfer_queue: bool,
    pub is_discrete_gpu: bool,
    pub features: PhysicalDeviceFeatures,
    pub features_12: PhysicalDeviceVulkan12Features<'a>,
    pub features_13: PhysicalDeviceVulkan13Features<'a>,
    pub extensions: Vec<*const i8>,
}

impl Default for DeviceRequirements<'_> {
    fn default() -> Self {
        let required_features = PhysicalDeviceFeatures::default()
            .sampler_anisotropy(true)
            .shader_clip_distance(true);
        let features_12 = PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true)
            .descriptor_indexing(true);
        let features_13 = PhysicalDeviceVulkan13Features::default()
            .synchronization2(true)
            .dynamic_rendering(true);

        let required_extensions =
            vec![unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0").as_ptr() }];

        Self {
            does_require_graphics_queue: true,
            does_require_present_queue: true,
            does_require_compute_queue: true,
            does_require_transfer_queue: true,
            is_discrete_gpu: false,
            features: required_features,
            features_12,
            features_13,
            extensions: required_extensions,
        }
    }
}

impl VulkanContext<'_> {
    pub fn init_device_requirements(&mut self) -> Result<(), ErrorCode> {
        // TODO: make the device requirements configurable
        self.device_requirements = Some(DeviceRequirements::default());
        Ok(())
    }

    pub fn clean_device_requirements(&mut self) -> Result<(), ErrorCode> {
        self.device_requirements = None;
        Ok(())
    }

    pub fn get_device_requirements(&self) -> Result<&DeviceRequirements, ErrorCode> {
        match &self.device_requirements {
            Some(requirements) => Ok(requirements),
            None => {
                error!("Can't access the vulkan device requirements");
                Err(ErrorCode::AccessFailure)
            }
        }
    }
}
