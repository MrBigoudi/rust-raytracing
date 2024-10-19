use std::path::Path;

use ash::{
    util::read_spv,
    vk::{ShaderModule, ShaderModuleCreateInfo},
    Device,
};
use log::error;

use crate::application::core::error::ErrorCode;

pub struct Shader;

impl Shader {
    pub fn get_compiled_shader_path(shader: &str) -> String {
        let base_path = Path::new("/target/shaders");
        let relative_path = Path::new(shader);
        base_path
            .join(relative_path)
            .with_extension("spv")
            .to_string_lossy()
            .into_owned()
    }

    pub fn load_shader_module(
        file_path: String,
        device: &Device,
    ) -> Result<ShaderModule, ErrorCode> {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let spv_path = crate_path.to_owned() + &file_path;
        // Open the file with cursor at the end
        let mut spv_file = match std::fs::File::open(spv_path.clone()) {
            Ok(file) => file,
            Err(err) => {
                error!("Failed to open the shader `{:?}': {:?}", spv_path, err);
                return Err(ErrorCode::IO);
            }
        };

        let spv_code = match read_spv(&mut spv_file) {
            Ok(code) => code,
            Err(err) => {
                error!("Failed to read the shader `{:?}': {:?}", spv_path, err);
                return Err(ErrorCode::IO);
            }
        };

        let create_info = ShaderModuleCreateInfo::default().code(&spv_code);

        match unsafe { device.create_shader_module(&create_info, None) } {
            Ok(module) => Ok(module),
            Err(err) => {
                error!("Failed to create a shader module: {:?}", err);
                Err(ErrorCode::VulkanFailure)
            }
        }
    }
}
