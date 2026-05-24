use std::io::{self, Write};

trait Greeter {
    fn greet(&self, name: &str) -> String;
}

struct Friendly;
impl Greeter for Friendly {
    fn greet(&self, name: &str) -> String {
        format!("Hey there, {} Great to see you!", name)
    }
}

struct Formal;
impl Greeter for Formal {
    fn greet(&self, name: &str) -> String {
        format!("Good Day To You, {}!", name)
    }
}

struct Sarcastic;
impl Greeter for Sarcastic {
    fn greet(&self, name: &str) -> String {
        format!("Oh Wow, {} showed up. Amazing!", name)
    }
}

fn main() {
    println!("Trait object Demo  - Pick personality");

    let mut greeter: Box<dyn Greeter> = Box::new(Friendly); // Default
    loop {
        println!("\nCHoose a mode :");
        println!("\n1.Friendly\n2.Formal\n3.Sarcastic\n4.Exit");
        let choice = input("> ");
        match choice.as_str() {
            "1" => {
                greeter = Box::new(Friendly);
                println!("Switched to a Friendly Mode");
            }
            "2" => {
                greeter = Box::new(Formal);
                println!("Switched to a Formal Mode");
            }
            "3" => {
                greeter = Box::new(Sarcastic);
                println!("Switched to a Sarcastic Mode");
            }
            "4" => {
                println!("Bye");
                break;
            }
            _ => println!("Invalid choice"),
        }
        let name = input("Enter your name: ");
        println!("{}", greeter.greet(&name));
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
