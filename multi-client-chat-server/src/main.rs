use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

fn main() -> std::io::Result<()> {
    println!("Multi-Client Chat Server Listening on  127.0.0.1:7878");

    let listening = TcpListener::bind("127.0.0.1:7878")?;
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    for stream in listening.incoming() {
        let stream = stream?;
        let addr = stream.peer_addr()?.to_string();
        println!("New connection: {}", addr);

        let clients = Arc::clone(&clients);
        clients
            .lock()
            .unwrap()
            .insert(addr.clone(), stream.try_clone()?);
        thread::spawn(move || handle_client(stream, addr, clients));
    }
    Ok(())
}

fn handle_client(stream: TcpStream, addr: String, clients: Clients) {
    let reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream;
    for line in reader.lines() {
        let msg = match line {
            Ok(msg) => msg,
            Err(_) => break,
        };

        let full_msg = format!("[{}]: {}", addr, msg);
        println!("{}", full_msg);

        let mut clients_lock = clients.lock().unwrap();
        let mut disconnected = vec![];
        for (peer, client_stream) in clients_lock.iter_mut() {
            if peer != &addr {
                if let Err(_) = writeln!(client_stream, "{}", full_msg) {
                    disconnected.push(peer.clone());
                }
            }
        }
        for peer in disconnected {
            clients_lock.remove(&peer);
        }
    }

    println!("Client {} disconnected", addr);
    clients.lock().unwrap().remove(&addr);
}
