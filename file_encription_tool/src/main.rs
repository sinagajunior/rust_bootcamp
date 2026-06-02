use aes::Aes256;
use cipher::{KeyIvInit, StreamCipher};
use std::fs::{self, File};
use std::io::{self, Read, Write};

type Aes256Ctr = ctr::Ctr64BE<Aes256>;

const IV: &[u8; 16] = b"uniqueinitvector"; // initialization Vector (fixed for demo)
fn main() {
    println!("File encryption Tool");
    println!("1. Encrypt file");
    println!("2. Decrypt file");

    let choice = input("Choose an option: ");
    match choice.as_str() {
        "1" => {
            let file = input("Enter file path to encrypt: ");
            let key = input("Enter key (hex): ");
            let out = input("output path: ");
            encrypt_file(&file, &key, &out).unwrap_or_else(|e| println!("Error: {}", e));
        }
        "2" => {
            let file = input("Enter file path to decrypt: ");
            let key = input("Enter key (hex): ");
            let out = input("output path: ");
            decrypt_file(&file, &key, &out).unwrap_or_else(|e| println!("Error: {}", e));
        }
        _ => {
            println!("Invalid choice");
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn encrypt_file(path: &str, key_hex: &str, out_path: &str) -> io::Result<()> {
    let key = hex::decode(key_hex).expect("Invalid hex key");
    let mut data = fs::read(path).expect("Failed to read file");
    let mut cipher = Aes256Ctr::new_from_slices(&key, IV).unwrap();
    cipher.apply_keystream(&mut data);
    fs::write(out_path, data)?;
    println!("File encrypted successfully");
    Ok(())
}

fn decrypt_file(path: &str, key_hex: &str, out_path: &str) -> io::Result<()> {
    encrypt_file(path, key_hex, out_path)
}
