use log::{LevelFilter};
use log4rs::config::runtime::ConfigBuilder;
use log4rs::config::{Appender, Root};
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
// use log4rs_web::Log4rsAppender;


pub fn init_logging() {
    // Define the pattern for log output
    let encoder = PatternEncoder::new("{d} - {l} - {m}{n}");

    // Create a console appender
    let console_appender = ConsoleAppender::builder()
        .encoder(Box::new(encoder))
        .build();

    // Create a log4rs configuration
    let config = ConfigBuilder::default()
        .appender(Appender::builder().build("console", Box::new(console_appender)))
        // .appender(Appender::builder().build("log4rs", Box::new(log4rs_appender)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))
        .unwrap();

    // Initialize log4rs with the configuration
    log4rs::init_config(config).unwrap();
}
