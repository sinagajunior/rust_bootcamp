use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() {
    println!("File compression tool");
    println!("1. Compress File");
    println!("2. Decompress File");

    let choice = input("Enter your choiche");

    match choice.as_str() {
        "1" => {
            let src = input("Source file path");
            let dest = input("Output .gz file path");
            if compress_file(&src, &dest).is_ok() {
                println!("Compression {} -> {} successful", src, dest);
            } else {
                println!("Compression failed");
            }
        }
        "2" => {
            let src = input("Enter input path");
            let dest = input("Enter output path");
            if decompress_file(&src, &dest).is_ok() {
                println!("Decompression {} -> {} successful", src, dest);
            } else {
                println!("Decompression failed");
            }
        }
        _ => {
            println!("Invalid choice");
        }
    }
}

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);
    let output_file = File::create(output_path)?;
    let mut writer = GzEncoder::new(output_file, Compression::default());
    io::copy(&mut reader, &mut writer).unwrap();
    writer.finish()?;
    Ok(())
}

fn decompress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = GzDecoder::new(BufReader::new(input_file));
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    writer.write_all(&buffer)?;
    Ok(())
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
