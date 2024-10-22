use log::warn;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use super::Application;


impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Err(err) = self.init(event_loop){
                panic!("Failed to initialize the winit window: {:?}", err);
            }
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        if let Err(err) = self.on_exit(){
            panic!("Failed to handle exiting event: {:?}", err);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window{
            window.request_redraw();
        } else {
            warn!("The window is not initialized correctly...");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(new_physical_size) => {
                if let Err(err) = self.on_resize(new_physical_size){
                    panic!("Failed to handle resizing event: {:?}", err);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.on_redraw(){
                    panic!("Failed to handle redrawing event: {:?}", err);
                }
            }
            _ => (),
        }
    }
}