use std::io::{self, Write};
use std::num::ParseFloatError;

fn main() {
    println!("Error Handling Calculator");

    loop {
        println!("\n 1. Add | 2. Divide | 3. Exit");

        let choice = input("Choose an option: ");

        match choice.as_str() {
            "1" => match parse_two_numbers() {
                Ok((a, b)) => println!("Result: {} + {}= {} ", a, b, a + b),
                Err(e) => eprintln!("Error {}", e),
            },
            "2" => match parse_two_numbers() {
                Ok((a, b)) => match divide(a, b) {
                    Ok(result) => println!("Result: {}/{}= {}", a, b, result),
                    Err(e) => eprintln!("Error {}", e),
                },
                Err(e) => eprintln!("Error {}", e),
            },
            "3" => {
                println!("Exiting");
                break;
            }
            _ => println!("Ivalid choice"),
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error when trying read line");
    input.trim().to_string()
}

// parse two number with error handling
fn parse_two_numbers() -> Result<(f64, f64), ParseFloatError> {
    let a = input("Enter first number:").parse::<f64>()?;
    let b = input("Enter second number:").parse::<f64>()?;
    Ok((a, b))
}

// division with custom error handling
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
