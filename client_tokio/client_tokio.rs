use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io;
use tokio::net::tcp::OwnedWriteHalf;
use std::io::stdin;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Message{
    user_sender: String,
    user_receiver: String,
    message_type: String,
    message_content: String, 
}


async fn client_input (mut s_write: OwnedWriteHalf, username_string: String) -> OwnedWriteHalf {
    loop{
        let mut s=String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
       
        // trim user input
        while s.ends_with('\n') || s.ends_with('\r') {
            s.pop();
        };
        // check if message is private or global
        let chunk: Vec<&str> = s.split(" ").collect();
        let mut type_message = String::new();
        let mut receiver_user = String::new();
        let mut message_must_sended = true;
        if chunk[0] == "/tell" {
            if chunk.len() < 2 {
                message_must_sended = false;
            }
            else {
                type_message = "private".to_string();
                receiver_user = chunk[1].to_string();
            }
        }
        else {
            type_message = "global".to_string();
        }

        // check if message is correct
        if message_must_sended {
            let message_to_send = Message{
                user_sender: username_string.to_string(),
                user_receiver: receiver_user,
                message_type: type_message,
                message_content: s,
            };
        
            // message sending
            let json_message = serde_json::to_string(&message_to_send).unwrap();
            s_write.write_all(json_message.as_bytes()).await.unwrap();
        }
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    // Username input
    let mut username = String::new();
    stdin().read_line(&mut username).expect("Did not enter a correct Username");
    print!("You Username is: {}",username);
    while username.ends_with('\n') || username.ends_with('\r') {
        username.pop();
    };


    // TCP Stream creation
    let mut _stream =  TcpStream::connect("127.0.0.1:1234").await?;
    let (mut reader, mut writer) = _stream.into_split();

    // Send username to server (formating that with json and message_type -> login)
    writer.write_all(username.as_bytes()).await.unwrap();

    // Spawn thread
    tokio::spawn(async move {
        client_input(writer, username).await;
    });
    loop {
        let mut buf = [0; 4096];
        let _readed = reader.read(&mut buf).await;
        println!("{}",String::from_utf8_lossy(&buf));
    }   
}