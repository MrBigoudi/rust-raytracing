use ash::vk::{CommandBuffer, CommandPool};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub const FRAME_OVERLAP: usize = 2;

pub struct VulkanFrameData {
    pub command_pool: CommandPool,
    pub main_command_buffer: CommandBuffer,
}

impl VulkanContext<'_> {
    pub fn get_current_frame(&self) -> Result<&VulkanFrameData, ErrorCode> {
        Ok(&self.frames[self.frame_index%FRAME_OVERLAP])
    }
}