use std::ffi::{CStr, CString};

use ash::{
    vk::{make_api_version, ApplicationInfo, InstanceCreateInfo, API_VERSION_1_3},
    Instance,
};
use log::{debug, error};
use winit::{raw_window_handle::HasDisplayHandle, window::Window};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

impl VulkanContext<'_> {
    pub fn get_instance(&self) -> Result<&Instance, ErrorCode> {
        match &self.instance {
            Some(instance) => Ok(instance),
            None => {
                error!("Can't access the vulkan instance");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    fn get_required_layers(&self) -> Result<Vec<*const i8>, ErrorCode> {
        let mut required_layers = Vec::new();

        #[cfg(debug_assertions)]
        required_layers.push(c"VK_LAYER_KHRONOS_validation".as_ptr());

        let available_layers = unsafe {
            match self.get_entry()?.enumerate_instance_layer_properties() {
                Ok(layers) => layers,
                Err(err) => {
                    error!("Failed to enumerate the available layers: {:?}", err);
                    return Err(ErrorCode::InitializationFailure);
                }
            }
        };
        for required in required_layers.clone() {
            let mut is_available = false;
            'inner: for available in &available_layers {
                let name = match available.layer_name_as_c_str() {
                    Ok(name) => name,
                    Err(err) => {
                        error!("Failed to fetch the layer name: {:?}", err);
                        return Err(ErrorCode::InitializationFailure);
                    }
                };
                if name == unsafe { CStr::from_ptr(required) } {
                    is_available = true;
                    break 'inner;
                }
            }
            if !is_available {
                error!("The required layer {:?} is not available!\n", required);
                return Err(ErrorCode::VulkanFailure);
            }
        }
        Ok(required_layers)
    }

    fn get_required_extensions(&self, window: &Window) -> Result<Vec<*const i8>, ErrorCode> {
        let raw_display_handle = match window.display_handle() {
            Ok(handle) => handle.as_raw(),
            Err(err) => {
                error!("Failed to get the window display handle: {:?}", err);
                return Err(ErrorCode::VulkanFailure);
            }
        };

        // Get the required extensions
        let mut required_extensions =
            match ash_window::enumerate_required_extensions(raw_display_handle) {
                Ok(extensions) => extensions.to_vec(),
                Err(err) => {
                    error!("Failed to get the required extensions: {:?}\n", err);
                    return Err(ErrorCode::VulkanFailure);
                }
            };

        #[cfg(debug_assertions)]
        required_extensions.push(c"VK_EXT_debug_utils".as_ptr());

        Ok(required_extensions)
    }

    fn display_extensions(extensions: &Vec<*const i8>) {
        debug!("Extensions:");
        for extension in extensions {
            let extension_name = unsafe { CStr::from_ptr(*extension).to_string_lossy() };
            debug!("\t{:?}", extension_name);
        }
    }

    fn display_layers(layers: &Vec<*const i8>) {
        debug!("Layers:");
        for layer in layers {
            let layer_name = unsafe { CStr::from_ptr(*layer).to_string_lossy() };
            debug!("\t{:?}", layer_name);
        }
    }

    pub fn init_instance(&mut self, window: &Window) -> Result<(), ErrorCode> {
        let application_name = self.parameters.window_title.as_str();
        let application_name_cstr = CString::new(application_name).unwrap();

        let application_info = ApplicationInfo::default()
            .api_version(API_VERSION_1_3)
            .application_name(&application_name_cstr)
            .application_version(make_api_version(0, 1, 0, 0));

        // Get the required extensions
        let required_extensions = self.get_required_extensions(window)?;

        // Get the required layers
        let required_layers = self.get_required_layers()?;

        #[cfg(debug_assertions)]
        Self::display_extensions(&required_extensions);
        #[cfg(debug_assertions)]
        Self::display_layers(&required_layers);

        let instance_create_info = InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&required_extensions)
            .enabled_layer_names(&required_layers);

        unsafe {
            match self
                .get_entry()?
                .create_instance(&instance_create_info, self.get_allocation_callback()?)
            {
                Ok(instance) => {
                    self.instance = Some(instance);
                    Ok(())
                }
                Err(err) => {
                    error!("Failed to create the vulkan instance: {:?}", err);
                    Err(ErrorCode::VulkanFailure)
                }
            }
        }
    }

    pub fn clean_instance(&mut self) -> Result<(), ErrorCode> {
        if self.instance.is_none() {
            return Ok(());
        }
        unsafe {
            self.get_instance()?
                .destroy_instance(self.get_allocation_callback()?);
        }
        self.instance = None;
        Ok(())
    }
}
