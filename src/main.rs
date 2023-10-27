use log::{error, info};
use crate::config::loader::load_config;

mod config;

fn main() {
    // Initialize the logger.
    if let Err(e) = log4rs::init_file("config/log4rs.yml", Default::default()) {
        eprintln!("Error initializing logger: {}", e);
        std::process::exit(1);
    }

    match load_config("config/config.yml") {
        Ok(config) => info!("{:?}", config),
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    }
}
