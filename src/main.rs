use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

async fn process(mut socket: TcpStream) {
    let mut buf = [0; 512];

    loop {
        let count = socket.read(&mut buf).await.unwrap();
        if count == 0 {
            break;
        }
        socket.write(b"+PONG\r\n").await.unwrap();
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
