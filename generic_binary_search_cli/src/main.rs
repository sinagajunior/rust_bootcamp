use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Generic binary search");
    let numbers = vec![1, 3, 5, 7, 9, 11, 13];
    let words = vec!["apple", "banana", "cherry", "date", "fig", "grape"];

    println!("\n 1. search Numbers :");
    println!("2. Search words");
    let mode = input("Choose list type");

    match mode.as_str() {
        "1" => {
            let query = input("Enter number to search");
            if let Ok(q) = query.parse::<i32>() {
                match binary_search(&numbers, &q) {
                    Some(idx) => println!("Found {} at index {}", q, idx),
                    None => println!("Not found"),
                }
            } else {
                println!("Invalid Number");
            }
        }

        "2" => {
            let query = input("Enter word to search: ");
            match binary_search(&words, &query.as_str()) {
                Some(idx) => println!("Found {} at index {}", query, idx),
                None => println!("Not Found"),
            }
        }
        _ => println!("Invalid choice"),
    }
}

fn binary_search<T: PartialOrd>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = (low + high) / 2;
        match list[mid].partial_cmp(target).unwrap() {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    None
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
