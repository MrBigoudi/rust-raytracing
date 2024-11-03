use compute_pipeline::ComputePipeline;
use implementation::raytracing_pipeline::RaytracingPipeline; //, test_pipeline::TestPipeline};
use log::error;

use super::{core::error::ErrorCode, scene::Scene, vulkan::types::VulkanContext};

pub mod compute_pipeline;
pub mod descriptor;
pub mod implementation;
pub mod push_constant;
pub mod shader;

pub struct Pipelines {
    // TODO: use the correct pipeline
    // pub test_pipeline: TestPipeline,
    pub raytracing_pipeline: RaytracingPipeline,
}

impl Pipelines {
    pub fn init(vulkan_context: &VulkanContext, scene: &Scene) -> Result<Self, ErrorCode> {
        // let mut test_pipeline = TestPipeline::default();
        // if let Err(err) = test_pipeline.init("test", "main", vulkan_context, scene) {
        //     error!("Failed to initialize the test pipeline: {:?}", err);
        //     return Err(ErrorCode::InitializationFailure);
        // };

        let mut raytracing_pipeline = RaytracingPipeline::new(vulkan_context, scene)?;
        if let Err(err) = raytracing_pipeline.init("raytracing", "main", vulkan_context, scene) {
            error!("Failed to initialize the raytracing pipeline: {:?}", err);
            return Err(ErrorCode::InitializationFailure);
        };

        Ok(Pipelines {
            // test_pipeline,
            raytracing_pipeline,
        })
    }

    pub fn clean(&mut self, vulkan_context: &VulkanContext) -> Result<(), ErrorCode> {
        vulkan_context.device_wait_idle()?;
        if let Err(err) = self.raytracing_pipeline.clean(vulkan_context) {
            error!("Failed to clean the raytracing pipeline: {:?}", err);
            return Err(ErrorCode::CleaningFailure);
        }
        // if let Err(err) = self.test_pipeline.clean(vulkan_context) {
        //     error!("Failed to clean the test pipeline: {:?}", err);
        //     return Err(ErrorCode::CleaningFailure);
        // }
        Ok(())
    }
}
