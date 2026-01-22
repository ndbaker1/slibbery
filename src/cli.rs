use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "dyll")]
#[command(about = "Generate stub libraries for dynamic libraries", long_about = None)]
pub struct Cli {
    /// Output directory for the generated stub library
    #[arg(short = 'o', long)]
    pub output_dir: PathBuf,

    /// Path to the input .so library
    #[arg(short = 'l', long = "lib-path")]
    pub lib_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate stubs from a C header file
    Header(HeaderArgs),
}

#[derive(clap::Args)]
pub struct HeaderArgs {
    /// Path to the C header file
    pub header_file: PathBuf,
}
