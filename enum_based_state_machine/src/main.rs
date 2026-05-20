use std::io::{self, Write};

fn main() {
    println!("State Machine Signup Wizard");

    let mut state = State::Start;
    loop {
        match state {
            State::Start => {
                println!("Welcome! Let's begin your sign up");
                state = State::EnterName;
            }
            State::EnterName => {
                let name = input("Enter your name:");
                if name.is_empty() {
                    println!("Name cannot be empty")
                } else {
                    state = State::EnterEmail(name);
                }
            }
            State::EnterEmail(ref name) => {
                let email = input("Enter your email:");
                if email.contains("@") {
                    state = State::Confirm {
                        name: name.to_string(),
                        email,
                    };
                } else {
                    println!("Invalid email format");
                }
            }
            State::Confirm { name, email } => {
                println!("Confirm your info:");
                println!("Name: {}", name);
                println!("email: {}", email);
                let confirm = input("Is this correct ? (Yes/No): ");
                state = match confirm.as_str() {
                    "Yes" => State::Complete,
                    "No" => State::EnterName,
                    _ => {
                        println!("Invalid input");
                        State::Confirm { name, email }
                    }
                };
            }

            State::Complete => {
                println!("Signup complete!");
                break;
            }
        }
    }
}

/// State Enum for signup Process
enum State {
    Start,
    EnterName,
    EnterEmail(String),
    Confirm { name: String, email: String },
    Complete,
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().expect("error when trying to flush");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("error when trying read line");
    buf.trim().to_string()
}
