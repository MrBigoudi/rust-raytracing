use ash::vk::PushConstantRange;

#[derive(Default)]
pub struct PushConstant {
    pub range: PushConstantRange,
}

impl PushConstant {
    pub fn data_to_u8_slice<T: Sized>(p: &T) -> &[u8] {
        unsafe {
            ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
        }
    }
}
