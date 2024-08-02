use std::collections::HashMap;
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod parser;

async fn process(mut socket: TcpStream) {
    let mut buf = [0; 512];
    let mut storage = HashMap::new();

    loop {
        let count = socket.read(&mut buf).await.unwrap();
        if count == 0 {
            break;
        }

        let request = String::from_utf8_lossy(&buf[..count]);
        let bulk_str = parser::parse(&request).await;

        match bulk_str.cmd.as_str() {
            "echo" => {
                let out = format!("+{}\r\n", bulk_str.value1);
                socket.write_all(out.as_bytes()).await.unwrap();
            }
            "ping" => {
                socket.write_all(b"+PONG\r\n").await.unwrap();
            }
            "set" => {
                storage.insert(bulk_str.value1.clone(), bulk_str.value2.clone());
                socket.write_all(b"+OK\r\n").await.unwrap();
            }
            "get" => {
                if let Some(value) = storage.get(&bulk_str.value1) {
                    let out = format!("${}\r\n{}\r\n", value.len(), value);
                    socket.write(out.as_bytes()).await.unwrap();
                } else {
                    socket.write(b"$-1\r\n").await.unwrap();
                }
            }
            _ => eprintln!("Unknown command"),
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Failed to bind to port 6379");

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
