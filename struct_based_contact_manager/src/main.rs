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
    println!("1. Add contact");
    println!("2. View contacts");
    println!("3. Search contacts");
    println!("4. Exit");

    let mut contacts: Vec<Contact> = Vec::new();
    let mut next_id =1;

    loop {
        
    

    let choice = input("Enter your choice: ");
    match choice.trim() {
        "1" =>{
            let name = input("Name :");
            let phone = input("Phone :");
            let email = input("Email :");
            contacts.push(Contact{id:next_id,name,phone,email});
            println!("Contact added with ID {}",next_id);
            next_id +=1;
            
        },
        "2" => { 
            if contacts.is_empty(){
                println!("No Contacts available ");
            }else {
                for c in &contacts{
                    println!("[{}]|{}|{}|{}",c.id,c.name,c.email,c.phone);
                }
            }
        },
        "3" => {
                let query = input("Search by name or email:");
                let results:Vec<&Contact> = contacts.iter()
                .filter(|c|c.name.contains(&query)|| c.email.contains(&query)).collect();
                if results.is_empty(){
                    println!("No match found");
                }else {
                    for c in results{
                        println!("[{}]|{}|{}|{}",c.id,c.name,c.email,c.phone);
                    }
                }

        },
        "4" => {
            let id = input("Enter id to delete :")
            .parse::<usize>().unwrap_or(0);
            let len_before = contacts.len();
            contacts.retain(|c|c.id!=id);
            if contacts.len() < len_before {
                println!("Contact delete");
            }else {
                println!("id not found");
            }
        },
        "5" => {
            println!("Exiting !");
            break;
        },
        
        _ => println!("Invalid choice, please try again."),
    }

}

}


   fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).expect("Failed to read input");
    buff.trim().to_string() 
   }
