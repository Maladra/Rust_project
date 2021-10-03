use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Box, Label, TextView};



fn main () {
    let app = Application::builder()
    .application_id("Client tchat")
    .build();

    app.connect_activate(build_ui);
    app.run();

}

fn build_ui(app: &Application) {

    let window_tchat = ApplicationWindow::builder()
    .application(app)
    .default_width(800)
    .default_height(800)
    .title("Client tchat")
    .build();



    let text_view = TextView::builder()
    .editable(false)
    .cursor_visible(false)
    .build();


    text_view.buffer()
    .expect("Couldn't get window").
    set_text("zzzz\naaa");



    // temp 
    let contatcs = ["#Global", "User 1", "User 2"];
    let contact_list = gtk::ListBox::new();
    for value in contatcs.iter() {
        let label = Label::new(Some(value));
        contact_list.insert(&label, -1);
    }
    


    let global_containeer = Box::new(gtk::Orientation::Horizontal, 800);
    let left_container = Box::new(gtk::Orientation::Horizontal, 400);
    let right_container = Box::new(gtk::Orientation::Vertical, 200);

    let button_send_message = Button::builder()
    .label("Send message")
    .build();
    
    button_send_message.connect_clicked(move |_|{
        println!("Send message");
    });

    left_container.add(&contact_list);
    right_container.add(&text_view);
    right_container.add(&button_send_message);

    global_containeer.add(&left_container);
    global_containeer.add(&right_container);
    window_tchat.add(&global_containeer);
    window_tchat.show_all();

}







/*

fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf);
    buf.replace("\n", "").replace("\r", "")
}

fn exchange_with_server(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = &mut [0; 3];

    println!("Enter 'quit' when you want to leave");
    loop {
        write!(io, "> ");
        io.flush();
        match &*get_entry() {
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

fn main() {
    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("127.0.0.1:1234") {
        Ok(stream) => {
            println!("Connexion au serveur réussie !");
            exchange_with_server(stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}
*/

