use crate::ws_header_parser::OperationCode;
use std::{io::Read, net::TcpStream, convert::TryInto};

// fn print_first_2_byte(slice: &[u8]) {
//     let value = u16::from_be_bytes(slice.try_into().unwrap());
//     println!("前两个字节为：{:x}", value);
//     for i in 0..16 {
//         let mask = 0x8000u16 >> i;
//         print!("{},", (value & mask) >> (15 - i));
//     }
//     println!("");
// }
use super::WebSocketHeader;

pub fn read_header(stream: &mut TcpStream) -> WebSocketHeader {
    let ws_header = WebSocketHeader::create_from_stream(stream);
    print!("client发送的header信息是：");
    ws_header.display();
    ws_header
}

pub fn read_payload(stream: &mut TcpStream, ws_header: &WebSocketHeader) -> Vec<u8> {
    let mut buf: Vec<u8> = vec![0u8; ws_header.payload_length as usize];
    let mask = ws_header.get_mask();
    // println!("{} {}", buf.len(), ws_header.payload_length);
    stream.read_exact(buf.as_mut_slice()).unwrap();
    for i in 0..ws_header.payload_length as usize {
        let j = i % 4;
        buf[i] = buf[i] ^ mask[j];
    }
    match ws_header.get_opcode() {
        OperationCode::Close => match ws_header.payload_length {
            0 | 1 => return buf,
            v @ _ => {
                let close_code = u16::from_be_bytes(buf[0..2].try_into().unwrap());
                println!("关闭码是{}", close_code);
                if v > 2 {
                    println!("{}", std::str::from_utf8(&buf[2..]).unwrap())
                }
            }
        },
        _ => println!("{}", std::str::from_utf8(buf.as_slice()).unwrap()),
    }
    buf
}
