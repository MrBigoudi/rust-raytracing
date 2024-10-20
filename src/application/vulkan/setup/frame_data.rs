use ash::{
    vk::{
        AllocationCallbacks, CommandBuffer, CommandPool, Fence, FenceCreateFlags, FenceCreateInfo,
        Semaphore, SemaphoreCreateInfo,
    },
    Device,
};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

use super::sync_structures::SyncStructures;

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

impl VulkanFrameData {
    pub fn clean_sync_structures(
        &self,
        device: &Device,
        allocation_callback: Option<&AllocationCallbacks>,
    ) -> Result<(), ErrorCode> {
        unsafe { device.destroy_fence(self.render_fence, allocation_callback) };
        unsafe { device.destroy_semaphore(self.swapchain_semaphore, allocation_callback) };
        unsafe { device.destroy_semaphore(self.render_semaphore, allocation_callback) };
        Ok(())
    }
}

impl VulkanContext<'_> {
    pub fn get_current_frame(&self) -> Result<&VulkanFrameData, ErrorCode> {
        Ok(&self.frames[self.frame_index % FRAME_OVERLAP])
    }

    pub fn clean_frames_sync_structures(&self) -> Result<(), ErrorCode> {
        let device = self.get_device()?;
        let allocation_callback = self.get_allocation_callback()?;
        for frame in &self.frames {
            frame.clean_sync_structures(device, allocation_callback)?;
        }
        Ok(())
    }

    pub fn init_frames_sync_structures(&mut self) -> Result<(), ErrorCode> {
        let mut render_fences = Vec::new();
        let mut swapchain_semaphores = Vec::new();
        let mut render_semaphores = Vec::new();

        let device = self.get_device()?;
        let allocation_callback = self.get_allocation_callback()?;

        for _ in 0..FRAME_OVERLAP {
            let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
            let render_fence =
                SyncStructures::init_fence(&fence_create_info, device, allocation_callback)?;
            let semaphore_create_info = SemaphoreCreateInfo::default();
            let swapchain_semaphore = SyncStructures::init_semaphore(
                &semaphore_create_info,
                device,
                allocation_callback,
            )?;
            let render_semaphore = SyncStructures::init_semaphore(
                &semaphore_create_info,
                device,
                allocation_callback,
            )?;

            render_fences.push(render_fence);
            swapchain_semaphores.push(swapchain_semaphore);
            render_semaphores.push(render_semaphore);
        }

        for i in 0..FRAME_OVERLAP {
            self.frames[i].render_fence = render_fences[i];
            self.frames[i].render_semaphore = render_semaphores[i];
            self.frames[i].swapchain_semaphore = swapchain_semaphores[i];
        }

        Ok(())
    }
}
