use std::path::{Path, PathBuf};

use glam::Mat4;
use log::{error, info};

use crate::application::core::error::ErrorCode;

use super::{material::Material, triangle::Triangle};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Model {
    pub model_matrix: Mat4,
    pub material_index: usize,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            model_matrix: Mat4::IDENTITY,
            material_index: 0,
        }
    }
}

impl Model {
    #[allow(unused)]
    pub fn triangle() -> (Model, Vec<Triangle>) {
        (Model::default(), vec![Triangle::default()])
    }

    #[allow(unused)]
    pub fn add_obj(
        object_file_name: &Path,
        has_material_file: bool,
        model_matrix: Option<Mat4>,
        in_out_triangles: &mut Vec<Triangle>,
        in_out_models: &mut Vec<Model>,
        in_out_materials: &mut Vec<Material>,
    ) -> Result<(), ErrorCode> {
        let (mut new_models, mut new_triangles, mut new_materials) =
            match Model::from_obj(object_file_name, has_material_file) {
                Ok((new_models, new_triangles, new_materials)) => {
                    (new_models, new_triangles, new_materials)
                }
                Err(err) => {
                    error!(
                        "Failed to create new models from a given file `{:?}': {:?}",
                        object_file_name, err
                    );
                    return Err(ErrorCode::InitializationFailure);
                }
            };

        // Update the new objects
        let new_model_first_idx = in_out_models.len();
        let new_materials_first_idx = in_out_materials.len();
        for new_triangle in &mut new_triangles {
            new_triangle.model_index += new_model_first_idx;
        }
        
        for new_model in &mut new_models {
            // Be careful about default material at index = 0
            if new_model.material_index > 0 {
                new_model.material_index += new_materials_first_idx - 1;
            }
            // Add a custom model matrix
            if let Some(model_matrix) = model_matrix {
                new_model.model_matrix = model_matrix;
            }
        }

        // Update the old objects
        in_out_triangles.append(&mut new_triangles);
        in_out_models.append(&mut new_models);
        in_out_materials.append(&mut new_materials);

        info!(
            "Number of triangles after adding a new object: {}",
            in_out_triangles.len()
        );

        Ok(())
    }

    fn display_material(tobj_material: &tobj::Material) {
        let material = tobj_material;
        info!("material.name = \'{}\'", material.name);
        if let Some(ambient) = material.ambient {
            info!(
                "    material.Ka = ({}, {}, {})",
                ambient[0], ambient[1], ambient[2]
            );
        }
        if let Some(diffuse) = material.diffuse {
            info!(
                "    material.Kd = ({}, {}, {})",
                diffuse[0], diffuse[1], diffuse[2]
            );
        }
        if let Some(specular) = material.specular {
            info!(
                "    material.Ks = ({}, {}, {})",
                specular[0], specular[1], specular[2]
            );
        }
        if let Some(shininess) = material.shininess {
            info!("    material.Ns = {}", shininess);
        }
        if let Some(dissolve) = material.dissolve {
            info!("    material.d = {}", dissolve);
        }
        if let Some(ambient_texture) = &material.ambient_texture {
            info!("    material.map_Ka = {}", ambient_texture);
        }
        if let Some(diffuse_texture) = &material.diffuse_texture {
            info!("    material.map_Kd = {}", diffuse_texture);
        }
        if let Some(specular_texture) = &material.specular_texture {
            info!("    material.map_Ks = {}", specular_texture);
        }
        if let Some(shininess_texture) = &material.shininess_texture {
            info!("    material.map_Ns = {}", shininess_texture);
        }
        if let Some(normal_texture) = &material.normal_texture {
            info!("    material.map_Bump = {}", normal_texture);
        }
        if let Some(dissolve_texture) = &material.dissolve_texture {
            info!("    material.map_d = {}", dissolve_texture);
        }

        for (k, v) in &material.unknown_param {
            info!("    material.{} = {}", k, v);
        }
    }

