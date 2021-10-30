use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use std::io;
use std::{thread, time};

// represent a user
struct User{
    username: String,
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    
}

async fn process (mut user : User, mut channel_snd : Sender<String>, mut channel_rcv : Receiver<String>) {
    loop{
        match channel_rcv.try_recv() {
            Ok(n) => {
                println!("{}", n);
                user.stream.write(n.as_bytes()).await;
            }
            Err(_) => {
            }
        }
        let mut data = vec![0; 4096];
        
        match user.stream.try_read(&mut data) {
            Ok(0) => {}
            Ok(n) => {
                println!("read {} bytes", n);  
                channel_snd.send(String::from_utf8_lossy(&data).to_string());
            }
            Err(e) => {}
        }
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let (chann_snd, mut chann_rcv)  = broadcast::channel(16);
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    loop {
        // User accept
        let (mut socket, addr) = listener.accept().await.unwrap();  
        socket.readable().await?;
        let mut username_buf = [0; 4096];
        match socket.try_read(&mut username_buf){
            Ok(0) => {}
            Ok(n) => {}
            Err(e) => {}
        
        }

        let mut username_string =String::from_utf8_lossy(&username_buf).to_string();
        if username_string.ends_with('\n') || username_string.ends_with('\r') {
            username_string.pop();
            if username_string.ends_with('\r') {
                username_string.pop();
            }
        }
        // User struct
        let mut user1 = User{
            username: username_string,
            stream: socket,
            addr: addr,
        };

        let username_send = user1.username.to_string();
        // Thread creation
        let thread_send = chann_snd.clone();
        let thread_rcv = chann_snd.subscribe();
        tokio::spawn(async move {
            process(user1, thread_send, thread_rcv).await;
        });
        chann_snd.send(username_send);
    }
}
