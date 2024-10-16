use ash::{
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo},
    Device,
};
use log::error;

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl VulkanContext<'_> {
    fn get_device_queue_create_infos(&self) -> Result<Vec<DeviceQueueCreateInfo>, ErrorCode> {
        // NOTE: do not create additional queues for shared indices
        let present_shares_graphics_queue =
            self.get_queues()?.graphics_family_index == self.get_queues()?.present_family_index;
        let transfer_shares_graphics_queue =
            self.get_queues()?.graphics_family_index == self.get_queues()?.transfer_family_index;

        let mut queue_indices = vec![self.get_queues()?.graphics_family_index.unwrap()];
        if !present_shares_graphics_queue {
            queue_indices.push(self.get_queues()?.present_family_index.unwrap());
        }
        if !transfer_shares_graphics_queue {
            queue_indices.push(self.get_queues()?.transfer_family_index.unwrap());
        }

        let mut queue_create_infos: Vec<DeviceQueueCreateInfo> = Vec::new();
        for queue_index in queue_indices {
            let queue_create_info = DeviceQueueCreateInfo::default()
                .queue_family_index(queue_index as u32)
                // TODO: change the queue priorities
                .queue_priorities(&[1.]);
            queue_create_infos.push(queue_create_info);
        }

        Ok(queue_create_infos)
    }

    pub fn init_device(&mut self) -> Result<(), ErrorCode> {
        let queue_create_infos = match self.get_device_queue_create_infos() {
            Ok(infos) => infos,
            Err(err) => {
                error!("Failed to create the device queue infos: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };

        let requirements = self.get_device_requirements()?;
        let mut features_12 = requirements.features_12;
        let mut features_13 = requirements.features_13;

        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(queue_create_infos.as_slice())
            .enabled_extension_names(requirements.extensions.as_slice())
            .enabled_features(&requirements.features)
            .push_next(&mut features_12)
            .push_next(&mut features_13);

        unsafe {
            match self.get_instance()?.create_device(
                *self.get_physical_device()?,
                &device_create_info,
                self.get_allocation_callback()?,
            ) {
                Ok(device) => self.device = Some(device),
                Err(err) => {
                    error!("Failed to initialize the vulkan logical device: {:?}", err);
                    return Err(ErrorCode::VulkanFailure);
                }
            }
        }

        Ok(())
    }

    pub fn clean_device(&mut self) -> Result<(), ErrorCode> {
        if self.device.is_none() {
            return Ok(());
        }
        unsafe {
            self.get_device()?
                .destroy_device(self.get_allocation_callback()?);
        }
        self.device = None;
        Ok(())
    }

    pub fn get_device(&self) -> Result<&Device, ErrorCode> {
        match &self.device {
            Some(device) => Ok(device),
            None => {
                error!("Can't access the vulkan device");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn device_wait_idle(&self) -> Result<(), ErrorCode> {
        let device = self.get_device()?;
        unsafe {
            if let Err(err) = device.device_wait_idle() {
                error!("Failed to wait idle the vulkan device: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        }
        Ok(())
    }
}
