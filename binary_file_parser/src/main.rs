use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("Binary File Parser");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: binary_file_parser <path_to_binary_file>");
        return;
    }
    match parse_binary_file(&args[1]) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}

fn parse_binary_file(path: &str) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = [0u8; 1024];
    let mut offset = 0;

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        print!("{:08x}:", offset);
        for i in 0..16 {
            if i < bytes_read {
                print!("{:02x} ", buffer[i]);
            } else {
                print!("   ");
            }
            if i == 7 {
                print!(" "); // extra space in the middle
            }
        }
        println!("|");

        for i in 0..bytes_read {
            let c = buffer[i];
            let display = if c.is_ascii_graphic() || c == b' ' {
                c as char
            } else {
                '.'
            };
            print!("{}", display);
        }
        println!("|");
        offset += bytes_read;
    }

    Ok(())
}
