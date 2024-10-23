use std::{
    mem::ManuallyDrop,
    sync::{Arc, Mutex},
};

use log::error;
use vk_mem::{Allocator, AllocatorCreateInfo};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

pub struct AllocatorWrapper {
    pub allocator: std::sync::Arc<std::sync::Mutex<Allocator>>,
}

impl<'a> VulkanContext<'a> {
    pub fn init_allocation_callback(&mut self) -> Result<(), ErrorCode> {
        // TODO: build an allocator
        self.allocation_callback = None;
        Ok(())
    }

    pub fn clean_allocation_callback(&mut self) -> Result<(), ErrorCode> {
        self.allocation_callback = None;
        Ok(())
    }

    pub fn get_allocation_callback(
        &self,
    ) -> Result<Option<&'a ash::vk::AllocationCallbacks<'a>>, ErrorCode> {
        Ok(self.allocation_callback)
    }

    pub fn get_allocator(&self) -> Result<&ManuallyDrop<AllocatorWrapper>, ErrorCode>{
        match &self.allocator {
            Some(allocator) => Ok(allocator),
            None => {
                error!("Can't access the vulkan allocator");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn init_allocator(&mut self) -> Result<(), ErrorCode> {
        let device = self.get_device()?;
        let physical_device = self.get_physical_device()?;
        let instance = self.get_instance()?;
        let allocator_create_info = AllocatorCreateInfo::new(instance, device, *physical_device);

        let allocator = match unsafe { Allocator::new(allocator_create_info) } {
            Ok(allocator) => allocator,
            Err(err) => {
                error!("Failed to create the vulkan memory allocator: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };
        self.allocator = Some(ManuallyDrop::new(AllocatorWrapper {
            allocator: Arc::new(Mutex::new(allocator)),
        }));
        Ok(())
    }

    pub fn clean_allocator(&mut self) -> Result<(), ErrorCode> {
        if self.allocator.is_none() {
            return Ok(());
        }
        unsafe { ManuallyDrop::drop(self.allocator.as_mut().unwrap()) }
        self.allocator = None;
        Ok(())
    }
}
