use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io;
use tokio::net::tcp::OwnedWriteHalf;
use std::process;


async fn client_input (mut s_write: OwnedWriteHalf) -> OwnedWriteHalf {
    loop{
        use std::io::{stdin};
        let mut s=String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        println!("You typed: {}",s);
        match s_write.write_all(s.as_bytes()).await{
            Ok(_n) => {
            }
            Err(e) => {
                s_write.shutdown();
                println!("{:?}",e);
                process::exit(1);
            }
        }

    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream =  TcpStream::connect("127.0.0.1:1234").await?;
    let (mut reader, mut writer) = stream.into_split();


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
            Err(e) => {
                println!("{:?}",e);
                process::exit(1);
            }
        }
    }   
}