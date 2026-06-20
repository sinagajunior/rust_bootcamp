use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> std::io::Result<()> {
    println!("Tcp Client connecting...to 127.0.0.1:7878");

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut reader = BufReader::new(stream.try_clone()?);

    //Thread to listen for message from server
    thread::spawn(move || {
        for line in reader.lines() {
            match line {
                Ok(line) => println!("Server: {}", line),
                Err(_) => {
                    println!("Server disconnected.");
                    break;
                }
            }
        }
    });

    // main thread: send input to server
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line?;
        if msg == "exit" {
            println!("Disconnecting.");
            break;
        }

        stream.write_all(msg.as_bytes())?;
        stream.write_all(b"\n")?;
    }

    Ok(())
}
