use crate::models;
use reqwest::{blocking, header::{HeaderMap, USER_AGENT}};

pub fn run(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "ma-alves".parse().unwrap());
    let client = blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let url = format!("https://api.github.com/users/{username}/events");
    let response = client.get(&url)
        .send();

    match response {
        Ok(response) => {
            let body = response.text();
            let events = serde_json::from_str::<Vec<models::Event>>(&body.unwrap());

            match events {
                Ok(events) => {
                    if events.is_empty() {
                        println!("\nNenhum evento recente encontrado :(");
                        return Ok(());
                    }

                    for event in events {
                        println!("{}", match_event(event));
                    }
                }
                Err(_) => {
                    println!("\nUsuário {username} não encontrado :(")
                    // println!("{}", Box::new(e));
                }
            }
            Ok(())
        }
        Err(e) => {
            Err(Box::new(e))
        }
    }
}

pub fn match_event(event: models::Event) -> String {
    match event.type_.as_str() {
        "CreateEvent" => {
            format!("\n[{}] {} criou o repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "DeleteEvent" => {
            format!("\n[{}] {} deletou o repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "ForkEvent" => {
            format!("\n[{}] {} deu um fork no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "IssueCommentEvent" => {
            format!("\n[{}] {} comentou em uma issue no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "IssuesEvent" => {
            format!("\n[{}] {} abriu uma issue no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "PullRequestEvent" => {
            format!("\n[{}] {} abriu um pull request no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "PullRequestReviewCommentEvent" => {
            format!("\n[{}] {} comentou em um pull request no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "PushEvent" => {
            format!("\n[{}] {} fez um push no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        "WatchEvent" => {
            format!("\n[{}] {} salvou o repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
        _ => {
            format!("\n[{}] {} fez algo (?!) no repositório \"{}\" em {}", event.id, event.actor.login, event.repo.name, event.created_at)
        }
    }
}