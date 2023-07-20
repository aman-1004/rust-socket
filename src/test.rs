/*
 * anomaly -> network disconnection. (sort of ENUM)
 * probe -> try connection for 30 seconds.
 * trigger -> check for three consecutive failures.
 * action -> the action.
* */

use std::net::{TcpStream, SocketAddr};
use std::{thread, time::Duration};
use std::io::Result;

const COUNTER_TRIGGER: u16 = 3;
fn main() {
    network_probe()
}

fn action() {
    println!("peforming action for network disconnection")
}

fn network_probe() {
    while(true) {
        if let Err(e) = check_connection(1) {
            if let Some(counter) = verify_disconnection() {
                action();
            }
        }
    }
}

fn check_connection(timeout_in_millis: u64) -> Result<String> {
    let ip = "10.0.0.5";
    let port = "445";
    let socketAddr = SocketAddr::from(([10, 0, 0, 5], 445));
    match TcpStream::connect_timeout(&socketAddr, Duration::from_millis(timeout_in_millis)) {
        Ok(stream) => {
            drop(stream);
            return Ok(String::from("Successfully Connected"))
        }

        Err(e) => {
            return Err(e)
        }
    }
}

fn verify_disconnection() -> Option<u16> {
    println!("Verifying Disconnection");
    let mut counter: u16 = 0;
    let timer = 30*1000;
    let timeout_in_millis: u64 = 3*1000;
    for _ in (0..timer).step_by(timeout_in_millis as usize) {
        if let Ok(success_msg) = check_connection(timeout_in_millis) {
            counter = 0;
            return None;
        }
        else {
            counter = counter+1;
        }
        
        if(counter >= COUNTER_TRIGGER) {
            return Some(counter); 
        }

        println!("{}", counter)
    }
    
    return Some(counter);
}

