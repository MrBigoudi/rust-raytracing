use ash::Entry;
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl VulkanContext<'_> {
    /// Loads the Vulkan library entry points
    pub fn init_entry(&mut self) -> Result<(), ErrorCode> {
        match unsafe { Entry::load() } {
            Ok(entry) => {
                self.entry = Some(entry);
                Ok(())
            }
            Err(err) => {
                error!("Failed to init the vulkan entry: {:?}", err);
                Err(ErrorCode::InitializationFailure)
            }
        }
    }

    /// Direct getter to the entry
    pub fn get_entry(&self) -> Result<&Entry, ErrorCode> {
        match &self.entry {
            Some(entry) => Ok(entry),
            None => {
                error!("Can't access the vulkan entry");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    /// Clean the entry
    pub fn clean_entry(&mut self) -> Result<(), ErrorCode> {
        self.entry = None;
        Ok(())
    }
}
