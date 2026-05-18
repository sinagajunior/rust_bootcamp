use std::io::{self,Write};




fn main() {
    println!("🛠️ String manipulation tool");

    loop {

        println!("\nChoose an operation:");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
       println!("4. Trim");
       println!("5. FFind a substring");
       println!("6. Replace Text");
       println!("7. Exit");


    let  choice = prompt("Enter your choice: ");
 
      match choice.trim() {
        "1" => {
            let s = prompt("Enter a string");
            println!("Reversed: {}", s.chars().rev().collect::<String>());
        }
        "2"=> {
            let s= prompt("Enter a string");
            println!("Uppercase: {}", s.to_uppercase());
        }
        "3" => {
            let s= prompt("Enter a string");
            println!("Lowercase: {}", s.to_lowercase());
        }
        "4" => {
            let s= prompt("Enter a string");
            println!("Trimmed: '{}'", s.trim());    
      }   
         "5" => {
            let s= prompt("Enter a string");
            let sub = prompt("Enter substring to find");
            if s.contains(&sub) {
                println!("'{}' found in '{}'", sub, s);
            } else {
                println!("'{}' not found in '{}'", sub, s);
            }
         }
         "6" => {
            let s= prompt("Enter a string");
            let from = prompt("Enter text to replace");
            let to = prompt("Enter replacement text");
            println!("Result: {}", s.replace(&from, &to));
         }
            "7" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        
        }
    }
       
}

fn prompt(message: &str)-> String {
    print!("{}", message);
   io::stdout().flush().unwrap();
   let mut input = String::new();
   io::stdin().read_line(&mut input).expect("Failed to read input");
   input.trim().to_string()
}
