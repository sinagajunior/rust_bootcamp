use std::fs;
use std::io;
use std::path::Path;

fn main() {
    println!("Directory scanner");

    let dir = input("Enter a directory path: ");
    let path = Path::new(&dir);
    if path.exists() && path.is_dir() {
        println!("Scanning dir {}", dir);
        scan_dir(path, 0);
    } else {
        println!(" is not a valid directory");
    }
}

fn scan_dir(path: &Path, depth: usize) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = entry.file_type().unwrap();
                let name = entry.file_name().into_string().unwrap_or_default();
                let indent = " ".repeat(depth);
                if file_type.is_dir() {
                    println!("{} {}", indent, name);
                    scan_dir(&entry.path(), depth + 1);
                } else if file_type.is_file() {
                    println!("{} {}", indent, name);
                }
            }
        }
    } else {
        println!("failed to read contents of {:?}", path);
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
