// comm.rs

use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const MAX_TEXT: usize = 2048;
const MESSAGE_BUF_SIZE: usize = 2048;

pub struct Connection {
    pub stream: Option<TcpStream>,
    pub addr: SocketAddr,
    pub last_time: i64,
    pub prompt: String,
    pub text: [u8; MAX_TEXT],
    pub text_end: usize,
    pub text_start: usize,
    pub message_buf: [u8; MESSAGE_BUF_SIZE],
    pub message_producer: usize,
    pub message_consumer: usize,
    pub message_length: usize,
    pub iflags: u32,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        let addr = stream.peer_addr().unwrap();
        Connection {
            stream: Some(stream),
            addr,
            last_time: chrono::Utc::now().timestamp(),
            prompt: "> ".to_string(),
            text: [0; MAX_TEXT],
            text_end: 0,
            text_start: 0,
            message_buf: [0; MESSAGE_BUF_SIZE],
            message_producer: 0,
            message_consumer: 0,
            message_length: 0,
            iflags: 0,
        }
    }
}

pub struct Shared {
    pub connections: Mutex<HashMap<SocketAddr, Arc<Mutex<Connection>>>>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            connections: Mutex::new(HashMap::new()),
        }
    }
}

pub async fn handle_connection(
    stream: TcpStream,
    shared: Arc<Shared>,
    addr: SocketAddr,
) {
    let conn_arc = Arc::new(Mutex::new(Connection::new(stream)));
    shared.connections.lock().unwrap().insert(addr, conn_arc.clone());

    let mut stream = conn_arc.lock().unwrap().stream.take().unwrap();

    loop {
        let mut buf = [0; 1024];
        match stream.read(&mut buf).await {
            Ok(0) => {
                // Connection closed
                break;
            }
            Ok(n) => {
                // Process input
                let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                println!("Received from {}: {}", addr, msg);
                stream.write_all(&buf[..n]).await.unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", addr, e);
                break;
            }
        }
    }

    shared.connections.lock().unwrap().remove(&addr);
}
