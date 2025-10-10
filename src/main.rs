mod client;
mod models;

use clap::Parser;
use crate::client::run;

// Clap setup
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    pub username: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    run(&args.username)?;

    Ok(())
}
