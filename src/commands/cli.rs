use clap::Parser;
use std::path::PathBuf;

pub const fn default_max_len() -> usize { 500 }
pub const fn default_port() -> u16 { 3030 }
pub const fn default_host() -> &'static str { "localhost" }

/// CLI example
#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    #[arg(long,short)]
    pub path: Option<PathBuf>,
    #[arg(long,short, default_value_t=default_max_len())]
    pub max_len: usize,
    #[arg(long, default_value_t=default_host().to_string())]
    pub host: String,
    #[arg(long, default_value_t=default_port())]
    pub port: u16,


}

