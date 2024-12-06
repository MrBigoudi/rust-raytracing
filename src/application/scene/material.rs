use glam::Vec4;
use rand::Rng;

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

impl Material {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen();
        let g = rng.gen();
        let b = rng.gen();

        let ambient = Vec4::new(r, g, b, 1.);
        Self {
            ambient,
        }
    }
}
