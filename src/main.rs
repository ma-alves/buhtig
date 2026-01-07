mod client;
mod models;

use clap::{Parser, Subcommand};
use crate::client::run;

// Clap setup
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Events { 
        #[arg(short, long)]
        username: String 
    },
    Repos { 
        #[arg(short, long)]
        owner: String,
        #[arg(short, long)]
        repo: String 
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Events { username }) => {
            println!("chamando {} no events endpoint", username);
            let _ = run(username);
        }
        _ => {
            println!("sem argumentos :(")
        }
    }

    Ok(())
}
