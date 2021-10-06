use std::net::TcpStream;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};
use std::io::{Write, Read, stdin, stdout};

// TODO: Thread
// TODO: Implement JSON
// TODO: SQL database support
// TODO: Implement command like /tell
// TODO: GUI


#[derive(Deserialize ,Serialize)]
struct Message {
    username: String,
    message: String,
    message_type: String,
}


fn get_input() -> String {
    let mut s = String::new();
    let _=stdout().flush();
    stdin().read_line(&mut s);
    s.replace("\n", "").replace("\r", "")
}

fn interact_server(mut stream: TcpStream){
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = &mut [0; 3];
    loop {
        write!(io, "> ");
        // pour afficher de suite
        io.flush();
        match &*get_input() {
            "quit" => {
                println!("bye !");
                return;
            }
            line => {
                write!(stream, "{}\n", line);
                match stream.read(buf) {
                    Ok(received) => {
                        if received < 1 {
                            println!("Perte de la connexion avec le serveur");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Perte de la connexion avec le serveur");
                        return;
                    }
                }
                println!("Réponse du serveur : {:?}", buf);
            }
        }
    }  
}

fn main(){
    println!("Enter a pseudo :");
    let mut username = get_input();
    println!("Your username is : {}",username);

    match TcpStream::connect("127.0.0.1:1234") {
        Ok(stream) => {
            println!("Connexion au serveur réussie.");
            interact_server(stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}