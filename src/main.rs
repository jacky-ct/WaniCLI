mod menu;
use reqwest::blocking::Client;
use serde_json::{Result, Value};

fn main() {
    let client = Client::new();

    let request = client
        .get("https://api.wanikani.com/v2/user")
        .bearer_auth("b320cff7-a178-4aa1-96b4-987347b41a21")
        .send()
        .expect("GET failed")
        .text()
        .unwrap();
    


    let v: Value = serde_json::from_str(&request)
        .expect("Invalid json sent by API");

    println!("{}", v["url"]);
    menu::menu();
}
