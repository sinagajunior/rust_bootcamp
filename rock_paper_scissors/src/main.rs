use std::{io, process::Child};
use rand::Rng;


fn main() {
    println!("Welcome to Rock, Paper, Scissors!");
    println!("Please enter your choice (rock, paper, or scissors) Type 'quit' to exit:");

   loop {
     println!("\n Make your choice: ");
   
   let user_choice = get_user_choice();
   if user_choice == "quit" {
       println!("Thanks for playing! Goodbye!");
       break;
   }

   let computer_coice = get_computer_choice();
   println!("Computer chose: {}", computer_coice);

   match determine_winner(&user_choice, &computer_coice) {
       GameResult::Win => println!("You win!"),
       GameResult::Lose => println!("You lose!"),
       GameResult::Draw => println!("It's a Draw"),
       _ => println!("Invalid input, please try again."),
   }
   }

}

fn get_user_choice()-> String {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
     let choice = choice.trim().to_lowercase();
     match choice.as_str() {
         "rock" | "paper" | "scissors" | "quit" => choice,
         _ => {
             println!("Invalid choice, please enter rock, paper, scissors, or quit.");
             get_user_choice()
         }
     }   

    }

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];
    let index = rand::thread_rng().gen_range(0..choices.len());
    choices[index].to_string()
}

//Enum to represent games outcomes
enum GameResult {
    Win,
    Lose,
    Draw,
}

/// Determines the game outcome

fn determine_winner(user:&str, computer:&str)-> GameResult {
    match(user,computer) {
       ("rock","scissors") => GameResult::Win,
       ("paper","rock") => GameResult::Win,
       ("scissors","paper")=> GameResult::Win,
       (a,b) if a==b=> GameResult::Draw,
       _=>GameResult::Lose
}
}

