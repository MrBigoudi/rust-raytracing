use log::warn;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use super::Application;

impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Err(err) = self.init(event_loop) {
                panic!("Failed to initialize the winit window: {:?}", err);
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let Err(err) = self.on_exit() {
            panic!("Failed to handle exiting event: {:?}", err);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        } else {
            warn!("The window is not initialized correctly...");
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        if let Err(err) = self.update() {
            panic!("Failed to update the application: {:?}", err);
        }

        // Handle event for the gui
        if self.vulkan_context.is_some() {
            let vulkan_context = self.vulkan_context.as_mut().unwrap();
            if let Err(err) = vulkan_context.on_new_event_gui(self.delta_time) {
                panic!("Failed to handle gui new events: {:?}", err);
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        // Handle event for the gui
        if self.vulkan_context.is_some() {
            let vulkan_context = self.vulkan_context.as_mut().unwrap();
            if let Err(err) = vulkan_context.on_device_event_gui(&event) {
                panic!("Failed to handle gui device event: {:?}", err);
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Handle event for the gui
        if self.window.is_some() && self.vulkan_context.is_some() {
            let window = self.window.as_ref().unwrap();
            let vulkan_context = self.vulkan_context.as_mut().unwrap();
            if let Err(err) = vulkan_context.on_window_event_gui(window, &event) {
                panic!("Failed to handle gui window event: {:?}", err);
            }
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_physical_size) => {
                if let Err(err) = self.on_resize(new_physical_size) {
                    panic!("Failed to handle resizing event: {:?}", err);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.on_redraw() {
                    panic!("Failed to handle redrawing event: {:?}", err);
                }
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                if let Err(err) = self.on_keyboard_input(device_id, event, is_synthetic) {
                    panic!("Failed to handle keyboard input event: {:?}", err);
                }
            }
            WindowEvent::CursorMoved {
                position,
                device_id,
            } => {
                if let Some(window) = &self.window {
                    let new_position = position.to_logical(window.scale_factor());
                    if let Err(err) = self.on_mouse_moved(device_id, new_position) {
                        panic!("Failed to handle mouse moved event: {:?}", err);
                    }
                } else {
                    warn!("The window is not initialized correctly...");
                }
            }
            _ => (),
        }
    }
}
