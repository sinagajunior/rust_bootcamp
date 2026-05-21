use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::task::Context;

fn main() {
    println!("Config File Parser (Key=Value Formar ) ");
    let file_path = prompt("Enter the path to the config file: eg. config.txt ");

    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let config = parse_config(&content);
            println!("Parsed config");
            for (key, value) in config {
                println!("{} = {}", key, value);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

/// parses Key  = value pairs from text
fn parse_config(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue; // skip commment and empty lines
        }
        if let Some((key, value)) = line.split_once('=') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    map
}

fn prompt(message: &str) -> String {
    println!("{}", message);
    io::stdout().flush().expect("error when doing flush");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error when reading input");
    input.trim().to_string()
}
