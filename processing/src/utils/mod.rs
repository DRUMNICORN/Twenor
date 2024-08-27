mod utils_correlation;
// mod utils_features;
mod utils_open_ai;
mod utils_python;
mod utils_rendering;
mod utils_chunking;
pub use utils_chunking::*;

use log4rs::append::file::FileAppender;
pub use utils_correlation::*;
// pub use utils_features::*;
pub use utils_open_ai::*;
pub use utils_python::*;
pub use utils_rendering::*;

use log::LevelFilter;
use log4rs::config::runtime::ConfigBuilder;
use log4rs::config::{Appender, Root};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
// use log4rs_web::Log4rsAppender;

pub fn init_logging() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Define the pattern for log output
    let encoder = PatternEncoder::new("{d} - {l} - {m}{n}");

    // Create a console appender
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .build();

    // Delete the log file if it already exists
    if std::path::Path::new("processing.log").exists() {
        std::fs::remove_file("processing.log")?;
    }

    // Create a file appender
    let file_appender = FileAppender::builder()
        .encoder(Box::new(encoder))
        .build("processing.log")?;

    // Create a log4rs configuration
    let config = match ConfigBuilder::default()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("console").appender("file").build(LevelFilter::Info))
        {
            Ok(config) => config,
            Err(e) => {
                log::error!("Error creating log4rs configuration: {}", e);
                return Err(Box::new(e));
            }
        };

    // Initialize log4rs with the configuration
    match log4rs::init_config(config){
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Error initializing log4rs: {}", e);
            Err(Box::new(e))
        }
    }
}
