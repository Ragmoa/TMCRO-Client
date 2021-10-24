use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};


fn handle_client (mut stream: TcpStream){
    let mut buffer = BufReader::new(&mut stream);
    let mut msg = Vec::new();
    buffer.read_until(10, &mut msg);
    let stri= String::from_utf8_lossy(&msg);
    println!("{:?}", stri);
    stream.write(b"Kek\n");
}

fn main() -> std::io::Result<()>{

    let listener = TcpListener::bind("127.0.0.1:65398")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
