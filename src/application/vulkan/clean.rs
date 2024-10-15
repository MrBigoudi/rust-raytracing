use log::debug;


use super::types::VulkanContext;

impl Drop for VulkanContext<'_> {
    fn drop(&mut self) {
        self.device_wait_idle().unwrap();

        if let Err(err) = self.clean_commands() {
            panic!("Failed to shutdown the vulkan commands: {:?}", err);
        } else {
            debug!("Vulkan commands cleaned successfully !");
        }

        if let Err(err) = self.clean_swpachain() {
            panic!("Failed to shutdown the vulkan swapchain: {:?}", err);
        } else {
            debug!("Vulkan swapchain cleaned successfully !");
        }

        if let Err(err) = self.clean_queues() {
            panic!(
                "Failed to shutdown the vulkan logical device queues: {:?}",
                err
            );
        } else {
            debug!("Vulkan logical device queues cleaned successfully !");
        }

        if let Err(err) = self.clean_device() {
            panic!("Failed to shutdown the vulkan logical device: {:?}", err);
        } else {
            debug!("Vulkan logical device cleaned successfully !");
        }

        if let Err(err) = self.clean_physical_device() {
            panic!("Failed to shutdown the vulkan physical device: {:?}", err);
        } else {
            debug!("Vulkan physical device cleaned successfully !");
        }

        if let Err(err) = self.clean_device_requirements() {
            panic!(
                "Failed to shutdown the vulkan device requirements: {:?}",
                err
            );
        } else {
            debug!("Vulkan device requirements cleaned successfully !");
        }

        if let Err(err) = self.clean_surface() {
            panic!("Failed to shutdown the vulkan surface: {:?}", err);
        } else {
            debug!("Vulkan surface cleaned successfully !");
        }

        #[cfg(debug_assertions)]
        {
            if let Err(err) = self.clean_debugger() {
                panic!("Failed to clean the vulkan debugger: {:?}", err);
            } else {
                debug!("Vulkan debugger cleaned successfully !");
            }
        }

        if let Err(err) = self.clean_instance() {
            panic!("Failed to clean the vulkan instance: {:?}", err);
        } else {
            debug!("Vulkan instance cleaned successfully !");
        }

        if let Err(err) = self.clean_allocator() {
            panic!("Failed to clean the vulkan allocator: {:?}", err);
        } else {
            debug!("Vulkan allocator cleaned successfully !");
        }

        if let Err(err) = self.clean_entry() {
            panic!("Failed to clean the vulkan entry: {:?}", err);
        } else {
            debug!("Vulkan entry cleaned successfully !");
        }

    }
}
