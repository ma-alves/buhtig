// Tratar erros no status_code, ver doc da api do github
// Trocar tipos em User para Result ou Option pois a api retorna null
// Mudar retorno de main pq né pqp
// TALVEZ usar async

use clap::Parser;
use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use reqwest::{blocking, header::{self, HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT}};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("Username: {:?}", args.username);

    run(&args.username)?;

    Ok(())
}

// Clap Code
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    pub username: String
}

// Serde Code
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    login: String,
    name: String,
    location: String,
    bio: String
}

// Reqwest Code
fn run(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let empty_headers = header::HeaderMap::new();
    let headers = set_headers(empty_headers);

    println!("{:?}", headers);
    
    let url = format!("https://api.github.com/users/{username}");
    
    let client = blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let response: User = client.get(url)
        .send()?
        .json()?;
    
    println!("{:#?}", response);
    
    Ok(())
}

// Utils
fn set_headers(mut header: HeaderMap) -> HeaderMap {
    dotenv().ok();
    let token = std::env::var("GITHUB_API_TOKEN").expect("GITHUB_API_TOKEN must be set");

    header.insert(USER_AGENT, "ma-alves".parse().unwrap());
    header.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());
    header.insert(AUTHORIZATION, token.parse().unwrap());
    header.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    header
}