use ash::vk::{DescriptorSet, DescriptorSetLayout};

pub struct Descriptor {
    pub set: DescriptorSet,
    pub set_layout: DescriptorSetLayout,
}