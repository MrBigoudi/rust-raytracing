use ash::{
    vk::{
        AccessFlags2, CommandBuffer, DependencyInfo, Image, ImageAspectFlags, ImageLayout,
        ImageMemoryBarrier2, ImageSubresourceRange, PipelineStageFlags2, REMAINING_ARRAY_LAYERS,
        REMAINING_MIP_LEVELS,
    },
    Device,
};

use crate::application::core::error::ErrorCode;

pub fn get_default_image_subresource_range() -> ImageSubresourceRange {
    ImageSubresourceRange::default()
        .aspect_mask(ImageAspectFlags::COLOR)
        .base_mip_level(0)
        .level_count(REMAINING_MIP_LEVELS)
        .base_array_layer(0)
        .layer_count(REMAINING_ARRAY_LAYERS)
}

pub fn transition_image(
    device: &Device,
    cmd: &CommandBuffer,
    image: &Image,
    old_layout: ImageLayout,
    new_layout: ImageLayout,
) -> Result<(), ErrorCode> {
    let aspect_mask = if new_layout == ImageLayout::DEPTH_ATTACHMENT_OPTIMAL {
        ImageAspectFlags::DEPTH
    } else {
        ImageAspectFlags::COLOR
    };

    // This lets us target a part of the image with the barrier
    // Its most useful for things like array images or mipmapped images,
    // where we would only need to barrier on a given layer or mipmap level
    // We are going to completely default it and have it transition all mipmap levels and layers
    let image_subresource_range = get_default_image_subresource_range().aspect_mask(aspect_mask);

    // VkImageMemoryBarrier2 contains the information for a given image barrier
    // Here, is where we set the old and new layouts
    // In the StageMask, we are doing ALL_COMMANDS. This is inefficient, as it will stall the GPU pipeline a bit
    // For our needs, its going to be fine as we are only going to do a few transitions per frame
    // If you are doing many transitions per frame as part of a post-process chain,
    // you want to avoid doing this, and instead use StageMasks more accurately
    let image_barriers = [ImageMemoryBarrier2::default()
        .src_stage_mask(PipelineStageFlags2::ALL_COMMANDS)
        .src_access_mask(AccessFlags2::MEMORY_WRITE)
        .dst_stage_mask(PipelineStageFlags2::ALL_COMMANDS)
        .dst_access_mask(AccessFlags2::MEMORY_WRITE | AccessFlags2::MEMORY_READ)
        .old_layout(old_layout)
        .new_layout(new_layout)
        .subresource_range(image_subresource_range)
        .image(*image)];

    let dependeny_info = DependencyInfo::default().image_memory_barriers(&image_barriers);

    unsafe { device.cmd_pipeline_barrier2(*cmd, &dependeny_info) };

    Ok(())
}
