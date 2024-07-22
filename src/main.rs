use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod parser;

async fn process(mut socket: TcpStream) {
    let mut buf = [0; 512];

    loop {
        let count = socket.read(&mut buf).await.unwrap();
        if count == 0 {
            break;
        }

        let request = String::from_utf8_lossy(&buf[..count]);
        let bulk_str = parser::parse(&request).await;

        match bulk_str.cmd.as_str() {
            "echo" => {
                let out = format!("+{}\r\n", bulk_str.value);
                socket.write(out.as_bytes()).await.unwrap();
            }
            "ping" => {
                socket.write(b"+PONG\r\n").await.unwrap();
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
