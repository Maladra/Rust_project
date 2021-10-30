use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io;
use tokio::net::tcp::OwnedWriteHalf;
use std::io::stdin;


struct _Message{
    user_sender: String,
    message_type: String,
    message_content: String, 
}


async fn client_input (mut s_write: OwnedWriteHalf) -> OwnedWriteHalf {
    loop{
        let mut s=String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        print!("You typed: {}",s);
        s_write.write_all(s.as_bytes()).await.unwrap();
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

    // Send username to server
    writer.write_all(username.as_bytes()).await.unwrap();

    // Spawn thread
    tokio::spawn(async move {
        client_input(writer).await;
    });
    loop {
        let mut buf = [0; 4096];
        let _readed = reader.read(&mut buf).await;
        println!("{}",String::from_utf8_lossy(&buf))
        
    }   
}