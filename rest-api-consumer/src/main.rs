use reqwest::blocking::get;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct Joke {
    id: u32,
    r#type: String,
    setup: String,
    punchline: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Random Joke Fetcher from REST API");

    let _ = prompt("Press Enter to fetch a joke");
    let url = "https://official-joke-api.appspot.com/random_joke";
    let response = get(url)?.json::<Joke>()?;
    println!("{:#?}", response);
    println!("{}", response.setup);
    println!("{}", response.punchline);

    Ok(())
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}
