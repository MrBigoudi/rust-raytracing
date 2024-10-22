use log::error;
use winit::{
    dpi::{LogicalSize, PhysicalSize, Position}, event_loop::ActiveEventLoop, window::Window
};

use crate::application::{core::error::ErrorCode, parameters::ApplicationParameters};

pub struct WindowContext;

impl WindowContext{
    pub fn init(parameters: &ApplicationParameters, event_loop: &ActiveEventLoop) -> Result<Window, ErrorCode> {
        let primary_monitor = event_loop.primary_monitor().unwrap();
        let scale_factor = primary_monitor.scale_factor();

        // Desired window size in logical units
        let logical_width = f64::from(parameters.window_width) / scale_factor;
        let logical_height = f64::from(parameters.window_height) / scale_factor;
        let window_size = LogicalSize::new(logical_width, logical_height);

        // Monitor size in physical pixels
        let monitor_size: PhysicalSize<u32> = primary_monitor.size();

        // Calculate the top-left position to center the window
        let monitor_width = monitor_size.width as f64;
        let monitor_height = monitor_size.height as f64;
        let pos_x = (monitor_width - logical_width * scale_factor) / 2.0;
        let pos_y = (monitor_height - logical_height * scale_factor) / 2.0;
        let position = Position::new(Position::Physical((pos_x, pos_y).into()));

        let window_attributes = Window::default_attributes()
            .with_title(&parameters.window_title)
            .with_position(position)
            .with_inner_size(window_size)
        ;

        match event_loop.create_window(window_attributes){
            Ok(window) => Ok(window),
            Err(err) => {
                error!("Failed to create a winit window: {:?}", err);
                Err(ErrorCode::InitializationFailure)
            }
        }
    }
}