use rsa::PublicKey;
use rsa::pkcs1::FromRsaPublicKey;
use tokio::io::AsyncReadExt;
use tokio::io::ReadBuf;
use tokio::macros::support::poll_fn;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use std::io;
use serde::{Deserialize, Serialize};
use rsa::{RsaPublicKey, RsaPrivateKey, pkcs8::FromPublicKey, pkcs8::ToPublicKey, PaddingScheme};
use rand::rngs::OsRng;

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
// message type : get_from_db
// message_type : global
async fn db_process (channel_snd: Sender<String>, mut channel_rcv : Receiver<String>){
    loop{
        match channel_rcv.try_recv(){
            Ok(mut n) => {
                let from_json_message: Message = serde_json::from_str(&n).unwrap();
                if from_json_message.message_type == "global" {
                // insert into db 
                }
                else if from_json_message.message_type == "get_from_db"{
                    // get from db
                }
            }
            Err(_) => {

            }

        }
    }
}

// message type : global
// message type : login
// message type : private
// message type : get_from_db

async fn process (mut user : User, channel_snd : Sender<String>, mut channel_rcv : Receiver<String>, srv_priv_key: RsaPrivateKey, clt_pub_key: RsaPublicKey, mut rng: OsRng) {
    // data from database 
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
                    let enc_data = clt_pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), n.as_bytes()).unwrap();
                    user.stream.write(&enc_data).await.unwrap();
                }
                else if from_json_message.message_type == "login" {
                    let enc_data = clt_pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), n.as_bytes()).unwrap();
                    user.stream.write(&enc_data).await.unwrap();
                }
            }
            Err(_) => {
            }
        }
        let mut data = vec![0; 4096];
        
        match user.stream.try_read(&mut data) {
            Ok(0) => {}
            Ok(n) => {
                let dec_data = srv_priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &data[..n]).expect("failed to decrypt");
                println!("read {} bytes", n);
                channel_snd.send(String::from_utf8_lossy(&dec_data).to_string()).unwrap();
            }
            Err(_e) => {}
        }
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let (chann_snd, mut _chann_rcv)  = broadcast::channel(64);
    let listener = TcpListener::bind("127.0.0.1:1234").await?;
    // Generate priv and pub key of server
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    let pub_key_pem = RsaPublicKey::to_public_key_pem(&pub_key).unwrap();
    

    loop {
        // User accept
        let (mut socket, addr) = listener.accept().await.unwrap();  
        socket.readable().await?;
        
        // Get client public key
        let mut client_pkey_buf = [0; 4096];
        match socket.try_read(&mut client_pkey_buf){
            Ok(0) => {}
            Ok(_n) => {}
            Err(_e) => {}
        }
        let mut client_pkey = String::from_utf8_lossy(&client_pkey_buf).to_string();
        while client_pkey.ends_with('\n') || client_pkey.ends_with('\r') || client_pkey.ends_with('\u{0}') {
            client_pkey.pop();
        }
        let mut json_pkey: Message = serde_json::from_str(&client_pkey).unwrap();
        println!("{}", json_pkey.message_content);
        let client_public_key = RsaPublicKey::from_public_key_pem(&json_pkey.message_content).unwrap();
        
        // Send Server public key
        let mut message_type = String::new();
        message_type = "pkey".to_string();
        let pkey_server_send = Message{
            user_sender: "".to_string(),
            user_receiver: "".to_string(),
            message_type: message_type,
            message_content: pub_key_pem.to_string(),
        };
        let json_message = serde_json::to_string(&pkey_server_send).unwrap();
        socket.writable().await?;
        socket.write_all(&json_message.as_bytes()).await.unwrap();
        
        // get Client Username
        let mut buf = [0; 10];
        let mut buf = ReadBuf::new(&mut buf);
        poll_fn(|cx|{
            socket.poll_peek(cx, &mut buf)
        }).await?;

        let mut username_buf = [0; 4096];
        
        socket.readable().await?;
        let n = socket.read(&mut username_buf[..]).await?;
        let dec_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &username_buf[..n]).expect("failed to decrypt");
        
        let mut username_string = String::from_utf8_lossy(&dec_data).to_string();
        while username_string.ends_with('\n') || username_string.ends_with('\r') || username_string.ends_with('\u{0}') {
            username_string.pop();
        };

        let mut json_login: Message = serde_json::from_str(&username_string).unwrap();

        while json_login.message_content.ends_with('\n') || json_login.message_content.ends_with('\r') || json_login.message_content.ends_with('\u{0}') {
            json_login.message_content.pop();
        };

        // User struct
        let user1 = User{
            username: json_login.user_sender,
            stream: socket,
            _addr: addr,
        };

          // Thread creation
        let thread_send = chann_snd.clone();
        let thread_rcv = chann_snd.subscribe();
        let priv_key_thread = priv_key.clone();
        let rng_thread = rng.clone();
        tokio::spawn(async move {
            process(user1, thread_send, thread_rcv, priv_key_thread, client_public_key, rng_thread).await;
        });
        chann_snd.send(username_string).unwrap();
    }
}
