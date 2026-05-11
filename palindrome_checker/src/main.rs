//racecar
// A man, A plan, A canal, Panama
// Hello

use std::io;
fn main() {
    println!("Palindrome Checker");
    println!("Enter a string to check it's a plindrome:");

    let mut input = String::new();
    io::stdin()
    .read_line(& mut input)
    .expect("Failed read input");

    let cleaned_input = clean_string(&input);
    if cleaned_input.is_empty(){
        println!("Please enter a valid non-empty string");
        return;
    }

     if is_palindrome(&cleaned_input){
        println!("'{}' is a palindrome ",input.trim());
     }else{
        println!("'{}' is not a palindrome ",input.trim());
     }



}
/// cleans a string: removes non alpahanumeric characters and lconverts to lowercase
fn clean_string(input: &str)-> String {
    // iterate over each character
    input
      .chars()
      .filter(|c| c.is_alphanumeric()) // keep only letters and numbers
      .map(|c| c.to_lowercase().to_string()) // convert to lowercase
      .collect::<String>()
}


// checks if a cleaned string is a palindrome
fn is_palindrome(s: &str)-> bool {
    s == s.chars().rev().collect::<String>()
}



