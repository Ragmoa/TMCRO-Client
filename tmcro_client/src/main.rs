use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use json::JsonValue;

mod session;

fn handle_json(){

}

fn read_message(buffer: &mut BufReader<&mut TcpStream>) -> json::Result<JsonValue>{
    let mut vecmsg = Vec::new();
    buffer.read_until(10,&mut vecmsg);
    let mut msg=std::str::from_utf8(&vecmsg).unwrap();
    let mut json_msg=json::parse(msg);
    json_msg
}



fn handle_client (mut stream: TcpStream){
    let mut buffer = BufReader::new(&mut stream);
    // TODO: Do this in a new thread
    loop{
        let jsonData=read_message(&mut buffer);
        // if (jsonData){
        //     handle_json(jsonData);
        // }
    }
}

fn main() -> std::io::Result<()>{

    let mut watch_test=session::Instruction::WatchByteInstruction{address:895452};
    println!("{:?}",watch_test.to_json());

    let listener = TcpListener::bind("127.0.0.1:65398")?;

    //TODO: Remove this loop ?

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
