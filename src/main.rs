use std::net::{TcpStream, SocketAddr};
use std::{thread, time::Duration};

fn main() {
    let ip = "10.0.0.5";
    let port = "445";
    let socketAddr = SocketAddr::from(([10, 0, 0, 5], 445));
    match TcpStream::connect_timeout(&socketAddr, Duration::from_millis(1000)) {
        Ok(stream) => {
            println!("Successfully connected to {}:{}", ip, port);
        thread::sleep(Duration::from_millis(8000));
            drop(stream);
        }
        Err(e) => {
            println!("Failed to connect to {}:{}", ip, port);
            println!("Error: {}", e);
        }
    }
}
