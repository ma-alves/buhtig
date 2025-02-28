use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use reqwest::{blocking, header::{self, HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT}};

pub fn run(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let empty_headers = header::HeaderMap::new();
    let headers = set_headers(empty_headers);
    
    let url = format!("https://api.github.com/users/{username}");
    
    let client = blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let response: User = client.get(url)
        .send()?
        .json().expect("Response body is not in json format.");
    
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

// Serde Code
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    login: String,
    name: String,
    location: String,
    bio: String
}