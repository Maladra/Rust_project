use tokio::net::TcpSocket;
use tokio::io::BufReader;
use tokio::io::AsyncReadExt;
use tokio::io::split;
use std::io;


// for the moment nothing very interesting to comment
#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;

    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(4096);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf){
            Ok(0) => break,
            Ok(n) => {
                println!("read {} bytes", n);
                println!("message : {}", String::from_utf8_lossy(&buf));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}