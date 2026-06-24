use std::io::{self, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    println!("Rust Port Scanner");

    let host = input("Enter host (e.g. 127.0.0.1 or google.com): ");
    let start_port = input("Enter start port: ");
    let end_port = input("Enter end port: ");
    let start_port = start_port.parse::<u16>().unwrap_or(1);
    let end_port = end_port.parse::<u16>().unwrap_or(1024);

    println!("Scanning {} ports {}..{}", host, start_port, end_port);

    for port in start_port..=end_port {
        let address = format!("{}:{}", host, port);
        let timeout = Duration::from_millis(300);
        if let Ok(addrs) = address.to_socket_addrs() {
            for addr in addrs {
                if is_port_open(addr, timeout) {
                    println!("Port {} is open", port);
                }
            }
        }
    }
    println!("Scan complete");
}

fn is_port_open(addr: SocketAddr, timeout: Duration) -> bool {
    TcpStream::connect_timeout(&addr, timeout).is_ok()
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