    #[allow(unused)]
    pub fn display_model(tobj_model: &tobj::Model) {
        let model = tobj_model;
        let mesh = &model.mesh;
        info!("model.name = \'{}\'", model.name);
        info!("model.mesh.material_id = {:?}", mesh.material_id);

        info!("Size of model.face_arities: {}", mesh.face_arities.len());

        let mut next_face = 0;
        for f in 0..mesh.face_arities.len() {
            let end = next_face + mesh.face_arities[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
            info!("    face[{}] = {:?}", f, face_indices);
            next_face = end;
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        info!("model.vertices: {}", mesh.positions.len() / 3);

        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            info!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }
    }

    fn create_materials(tobj_materials: Vec<tobj::Material>) -> Vec<Material> {
        let mut new_materials = Vec::new();

        for material in &tobj_materials {
            let mut new_material = Material::default();
            Self::display_material(material);
            // TODO: add other material properties
            if let Some(ambient) = &material.ambient {
                new_material.ambient = glam::Vec4::new(ambient[0], ambient[1], ambient[2], 1.);
            }

            new_materials.push(new_material);
        }

        new_materials
    }

    fn create_triangles(
        model_index: usize,
        faces: &Vec<Vec<usize>>,
        vertices: &[glam::Vec4],
    ) -> Vec<Triangle> {
        let mut new_triangles = Vec::new();
        for face in faces {
            let new_triangle = Triangle {
                p0: vertices[face[0]],
                p1: vertices[face[1]],
                p2: vertices[face[2]],
                model_index,
            };
            new_triangles.push(new_triangle);
        }

        new_triangles
    }

    fn create_models_triangles(tobj_models: Vec<tobj::Model>) -> (Vec<Model>, Vec<Triangle>) {
        let mut new_models = Vec::new();
        let mut new_triangles = Vec::new();

        for (model_index, model) in tobj_models.iter().enumerate() {
            let mut new_model = Model::default();

            // Self::display_model(model);
            let mesh = &model.mesh;
            if let Some(id) = mesh.material_id {
                // +1 to avoid hitting default material
                new_model.material_index = id + 1;
            }

            // Get the faces
            let mut faces = Vec::new();
            let mut next_face = 0;
            // Assume only triangles
            for _ in 0..(mesh.indices.len() / 3) {
                let end = next_face + 3;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                faces.push(vec![
                    *face_indices[0] as usize,
                    *face_indices[1] as usize,
                    *face_indices[2] as usize,
                ]);
                next_face = end;
            }

            // Get the vertices
            let mut vertices = Vec::new();
            assert!(mesh.positions.len() % 3 == 0);
            for v in 0..mesh.positions.len() / 3 {
                let vx = mesh.positions[3 * v];
                let vy = mesh.positions[3 * v + 1];
                let vz = mesh.positions[3 * v + 2];
                vertices.push(glam::Vec4::new(vx, vy, vz, 1.));
            }

            // Create the triangles
            let mut triangles_tmp = Self::create_triangles(model_index, &faces, &vertices);
            new_triangles.append(&mut triangles_tmp);
            new_models.push(new_model);
        }

        (new_models, new_triangles)
    }

    fn create_models_triangles_materials(
        tobj_models: Vec<tobj::Model>,
        tobj_materials: Vec<tobj::Material>,
    ) -> (Vec<Model>, Vec<Triangle>, Vec<Material>) {
        info!("# of models: {}", tobj_models.len());
        info!("# of materials: {}", tobj_materials.len());
        let materials = Self::create_materials(tobj_materials);
        let (models, triangles) = Self::create_models_triangles(tobj_models);

        (models, triangles, materials)
    }

    #[allow(clippy::type_complexity)]
    fn from_obj(
        object_file_name: &Path,
        has_material_file: bool,
    ) -> Result<(Vec<Model>, Vec<Triangle>, Vec<Material>), ErrorCode> {
        info!("Loading a new object...");
        let crate_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let base_path = crate_path.join(Path::new("src/assets/models"));
        let file_path = base_path.join(object_file_name);

        let load_options = tobj::GPU_LOAD_OPTIONS;
        let (models, materials) = match tobj::load_obj(&file_path, &load_options) {
            Ok(models) => models,
            Err(err) => {
                error!(
                    "Failed to load the object file `{:?}': {:?}",
                    file_path, err
                );
                return Err(ErrorCode::InitializationFailure);
            }
        };

        let materials = if has_material_file {
            match materials {
                Ok(materials) => materials,
                Err(err) => {
                    error!(
                        "Failed to load the materials for the object `{:?}': {:?}",
                        object_file_name, err
                    );
                    return Err(ErrorCode::InitializationFailure);
                }
            }
        } else {
            Vec::new()
        };

        Ok(Self::create_models_triangles_materials(models, materials))
    }

    #[allow(unused)]
    pub fn add_sphere(
        resolution: u32,
        radius: f32,
        center: glam::Vec3,
        material: Option<Material>,
        in_out_triangles: &mut Vec<Triangle>,
        in_out_models: &mut Vec<Model>,
        in_out_materials: &mut Vec<Material>,
    ) -> Result<(), ErrorCode> {
        let step_phi = std::f32::consts::PI / (resolution as f32);
        let step_theta = 2. * std::f32::consts::PI / (resolution as f32);

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Build the vertices
        for i in 0..=resolution {
            let phi = (i as f32) * step_phi;
            for j in 0..=resolution {
                let theta = (j as f32) * step_theta;

                let x = phi.sin() * theta.sin();
                let y = phi.cos();
                let z = phi.sin() * theta.cos();

                vertices.push(glam::Vec3 { x, y, z });
            }
        }

        // Build the indices
        for i in 0..resolution {
            for j in 0..resolution {
                let first = i * (resolution + 1) + j;
                let second = first + resolution + 1;

                indices.push((first, second, first + 1));
                indices.push((second, second + 1, first + 1));
            }
        }

        // Build the material
        let material_index = if material.is_some() {
            in_out_materials.len()
        } else {
            0
        };
        if let Some(material) = material {
            in_out_materials.push(material);
        }

        // Build the model
        let sphere_model = Model {
            model_matrix: glam::Mat4 {
                x_axis: glam::Vec4::new(radius, 0., 0., 0.),
                y_axis: glam::Vec4::new(0., radius, 0., 0.),
                z_axis: glam::Vec4::new(0., 0., radius, 0.),
                w_axis: glam::Vec4::new(center.x, center.y, center.z, 1.),
            },
            material_index,
        };

        // Build the triangles
        let sphere_model_index = in_out_models.len();
        for index in indices {
            let x = index.0 as usize;
            let y = index.1 as usize;
            let z = index.2 as usize;
            let p0 = glam::Vec4::new(vertices[x].x, vertices[x].y, vertices[x].z, 1.);
            let p1 = glam::Vec4::new(vertices[y].x, vertices[y].y, vertices[y].z, 1.);
            let p2 = glam::Vec4::new(vertices[z].x, vertices[z].y, vertices[z].z, 1.);

            let triangle = Triangle {
                p0,
                p1,
                p2,
                model_index: sphere_model_index,
            };

            in_out_triangles.push(triangle);
        }

        in_out_models.push(sphere_model);

        info!(
            "Number of triangles after adding a new sphere: {}",
            in_out_triangles.len()
        );

        Ok(())
    }
}
