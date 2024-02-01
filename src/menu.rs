use std::path::Path;
use crate::api;
use reqwest;

pub fn menu() {
    println!("Welcome to WaniCLI!\n");

    match api::check_api_key() {
        Ok(_) => println!("API key found!"),
        Err(_) => panic!("Please place a valid Wanikani APi key in the same diectory as the executable!"),
    }

    let mut i = 1;

    loop {
        print!("{}\r", i);
        i = i + 1
    }
}


