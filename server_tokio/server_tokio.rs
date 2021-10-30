use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use std::io;
use std::{thread, time};

// represent a user
struct User{
    username: String,
    stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let mut user_vector: Vec<User> = Vec::new();
    let listener = TcpListener::bind("127.0.0.1:1234").await?;

    // move that on a function probably ??
    loop {
        // user connection
        let (mut socket, addr) = listener.accept().await.unwrap();
        socket.readable().await?;
        let mut username_buf = [0; 4096];
        match socket.try_read(&mut username_buf){
            Ok(0) => {}
            Ok(n) => {
                println!("Username is : {} ",String::from_utf8_lossy(&username_buf))
            }
            Err(e) => {}
        
        }

        let mut user1 = User{
            username: String::from("toto"),
            stream: socket,
            addr: addr,
        };

        // check if username is present
        let mut username_is_present = false;
        for item in &mut user_vector.iter() {
            if item.username == user1.username{
                username_is_present = true;
                break;
            }
        }
        // if username is present drop the socket
        // else add User struct in the vector
        if username_is_present{
            user1.stream.write(b"Username already use choose another one and come back !").await?;
        }
        else{
            user_vector.push(user1);
        }

        // send to all socket(stream) in user_vector 
        // move that in a function and if username or #global send to everyone or just to username
        // and spawn that in another async/thread for no blocking accept connection loop
        loop {
            for x in &mut user_vector {
                x.stream.write(b"toto").await;
                let ten_millis = time::Duration::from_secs(5);
                let now = time::Instant::now();
                thread::sleep(ten_millis);
                println!("{:?}", x.stream)
            }
        }
    }
}
