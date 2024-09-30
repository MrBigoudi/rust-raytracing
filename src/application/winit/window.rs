use log::error;
use winit::{
    dpi::{LogicalSize, PhysicalSize, Position},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::application::{core::error::ErrorCode, parameters::ApplicationParameters, Application};

impl Application {
    /// Initializes the window with given application parameters and event loop
    pub fn init_window(
        parameters: &ApplicationParameters,
        event_loop: &EventLoop<()>,
    ) -> Result<Window, ErrorCode> {
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

        match WindowBuilder::new()
            .with_title(&parameters.window_title)
            .with_resizable(false)
            .with_inner_size(window_size)
            .with_position(position)
            .build(event_loop)
        {
            Ok(window) => Ok(window),
            Err(err) => {
                error!("Failed to initialize the window: {:?}", err);
                Err(ErrorCode::InitializationFailure)
            }
        }
    }
}
