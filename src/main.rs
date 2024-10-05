use std::fs::File;

use application::Application;

pub mod application;

fn main() {
    let target = Box::new(File::create("output.log").expect("Failed to create the log file"));
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(target))
        .init();

    // run the app
    if let Err(err) = Application::run() {
        panic!("Failed to run the application: {:?}", err);
    }
}
