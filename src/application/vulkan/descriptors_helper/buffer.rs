use std::sync::{Arc, Mutex};

use ash::vk::{Buffer, BufferCopy, BufferCreateInfo, BufferUsageFlags, DeviceSize};
use log::error;
use vk_mem::{
    Alloc, Allocation, AllocationCreateFlags, AllocationCreateInfo, Allocator, MemoryUsage,
};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub struct AllocatedBuffer {
    pub buffer: Buffer,
    pub allocation: Allocation,
}

impl AllocatedBuffer {
    pub fn from_usage(
        allocator: &Arc<Mutex<Allocator>>,
        size: DeviceSize,
        usage_flags: BufferUsageFlags,
        memory_usage: MemoryUsage,
        memory_flags: AllocationCreateFlags,
    ) -> Result<AllocatedBuffer, ErrorCode> {
        let buffer_create_info = BufferCreateInfo::default().usage(usage_flags).size(size);

        let allocation_info = AllocationCreateInfo {
            usage: memory_usage,
            flags: AllocationCreateFlags::MAPPED | memory_flags,
            ..Default::default()
        };

        let allocator = allocator.lock().unwrap();
        match unsafe { allocator.create_buffer(&buffer_create_info, &allocation_info) } {
            Ok((buffer, allocation)) => Ok(AllocatedBuffer { buffer, allocation }),
            Err(err) => {
                error!("Failed to allocate a buffer: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    pub fn from_info_struct(
        allocator: &Arc<Mutex<Allocator>>,
        buffer_create_info: &BufferCreateInfo,
        allocation_info: &AllocationCreateInfo,
    ) -> Result<AllocatedBuffer, ErrorCode> {
        let allocator = allocator.lock().unwrap();
        match unsafe { allocator.create_buffer(buffer_create_info, allocation_info) } {
            Ok((buffer, allocation)) => Ok(AllocatedBuffer { buffer, allocation }),
            Err(err) => {
                error!("Failed to allocate a buffer: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    pub fn clean(&mut self, allocator: &Arc<Mutex<Allocator>>) -> Result<(), ErrorCode> {
        let allocator = allocator.lock().unwrap();
        unsafe { allocator.destroy_buffer(self.buffer, &mut self.allocation) };
        Ok(())
    }
}

impl VulkanContext<'_> {
    fn copy_buffer_cpu<T>(
        &self,
        dst_buffer: &AllocatedBuffer,
        data: &[T],
        data_size: usize,
    ) -> Result<(), ErrorCode> {
        let allocator = &self.get_allocator()?.allocator;
        // Lock the allocator and map the staging buffer
        {
            let allocator = allocator.lock().unwrap();
            let mapped_data = allocator
                .get_allocation_info(&dst_buffer.allocation)
                .mapped_data as *mut u8;

            // Copy into buffer
            unsafe {
                let data_slice = std::slice::from_raw_parts_mut(mapped_data, data_size);
                let input_slice = std::slice::from_raw_parts(data.as_ptr() as *const u8, data_size);
                data_slice.copy_from_slice(input_slice);
            };
        } // The lock on the allocator is released here

        Ok(())
    }

    fn copy_buffer_gpu(
        &self,
        src_buffer: &AllocatedBuffer,
        dst_buffer: &AllocatedBuffer,
        data_size: DeviceSize,
    ) -> Result<(), ErrorCode> {
        // Run a GPU side command to perform the copy using the immediate submit
        if let Err(err) = self.immediate_submit(&|vulkan_context, cmd| {
            let elements_copy = [BufferCopy::default()
                .dst_offset(0)
                .src_offset(0)
                .size(data_size)];

            let device = vulkan_context.get_device()?;
            unsafe {
                device.cmd_copy_buffer(cmd, src_buffer.buffer, dst_buffer.buffer, &elements_copy);
            };
            Ok(())
        }) {
            error!(
                "Failed to send an immediate submit command when copying buffers: {:?}",
                err
            );
            return Err(ErrorCode::Unknown);
        }

        Ok(())
    }

    fn create_staging_buffer(&self, data_size: DeviceSize) -> Result<AllocatedBuffer, ErrorCode> {
        let allocator = &self.get_allocator()?.allocator;
        match AllocatedBuffer::from_usage(
            allocator,
            data_size,
            BufferUsageFlags::TRANSFER_SRC,
            MemoryUsage::AutoPreferHost,
            AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
        ) {
            Ok(staging_buffer) => Ok(staging_buffer),
            Err(err) => {
                error!("Failed to create a staging buffer: {:?}", err);
                Err(ErrorCode::Unknown)
            }
        }
    }

    fn create_data_buffer(
        &self,
        data_size: DeviceSize,
        buffer_type: BufferUsageFlags,
    ) -> Result<AllocatedBuffer, ErrorCode> {
        let allocator = &self.get_allocator()?.allocator;
        match AllocatedBuffer::from_usage(
            allocator,
            data_size,
            buffer_type | BufferUsageFlags::TRANSFER_DST,
            MemoryUsage::AutoPreferDevice,
            AllocationCreateFlags::HOST_ACCESS_RANDOM,
        ) {
            Ok(buffer) => Ok(buffer),
            Err(err) => {
                error!("Failed to create a data buffer: {:?}", err);
                Err(ErrorCode::Unknown)
            }
        }
    }

    pub fn map_data_to_buffer<T>(
        &self,
        data: &[T],
        buffer_type: BufferUsageFlags,
    ) -> Result<AllocatedBuffer, ErrorCode> {
        let data_size = std::mem::size_of_val(data) as DeviceSize;
        // As GPU_ONLY memory cant be written on CPU,
        // we first write the memory on a temporal staging buffer that is CPU writeable,
        // and then execute a copy command to copy this buffer into the GPU buffers
        let mut staging_buffer = match self.create_staging_buffer(data_size) {
            Ok(staging) => staging,
            Err(err) => {
                error!(
                    "Failed to create the staging buffer when mapping the data to a buffer: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };
        // Copy the data to the staging buffer (CPU side)
        if let Err(err) = self.copy_buffer_cpu(&staging_buffer, data, data_size as usize) {
            error!("Failed to copy the data to the staging buffer when mapping the data to a buffer: {:?}", err);
            return Err(ErrorCode::Unknown);
        }
        // Create the GPU side buffer
        let data_buffer = match self.create_data_buffer(data_size, buffer_type) {
            Ok(buffer) => buffer,
            Err(err) => {
                error!(
                    "Failed to create the gpu buffer when mapping the data to a buffer: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        };
        // Copy the data from the staging buffer (GPU side)
        if let Err(err) = self.copy_buffer_gpu(&staging_buffer, &data_buffer, data_size) {
            error!("Failed to copy the data to the buffer on the gpu side when mapping the data to a buffer: {:?}", err);
            return Err(ErrorCode::Unknown);
        }
        // Clean the staging buffer
        let allocator = &self.get_allocator()?.allocator;
        if let Err(err) = staging_buffer.clean(allocator) {
            error!("Failed to clean the staging buffer: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        }
        // Return the GPU side buffer
        Ok(data_buffer)
    }
}
