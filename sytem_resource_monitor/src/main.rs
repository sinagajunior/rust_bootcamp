use std::thread;
use std::time::Duration;
use sysinfo::{Cpu, Process, System};

fn main() {
    println!("System Resource Monitor");
    let mut sys = System::new_all();

    // refresh every 2 seconds
    loop {
        sys.refresh_all();
        let total_memory = sys.total_memory() / 1024;
        let used_memory = sys.used_memory() / 1024;

        println!("\n========================================");
        println!("CPU usage");
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("core {}: {:.2}%", i, cpu.cpu_usage());
        }
        println!("Memory usage: {} MB / {} MB", used_memory, total_memory);
        println!("Total Processes: {}", sys.processes().iter().len());
        println!("Top five process CPU ");
        let mut processess: Vec<_> = sys.processes().values().collect();
        processess.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());
        for proc in processess.iter().take(5) {
            println!(
                "PID: {:<6} CPU: {:>5.1}% Name: {}",
                proc.pid(),
                proc.cpu_usage(),
                proc.name()
            );
        }

        println!("=========================================================================");
        thread::sleep(Duration::from_secs(2));
    }
}
