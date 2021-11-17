use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io;
use tokio::net::tcp::OwnedWriteHalf;
use std::io::stdin;
use serde::{Deserialize, Serialize};
use rsa::{RsaPublicKey, RsaPrivateKey, pkcs8::FromPublicKey, pkcs8::ToPublicKey, PaddingScheme};
use rand::rngs::OsRng;

#[derive(Serialize, Deserialize)]
struct Message{
    user_sender: String,
    user_receiver: String,
    message_type: String,
    message_content: String, 
}

// not working for moment
fn _trim_newline(s: &mut String){
    while s.ends_with('\n') || s.ends_with('\r') {
        s.to_string().pop();
    };
}

async fn client_input (mut s_write: OwnedWriteHalf, username_string: String, clt_priv_key: RsaPrivateKey) -> OwnedWriteHalf {
    loop{
        let mut s=String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
       
        //trim_newline(&mut s);
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
    // Generate priv and pub key of client
    println!("----------------------\nInitialize pub and private key\n----------------------");
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_pem = RsaPublicKey::to_public_key_pem(&pub_key).unwrap();
    println!("----------------------\nPub and Private key was initialize\n----------------------");
    println!("----------------------\nChoose a username :\n----------------------");    
    
    // Username input
    let mut username = String::new();
    stdin().read_line(&mut username).expect("Did not enter a correct Username");  
    while username.ends_with('\n') || username.ends_with('\r') {
        username.pop();
    };
    println!("Your Username is: {}",username);
    println!("----------------------\nConnect to Server\n----------------------");
    
    // TCP Stream creation
    let mut _stream =  TcpStream::connect("127.0.0.1:1234").await?;
    let (mut reader, mut writer) = _stream.into_split();
    println!("----------------------\nConnected to Server\n----------------------");
    println!("----------------------\nSend Public Key to Server\n----------------------");
    
    // Send public key
    let mut message_type = String::new();
    message_type = "pkey".to_string();
    let pbkey_to_send = Message{
        user_sender: "".to_string(),
        user_receiver: "".to_string(),
        message_type: message_type,
        message_content: pub_key_pem,
    };
    let json_message = serde_json::to_string(&pbkey_to_send).unwrap();
    writer.write_all(json_message.as_bytes()).await.unwrap();
    
    // Get public key from server
    let mut buf = [0; 4096];
    let _readed = reader.read(&mut buf).await;
    let mut rcv_msg = String::from_utf8_lossy(&buf).to_string();
    while rcv_msg.ends_with('\n') || rcv_msg.ends_with('\r') || rcv_msg.ends_with('\u{0}') {
        rcv_msg.pop();
    };
    let json_message: Message = serde_json::from_str(&rcv_msg).unwrap();
    println!("{:?}", json_message.message_content);
    println!("----------------------\nSend username to server\n----------------------");
    
    // send username to server
    let mut message_type = String::new();
    message_type = "login".to_string();

    let username_to_send = Message{
        user_sender: username.to_string(),
        user_receiver: username.to_string(),
        message_type: message_type,
        message_content: username.to_string(),
    };
    let json_message = serde_json::to_string(&username_to_send).unwrap();
    println!("{}", json_message);
    writer.write_all(json_message.as_bytes()).await.unwrap();

    // Spawn thread
    tokio::spawn(async move {
        client_input(writer, username,priv_key).await;
    });
    loop {
        let mut buf = [0; 4096];
        let _readed = reader.read(&mut buf).await;
        let mut rcv_msg = String::from_utf8_lossy(&buf).to_string();
        while rcv_msg.ends_with('\n') || rcv_msg.ends_with('\r') || rcv_msg.ends_with('\u{0}') {
            rcv_msg.pop();
        };
        let json_message: Message = serde_json::from_str(&rcv_msg).unwrap();
        if json_message.message_type == "login" {
            println!(">> New user logged in : {}",json_message.message_content );    
            
        }
        else {
        println!("In {} From {} :\n{}\n----------", json_message.message_type, json_message.user_sender, json_message.message_content );
        }
    }   
}