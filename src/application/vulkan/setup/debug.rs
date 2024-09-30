use std::{borrow::Cow, ffi::CStr};

use ash::{ext::debug_utils, vk};
use log::{debug, error, info, warn};

use crate::application::{core::error::ErrorCode, vulkan::types::VulkanContext};

/// Callback function for Vulkan debug messages.
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {
        error!("VULKAN: {message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
    }
    if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {
        warn!("VULKAN: {message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
    }
    if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::INFO {
        debug!("VULKAN: {message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
    }
    if message_severity == vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE {
        info!("VULKAN: {message_type:?} [{message_id_name} ({message_id_number})] : {message}\n");
    }

    vk::FALSE
}

impl VulkanContext<'_> {
    pub fn get_debug_loader(&self) -> Result<&debug_utils::Instance, ErrorCode> {
        match &self.debug_utils_loader {
            Some(instance) => Ok(instance),
            None => {
                error!("Can't access the vulkan debug loader");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn get_debug_callback(&self) -> Result<&vk::DebugUtilsMessengerEXT, ErrorCode> {
        match &self.debug_callback {
            Some(debug_callback) => Ok(debug_callback),
            None => {
                error!("Can't access the vulkan debug callback");
                Err(ErrorCode::AccessFailure)
            }
        }
    }

    pub fn init_debugger(&mut self) -> Result<(), ErrorCode> {
        // Setup debug callback
        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING,
            )
            .pfn_user_callback(Some(vulkan_debug_callback));

        let debug_utils_loader =
            debug_utils::Instance::new(self.get_entry()?, self.get_instance()?);
        let debug_callback = unsafe {
            match debug_utils_loader
                .create_debug_utils_messenger(&debug_info, self.get_allocator()?)
            {
                Ok(callback) => callback,
                Err(err) => {
                    error!("Failed to initialize the debug callbacks: {:?}", err);
                    return Err(ErrorCode::VulkanFailure);
                }
            }
        };

        self.debug_utils_loader = Some(debug_utils_loader);
        self.debug_callback = Some(debug_callback);
        Ok(())
    }

    pub fn clean_debugger(&mut self) -> Result<(), ErrorCode> {
        unsafe {
            self.get_debug_loader()?
                .destroy_debug_utils_messenger(*self.get_debug_callback()?, self.get_allocator()?);
        }
        self.debug_callback = None;
        self.debug_utils_loader = None;
        Ok(())
    }
}
