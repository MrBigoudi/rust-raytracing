use compute_pipeline::ComputePipeline;
use implementation::test_pipeline::TestPipeline;
use log::error;

use super::{core::error::ErrorCode, scene::Scene, vulkan::types::VulkanContext};

pub mod compute_pipeline;
pub mod descriptor;
pub mod implementation;
pub mod push_constant;
pub mod shader;

#[derive(Default)]
pub struct Pipelines {
    // TODO: use the correct pipeline
    pub test_pipeline: TestPipeline,
}

impl Pipelines {
    pub fn init(vulkan_context: &VulkanContext, scene: &Scene) -> Result<Self, ErrorCode> {
        let mut pipelines = Self::default();
        // TODO: use the correct shader
        if let Err(err) = pipelines
            .test_pipeline
            .init("test", "main", vulkan_context, scene)
        {
            error!("Failed to initialize the test pipeline: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        };
        Ok(pipelines)
    }

    pub fn clean(&self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        vulkan_context.device_wait_idle()?;
        if let Err(err) = self.test_pipeline.clean(vulkan_context) {
            error!("Failed to clean the test pipeline: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        }
        Ok(())
    }
}
