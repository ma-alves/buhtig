use std::env;
use dotenv::dotenv;
use crate::models;
use reqwest::{blocking, header::{HeaderMap, USER_AGENT, AUTHORIZATION, ACCEPT}};

fn get_client() -> blocking::Client {
    dotenv().ok();
    let github_token = env::var("GITHUB_TOKEN").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());
    headers.insert(USER_AGENT, "ma-alves".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", github_token)
        .parse()
        .unwrap());

    let client = blocking::Client::builder()
        .default_headers(headers)
        .build().unwrap();

    client
}

pub fn get_user_events(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{username}/events");
    let client = get_client();
    let response = client.get(&url).send()?;

    match response.status() {
        reqwest::StatusCode::OK | reqwest::StatusCode::NOT_MODIFIED => {
            let body = response.text();
            let events = serde_json::from_str::<Vec<models::Event>>(&body.unwrap());
            
            match events {
                Ok(events) => {
                    if events.is_empty() {
                        println!("\nNenhum evento recente encontrado :(");
                        return Ok(());
                    }
                    for event in events {
                        let fmt_event = format!("\nID: {}\nEvento: {}\nUsuário: {}\nData: {}", event.id, event.type_, event.actor.login, event.created_at);
                        println!("{}", fmt_event);
                    }
                }
                Err(e) => {
                    eprintln!("{}", Box::new(e));
                }
            }
        }
        _ => {
            return Err(format!("{}", response.status()).into());
        }
    }
    Ok(())
}

pub fn get_repo_events(owner: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/events");
    let client = get_client();
    let response = client.get(&url).send()?;

    match response.status() {
        reqwest::StatusCode::OK | reqwest::StatusCode::NOT_MODIFIED => {
            let body = response.text();
            let events = serde_json::from_str::<Vec<models::Event>>(&body.unwrap());
            
            match events {
                Ok(events) => {
                    if events.is_empty() {
                        println!("\nNenhum evento recente encontrado.");
                        return Ok(());
                    }
                    for event in events {
                        let fmt_event = format!("\nID: {}\nEvento: {}\nUsuário: {}\nData: {}", event.id, event.type_, event.actor.login, event.created_at);
                        println!("{}", fmt_event);
                    }
                }
                Err(e) => {
                    eprintln!("{}", Box::new(e));
                }
            }
        }
        _ => {
            return Err(format!("{}", response.status()).into());
        }
    }
    Ok(())
}