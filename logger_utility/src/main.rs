use chrono::Local;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::panic::PanicHookInfo;

fn main() {
    println!("Logger utility (write to log.txt)");

    loop {
        println!("\n Log Levels: 1. INFO |2. WARN |3. ERROR | 4. Exit");
        let choice = input("Select a level :");

        match choice.as_str() {
            "1" => log_message("INFO"),
            "2" => log_message("WARN"),
            "3" => log_message("ERROR"),
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please select a valid log level."),
        }
    }
}

fn log_message(level: &str) {
    let message = input(&format!("Enter {} message:", level));
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let entry = format!("{} [{}]: {}\n", timestamp, level, message);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("error when trying to open log file");
    file.write_all(entry.as_bytes()).expect("Write failed");
    println!("Logged successfully.");
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("error when trying to flush");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("error when trying to read");
    buf.trim().to_string()
}
