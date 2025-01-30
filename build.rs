use std::fs::create_dir_all;
use std::path::Path;
use std::process::Command;

fn main() {
    // Specify the list of shaders and their entry points
    let shaders = vec![("src/shaders/raytracing.slang", "main")];

    // Define the base output directory
    let out_dir = Path::new("target/shaders");

    for (shader, entry_point) in shaders {
        // Get the output file path
        let shader_path = Path::new(shader);
        let relative_path = shader_path.strip_prefix("src/shaders").unwrap();
        let output_path = out_dir.join(relative_path).with_extension("spv");

        // Ensure the output directory exists
        if let Some(parent) = output_path.parent() {
            create_dir_all(parent).unwrap();
        }

        let (status, is_glslc) = match Command::new("slangc")
            .arg(shader)
            .arg("-emit-spirv-directly")
            .arg("-g2")
            .arg("-profile")
            .arg("glsl_460")
            .arg("-target")
            .arg("spirv")
            .arg("-o")
            .arg(&output_path)
            .arg("-entry")
            .arg(entry_point)
            .status()
        {
            Ok(status) => (status, false),
            Err(_) => {
                let glsl_path = shader_path.join(".glsl");
                let glslc_status = Command::new("glslc")
                    .arg(glsl_path)
                    .arg("-o")
                    .arg(&output_path)
                    .status()
                    .unwrap();
                (glslc_status, true)
            }
        };

        if status.success() && !is_glslc {
            // Recompile the glsl version
            let glsl_path = shader_path.with_extension("glsl");
            let status = Command::new("slangc")
                .arg(shader)
                .arg("-emit-spirv-directly")
                .arg("-g2")
                .arg("-profile")
                .arg("glsl_460")
                .arg("-target")
                .arg("glsl")
                .arg("-o")
                .arg(&glsl_path)
                .arg("-entry")
                .arg(entry_point)
                .status()
                .unwrap();
            if !status.success() {
                panic!("Shader compilation to glsl failed for {}", shader);
            }
        } else {
            panic!("Shader compilation failed for {}", shader);
        }
    }

    // Rerun when shaders change
    println!("cargo:rerun-if-changed=src/shaders");
}
