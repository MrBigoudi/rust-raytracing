use std::fs::File;
use std::io::Write;

use application::Application;

pub mod application;

fn main() {
    // Create the output logging file
    let target = Box::new(File::create("output.log").expect("Failed to create the log file"));
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            // Split the message by newlines and write each line separately with a prefix
            let message = format!("{}", record.args()); // Retrieve message as raw string
            let lines: Vec<_> = message.split('\n').collect();

            for (i, line) in lines.into_iter().enumerate() {
                // Format the first line with metadata
                if i == 0 {
                    writeln!(
                        buf,
                        "{}:{} [{}] - {}",
                        record.file().unwrap_or("unknown"),
                        record.line().unwrap_or(0),
                        record.level(),
                        line
                    )?;
                // Process each remaining line in `lines`
                } else {
                    writeln!(buf, "    {}", line)?; // Indent continuation lines for clarity
                }
            }
            Ok(())
        })
        .target(env_logger::Target::Pipe(target))
        .init();

    // Run the app
    if let Err(err) = Application::run() {
        panic!("Failed to run the application: {:?}", err);
    }
}
