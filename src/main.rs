use clap::{App, Arg, SubCommand};
use log::{error, info};

use crate::config::loader::load_config;

mod config;
mod dram;

fn main() {
    let matches = parse_args();

    if let Some(run_matches) = matches.subcommand_matches("run") {
        handle_run_subcommand(run_matches);
    } else {
        eprintln!("No subcommand was used or subcommand was not recognized");
        std::process::exit(1);
    }
}

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
                        .help("Sets the configuration directory. Defaults to 'config'")
                        .takes_value(true),
                ),
        )
        .get_matches()
}

fn handle_run_subcommand(matches: &clap::ArgMatches<'_>) {
    let config_dir = matches.value_of("config-dir").unwrap_or("config");

    initialize_logger(&config_dir);

    if let Err(e) = load_and_print_config(&config_dir) {
        error!("Failed to load configuration: {}", e);
        std::process::exit(1);
    }
}

fn initialize_logger(config_dir: &str) {
    if let Err(e) = log4rs::init_file(&format!("{}/log4rs.yml", config_dir), Default::default()) {
        eprintln!("Error initializing logger: {}", e);
        std::process::exit(1);
    }
}

fn load_and_print_config(config_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(&format!("{}/config.yml", config_dir))?;
    info!("Config:\n{}", config);
    Ok(())
}
