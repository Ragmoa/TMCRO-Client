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
    if (msg.chars().count() > 0){
        println!("Recived {:?}",msg);
    }
        let mut json_msg=json::parse(msg);
        json_msg
}

fn send_message(stream: &mut BufWriter<TcpStream>,message:String) -> (){
    let wbytes=stream.write(message.as_bytes());
    stream.write(b"\n");
    stream.flush();
    println!("Sent {:?} Bytes",wbytes.unwrap());
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
    let mut watch_test1=session::Instruction::WatchRangeInstruction{range:[33565490,33565497],exclude:vec![33565496]};
    let mut watch_test2=session::Instruction::WatchRangeInstruction{range:[33565505,33565509],exclude:vec![]};
    let mut watch_test3=session::Instruction::WatchRangeInstruction{range:[33565518,33565519],exclude:vec![]};
    send_message(&mut writer,watch_test1.to_json());
    send_message(&mut writer,watch_test2.to_json());
    send_message(&mut writer,watch_test3.to_json());
    // TODO: Do this in a new thread
    loop{
        let jsonData=read_message(&mut reader);
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
