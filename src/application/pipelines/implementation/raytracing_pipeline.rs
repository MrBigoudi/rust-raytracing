use ash::vk::{
    BufferUsageFlags, DescriptorBufferInfo, DescriptorImageInfo, DescriptorSet,
    DescriptorSetLayoutCreateFlags, DescriptorType, ImageLayout, Pipeline, PipelineBindPoint,
    PipelineLayout, PushConstantRange, ShaderStageFlags, WriteDescriptorSet, WHOLE_SIZE,
};
use log::{error, info};

use crate::application::{
    core::error::ErrorCode,
    pipelines::{
        compute_pipeline::{ComputePipeline, PipelineAttributes},
        descriptor::Descriptor,
        push_constant::PushConstant,
    },
    scene::{bvh::BvhType, Scene},
    vulkan::{
        descriptors_helper::{
            allocator::DescriptorPoolSizeRatio, buffer::AllocatedBuffer,
            layout_builder::DescriptorLayoutBuilder,
        },
        types::VulkanContext,
    },
};

pub struct RaytracingPipeline {
    pub base: PipelineAttributes,
    pub buffers: RaytracingBuffers,
}

pub struct RaytracingBuffers {
    pub triangles_ssbo: AllocatedBuffer,
    pub models_ssbo: AllocatedBuffer,
    pub materials_ssbo: AllocatedBuffer,
    pub bvhs_ssbo: Option<AllocatedBuffer>,
    pub camera_ubo: AllocatedBuffer,
}

#[derive(Default)]
#[repr(C)]
pub struct RaytracingPushConstant {
    pub nb_triangles: u32,
    pub is_wireframe_on: u32,
    pub bvh_type: u32,
}

