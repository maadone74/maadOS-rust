// backend.rs

use crate::comm::{self, Shared};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::time::{self, Duration};

pub struct Heartbeat {
    // For now, we'll just use a simple vector of connection addresses
    // In a real implementation, this would be a more complex data structure
    // that stores information about each object's heartbeat.
    pub list: Mutex<Vec<std::net::SocketAddr>>,
}

impl Heartbeat {
    pub fn new() -> Self {
        Heartbeat {
            list: Mutex::new(Vec::new()),
        }
    }

    pub fn set_heartbeat(&self, addr: std::net::SocketAddr, to: bool) {
        let mut list = self.list.lock().unwrap();
        if to {
            if !list.contains(&addr) {
                list.push(addr);
            }
        } else {
            list.retain(|&x| x != addr);
        }
    }

    pub fn call_heartbeat(&self) {
        let list = self.list.lock().unwrap();
        for addr in list.iter() {
            println!("Calling heartbeat for {}", addr);
        }
    }
}

pub struct Swap;

impl Swap {
    pub fn new() -> Self {
        Swap
    }

    pub fn look_for_objects_to_swap(&self) {
        println!("Looking for objects to swap...");
    }
}

pub async fn backend() {
    let listener = TcpListener::bind("127.0.0.1:6666").await.unwrap();
    let shared = Arc::new(Shared::new());
    let heartbeat = Arc::new(Heartbeat::new());
    let swap = Arc::new(Swap::new());

    let mut hb_interval = time::interval(Duration::from_secs(1));
    let mut swap_interval = time::interval(Duration::from_secs(60));

    loop {
        tokio::select! {
            Ok((socket, addr)) = listener.accept() => {
                let shared = shared.clone();
                let heartbeat = heartbeat.clone();
                tokio::spawn(async move {
                    heartbeat.set_heartbeat(addr, true);
                    comm::handle_connection(socket, shared, addr).await;
                    heartbeat.set_heartbeat(addr, false);
                });
            }
            _ = hb_interval.tick() => {
                heartbeat.call_heartbeat();
            }
            _ = swap_interval.tick() => {
                swap.look_for_objects_to_swap();
            }
        }
    }
}
