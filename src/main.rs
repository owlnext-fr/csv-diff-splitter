use clap::Parser;
use cli_utils::cli::Cli;
use cli_utils::validators;
use config::config_options::ConfigOptions;
use std::{env, fs, path::PathBuf};
use stopwatch::Stopwatch;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod cli_utils;
pub mod config;
pub mod csv_utils;
pub mod macros;
pub mod middleware;

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
    let source_path = cli.source_file;
    validators::validate_source_file(source_path.clone().as_path());

    // handling target path
    let target_path = cli.target_file;
    validators::validate_target_file(target_path.clone().as_path());

    // handling configuration
    let config: ConfigOptions = match cli.config_file {
        // case: a config file is passed with '--config-file'
        Some(pathbuff) => {
            let config_path = pathbuff.as_path();

            validators::validate_config_file(config_path);

            let temp_json_config = fs::read_to_string(config_path).unwrap();

            serde_json::from_str(temp_json_config.as_str()).unwrap()
        }
        // case: no config file is passed with '--config-file'
        None => ConfigOptions::default(),
    };

    // generting or getting output path
    let output_path: PathBuf = match cli.output_path.clone() {
        Some(pathbuff) => {
            validators::validate_output_path(pathbuff.as_path());
            pathbuff
        }
        None => {
            let current_path_buffer: PathBuf = env::current_dir().unwrap();
            current_path_buffer
        }
    };

    // debug info, use -vvv to get it.
    info!("Configuration loaded: !");
    debug!("Identifier index: {:?}", config.id_index);
    debug!("Update marker indexes: {:?}", config.update_markers);
    debug!("Print marker indexes: {:?}", config.print_markers);
    debug!("Separator: {:?}", config.separator);
    debug!("has headers: {:?}", config.has_headers);
    debug!(
        "Output path: {:}",
        output_path.as_os_str().to_string_lossy()
    );

    // main program.
    middleware::middleware::process(source_path, target_path, config, output_path);

    // setting time elapsed since main start
    info!("{}", format!("Elapsed: {}ms", stopwatch.elapsed_ms()));
}
