use std::ffi::CString;

use ash::vk::{
    ComputePipelineCreateInfo, DescriptorSetLayout, Pipeline, PipelineCache, PipelineLayout,
    PipelineLayoutCreateInfo, PipelineShaderStageCreateInfo, ShaderStageFlags,
};
use log::error;

use crate::application::{
    core::error::ErrorCode,
    scene::Scene,
    vulkan::{descriptors_helper::allocator::DescriptorAllocator, types::VulkanContext},
};

use super::{descriptor::Descriptor, shader::Shader};

#[derive(Default)]
pub struct PipelineAttributes {
    pub descriptor_allocator: DescriptorAllocator,
    pub descriptors: Vec<Descriptor>,
    pub pipeline: Pipeline,
    pub pipeline_layout: PipelineLayout,
    // TODO: handle push constants
}

pub trait ComputePipeline {
    fn get_attributes(&self) -> Result<&PipelineAttributes, ErrorCode>;
    fn init_descriptors(
        &mut self,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<(), ErrorCode>;
    fn run(&self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode>;

    fn set_pipeline(&mut self, pipeline: Pipeline);
    fn set_pipeline_layout(&mut self, pipeline_layout: PipelineLayout);

    fn init_pipeline_layout(&mut self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        let set_layouts = self
            .get_attributes()?
            .descriptors
            .iter()
            .map(|descriptor| descriptor.set_layout)
            .collect::<Vec<DescriptorSetLayout>>();

        // TODO: handle push constants
        let push_constant_ranges = [];

        let create_info = PipelineLayoutCreateInfo::default()
            .set_layouts(&set_layouts)
            .push_constant_ranges(&push_constant_ranges);

        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        let pipeline_layout =
            match unsafe { device.create_pipeline_layout(&create_info, allocation_callback) } {
                Ok(layout) => layout,
                Err(err) => {
                    error!(
                        "Failed to create a pipeline layout for the test pipeline: {:?}",
                        err
                    );
                    return Err(ErrorCode::VulkanFailure);
                }
            };
        self.set_pipeline_layout(pipeline_layout);
        Ok(())
    }

    fn init_pipeline(
        &mut self,
        shader_name: &str,
        entry_point: &str,
        vulkan_context: &VulkanContext,
    ) -> Result<(), ErrorCode> {
        let device = vulkan_context.get_device()?;
        let shader_module =
            match Shader::load_shader_module(Shader::get_compiled_shader_path(shader_name), device)
            {
                Ok(module) => module,
                Err(err) => {
                    error!(
                        "Failed to load the vulkan shader module `{:?}': {:?}",
                        shader_name, err
                    );
                    return Err(ErrorCode::VulkanFailure);
                }
            };
        let name = match CString::new(entry_point) {
            Ok(name) => name,
            Err(err) => {
                error!("Failed to create a cstr from the given entry point `{:?}' in the shader `{:?}': {:?}", entry_point, shader_name, err);
                return Err(ErrorCode::Unknown);
            }
        };
        let shader_stage_create_info = PipelineShaderStageCreateInfo::default()
            .stage(ShaderStageFlags::COMPUTE)
            .module(shader_module)
            .name(name.as_c_str());

        let compute_pipeline_create_info = [ComputePipelineCreateInfo::default()
            .layout(self.get_attributes()?.pipeline_layout)
            .stage(shader_stage_create_info)];

        let allocation_callback = vulkan_context.get_allocation_callback()?;
        let pipeline = match unsafe {
            device.create_compute_pipelines(
                PipelineCache::null(),
                &compute_pipeline_create_info,
                allocation_callback,
            )
        } {
            Ok(pipelines) => pipelines[0],
            Err(err) => {
                error!(
                    "Failed to create a compute pipeline for the shader `{:?}': {:?}",
                    shader_name, err
                );
                return Err(ErrorCode::VulkanFailure);
            }
        };

        unsafe { device.destroy_shader_module(shader_module, allocation_callback) };
        self.set_pipeline(pipeline);
        Ok(())
    }

    fn init(
        &mut self,
        shader_name: &str,
        entry_point: &str,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<(), ErrorCode> {
        if let Err(err) = self.init_descriptors(vulkan_context, scene) {
            error!(
                "Failed to initialize the vulkan descriptors in a compute pipeline: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        }
        if let Err(err) = self.init_pipeline_layout(vulkan_context) {
            error!(
                "Failed to initialize the vulkan pipeline layout in a compute pipeline: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        }
        if let Err(err) = self.init_pipeline(shader_name, entry_point, vulkan_context) {
            error!(
                "Failed to initialize the vulkan pipeline handler in a compute pipeline: {:?}",
                err
            );
            return Err(ErrorCode::InitializationFailure);
        }

        Ok(())
    }

    fn clean_descriptors(&self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        let attributes = self.get_attributes()?;
        for descriptor in &attributes.descriptors {
            unsafe {
                device.destroy_descriptor_set_layout(descriptor.set_layout, allocation_callback)
            };
        }
        attributes
            .descriptor_allocator
            .destroy_pool(device, allocation_callback)?;
        Ok(())
    }

    fn clean_pipeline(&self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        let attributes = self.get_attributes()?;
        unsafe {
            device.destroy_pipeline_layout(attributes.pipeline_layout, allocation_callback);
            device.destroy_pipeline(attributes.pipeline, allocation_callback);
        }
        Ok(())
    }

    fn clean(&self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        if let Err(err) = self.clean_descriptors(vulkan_context) {
            error!(
                "Failed to clean the vulkan descriptors in a compute pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.clean_pipeline(vulkan_context) {
            error!(
                "Failed to clean the vulkan pipeline handler in a compute pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        Ok(())
    }
}