impl RaytracingPipeline {
    pub fn update_camera_buffer(
        &mut self,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<(), ErrorCode> {
        let dst_offset = 0;
        let data = [scene.camera.get_gpu_data()];
        if let Err(err) = vulkan_context.update_buffer(&self.buffers.camera_ubo, &data, dst_offset)
        {
            error!(
                "Failed to update the camera ubo in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::VulkanFailure);
        }
        Ok(())
    }

    pub fn update_bvhs_buffer(
        &mut self,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<(), ErrorCode> {
        // Clean old buffer
        vulkan_context.device_wait_idle()?;
        let allocator = &vulkan_context.get_allocator()?.allocator;
        if let Some(bvh_ssbo) = self.buffers.bvhs_ssbo.as_mut() {
            if let Err(err) = bvh_ssbo.clean(allocator) {
                error!(
                    "Failed to clean the bvhs buffer in the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::CleaningFailure);
            }
        };

        // Create new buffer
        self.buffers.bvhs_ssbo = if scene.bvh_type == BvhType::None {
            None
        } else {
            let data = match scene.get_bvh() {
                Ok(bvh) => {
                    if bvh.is_empty() {
                        error!("Can't get the bvh from the scene in the raytracing pipeline, BVH tree is empty");
                        return Err(ErrorCode::Unknown);
                    } else {
                        bvh.as_slice()
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to get the bvh from the scene in the raytracing pipeline: {:?}",
                        err
                    );
                    return Err(ErrorCode::Unknown);
                }
            };
            match vulkan_context.map_data_to_buffer(data, BufferUsageFlags::STORAGE_BUFFER) {
                Ok(buffer) => Some(buffer),
                Err(err) => {
                    error!(
                        "Failed to create the bvhs ssbo for the raytracing pipeline: {:?}",
                        err
                    );
                    return Err(ErrorCode::InitializationFailure);
                }
            }
        };

        // Create new set
        if scene.bvh_type != BvhType::None {
            let descriptor_bvhs_info = [DescriptorBufferInfo::default()
                .buffer(self.buffers.bvhs_ssbo.as_ref().unwrap().buffer)
                .range(WHOLE_SIZE)
                .offset(0)];

            // Get the corresponding descriptor set (bvhs are on set 1)
            let set = 1;
            let descriptor_set = self.base.descriptors[set].set;

            // Updates to perform
            let writes_descriptor_set = [
                // TODO: add other things
                // BVHs
                WriteDescriptorSet::default()
                    .dst_set(descriptor_set)
                    .dst_binding(0)
                    .descriptor_count(1)
                    .descriptor_type(DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&descriptor_bvhs_info),
            ];

            let device = vulkan_context.get_device()?;
            vulkan_context.device_wait_idle()?;
            unsafe { device.update_descriptor_sets(&writes_descriptor_set, &[]) };
            info!("BVH descriptor set updated in the raytracing pipeline");
        }

        Ok(())
    }

    fn init_buffers(
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<RaytracingBuffers, ErrorCode> {
        // TODO: add other things
        let triangles_ssbo = match vulkan_context
            .map_data_to_buffer(scene.triangles.as_slice(), BufferUsageFlags::STORAGE_BUFFER)
        {
            Ok(buffer) => buffer,
            Err(err) => {
                error!(
                    "Failed to create the triangles ssbo for the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let models_ssbo = match vulkan_context
            .map_data_to_buffer(scene.models.as_slice(), BufferUsageFlags::STORAGE_BUFFER)
        {
            Ok(buffer) => buffer,
            Err(err) => {
                error!(
                    "Failed to create the models ssbo for the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let materials_ssbo = match vulkan_context
            .map_data_to_buffer(scene.materials.as_slice(), BufferUsageFlags::STORAGE_BUFFER)
        {
            Ok(buffer) => buffer,
            Err(err) => {
                error!(
                    "Failed to create the materials ssbo for the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let bvhs_ssbo = if scene.bvh_type == BvhType::None {
            None
        } else {
            let data = match scene.get_bvh() {
                Ok(bvh) => bvh,
                Err(err) => {
                    error!("Failed to get the bvh from the scene when initializing the bvhs ssbo in the raytracing pipeline: {:?}", err);
                    return Err(ErrorCode::Unknown);
                }
            };
            match vulkan_context
                .map_data_to_buffer(data.as_slice(), BufferUsageFlags::STORAGE_BUFFER)
            {
                Ok(buffer) => Some(buffer),
                Err(err) => {
                    error!(
                        "Failed to create the bvhs ssbo for the raytracing pipeline: {:?}",
                        err
                    );
                    return Err(ErrorCode::InitializationFailure);
                }
            }
        };

        let camera_ubo = match vulkan_context.map_data_to_buffer(
            &[scene.camera.get_gpu_data()],
            BufferUsageFlags::UNIFORM_BUFFER,
        ) {
            Ok(buffer) => buffer,
            Err(err) => {
                error!(
                    "Failed to create the camera ubo for the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };

        Ok(RaytracingBuffers {
            triangles_ssbo,
            models_ssbo,
            materials_ssbo,
            bvhs_ssbo,
            camera_ubo,
        })
    }

    fn clean_buffers(&mut self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        let allocator = &vulkan_context.get_allocator()?.allocator;
        if let Err(err) = self.buffers.triangles_ssbo.clean(allocator) {
            error!(
                "Failed to clean the triangles buffer in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.buffers.models_ssbo.clean(allocator) {
            error!(
                "Failed to clean the models buffer in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.buffers.materials_ssbo.clean(allocator) {
            error!(
                "Failed to clean the materials buffer in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Some(bvh_ssbo) = self.buffers.bvhs_ssbo.as_mut() {
            if let Err(err) = bvh_ssbo.clean(allocator) {
                error!(
                    "Failed to clean the bvhs buffer in the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::CleaningFailure);
            }
        }
        if let Err(err) = self.buffers.camera_ubo.clean(allocator) {
            error!(
                "Failed to clean the camera buffer in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        Ok(())
    }

    pub fn new(vulkan_context: &VulkanContext, scene: &Scene) -> Result<Self, ErrorCode> {
        let base = PipelineAttributes::default();
        let buffers = Self::init_buffers(vulkan_context, scene)?;
        Ok(RaytracingPipeline { base, buffers })
    }

    fn init_set_0(
        &mut self,
        vulkan_context: &VulkanContext,
        _scene: &Scene,
    ) -> Result<Descriptor, ErrorCode> {
        // Organize the set layout
        let mut layout_builder = DescriptorLayoutBuilder::default();
        // TODO: add other things
        // Framebuffer
        layout_builder.add_binding(0, DescriptorType::STORAGE_IMAGE)?;
        // Triangles
        layout_builder.add_binding(1, DescriptorType::STORAGE_BUFFER)?;
        // Models
        layout_builder.add_binding(2, DescriptorType::STORAGE_BUFFER)?;
        // Materials
        layout_builder.add_binding(3, DescriptorType::STORAGE_BUFFER)?;

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
        // TODO: add other things
        // Framebuffer
        let descriptor_framebuffer_info = [DescriptorImageInfo::default()
            // Image data we want to bind (here the image we want to draw into)
            .image_view(vulkan_context.get_draw_image()?.image_view)
            .image_layout(ImageLayout::GENERAL)];
        // Triangles
        let descriptor_triangles_info = [DescriptorBufferInfo::default()
            .buffer(self.buffers.triangles_ssbo.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];
        // Models
        let descriptor_models_info = [DescriptorBufferInfo::default()
            .buffer(self.buffers.models_ssbo.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];
        // Materials
        let descriptor_materials_info = [DescriptorBufferInfo::default()
            .buffer(self.buffers.materials_ssbo.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];

        // Updates to perform
        let writes_descriptor_set = [
            // TODO: add other things
            // Framebuffer
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_IMAGE)
                .image_info(&descriptor_framebuffer_info),
            // Triangles
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(1)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&descriptor_triangles_info),
            // Models
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(2)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&descriptor_models_info),
            // Materials
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(3)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::STORAGE_BUFFER)
                .buffer_info(&descriptor_materials_info),
        ];

        unsafe { device.update_descriptor_sets(&writes_descriptor_set, &[]) };

        Ok(Descriptor {
            set: descriptor_set,
            set_layout: descriptor_set_layout,
        })
    }

    fn init_set_1(
        &mut self,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<Descriptor, ErrorCode> {
        // Organize the set layout
        let mut layout_builder = DescriptorLayoutBuilder::default();
        // TODO: add other things
        // BVHs
        layout_builder.add_binding(0, DescriptorType::STORAGE_BUFFER)?;

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
        // TODO: add other things
        // BVHs
        if scene.bvh_type != BvhType::None {
            let descriptor_bvhs_info = [DescriptorBufferInfo::default()
                .buffer(self.buffers.bvhs_ssbo.as_ref().unwrap().buffer)
                .range(WHOLE_SIZE)
                .offset(0)];

            // Updates to perform
            let writes_descriptor_set = [
                // TODO: add other things
                // BVHs
                WriteDescriptorSet::default()
                    .dst_set(descriptor_set)
                    .dst_binding(0)
                    .descriptor_count(1)
                    .descriptor_type(DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&descriptor_bvhs_info),
            ];

            unsafe { device.update_descriptor_sets(&writes_descriptor_set, &[]) };
        }

        Ok(Descriptor {
            set: descriptor_set,
            set_layout: descriptor_set_layout,
        })
    }

    fn init_set_2(
        &mut self,
        vulkan_context: &VulkanContext,
        _scene: &Scene,
    ) -> Result<Descriptor, ErrorCode> {
        // Organize the set layout
        let mut layout_builder = DescriptorLayoutBuilder::default();
        // TODO: add other things
        // Camera
        layout_builder.add_binding(0, DescriptorType::UNIFORM_BUFFER)?;

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
        // TODO: add other things
        // Camera
        let descriptor_camera_info = [DescriptorBufferInfo::default()
            .buffer(self.buffers.camera_ubo.buffer)
            .range(WHOLE_SIZE)
            .offset(0)];

        // Updates to perform
        let writes_descriptor_set = [
            // TODO: add other things
            // Camera
            WriteDescriptorSet::default()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .descriptor_count(1)
                .descriptor_type(DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&descriptor_camera_info),
        ];

        unsafe { device.update_descriptor_sets(&writes_descriptor_set, &[]) };

        Ok(Descriptor {
            set: descriptor_set,
            set_layout: descriptor_set_layout,
        })
    }
}

impl ComputePipeline for RaytracingPipeline {
    fn get_attributes(&self) -> Result<&PipelineAttributes, ErrorCode> {
        Ok(&self.base)
    }

    fn init_descriptors(
        &mut self,
        vulkan_context: &VulkanContext,
        scene: &Scene,
    ) -> Result<(), ErrorCode> {
        // Set 0
        let set_0 = match self.init_set_0(vulkan_context, scene) {
            Ok(set) => set,
            Err(err) => {
                error!(
                    "Failed to initialize the descriptor set 0 in the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };
        self.base.descriptors.push(set_0);

        // Set 1
        let set_1 = match self.init_set_1(vulkan_context, scene) {
            Ok(set) => set,
            Err(err) => {
                error!(
                    "Failed to initialize the descriptor set 1 in the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };
        self.base.descriptors.push(set_1);

        // Set 2
        let set_2 = match self.init_set_2(vulkan_context, scene) {
            Ok(set) => set,
            Err(err) => {
                error!(
                    "Failed to initialize the descriptor set 2 in the raytracing pipeline: {:?}",
                    err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };
        self.base.descriptors.push(set_2);
        Ok(())
    }

    fn init_push_constants(
        &mut self,
        _vulkan_context: &VulkanContext,
        _scene: &Scene,
    ) -> Result<(), ErrorCode> {
        let range = PushConstantRange::default()
            .offset(0)
            .size(size_of::<RaytracingPushConstant>() as u32)
            .stage_flags(ShaderStageFlags::COMPUTE);
        let push_constant = PushConstant { range };
        self.base.push_constants = Some(push_constant);
        Ok(())
    }

    fn run(&mut self, vulkan_context: &VulkanContext, scene: &Scene) -> Result<(), ErrorCode> {
        // Update camera
        self.update_camera_buffer(vulkan_context, scene)?;

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

        // Bind the descriptor sets
        let descriptor_sets = self
            .base
            .descriptors
            .iter()
            .map(|d| d.set)
            .collect::<Vec<DescriptorSet>>();
        unsafe {
            device.cmd_bind_descriptor_sets(
                command_buffer,
                PipelineBindPoint::COMPUTE,
                self.base.pipeline_layout,
                0,
                &descriptor_sets,
                &[],
            )
        };

        // TODO: add push constants if needed
        let push_constant = RaytracingPushConstant {
            nb_triangles: scene.triangles.len() as u32,
            is_wireframe_on: scene.is_wireframe_on as u32,
            bvh_type: scene.bvh_last_type as u32,
        };
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
        // We are using 32x32 workgroup size so we need to divide the drawing size by 32
        let thread_group_size_x = 32;
        let thread_group_size_y = 32;
        let thread_group_size_z = 1;
        unsafe {
            device.cmd_dispatch(
                command_buffer,
                vulkan_context.draw_extent.width / thread_group_size_x + 1,
                vulkan_context.draw_extent.height / thread_group_size_y + 1,
                thread_group_size_z,
            )
        };

        Ok(())
    }

    fn set_pipeline(&mut self, pipeline: Pipeline) {
        self.base.pipeline = pipeline;
    }
    fn set_pipeline_layout(&mut self, pipeline_layout: PipelineLayout) {
        self.base.pipeline_layout = pipeline_layout;
    }

    fn init_pool_size_ratios(&mut self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        // TODO: add other things
        let pool_size_ratios = [
            // Framebuffer
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_IMAGE,
                ratio: 1.0,
            },
            // Triangles
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
            // Models
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
            // Materials
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
            // BVHs
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::STORAGE_BUFFER,
                ratio: 1.0,
            },
            // Camera
            DescriptorPoolSizeRatio {
                descriptor_type: DescriptorType::UNIFORM_BUFFER,
                ratio: 1.0,
            },
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
        if let Err(err) = self.clean_buffers(vulkan_context) {
            error!(
                "Failed to clean the buffers in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.clean_descriptors(vulkan_context) {
            error!(
                "Failed to clean the vulkan descriptors in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        if let Err(err) = self.clean_pipeline(vulkan_context) {
            error!(
                "Failed to clean the vulkan pipeline handler in the raytracing pipeline: {:?}",
                err
            );
            return Err(ErrorCode::CleaningFailure);
        }
        Ok(())
    }
}
