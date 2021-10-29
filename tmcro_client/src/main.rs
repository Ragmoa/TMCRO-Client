use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::{TcpListener, TcpStream};
use json::JsonValue;

mod session;

fn handle_json(){

}

fn read_message(buffer: &mut BufReader<TcpStream>) -> json::Result<JsonValue>{
    let mut vecmsg = Vec::new();
    buffer.read_until(10,&mut vecmsg);
    let mut msg=std::str::from_utf8(&vecmsg).unwrap();
    let mut json_msg=json::parse(msg);
    json_msg
}

fn send_message(stream: &mut BufWriter<TcpStream>,message:String) -> (){
    let wbytes=stream.write(message.as_bytes());
    // if wbytes==message.len(){
    //     true
    // } else {
    //     false
    // }
}


fn handle_client (stream: TcpStream){

    let stream2 = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream2);
    let mut watch_test=session::Instruction::WatchByteInstruction{address:33565428};
    send_message(&mut writer,watch_test.to_json());
    // TODO: Do this in a new thread
    loop{
        let jsonData=read_message(&mut reader);

        // if (jsonData){
        //     handle_json(jsonData);
        // }
    }
}

fn main() -> std::io::Result<()>{

    let listener = TcpListener::bind("127.0.0.1:65398")?;

    //TODO: Remove this loop ?

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
