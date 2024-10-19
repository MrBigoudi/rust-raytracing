use ash::{
    vk::{
        AllocationCallbacks, DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolResetFlags,
        DescriptorPoolSize, DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout,
        DescriptorType,
    },
    Device,
};
use log::error;

use crate::application::core::error::ErrorCode;

pub struct DescriptorPoolSizeRatio {
    pub descriptor_type: DescriptorType,
    pub ratio: f32,
}

#[derive(Default)]
pub struct DescriptorAllocator {
    pub pool: DescriptorPool,
}

impl DescriptorAllocator {
    pub fn init_pool(
        &mut self,
        device: &Device,
        allocation_callback: Option<&AllocationCallbacks>,
        max_sets: u32,
        pool_ratios: &[DescriptorPoolSizeRatio],
    ) -> Result<(), ErrorCode> {
        // Structure that contains a type of descriptor and
        // a ratio to multiply the maxSets parameter
        // This lets us directly control how big the pool is going to be
        // maxSets controls how many VkDescriptorSets we can create from the pool in total
        // and the pool sizes give how many individual bindings of a given type are owned
        let mut pool_sizes: Vec<DescriptorPoolSize> = Vec::new();
        for pool_ratio in pool_ratios {
            let new_pool_size = DescriptorPoolSize::default()
                .descriptor_count((pool_ratio.ratio * (max_sets as f32)) as u32)
                .ty(pool_ratio.descriptor_type);
            pool_sizes.push(new_pool_size);
        }
        let descriptor_pool_create_info = DescriptorPoolCreateInfo::default()
            .max_sets(max_sets)
            .pool_sizes(pool_sizes.as_slice());
        match unsafe {
            device.create_descriptor_pool(&descriptor_pool_create_info, allocation_callback)
        } {
            Ok(pool) => self.pool = pool,
            Err(err) => {
                error!("Failed to create a descriptor pool: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        }
        Ok(())
    }

    pub fn reset_pool(&mut self, device: &Device) -> Result<(), ErrorCode> {
        if let Err(err) =
            unsafe { device.reset_descriptor_pool(self.pool, DescriptorPoolResetFlags::empty()) }
        {
            error!("Failed to reset a descriptor pool: {:?}", err);
            return Err(ErrorCode::VulkanFailure);
        }
        Ok(())
    }

    pub fn destroy_pool(
        &self,
        device: &Device,
        allocation_callback: Option<&AllocationCallbacks>,
    ) -> Result<(), ErrorCode> {
        unsafe { device.destroy_descriptor_pool(self.pool, allocation_callback) }
        Ok(())
    }

    pub fn allocate(
        &self,
        device: &Device,
        layout: DescriptorSetLayout,
    ) -> Result<DescriptorSet, ErrorCode> {
        let layouts = [layout];
        let allocate_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(&layouts);

        match unsafe { device.allocate_descriptor_sets(&allocate_info) } {
            Ok(descriptor_sets) => Ok(descriptor_sets[0]),
            Err(err) => {
                error!("Failed to allocate a descriptor set: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }
}
