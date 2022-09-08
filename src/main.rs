use clap::Parser;
use cli_utils::cli::Cli;
use config::config_options::ConfigOptions;
use stopwatch::Stopwatch;
use std::fs;

use crate::cli_utils::error_codes;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub mod cli_utils;
pub mod macros;
pub mod config;

#[tokio::main]
async fn main() {
    let stopwatch = Stopwatch::start_new();

    let cli: Cli = Cli::parse();

    pretty_env_logger::formatted_timed_builder()
        .filter_level(cli.verbosity.log_level_filter())
        .init();

    let source_path = cli.source_file.as_path();

    if !source_path.exists() || !source_path.is_file() {
        crash!(format!("Cannot find source file {}", source_path.as_os_str().to_string_lossy()), error_codes::ERROR_SOURCE_FILE_NOT_FOUND);
    }

    info!("Source file location validated !");


    let target_path = cli.target_file.as_path();

    if !target_path.exists() || !target_path.is_file() {
        crash!(format!("Cannot find target file {}", target_path.as_os_str().to_string_lossy()), error_codes::ERROR_SOURCE_FILE_NOT_FOUND);
    }

    info!("Target file location validated !");

    let config: ConfigOptions = match cli.config_file {
        Some(pathbuff) => {
            let config_path = pathbuff.as_path();

            if !config_path.exists() || !config_path.is_file() {
                crash!(format!("Cannot find config file {}", config_path.as_os_str().to_string_lossy()), error_codes::ERROR_CONFIG_FILE_NOT_FOUND);
            }

            let temp_json_config = fs::read_to_string(config_path).unwrap();
            let json_config = temp_json_config.as_str();

            let config: ConfigOptions = serde_json::from_str(json_config).unwrap();

            config
        },
        None => ConfigOptions::default(),
    };

    info!("Configuration loaded: {:?}", config);



    info!("{}", format!("Elapsed: {}ms", stopwatch.elapsed_ms()));
}