use ash::vk::{CommandBuffer, CommandPool, Fence, Semaphore};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub const FRAME_OVERLAP: usize = 2;

pub struct VulkanFrameData {
    pub command_pool: CommandPool,
    pub main_command_buffer: CommandBuffer,

    /// Used so that our render commands wait on the swapchain image request
    pub swapchain_semaphore: Semaphore,
    /// Used to control presenting the image to the OS once the drawing finishes
    pub render_semaphore: Semaphore,
    /// Lets us wait for the draw commands of a given frame to be finished
    pub render_fence: Fence,
}

impl VulkanContext<'_> {
    pub fn get_current_frame(&self) -> Result<&VulkanFrameData, ErrorCode> {
        Ok(&self.frames[self.frame_index % FRAME_OVERLAP])
    }
}
