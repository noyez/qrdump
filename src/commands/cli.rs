use clap::Parser;
use std::path::PathBuf;

pub const fn default_max_len() -> usize { 500 }

/// CLI example
#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(long,short)]
    pub path: Option<PathBuf>,
    #[arg(long,short, default_value_t=default_max_len())]
    pub max_len: usize


}

