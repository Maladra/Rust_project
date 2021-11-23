use rsa::PublicKey;
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
use rusqlite::{params, Connection, Result};


// represent a user
struct User{
    username: String,
    stream: tokio::net::TcpStream,
    _addr: std::net::SocketAddr, 
}


#[derive(Serialize, Deserialize, Debug)]
struct Message{
    user_sender: String,
    user_receiver: String,
    message_type: String,
    message_content: String, 
}
// message type : get_from_db
// message_type : global
async fn db_process (channel_snd: Sender<String>, mut channel_rcv : Receiver<String>){
    let conn = Connection::open("/tmp/rust_project.db").unwrap();
    loop{
        match channel_rcv.try_recv(){
            Ok(n) => {
                let from_json_message: Message = serde_json::from_str(&n).unwrap();
                if from_json_message.message_type == "global" {
                    conn.execute("INSERT INTO message (sender, message_type, message_content) VALUES (?1, ?2, ?3)",
                     params![from_json_message.user_sender, from_json_message.message_type, from_json_message.message_content]).unwrap();
                // insert into db 
                }
                else if from_json_message.message_type == "get_from_db"{
                    let last_id: Result<i64> = conn.query_row("SELECT * FROM message ORDER BY id DESC LIMIT 1",[], |row| row.get(0));
                    match last_id {
                        Ok(n) =>{
                            if n < 10 {
                                let mut sql_prepared = conn.prepare("SELECT * FROM message",).unwrap();
                                let messages_sql = sql_prepared.query_map([], |row| {
                                    Ok(Message{
                                        user_sender: row.get(1)?,
                                        user_receiver: from_json_message.user_sender.clone(),
                                        message_type: "set_from_db".to_string(),
                                        message_content: row.get(3)?, 
                                    })
                                }).unwrap();
                                for message_sql in messages_sql {
                                    match message_sql {
                                        Ok(n) =>{
                                            let json_message = serde_json::to_string(&n).unwrap();
                                            //println!("{:?}", json_message);
                                            channel_snd.send(json_message).unwrap();

                                        }
                                        Err(_) =>{
                                        }  
                                    }
                                }
                            }
                            else {
                                let min = n - 10;
                                let mut sql_prepared = conn.prepare("SELECT * FROM message WHERE id BETWEEN ? AND ? ",).unwrap();
                                let messages_sql = sql_prepared.query_map([min, n], |row| {
                                    Ok(Message{
                                        user_sender: row.get(1)?,
                                        user_receiver: from_json_message.user_sender.clone(),
                                        message_type: "set_from_db".to_string(),
                                        message_content: row.get(3)?, 
                                    })
                                }).unwrap();
                                for message_sql in messages_sql {
                                    match message_sql {
                                        Ok(n) =>{
                                            let json_message = serde_json::to_string(&n).unwrap();
                                            //println!("{:?}", json_message);
                                            channel_snd.send(json_message).unwrap();

                                        }
                                        Err(_) =>{
                                        }  
                                    }
                                }
                            }
                        },
                        Err(_) =>{}
                    }
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
// message type : set_from_db
async fn process (mut user : User, channel_snd : Sender<String>, mut channel_rcv : Receiver<String>, srv_priv_key: RsaPrivateKey, clt_pub_key: RsaPublicKey, mut rng: OsRng) {
    // data from database
    let message_back_from_db = Message{
        user_sender: user.username.clone(),
        user_receiver: "".to_string(),
        message_type: "get_from_db".to_string(),
        message_content: "".to_string(),
    };
    let json_message = serde_json::to_string(&message_back_from_db).unwrap();
    channel_snd.send(json_message).unwrap();
    loop{
        match channel_rcv.try_recv() {
            Ok(mut n) => {
                while user.username.ends_with('\n') || user.username.ends_with('\r') || user.username.ends_with('\u{0}') {
                    user.username.pop();
                };
                while n.ends_with('\n') || n.ends_with('\r') || n.ends_with('\u{0}') {
                    n.pop();
                };
                let mut from_json_message: Message = serde_json::from_str(&n).unwrap();
                 
                if user.username == from_json_message.user_receiver || from_json_message.message_type == "global"{
                    if from_json_message.message_type == "private" {
                        let split = from_json_message.message_content.splitn(3, " ");
                        match split.last() {
                            Some(value) => {
                                from_json_message.message_content = value.to_string();
                            }
                            None => {
                            }
                        }
                    }
                    let message_to_send = serde_json::to_string(&from_json_message).unwrap();
                    let enc_data = clt_pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), message_to_send.as_bytes()).unwrap();
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
                assert_ne!(&dec_data, &data[..n]);
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
    

    let thread_db_send = chann_snd.clone();
    let thread_db_rcv = chann_snd.subscribe();
    tokio::spawn(async move {
        db_process(thread_db_send, thread_db_rcv).await;
    });
    println!("Server Initialized");
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
        let json_pkey: Message = serde_json::from_str(&client_pkey).unwrap();
        println!("{}", json_pkey.message_content);
        let client_public_key = RsaPublicKey::from_public_key_pem(&json_pkey.message_content).unwrap();
        
        // Send Server public key
        let message_type = "pkey".to_string();
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
