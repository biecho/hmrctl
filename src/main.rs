use std::path::Path;

use clap::{App, Arg, SubCommand};
use log::{error, info};

use crate::config::loader::load_config;

mod config;


// Constants for default and file names
const DEFAULT_CONFIG_DIR: &str = "config";
const LOG_CONFIG_FILE: &str = "log4rs.yml";
const MAIN_CONFIG_FILE: &str = "config.yml";

fn main() {
    let matches = parse_args();

    if let Some(run_matches) = matches.subcommand_matches("run") {
        handle_run_subcommand(run_matches);
    } else {
        eprintln!("No subcommand was used or subcommand was not recognized");
        std::process::exit(1);
    }
}

/// Parses the command line arguments using clap.
fn parse_args() -> clap::ArgMatches<'static> {
    App::new("hmrctl")
        .version("1.0")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs the main functionality")
                .arg(
                    Arg::with_name("config-dir")
                        .long("config-dir")
                        .value_name("DIRECTORY")
                        .help(&format!("Sets the configuration directory. Defaults to '{}'", DEFAULT_CONFIG_DIR))
                        .takes_value(true),
                ),
        )
        .get_matches()
}

/// Handles the "run" subcommand and associated actions.
fn handle_run_subcommand(matches: &clap::ArgMatches<'_>) {
    let config_dir = Path::new(matches.value_of("config-dir").unwrap_or(DEFAULT_CONFIG_DIR));

    initialize_logger(&config_dir);

    if let Err(e) = load_and_print_config(&config_dir) {
        error!("Failed to load configuration: {}", e);
        std::process::exit(1);
    }
}

/// Initializes the logger using the log4rs configuration from the given directory.
fn initialize_logger(config_dir: &Path) {
    let log_config_path = config_dir.join(LOG_CONFIG_FILE);

    if let Err(e) = log4rs::init_file(&log_config_path, Default::default()) {
        eprintln!("Error initializing logger: {}", e);
        std::process::exit(1);
    }
}

/// Loads the main configuration file from the given directory and logs its contents.
fn load_and_print_config(config_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = config_dir.join(MAIN_CONFIG_FILE);

    let config = load_config(&config_path)?;
    info!("Config:\n{}", config);
    Ok(())
}

// ... [rest of your code, like the load_config function definition, etc.]
