use ash::vk::{
    DescriptorImageInfo, DescriptorSetLayoutCreateFlags, DescriptorType, ImageLayout, Pipeline,
    PipelineBindPoint, PipelineLayout, ShaderStageFlags, WriteDescriptorSet,
};
use log::error;

use crate::application::{
    core::error::ErrorCode,
    pipelines::{
        compute_pipeline::{ComputePipeline, PipelineAttributes},
        descriptor::Descriptor,
    },
    scene::Scene,
    vulkan::{
        descriptors_helper::{
            allocator::DescriptorPoolSizeRatio, layout_builder::DescriptorLayoutBuilder,
        },
        types::VulkanContext,
    },
};

#[derive(Default)]
pub struct TestPipeline {
    pub base: PipelineAttributes,
}

impl TestPipeline {
    pub fn init_pool_size_ratios(
        &mut self,
        vulkan_context: &VulkanContext,
    ) -> Result<(), ErrorCode> {
        let pool_size_ratios = [
            // Framebuffer
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_IMAGE,
                ratio: 1.0,
            },
            // TODO: add other things
        ];
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        self.base.descriptor_allocator.init_pool(
            device,
            allocation_callback,
            10,
            &pool_size_ratios,
        )?;
        Ok(())
    }

    pub fn init_set_0(
        &mut self,
        vulkan_context: &VulkanContext,
        _scene: &Scene,
    ) -> Result<Descriptor, ErrorCode> {
        // Organize the set layout
        let mut layout_builder = DescriptorLayoutBuilder::default();
        // Framebuffer
        layout_builder.add_binding(0, DescriptorType::STORAGE_IMAGE)?;
        // TODO: add other things

        // Build the layout
        let device = vulkan_context.get_device()?;
        let allocation_callback = vulkan_context.get_allocation_callback()?;
        let descriptor_set_layout = layout_builder.build(
            device,
            allocation_callback,
            ShaderStageFlags::COMPUTE,
            DescriptorSetLayoutCreateFlags::empty(),
        )?;

        // Allocate the set
        let descriptor_set = self
            .base
            .descriptor_allocator
            .allocate(device, descriptor_set_layout)?;

        // Send the data to the GPU
        // Framebuffer
        let descriptor_framebuffer_info = [DescriptorImageInfo::default()
            // Image data we want to bind (here the image we want to draw into)
            .image_view(vulkan_context.get_draw_image()?.image_view)
            .image_layout(ImageLayout::GENERAL)];
        // TODO: add other things

        // Updates to perform
        let writes_descriptor_set = [
            // Framebuffer
            WriteDescriptorSet::default()
                .dst_binding(0)
                .dst_set(descriptor_set)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_IMAGE)
                .image_info(&descriptor_framebuffer_info),
            // TODO: add other things
        ];

        unsafe { device.update_descriptor_sets(&writes_descriptor_set, &[]) };

        Ok(Descriptor {
            set: descriptor_set,
            set_layout: descriptor_set_layout,
        })
    }
}

impl ComputePipeline for TestPipeline {
    fn get_attributes(&self) -> Result<&PipelineAttributes, ErrorCode> {
        Ok(&self.base)
    }
    fn set_pipeline(&mut self, pipeline: Pipeline) {
        self.base.pipeline = pipeline;
    }
    fn set_pipeline_layout(&mut self, pipeline_layout: PipelineLayout) {
        self.base.pipeline_layout = pipeline_layout;
    }

    fn init_descriptors(
        &mut self,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<(), ErrorCode> {
        self.init_pool_size_ratios(vulkan_context)?;
        let set_0 = match self.init_set_0(vulkan_context, scene) {
            Ok(set) => set,
            Err(err) => {
                error!(
                    "Failed to initialize the descriptor set 0 in the test pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };
        self.base.descriptors.push(set_0);
        Ok(())
    }

    fn run(&self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        let device = vulkan_context.get_device()?;
        let command_buffer = vulkan_context.get_current_frame()?.main_command_buffer;

        // Bind the compute pipeline
        unsafe {
            device.cmd_bind_pipeline(
                command_buffer,
                PipelineBindPoint::COMPUTE,
                self.base.pipeline,
            )
        };

        // Bind the descriptor set containing the storage image
        unsafe {
            device.cmd_bind_descriptor_sets(
                command_buffer,
                PipelineBindPoint::COMPUTE,
                self.base.pipeline_layout,
                0,
                &[self.base.descriptors[0].set],
                &[],
            )
        };

        // Execute the compute pipeline dispatch
        // We are using 16x16 workgroup size so we need to divide the drawing size by 16
        unsafe {
            device.cmd_dispatch(
                command_buffer,
                vulkan_context.draw_extent.width / 16,
                vulkan_context.draw_extent.height / 16,
                1,
            )
        };
        Ok(())
    }
}
