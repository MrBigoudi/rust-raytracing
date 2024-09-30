use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl<'a> VulkanContext<'a> {
    pub fn init_allocator(&mut self) -> Result<(), ErrorCode> {
        // TODO: build an allocator
        self.allocator = None;
        Ok(())
    }

    pub fn clean_allocator(&mut self) -> Result<(), ErrorCode> {
        self.allocator = None;
        Ok(())
    }

    pub fn get_allocator(&self) -> Result<Option<&'a ash::vk::AllocationCallbacks<'a>>, ErrorCode> {
        Ok(self.allocator)
    }
}
