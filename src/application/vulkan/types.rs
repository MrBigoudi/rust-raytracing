use std::mem::ManuallyDrop;

use ash::{
    ext::debug_utils,
    khr::surface,
    vk::{self, AllocationCallbacks, Extent2D, PhysicalDevice, SurfaceKHR},
    Device, Entry, Instance,
};

use crate::application::parameters::ApplicationParameters;

use super::setup::{
    allocator::AllocatorWrapper,
    devices::{device_requirements::DeviceRequirements, physical_device::PhysicalDeviceInfo},
    draw_resources::AllocatedImage,
    frame_data::VulkanFrameData,
    swapchains::swapchain::SwapchainHandler,
};

#[derive(Default)]
pub struct VulkanContext<'a> {
    pub parameters: ApplicationParameters,

    pub entry: Option<Entry>,
    pub allocation_callback: Option<&'a AllocationCallbacks<'a>>,
    pub instance: Option<Instance>,
    pub allocator: Option<ManuallyDrop<AllocatorWrapper>>,

    pub debug_utils_loader: Option<debug_utils::Instance>,
    pub debug_callback: Option<vk::DebugUtilsMessengerEXT>,

    pub surface_loader: Option<surface::Instance>,
    pub surface: Option<SurfaceKHR>,

    pub device_requirements: Option<DeviceRequirements<'a>>,
    pub physical_device_info: Option<PhysicalDeviceInfo>,
    pub physical_device: Option<PhysicalDevice>,
    pub device: Option<Device>,

    pub framebuffer_width: u32,
    pub framebuffer_height: u32,

    pub swapchain_handler: Option<SwapchainHandler>,

    pub frames: Vec<VulkanFrameData>,
    pub frame_index: usize,

    pub draw_image: Option<AllocatedImage>,
    pub draw_extent: Extent2D,
}
