extern crate websocket;
use std::thread;
use websocket::Message;
use websocket::sync::Server;




fn main() {
    let srv = Server::bind("127.0.0.1:43884").unwrap();
    for connection in srv.filter_map(Result::ok) {
        // Spawn a new thread for each connection.
        println!("New connection");
        thread::spawn(move || {
    	      let mut client = connection.accept().unwrap();
    	      let message = Message::text("Hello, client!");
    	      let _ = client.send_message(&message);
              println!("New connection");
       });
    }
}
