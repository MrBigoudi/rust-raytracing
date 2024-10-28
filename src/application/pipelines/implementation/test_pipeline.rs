use std::time::Instant;

use ash::vk::{
    DescriptorImageInfo, DescriptorSetLayoutCreateFlags, DescriptorType, ImageLayout, Pipeline,
    PipelineBindPoint, PipelineLayout, PushConstantRange, ShaderStageFlags, WriteDescriptorSet,
};
use log::error;

use crate::application::{
    core::error::ErrorCode,
    pipelines::{
        compute_pipeline::{ComputePipeline, PipelineAttributes},
        descriptor::Descriptor,
        push_constant::PushConstant,
    },
    scene::Scene,
    vulkan::{
        descriptors_helper::{
            allocator::DescriptorPoolSizeRatio, layout_builder::DescriptorLayoutBuilder,
        },
        types::VulkanContext,
    },
};

pub struct TestPipeline {
    pub base: PipelineAttributes,
    #[allow(dead_code)]
    pub push_constant: TestPushConstant,
    pub last_frame: Instant,
}

impl Default for TestPipeline {
    fn default() -> Self {
        Self {
            base: Default::default(),
            push_constant: Default::default(),
            last_frame: Instant::now(),
        }
    }
}

#[derive(Default)]
pub struct TestPushConstant {
    #[allow(dead_code)]
    pub data0: glam::Vec4,
    #[allow(dead_code)]
    pub data1: glam::Vec4,
    #[allow(dead_code)]
    pub data2: glam::Vec4,
    #[allow(dead_code)]
    pub data3: glam::Vec4,
}

impl TestPipeline {
    fn init_set_0(
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

    fn run(&mut self, vulkan_context: &VulkanContext, _scene: &Scene) -> Result<(), ErrorCode> {
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

        // Setup the push constants
        // TODO: Setup other push constants
        let now = Instant::now();
        let value = (self.last_frame.elapsed().as_nanos() as f32).sin();
        let push_constant = TestPushConstant {
            data0: glam::vec4(value, 1., 0.2, 0.6),
            data1: glam::vec4(0., 1., 0.4, 0.6),
            data2: glam::vec4(0., 2., 0.3, 0.6),
            data3: glam::vec4(0.4, 0.5, 0.6, 0.7),
        };
        self.last_frame = now;
        unsafe {
            device.cmd_push_constants(
                command_buffer,
                self.base.pipeline_layout,
                ShaderStageFlags::COMPUTE,
                0,
                PushConstant::data_to_u8_slice(&push_constant),
            );
        }

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

    fn init_push_constants(
        &mut self,
        _vulkan_context: &VulkanContext,
        _scene: &Scene,
    ) -> Result<(), ErrorCode> {
        let range = PushConstantRange::default()
            .offset(0)
            .size(size_of::<TestPushConstant>() as u32)
            .stage_flags(ShaderStageFlags::COMPUTE);
        let push_constant = PushConstant { range };

        self.base.push_constants = Some(push_constant);

        Ok(())
    }

    fn init_pool_size_ratios(
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

    fn clean(&mut self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        if let Err(err) = self.clean_descriptors(vulkan_context) {
            error!(
                "Failed to clean the vulkan descriptors in the test pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.clean_pipeline(vulkan_context) {
            error!(
                "Failed to clean the vulkan pipeline handler in the test pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        Ok(())
    }
}
