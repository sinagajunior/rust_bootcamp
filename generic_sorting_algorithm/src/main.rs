use std::io::{self, Write};

fn main() {
    println!("Generic Sorting Demo");

    loop {
        println!("\n Choose type to sort");
        println!("1. Integers ");
        println!("2. Words ");
        println!("3. Exit ");

        let choice = input("Your choice: ");
        match choice.as_str() {
            "1" => {
                let raw = input("Enter comma separated integers: ");
                let mut nums: Vec<i32> = raw
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                buble_sort(&mut nums);
                println!("sorted {:?}", nums);
            }
            "2" => {
                let raw = input("Enter comma separated words: ");
                let mut words: Vec<String> = raw.split(',').map(|s| s.trim().to_string()).collect();
                insertion_sort(&mut words);
                println!("sorted {:?}", words);
            }
            "3" => {
                println!("Exisitng");
                break;
            }

            _ => println!("Invalid choice"),
        }
    }
}

///Generic buble sort
fn buble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

///Generic Insertion Sort
fn insertion_sort<T: PartialOrd + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        arr[j] = key;
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
