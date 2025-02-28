mod client;

use clap::Parser;
use crate::client::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    println!("User: {:?}", args.username);

    run(&args.username).expect("omg im sorry");

    Ok(())
}

// Clap Code
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    pub username: String
}
