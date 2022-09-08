use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, ErrorLevel};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Source file to analyze.
    #[clap(value_parser)]
    pub source_file: PathBuf,
    /// Target file to analyze.
    #[clap(value_parser)]
    pub target_file: PathBuf,
    /// adds a configuration file to process source and target files.
    #[clap(value_parser, short, long)]
    pub config_file: Option<PathBuf>,
    /// Verbosity level
    #[clap(flatten)]
    pub verbosity: Verbosity<ErrorLevel>,
}