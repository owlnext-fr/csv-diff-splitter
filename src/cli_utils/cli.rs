use std::path::PathBuf;

use clap::Parser;
use clap_verbosity_flag::{Verbosity, ErrorLevel};

#[derive(Parser)]
#[clap(name = "csv-diff-splitter")]
#[clap(author = "Adrien G. for Owlnext <contact@owlnext.fr>")]
#[clap(version = "0.1.0")]
#[clap(about = "Split CSV files using comparison algorithms.", long_about = None)]
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