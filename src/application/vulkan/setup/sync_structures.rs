use ash::{vk::{AllocationCallbacks, Fence, FenceCreateInfo, Semaphore, SemaphoreCreateInfo}, Device};
use log::error;

use crate::application::core::error::ErrorCode;

pub struct SyncStructures;

impl SyncStructures {
    pub fn init_semaphore(info: &SemaphoreCreateInfo, device: &Device, allocation_callback: Option<&AllocationCallbacks>) -> Result<Semaphore, ErrorCode> {
        match unsafe { device.create_semaphore(info, allocation_callback) } {
            Ok(semaphore) => Ok(semaphore),
            Err(err) => {
                error!("Failed to create a semaphore: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }

    pub fn init_fence(info: &FenceCreateInfo, device: &Device, allocation_callback: Option<&AllocationCallbacks>) -> Result<Fence, ErrorCode> {
        match unsafe { device.create_fence(info, allocation_callback) } {
            Ok(fence) => Ok(fence),
            Err(err) => {
                error!("Failed to create a fence: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }
}