use std::{io, thread, time::Duration};
use std::io::{self, Write};



fn main() {
    println!("Basic Timer tool");
    println!("Enter the timer duration (format: hours minutes second) ");

    let duration = match get_timer_input() {
        Some(dur) => dur,
        None => {
            println!(" Invalid input. Please enter number only (e.g., 0 1 30 for )");
            return;
        }
    };

    println!("Timer set for : {} hours, {} minute , {} seconds",duration.0,duration.1,duration.2);
    start_timer(duration.0,duration.1,duration.2);
    println!(" Time's up");
}


fn get_timer_input()-> Option<(u64,u64,u64)> {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
let parts: Vec<&str> = input.trim().split_whitespace().collect();
if parts.len() != 3 {
    return None;
}

let hour = parts[0].parse::<u64>().ok()?;
let minutes = parts[1].parse::<u64>().ok()?;
let seconds = parts[2].parse::<u64>().ok()?;
Some((hours,minutes,seconds))

}

// Start the timer and display countdown
fn start_timer(hours:u64,minutes:u64,seconds:u64){
    let total_seconds = hours * 3600 + minutes * 60 + seconds;
}