use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use std::io;
use serde::{Deserialize, Serialize};

// represent a user
struct User{
    username: String,
    stream: tokio::net::TcpStream,
    _addr: std::net::SocketAddr, 
}


#[derive(Serialize, Deserialize)]
struct Message{
    user_sender: String,
    user_receiver: String,
    message_type: String,
    message_content: String, 
}



async fn process (mut user : User, channel_snd : Sender<String>, mut channel_rcv : Receiver<String>) {
    loop{
        match channel_rcv.try_recv() {
            Ok(mut n) => {
                while user.username.ends_with('\n') || user.username.ends_with('\r') || user.username.ends_with('\u{0}') {
                    user.username.pop();
                };
                while n.ends_with('\n') || n.ends_with('\r') || n.ends_with('\u{0}') {
                    n.pop();
                };
                let from_json_message: Message = serde_json::from_str(&n).unwrap();

                if user.username == from_json_message.user_receiver || from_json_message.message_type == "global"{
                    user.stream.write(n.as_bytes()).await.unwrap();
                }
                else if from_json_message.message_type == "login" {
                    user.stream.write(n.as_bytes()).await.unwrap();
                }
            }
            Err(_) => {
            }
        }
        let mut data = vec![0; 4096];
        
        match user.stream.try_read(&mut data) {
            Ok(0) => {}
            Ok(n) => {
                println!("read {} bytes", n);  
                channel_snd.send(String::from_utf8_lossy(&data).to_string()).unwrap();
            }
            Err(_e) => {}
        }
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let (chann_snd, mut _chann_rcv)  = broadcast::channel(16);
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    loop {
        // User accept
        let (socket, addr) = listener.accept().await.unwrap();  
        socket.readable().await?;
        let mut username_buf = [0; 4096];
        match socket.try_read(&mut username_buf){
            Ok(0) => {}
            Ok(_n) => {}
            Err(_e) => {}
        
        }
        let mut username_string = String::from_utf8_lossy(&username_buf).to_string();
        while username_string.ends_with('\n') || username_string.ends_with('\r') || username_string.ends_with('\u{0}') {
            username_string.pop();
        };
        let json_login: Message = serde_json::from_str(&username_string).unwrap();
        while username_string.ends_with('\n') || username_string.ends_with('\r') || username_string.ends_with('\u{0}') {
            username_string.pop();
        };
        // User struct
        let user1 = User{
            username: json_login.user_sender,
            stream: socket,
            _addr: addr,
        };

        let username_send = user1.username.to_string();
        // Thread creation
        let thread_send = chann_snd.clone();
        let thread_rcv = chann_snd.subscribe();
        tokio::spawn(async move {
            process(user1, thread_send, thread_rcv).await;
        });
        chann_snd.send(username_string).unwrap();
    }
}
