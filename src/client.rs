// use crate::models;
use reqwest::{blocking, header::{HeaderMap, USER_AGENT}};


pub fn run(username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    
    headers.insert(USER_AGENT, "ma-alves".parse().unwrap());
    
    let url = format!("https://api.github.com/users/{username}/events");
    let client = blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    let response = client.get(&url)
        .send();
    // let response: models::EventType = client.get(url)
    //     .send()?
    //     .json().expect("Response body is not in json format.");
    let response_result = client.get(&url)
        .send()?
        .json()?;
   
    println!("{:#?}", response);
    
    Ok(())
}
