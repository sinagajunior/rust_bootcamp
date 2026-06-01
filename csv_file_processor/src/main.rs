use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

fn main() {
    println!("CSV File processor");
    let filepath = input("Enter the path to the CSV file: ");
    let column_name = input("Enter the column to filter by: ");
    let keyword = input("Enter value to match: ");

    if let Err(e) = filter_csv(&filepath, &column_name, &keyword) {
        eprintln!("Error: {}", e);
    }
}

fn filter_csv(filepath: &str, column_name: &str, keyword: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    let headers = reader.headers()?.clone();
    let col_index = headers
        .iter()
        .position(|h| h == column_name)
        .ok_or("column not found")?;

    println!("Matching rows:");
    //println!("{}", headers.join(","));

    for result in reader.records() {
        let record = result?;
        if record.get(col_index).unwrap_or("") == keyword {
            println!("{}", record.iter().collect::<Vec<_>>().join(","));
        }
    }
    Ok(())
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
