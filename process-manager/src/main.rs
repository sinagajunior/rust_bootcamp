use std::io::{self, Write};
use sysinfo::{Pid, Process, Signal, System};
fn main() {
    println!("Rust Process Manager ");

    let mut sys = System::new_all();
    sys.refresh_all();

    loop {
        println!("\n1. List all processes  ");
        println!("2. Search processes by name ");
        println!("3. Kill process by PID ");
        println!("4. Exit ");

        let choice = prompt("Choose an option: ");
        match choice.as_str() {
            "1" => list_processes(&sys),
            "2" => {
                let keyword = prompt("Enter search keyword: ");
                search_processes(&sys, &keyword);
            }
            "3" => {
                let pid = prompt("Enter PID to kill: ");

                kill_process(pid.parse::<usize>().unwrap());
            }
            "4" => {
                println!("👋 Exiting Process Manager.");
                break;
            }
            _ => println!("Ivalid choice: {}", choice),
        }
    }
}

fn list_processes(sys: &System) {
    println!("{:<8} {:<20} {:<10}", "PID", "Name", "CPU%");
    for proc in sys.processes().values() {
        println!(
            "{:<8} {:<20} {:<10}",
            proc.pid(),
            proc.name(),
            proc.cpu_usage()
        );
    }
}

fn search_processes(sys: &System, keyword: &str) {
    let keyword = keyword.to_lowercase();
    let found: Vec<_> = sys
        .processes()
        .values()
        .filter(|p| p.name().to_lowercase().contains(&keyword))
        .collect();
    if found.is_empty() {
        println!("No processes found matching '{}'", keyword);
    } else {
        println!("{:<8} {:<20} {:<10}", "PID", "Name", "CPU%");
        for proc in found {
            println!(
                "{:<8} {:<20} {:<10}",
                proc.pid(),
                proc.name(),
                proc.cpu_usage()
            );
        }
    }
}

fn kill_process(pid: usize) {
    let mut sys = System::new();
    //let pid = Pid(pid);
    sys.refresh_processes();
    if let Some(process) = sys.process(Pid::from(pid)) {
        if process.kill_with(Signal::Kill).is_some() {
            println!(
                "Process {} (PID: {}) killed successfully",
                process.name(),
                process.pid()
            );
        } else {
            println!("Failed to kill process {}", pid);
        }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
