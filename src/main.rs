use std::fs::File;

use application::Application;

pub mod application;

fn main() {
    // Create the output logging file
    let target = Box::new(File::create("output.log").expect("Failed to create the log file"));
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(target))
        .init();

    // Run the app
    if let Err(err) = Application::run() {
        panic!("Failed to run the application: {:?}", err);
    }
}
