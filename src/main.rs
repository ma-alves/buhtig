mod client;
mod models;

use clap::{Parser, Subcommand};
use crate::client::{get_repo_events, get_user_events};

#[derive(Debug, Parser)]
#[command(name="buhtig")]
#[command(version, about = "CLI client for GitHub user and repo events", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    User {
        username: String 
    },
    #[command(arg_required_else_help = true)]
    Repo { 
        owner: String,
        repo: String
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::User { username }) => {
            println!("Chamando {} no user events endpoint", username);
            let _ = get_user_events(username)?;
        }
        Some(Commands::Repo { owner, repo }) => {
            println!("Chamando {}/{} no repo events endpoint", owner, repo);
            let _ = get_repo_events(owner, repo)?;
        }
        _ => {
            println!("sem argumentos :(")
        }
    }

    Ok(())
}