use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};

const STORE_FILE: &str = "kv_store.json";

#[derive(Serialize, Deserialize)]
struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn new() -> Self {
        if let Ok(file) = File::open(STORE_FILE) {
            let reader = BufReader::new(file);
            if let Ok(store) = serde_json::from_reader(reader) {
                return store;
            }
        }
        Store {
            data: HashMap::new(),
        }
    }

    fn save(&self) {
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(STORE_FILE)
        {
            serde_json::to_writer_pretty(&mut file, self).unwrap();
        }
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        self.save();
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save();
    }

    fn list(&self) {
        println!("Stored Key");
        for key in self.data.keys() {
            println!("{}", key);
        }
    }
}

fn main() {
    println!("Key-Value Store (set/get/delete/list/exit)");
    let mut store = Store::new();
    loop {
        let cmd = prompt("\n>");
        let parts: Vec<&str> = cmd.trim().splitn(3,' ').collect();

        match parts.get(0).map(|s| s.to_lowercase()).as_deref() {
            Some("set") if parts.len() == 3 => {
                store.set(parts[1].to_string(), parts[2].to_string());
                println!("Set: {} = {}", parts[1], parts[2]);
            }
            Some("get") if parts.len() == 2 => {
                match store.get(parts[1]) {
                    Some(value) => println!("{} = {}", parts[1], value),
                    None => println!("Key not found: {}", parts[1]),
                }
            }
            Some("delete") if parts.len() == 2 => {
                store.delete(parts[1]);
                println!("Deleted: {}", parts[1]);
            }
            Some("list") => {
                store.list();
            }
            Some("exit") => {
                println!("Exiting...");
                break;
            }

            _ => println!("Invalid command: {} use set/get/delete/list/exit", cmd),
        }
    }
}

fn prompt(msg: &str) -> String {
    println!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
