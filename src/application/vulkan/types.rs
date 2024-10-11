use ash::{
    ext::debug_utils,
    khr::surface,
    vk::{self, AllocationCallbacks, PhysicalDevice, SurfaceKHR},
    Device, Entry, Instance,
};

use super::setup::{
    devices::{device_requirements::DeviceRequirements, physical_device::PhysicalDeviceInfo},
    frame_data::VulkanFrameData,
    swapchains::swapchain::SwapchainHandler,
};

#[derive(Default)]
pub struct VulkanContext<'a> {
    pub entry: Option<Entry>,
    pub allocator: Option<&'a AllocationCallbacks<'a>>,
    pub instance: Option<Instance>,

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
}
