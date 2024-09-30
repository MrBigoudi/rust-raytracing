use application::Application;
use log::error;

pub mod application;

fn main() {
    env_logger::init();
    // run the app
    if let Err(err) = Application::run() {
        error!("Failed to run the application: {:?}", err);
        panic!()
    }
}
