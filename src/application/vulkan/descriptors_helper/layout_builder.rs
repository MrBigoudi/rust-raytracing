use ash::{
    vk::{
        AllocationCallbacks, DescriptorSetLayout, DescriptorSetLayoutBinding,
        DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo, DescriptorType,
        ShaderStageFlags,
    },
    Device,
};
use log::error;

use crate::application::core::error::ErrorCode;

#[derive(Default)]
pub struct DescriptorLayoutBuilder<'a> {
    pub bindings: Vec<DescriptorSetLayoutBinding<'a>>,
}

impl DescriptorLayoutBuilder<'_> {
    pub fn add_binding(
        &mut self,
        binding: u32,
        descriptor_type: DescriptorType,
    ) -> Result<(), ErrorCode> {
        let new_binding = DescriptorSetLayoutBinding::default()
            .binding(binding)
            .descriptor_type(descriptor_type)
            .descriptor_count(1);
        self.bindings.push(new_binding);
        Ok(())
    }

    #[allow(unused)]
    pub fn clear(&mut self) -> Result<(), ErrorCode> {
        self.bindings.clear();
        Ok(())
    }

    pub fn build(
        &mut self,
        device: &Device,
        allocation_callback: Option<&AllocationCallbacks>,
        shader_stage: ShaderStageFlags,
        flags: DescriptorSetLayoutCreateFlags,
    ) -> Result<DescriptorSetLayout, ErrorCode> {
        // Do not support per-binding stage flags
        for binding in &mut self.bindings {
            binding.stage_flags = shader_stage;
        }

        let descriptor_set_layout_info = DescriptorSetLayoutCreateInfo::default()
            .bindings(self.bindings.as_slice())
            .flags(flags);

        match unsafe {
            device.create_descriptor_set_layout(&descriptor_set_layout_info, allocation_callback)
        } {
            Ok(descriptor_set_layout) => Ok(descriptor_set_layout),
            Err(err) => {
                error!("Failed to create a descriptor set layout: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }
}
