use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io;
use tokio::net::tcp::OwnedWriteHalf;
use std::io::stdin;

async fn client_input (mut s_write: OwnedWriteHalf) -> OwnedWriteHalf {
    loop{
        let mut s=String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        println!("You typed: {}",s);
        s_write.write_all(s.as_bytes()).await;
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    // Username input
    let mut username = String::new();
    stdin().read_line(&mut username).expect("Did not enter a correct Username");
    println!("You Username is: {}",username);
    
    // TCP Stream creation
    let mut stream =  TcpStream::connect("127.0.0.1:1234").await?;
    let (mut reader, mut writer) = stream.into_split();

    // Send username to server
    writer.write_all(username.as_bytes()).await;

    // Spawn thread
    tokio::spawn(async move {
        client_input(writer).await;
    });
    
    loop {
        let mut buf = [0; 4096];
        match reader.read(&mut buf).await{
            Ok(0)=> {
                continue;
            }
            Ok(_n)=> {
                println!("{}", String::from_utf8_lossy(&buf));
            }
            Err(_e) => {
            }
        }
    }   
}