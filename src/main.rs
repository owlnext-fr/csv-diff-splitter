use clap::Parser;
use cli_utils::cli::Cli;
use config::config_options::ConfigOptions;
use stopwatch::Stopwatch;
use std::fs;
use cli_utils::validators;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub mod cli_utils;
pub mod macros;
pub mod config;

#[tokio::main]
async fn main() {
    // start stopwatch to monitor time
    let stopwatch = Stopwatch::start_new();

    // parsing cli args & options
    let cli: Cli = Cli::parse();

    // initializing logger with verbosity given by options
    pretty_env_logger::formatted_timed_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    // handling source path
    let source_path = cli.source_file.as_path();
    validators::validate_source_file(source_path);

    // handling target path
    let target_path = cli.target_file.as_path();
    validators::validate_target_file(target_path);

    // handling configuration
    let config: ConfigOptions = match cli.config_file {
        // case: a config file is passed with '--config-file'
        Some(pathbuff) => {
            let config_path = pathbuff.as_path();

            validators::validate_config_file(config_path);

            let temp_json_config = fs::read_to_string(config_path).unwrap();

            serde_json::from_str(temp_json_config.as_str()).unwrap()
        },
        // case: no config file is passed with '--config-file'
        None => ConfigOptions::default(),
    };

    info!("Configuration loaded: !");
    debug!("Identifier index: {}", config.id_index);
    debug!("Update marker indexes: {:?}", config.update_markers);
    debug!("Print marker indexes: {:?}", config.print_markers);


    // setting time elapsed since main start
    info!("{}", format!("Elapsed: {}ms", stopwatch.elapsed_ms()));
}