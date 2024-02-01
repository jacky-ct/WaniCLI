use reqwest;
use reqwest::blocking::Client;
use std::path::Path;
use std::fs::read_to_string;
use std::io;
use std::error::Error;

pub fn check_api_key() -> Result<String, Box<dyn Error>> {
    println!("{}", std::env::current_dir().unwrap().display());
    let username = read_to_string("api_key.txt")?;

    Client::new()
        .get("https://api.wanikani.com/v2/review_statistics")
        .bearer_auth("b320cff7-a178-4aa1-96b4-987347b41a21")
        .send()?;
    

    Ok(username)
}
