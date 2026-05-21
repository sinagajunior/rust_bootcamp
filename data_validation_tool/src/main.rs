use fancy_regex::Regex;
use std::io::{self, Write};
use std::sync::OnceLock;

fn main() {
    println!("Data validation tool");
    loop {
        println!("\n Choose what to validate");
        println!("1. Email");
        println!("2. Phone");
        println!("3. Password");
        println!("4. Exit");
        let choice = input("Enter your choice: ");
        match choice.as_str() {
            "1" => {
                let email = input("Enter email: ");
                println!("Email is valid: {}", is_valid_email(&email));
            }
            "2" => {
                let phone = input("Enter phone: ");
                println!("Phone is valid: {}", is_valid_phone(&phone));
            }
            "3" => {
                let password = input("Enter password: ");
                println!("Password is valid: {}", is_strong_password(&password));
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn is_valid_email(email: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();

    let re = RE.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("expected format email")
    });
    re.is_match(email).unwrap_or(false)
}

fn is_valid_phone(phone: &str) -> bool {
    let re = Regex::new(r"^\+?[1-9]\d{1,14}$").expect("expected format phone");
    re.is_match(phone).unwrap_or(false)
}

fn is_strong_password(password: &str) -> bool {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[\W_]).{8,}$").expect("Invalid pattern")
    });
    re.is_match(password).unwrap_or(false)
}
