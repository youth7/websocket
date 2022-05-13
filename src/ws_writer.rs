use std::{net::TcpStream, io::Write};
use super::WebSocketHeader;

pub fn write_ws_message(header:&WebSocketHeader, payload: &Vec<u8>, stream: &mut TcpStream) {
    stream.write(&header.bytes).unwrap();
    // match length {
    //     0..=125 => {
    //         println!("写出8位长度数据");
    //         stream.write(&[first_byte, second_byte as u8]).unwrap();
    //     },
    //     126..=0xFFFF =>{
    //         println!("写出16位长度数据");
    //         stream.write(&[first_byte, 126 as u8]).unwrap();
    //         stream.write(&(length as u16).to_be_bytes()).unwrap();
    //     }
    //     _=>{
    //         println!("写出64位长度数据");
    //         stream.write(&[first_byte, 127 as u8]).unwrap();
    //         stream.write(&(length as u64).to_be_bytes()).unwrap();
    //     }
    // }
    stream.write(&payload).unwrap();
}