//! Binary for Server.
use env_logger::Builder as LoggerBuilder;
use log::LevelFilter;
use server::{start, Config};

/// Simple shell around starting the server.
fn main() {
    LoggerBuilder::new().filter_level(LevelFilter::Info).init(); // TODO consider making logging level configurable at the CLI level.
    let config: Config = Config::parse_command_line_arguments();
    start(config)
}
