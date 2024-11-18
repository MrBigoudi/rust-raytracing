use ash::vk::{
    CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferResetFlags,
    CommandBufferSubmitInfo, CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags,
    CommandPoolCreateInfo, Fence, FenceCreateFlags, FenceCreateInfo, SubmitInfo2,
};
use log::error;

use crate::application::core::error::ErrorCode;

use super::types::VulkanContext;

/// Uses a fence and a different command buffer from the one we use on draws
/// to send some commands to the GPU without syncronizing with swapchain or with rendering logic
#[derive(Default)]
pub struct Immediate {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub fence: Fence,
}

impl Immediate {
    fn init_command_pool(vulkan_context: &VulkanContext<'_>) -> Result<CommandPool, ErrorCode> {
        let graphics_queue_index =
            vulkan_context.get_queues()?.graphics_family_index.unwrap() as u32;
        let command_pool_info = CommandPoolCreateInfo::default()
            .queue_family_index(graphics_queue_index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        match unsafe { device.create_command_pool(&command_pool_info, allocation_callback) } {
            Ok(pool) => Ok(pool),
            Err(err) => {
                error!(
                    "Failed to create the command pool for an immediate structure: {:?}",
                    err
                );
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    fn init_command_buffer(
        vulkan_context: &VulkanContext<'_>,
        command_pool: &CommandPool,
    ) -> Result<CommandBuffer, ErrorCode> {
        let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
            .command_buffer_count(1)
            .command_pool(*command_pool);
        let device = vulkan_context.get_device()?;
        match unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) } {
            Ok(buffer) => Ok(buffer[0]),
            Err(err) => {
                error!(
                    "Failed to allocate the command buffer for an immediate structure: {:?}",
                    err
                );
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    fn init_fence(vulkan_context: &VulkanContext<'_>) -> Result<Fence, ErrorCode> {
        let fence_create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        match unsafe { device.create_fence(&fence_create_info, allocation_callback) } {
            Ok(fence) => Ok(fence),
            Err(err) => {
                error!(
                    "Failed to create the fence for an immediate structure: {:?}",
                    err
                );
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    pub fn init(vulkan_context: &VulkanContext<'_>) -> Result<Immediate, ErrorCode> {
        let command_pool = Immediate::init_command_pool(vulkan_context)?;
        let command_buffer = Immediate::init_command_buffer(vulkan_context, &command_pool)?;
        let fence = Immediate::init_fence(vulkan_context)?;

        Ok(Immediate {
            command_pool,
            command_buffer,
            fence,
        })
    }

    pub fn clean(&self, vulkan_context: &VulkanContext<'_>) -> Result<(), ErrorCode> {
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        unsafe {
            device.destroy_command_pool(self.command_pool, allocation_callback);
            device.destroy_fence(self.fence, allocation_callback);
        }
        Ok(())
    }
}

impl VulkanContext<'_> {
    pub fn init_immediate(&mut self) -> Result<(), ErrorCode> {
        match Immediate::init(self) {
            Ok(immediate) => self.immediate_submit = immediate,
            Err(err) => {
                error!(
                    "Failed to initialize the immediate submit structure: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        }
        Ok(())
    }

    pub fn clean_immediate(&self) -> Result<(), ErrorCode> {
        if let Err(err) = self.immediate_submit.clean(self) {
            error!("Failed to clean the immediate submit structure: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        }
        Ok(())
    }

    pub fn immediate_submit(
        &self,
        fct: &dyn Fn(&VulkanContext, CommandBuffer) -> Result<(), ErrorCode>,
    ) -> Result<(), ErrorCode> {
        let device = self.get_device()?;
        let immediate = &self.immediate_submit;
        if let Err(err) = unsafe { device.reset_fences(&[immediate.fence]) } {
            error!(
                "Failed to reset the fence when submitting an immediate structure: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }
        if let Err(err) = unsafe {
            device.reset_command_buffer(immediate.command_buffer, CommandBufferResetFlags::empty())
        } {
            error!(
                "Failed to reset the command buffer when submitting an immediate structure: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        let command_buffer_begin_info =
            CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        if let Err(err) = unsafe {
            device.begin_command_buffer(immediate.command_buffer, &command_buffer_begin_info)
        } {
            error!(
                "Failed to begin the command buffer when submitting an immediate structure: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        if let Err(err) = fct(self, immediate.command_buffer) {
            error!("Failed to run the custom submit function when submitting an immediate structure: {:?}", err);
            return Err(ErrorCode::Unknown);
        }

        if let Err(err) = unsafe { device.end_command_buffer(immediate.command_buffer) } {
            error!(
                "Failed to end the command buffer when submitting an immediate structure: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        let command_buffer_submit_info =
            [CommandBufferSubmitInfo::default().command_buffer(immediate.command_buffer)];
        let submit_info =
            [SubmitInfo2::default().command_buffer_infos(&command_buffer_submit_info)];

        // Submit command buffer to the queue and execute it
        // the render fence will now block until the graphics commands finish execution
        let graphics_queue = self.get_queues()?.graphics_queue.unwrap();
        if let Err(err) =
            unsafe { device.queue_submit2(graphics_queue, &submit_info, immediate.fence) }
        {
            error!(
                "Failed to submit the command buffer when submitting an immediate structure: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        let timeout = 10_000_000_000_u64; // 10 secs
        let should_wait_all = true;
        if let Err(err) =
            unsafe { device.wait_for_fences(&[immediate.fence], should_wait_all, timeout) }
        {
            error!(
                "Failed to wait for fences when submitting an immediate structure: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        Ok(())
    }
}
