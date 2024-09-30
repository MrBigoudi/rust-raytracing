use log::error;
use winit::event_loop::EventLoop;

use crate::application::{core::error::ErrorCode, Application};

impl Application {
    /// Initializes the event loop for handling window events
    pub fn init_event_loop() -> Result<EventLoop<()>, ErrorCode> {
        match EventLoop::new() {
            Ok(event_loop) => Ok(event_loop),
            Err(err) => {
                error!("failed to create the event loop: {:?}", err);
                Err(ErrorCode::InitializationFailure)
            }
        }
    }
}
