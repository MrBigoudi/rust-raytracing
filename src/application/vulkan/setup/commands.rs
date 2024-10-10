use ash::vk::{
    CommandBufferAllocateInfo, CommandBufferLevel, CommandPoolCreateFlags, CommandPoolCreateInfo,
    Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateInfo,
};
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

use super::frame_data::{VulkanFrameData, FRAME_OVERLAP};

impl VulkanContext<'_> {
    fn init_semaphore(&mut self, info: &SemaphoreCreateInfo) -> Result<Semaphore, ErrorCode> {
        let device = self.get_device()?;
        let allocator = self.get_allocator()?;
        match unsafe { device.create_semaphore(info, allocator) } {
            Ok(semaphore) => Ok(semaphore),
            Err(err) => {
                error!("Failed to create a semaphore: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    fn init_fence(&mut self, info: &FenceCreateInfo) -> Result<Fence, ErrorCode> {
        let device = self.get_device()?;
        let allocator = self.get_allocator()?;
        match unsafe { device.create_fence(info, allocator) } {
            Ok(fence) => Ok(fence),
            Err(err) => {
                error!("Failed to create a fence: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    pub fn init_commands(&mut self) -> Result<(), ErrorCode> {
        // Create a command pool for commands submitted to the graphics queue
        let graphics_commands_pool_info = CommandPoolCreateInfo::default()
            .queue_family_index(self.get_queues()?.graphics_family_index.unwrap() as u32)
            // Allow command buffers to be rerecorded individually, without this flag they all have to be reset together
            // We will be recording a command buffer every frame, so we want to be able to reset and rerecord over it
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        for _ in 0..FRAME_OVERLAP {
            let device = self.get_device()?;
            let command_pool = unsafe {
                match device
                    .create_command_pool(&graphics_commands_pool_info, self.get_allocator()?)
                {
                    Ok(pool) => pool,
                    Err(err) => {
                        error!("Failed to create a command pool: {:?}", err);
                        return Err(ErrorCode::VulkanFailure);
                    }
                }
            };
            let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
                .command_pool(command_pool)
                // Can be submitted to a queue for execution, but cannot be called from other command buffers
                .level(CommandBufferLevel::PRIMARY)
                // Since we are only allocating one command buffer, the commandBufferCount parameter is just 1
                .command_buffer_count(1);
            let main_command_buffer =
                match unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) } {
                    Ok(command_buffers) => command_buffers[0],
                    Err(err) => {
                        error!("Failed to allocate a command buffer: {:?}", err);
                        return Err(ErrorCode::VulkanFailure);
                    }
                };

            // Fence starts signalled so we can wait on it on the first frame
            let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
            let render_fence = self.init_fence(&fence_create_info)?;
            let semaphore_create_info = SemaphoreCreateInfo::default();
            let swapchain_semaphore = self.init_semaphore(&semaphore_create_info)?;
            let render_semaphore = self.init_semaphore(&semaphore_create_info)?;

            let new_frame = VulkanFrameData {
                command_pool,
                main_command_buffer,
                swapchain_semaphore,
                render_semaphore,
                render_fence,
            };
            self.frames.push(new_frame);
        }
        if self.frames.len() != FRAME_OVERLAP {
            error!(
                "The number of frames is not correct: expecting {:?} frames, got {:?}",
                FRAME_OVERLAP,
                self.frames.len()
            );
            return Err(ErrorCode::InitializationFailure);
        }

        Ok(())
    }

    pub fn clean_commands(&mut self) -> Result<(), ErrorCode> {
        for frame in &self.frames {
            let device = self.get_device()?;
            let allocator = self.get_allocator()?;
            // Destroy command pools
            unsafe { device.destroy_command_pool(frame.command_pool, allocator) };
            // Destroy sync structures
            unsafe { device.destroy_fence(frame.render_fence, allocator) };
            unsafe { device.destroy_semaphore(frame.swapchain_semaphore, allocator) };
            unsafe { device.destroy_semaphore(frame.render_semaphore, allocator) };
        }
        Ok(())
    }
}
