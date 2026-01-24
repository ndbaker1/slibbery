use clap::Parser;
use dyll::cli::{Cli, Commands};
use dyll::commands::header;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Header(ref args) => header::run(&cli, args),
    }
}
