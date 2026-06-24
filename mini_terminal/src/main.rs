use std::env;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    println!("Rust Mini Shell (type exit to quit)");

    loop {
        print!("rust shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let input = input.trim();
        if input == "exit" {
            println!("Exiting...");
            break;
        }

        if input.starts_with("cd ") {
            let path = input.strip_prefix("cd ").unwrap().trim();
            let result = env::set_current_dir(path);
            if let Err(e) = result {
                println!("CD failed: {}", e);
            }
            continue;
        }

        if input == "pwd" {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => println!("Pwd failed: {}", e),
            }
            continue;
        }

        run_command(input);
    }
}

fn run_command(command_line: &str) {
    let parts: Vec<&str> = command_line.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let (cmd, args) = parts.split_first().unwrap();

    match Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(mut child) => {
            let _ = child.wait();
        }
        Err(e) => println!("Failed to run command: {}", e),
    }
}
