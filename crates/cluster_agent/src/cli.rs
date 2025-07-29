use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, value_name = "CONFIG_FILE")]
    pub config: Option<PathBuf>,
    #[arg(short, long, default_value = "[::]:50051")]
    pub address: SocketAddr,
    #[arg(short, long)]
    pub params: Vec<String>,
}
