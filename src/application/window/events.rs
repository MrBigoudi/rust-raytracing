use log::{debug, error, warn};
use winit::{
    dpi::{LogicalPosition, PhysicalSize},
    event::{DeviceId, KeyEvent},
};

use crate::application::{core::error::ErrorCode, Application};

impl Application<'_> {
    pub fn on_exit(&mut self) -> Result<(), ErrorCode> {
        if let Some(vulkan_context) = &mut self.vulkan_context {
            if let Some(pipelines) = &mut self.pipelines {
                if let Err(err) = pipelines.clean(vulkan_context) {
                    error!("Failed to clean the pipelines: {:?}", err);
                    return Err(ErrorCode::CleaningFailure);
                } else {
                    debug!("Pipelines cleaned successfully !");
                }
            } else {
                warn!("The pipelines are not initialized correctly...")
            }

            if let Err(err) = vulkan_context.clean() {
                error!("Failed to clean the vulkan context: {:?}", err);
                return Err(ErrorCode::CleaningFailure);
            } else {
                debug!("Vulkan context cleaned successfully !");
            }
        } else {
            warn!("The vulkan context is not initialized correctly...");
        }

        Ok(())
    }

    pub fn on_resize(&mut self, new_physical_size: PhysicalSize<u32>) -> Result<(), ErrorCode> {
        let new_width = new_physical_size.width as u16;
        let new_height = new_physical_size.height as u16;

        self.parameters.window_width = new_width;
        self.parameters.window_height = new_height;
        if let Some(vulkan_context) = &mut self.vulkan_context {
            vulkan_context.parameters.window_width = new_height;
            vulkan_context.parameters.window_height = new_width;
        } else {
            warn!("The vulkan context is not initialized correctly...");
        }
        if let Some(scene) = &mut self.scene {
            scene.camera.on_resize(new_width, new_height);
        } else {
            warn!("The vulkan scene is not initialized correctly...");
        }
        Ok(())
    }

    pub fn on_redraw(&mut self) -> Result<(), ErrorCode> {
        if let Some(vulkan_context) = &mut self.vulkan_context {
            if let Some(pipelines) = &mut self.pipelines {
                if let Some(window) = &self.window {
                    if let Some(scene) = &self.scene {
                        if let Err(err) = vulkan_context.draw(pipelines, window, scene) {
                            error!("The vulkan context failed to draw stuff: {:?}", err);
                            return Err(ErrorCode::VulkanFailure);
                        }
                    } else {
                        warn!("The scene is not initialized correctly...")
                    }
                } else {
                    warn!("The window is not initialized correctly...")
                }
            } else {
                warn!("The pipelines are not initialized correctly...")
            }
        } else {
            warn!("The vulkan context is not initialized correctly...");
        }
        Ok(())
    }

    pub fn on_keyboard_input(
        &mut self,
        device_id: DeviceId,
        event: KeyEvent,
        is_synthetic: bool,
    ) -> Result<(), ErrorCode> {
        let delta_time = self.delta_time.as_secs_f64();
        if let Some(scene) = &mut self.scene {
            if let Err(err) = scene.on_keyboard_input(device_id, event, is_synthetic, delta_time) {
                error!(
                    "Failed to handle keyboard input event in the scene: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        } else {
            warn!("The vulkan scene is not initialized correctly...");
        }
        Ok(())
    }

    pub fn on_mouse_moved(
        &mut self,
        device_id: DeviceId,
        new_position: LogicalPosition<f64>,
    ) -> Result<(), ErrorCode> {
        let delta_time = self.delta_time.as_secs_f64();
        if let Some(scene) = &mut self.scene {
            if let Err(err) = scene.on_mouse_moved(device_id, new_position, delta_time) {
                error!(
                    "Failed to handle mouse moved event in the scene: {:?}",
                    err
                );
                return Err(ErrorCode::Unknown);
            }
        } else {
            warn!("The vulkan scene is not initialized correctly...");
        }
        Ok(())
    }
}
