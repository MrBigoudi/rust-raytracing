use glam::Vec4;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Material {
    pub ambient: Vec4,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vec4::from_array([1., 1., 1., 1.]),
        }
    }
}
