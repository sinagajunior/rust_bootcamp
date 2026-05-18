use std::io::{self,Write};


#[derive(Debug)]
struct Contact{
    id: usize,
    name: String,
    phone: String,
    email: String,
}

fn main() {
    println!("📒\n Contact manager");
    printlpn!("1. Add contact");
    println!("2. View contacts");
    println!("3. Search contacts");
    println!("4. Exit");

    let choice = input("Enter your choice: ");
    match coice.trim() {
        "1" => add_contact(),
        "2" => view_contacts(),
        "3" => search_contacts(),
        "4" => println!("Goodbye!"),
        _ => println!("Invalid choice, please try again."),
    }

}


   fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Failed to read input");
    buff.trim().to_string() 
   }
