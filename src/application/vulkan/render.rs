use ash::vk::{
    ClearColorValue, CommandBufferBeginInfo, CommandBufferResetFlags, CommandBufferSubmitInfo,
    CommandBufferUsageFlags, Fence, ImageAspectFlags, ImageLayout, PipelineStageFlags2,
    PresentInfoKHR, SemaphoreSubmitInfo, SubmitInfo2,
};
use log::error;
use winit::window::Window;

use crate::application::core::error::ErrorCode;

use super::{
    descriptors::images::{get_default_image_subresource_range, transition_image},
    setup::frame_data::FRAME_OVERLAP,
    types::VulkanContext,
};

impl VulkanContext<'_> {
    fn reset_render_fence(&self, timeout_in_ns: u64) -> Result<(), ErrorCode> {
        let device = self.get_device()?;
        // Wait 1s for the GPU to have finished its work, and after it we reset the fence
        let render_fence = [self.get_current_frame()?.render_fence];
        unsafe {
            if let Err(err) = device.wait_for_fences(&render_fence, true, timeout_in_ns) {
                error!("Failed to wait for the render fence: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
            if let Err(err) = device.reset_fences(&render_fence) {
                error!("Failed to reset the render fence: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        }
        Ok(())
    }

    fn acquire_next_swapchain_image(&self, timeout_in_ns: u64) -> Result<(u32, bool), ErrorCode> {
        // Request the image index from the swapchain
        // If the swapchain doesn’t have any image, block the thread
        let swapchain_handler = self.get_swapchain_handler()?;
        let swapchain_semaphore = self.get_current_frame()?.swapchain_semaphore;
        match unsafe {
            swapchain_handler.device.acquire_next_image(
                swapchain_handler.handler,
                timeout_in_ns,
                swapchain_semaphore,
                Fence::null(),
            )
        } {
            Ok((index, flag)) => Ok((index, flag)),
            Err(err) => {
                error!("Failed to acquire the next swapchain image: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    fn prepare_clear_screen_command(&self, swapchain_next_index: usize) -> Result<(), ErrorCode> {
        // vkCmdClearColorImage requires 3 main parameters to work
        // First is the image, which is going to be the one from the swapchain
        let image = self.get_swapchain_handler()?.images[swapchain_next_index];
        // Then a clear color
        let flash = ((self.frame_index as f32) / 120.).sin().abs();
        // let flash = 1.;
        let clear_color = ClearColorValue {
            float32: [0., 0., flash, 1.],
        };
        // Finaly it needs a subresource range for what part of the image to clear
        // which we are going to use a default ImageSubresourceRange for
        let subresource_ranges =
            [get_default_image_subresource_range().aspect_mask(ImageAspectFlags::COLOR)];

        // Clear the image
        let main_command_buffer = self.get_current_frame()?.main_command_buffer;
        let device = self.get_device()?;
        unsafe {
            device.cmd_clear_color_image(
                main_command_buffer,
                image,
                ImageLayout::GENERAL,
                &clear_color,
                &subresource_ranges,
            )
        };
        Ok(())
    }

    fn prepare_rendering_commands(&self, swapchain_next_index: usize) -> Result<(), ErrorCode> {
        // Vulkan handles are just a 64 bit handle/pointer, so its fine to copy them around
        // But remember that their actual data is handled by vulkan itself
        let main_command_buffer = self.get_current_frame()?.main_command_buffer;

        // Now that we are sure that the commands finished executing, we can safely
        // reset the command buffer to begin recording again
        let device = self.get_device()?;
        if let Err(err) = unsafe {
            device.reset_command_buffer(main_command_buffer, CommandBufferResetFlags::empty())
        } {
            error!("Failed to reset the main command buffer: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }

        // The flag VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT is optional, but we might get a small speedup
        // if we tell the drivers that this buffer will only be submitted and executed once
        // We only do 1 submit per frame before the command buffer is reset, so this work for us
        let command_buffer_begin_info =
            CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        // Begin to record commands
        if let Err(err) =
            unsafe { device.begin_command_buffer(main_command_buffer, &command_buffer_begin_info) }
        {
            error!(
                "Failed to begin recording commands from the main command buffer: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        // We begin by transitioning the swapchain image
        let image = self.get_swapchain_handler()?.images[swapchain_next_index];
        // VK_IMAGE_LAYOUT_UNDEFINED Is the “dont care” layout
        // Its also the layout newly created images will be at
        // We use it when we dont care about the data that is already in the image, and we are fine with the GPU destroying it
        // The target layout we want is VK_IMAGE_LAYOUT_GENERAL
        // This is a general purpose layout, which allows reading and writing from the image
        // Its not the most optimal layout for rendering, but it is the one we want for vkCmdClearColorImage
        // This is the image layout you want to use if you want to write a image from a compute shader
        // If you want a read-only image or a image to be used with rasterization commands, there are better options
        transition_image(
            device,
            &main_command_buffer,
            &image,
            ImageLayout::UNDEFINED,
            ImageLayout::GENERAL,
        )?;

        self.prepare_clear_screen_command(swapchain_next_index)?;

        // Transition the image to VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
        // This is the only image layout that the swapchain allows for presenting to screen
        transition_image(
            device,
            &main_command_buffer,
            &image,
            ImageLayout::GENERAL,
            ImageLayout::PRESENT_SRC_KHR,
        )?;

        // And at the end, we finish by calling vkEndCommandBuffer
        // This finalizes the command buffer (we can no longer add commands, but it can now be executed)
        if let Err(err) = unsafe { device.end_command_buffer(main_command_buffer) } {
            error!("Failed to end the command buffer recording: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }

        Ok(())
    }

    fn submit_rendering_commands(&self) -> Result<(), ErrorCode> {
        let current_frame = self.get_current_frame()?;

        // Prepare the submission to the queue
        let main_command_buffer = current_frame.main_command_buffer;
        let command_buffer_submit_infos = [
            CommandBufferSubmitInfo::default()
                .command_buffer(main_command_buffer)
                .device_mask(0)
        ];

        // For the wait info, we are going to use the swapchain semaphore of the current frame.
        // When we called vkAcquireNextImageKHR, we set this same semaphore to be signaled,
        // so with this, we make sure that the commands executed here wont begin until the swapchain image is ready
        let swapchain_semaphore = current_frame.swapchain_semaphore;
        let wait_infos = [SemaphoreSubmitInfo::default()
            .semaphore(swapchain_semaphore)
            .stage_mask(PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_KHR)];

        // For signal info, we will be using the render semaphore of the current frame
        // which will lets us syncronize with presenting the image on the screen
        let render_semaphore = current_frame.render_semaphore;
        let signal_infos = [SemaphoreSubmitInfo::default()
            .semaphore(render_semaphore)
            .stage_mask(PipelineStageFlags2::ALL_GRAPHICS)];

        // And for the fence, we are going to use the current frame render fence
        // At the start of the draw loop, we waited for that same fence to be ready
        // This is how we are going to syncronize our gpu to the cpu,
        // as when the cpu goes ahead of the GPU, the fence will stop us so we dont use any of the other structures
        // from this frame until the draw commands are executed
        let render_fence = current_frame.render_fence;

        // Submit command buffer to the queue and execute it
        let submit_infos = [SubmitInfo2::default()
            .command_buffer_infos(&command_buffer_submit_infos)
            .signal_semaphore_infos(&signal_infos)
            .wait_semaphore_infos(&wait_infos)];
        // render fence will now block until the graphic commands finish execution
        let device = self.get_device()?;
        let graphics_queue = self.get_queues()?.graphics_queue.unwrap();
        if let Err(err) =
            unsafe { device.queue_submit2(graphics_queue, &submit_infos, render_fence) }
        {
            error!(
                "Failed to submit the rendering commands to the graphics queue: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }

        Ok(())
    }

    fn present_frame_to_screen(&self, swapchain_next_index: u32) -> Result<(), ErrorCode> {
        // We will wait on the render semaphore, and connect it to our swapchain
        // This way, we wont be presenting the image to the screen until it has finished the rendering commands
        //  from the submit right before it
        let render_semaphores = [self.get_current_frame()?.render_semaphore];

        let swapchain_handler = self.get_swapchain_handler()?;
        let swapchains = [swapchain_handler.handler];
        let image_indices = [swapchain_next_index];

        let present_info = PresentInfoKHR::default()
            .swapchains(&swapchains)
            .wait_semaphores(&render_semaphores)
            .image_indices(&image_indices);

        let graphics_queue = self.get_queues()?.graphics_queue.unwrap();
        if let Err(err) = unsafe {
            swapchain_handler
                .device
                .queue_present(graphics_queue, &present_info)
        } {
            error!("Failed to present the image into the screen: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }
        Ok(())
    }

    pub fn draw(&mut self, window: &Window) -> Result<(), ErrorCode> {
        let timeout_in_ns: u64 = 1_000_000_000;
        self.reset_render_fence(timeout_in_ns)?;
        let (swapchain_next_index, is_swapchain_suboptimal) =
            self.acquire_next_swapchain_image(timeout_in_ns)?;
        self.prepare_rendering_commands(swapchain_next_index as usize)?;
        self.submit_rendering_commands()?;
        self.present_frame_to_screen(swapchain_next_index)?;
        self.frame_index = self.frame_index + 1;
        Ok(())
    }
}
