use base64;
use sha1_smol;
// use tungstenite::{protocol::frame::coding::OpCode, http::header};
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
    thread,
};

mod http_header_parser;
mod ws_header_parser;
mod ws_reader;
mod ws_writer;
use http_header_parser::get_sec_key;

use ws_header_parser::{OperationCode, WebSocketHeader};

use crate::ws_reader::read_payload;

pub fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").expect("绑定失败");
    println!("服务启动，监听端口3333");
    loop {
        let (stream, _) = listener.accept().expect("获取流信息失败");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        println!("new connection from {}", stream.peer_addr().unwrap());
        match get_sec_key(&mut stream) {
            Ok(client_key) => {
                upgrade_protocol(&mut stream, client_key); //todo：处理异常应该放到这里进行
                echo(&mut stream);
            }
            Err(_) => {
                println!("建立握手失败，即将自动关闭连接...");
            }
        }
    });
}

// struct ContinuePacket{
//     bytes : Vec<u8>,
//     op_code : u8
// }

fn echo(stream: &mut TcpStream) {
    let mut receiving_continue_frame = false;
    let mut op_code_of_continue: OperationCode = OperationCode::Text;
    let mut whole_playload: Vec<u8> = Vec::new();
    let mut end = false;
    while !end {
        let header = ws_reader::read_header(stream);
        let op_code = header.get_opcode();
        let fin = header.get_fin();
        if fin == 1 {
            // 非分片帧分为两种情况：1，纯粹的非分片帧；2，夹杂在分片帧中控制帧如close和ping/pong
            if !receiving_continue_frame {
                handle_non_continue_frame(stream, header, &mut end); // 情况1
                continue;
            }
            match op_code {
                //分片帧中的末帧
                OperationCode::Continue => {
                    println!("接收到分片帧的最后一帧");
                    receiving_continue_frame = false;
                    let mut fragment = read_payload(stream, &header);
                    whole_playload.append(&mut fragment);
                    let header = WebSocketHeader::new(true, op_code_of_continue.clone(), whole_playload.len() as u64);
                    ws_writer::write_ws_message(&header, &whole_playload, stream);
                }
                OperationCode::NonContorlPreserve(_) | OperationCode::PreserveControl(_) =>{
                    panic!("暂不支持在分片帧中夹杂其它控制帧");
                }
                _ => {
                    panic!("不可能的情况{:?}", op_code);
                }
            }
        } else {
            // 分片帧的类型由首帧决定
            if !receiving_continue_frame {
                println!("接收到分片帧的第1帧");
                receiving_continue_frame = true;
                whole_playload = Vec::new();
                op_code_of_continue = op_code;
            }else{
                println!("接收到分片帧的中间帧");
            }
            let mut fragment = read_payload(stream, &header);
            whole_playload.append(&mut fragment);
        }
    }
    close_websocket(stream);

    fn handle_non_continue_frame(stream: &mut TcpStream, header: WebSocketHeader, end: &mut bool) {
        let op_code = header.get_opcode();
        match op_code {
            OperationCode::Text => {
                let payload = ws_reader::read_payload(stream, &header);
                let header = WebSocketHeader::new(true, op_code, payload.len() as u64);
                ws_writer::write_ws_message(&header, &payload, stream);
            }
            OperationCode::Close => {
                ws_reader::read_payload(stream, &header);
                close_websocket(stream);
                *end = true;
            }
            OperationCode::Ping =>{
                ws_reader::read_payload(stream, &header);
                let payload = Vec::from("卧槽，pong还能带数据？".as_bytes());
                let header = WebSocketHeader::new(true, OperationCode::Pong, payload.len() as u64);
                ws_writer::write_ws_message(&header, &payload, stream);
            }

            _ => {
                println!("暂不能支持的帧{:?}", op_code);
                close_websocket(stream);
                *end = true;
            }
        };
    }
}

fn close_websocket(stream: &mut TcpStream) {
    let close_code = 1000u16;
    let reason = "这水厂要倒闭了，哎...".as_bytes();
    let header = WebSocketHeader::new(true, OperationCode::Close, 2 + reason.len() as u64);
    stream.write(&header.bytes).unwrap();
    stream.write(&close_code.to_be_bytes()).unwrap();
    stream.write(reason).unwrap();
    stream.flush().unwrap();
    println!("发送关闭帧完成..........");
}

fn upgrade_protocol(stream: &mut TcpStream, client_key: String) {
    let server_accept_key = format!(
        "Sec-WebSocket-Accept: {}",
        generate_server_accept_key(&client_key)
    );
    let headers = [
        "HTTP/1.1 101 Switching Protocols",
        "Connection: Upgrade",
        "Upgrade: websocket",
        server_accept_key.as_str(),
        "\r\n",
    ];
    let response = headers.join("\r\n");
    println!("响应的握手是\n{}", response);
    stream.write(response.as_bytes()).unwrap();
}

fn generate_server_accept_key(client_key: &str) -> String {
    println!("client key是 {}", client_key);
    let source = format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", client_key);
    println!("server accept key的source是 {}", source);
    let mut hasher = sha1_smol::Sha1::new();
    hasher.update(source.as_bytes());
    let server_accept_key = base64::encode(hasher.digest().bytes());
    println!("server accept key是 {}", server_accept_key);
    server_accept_key
}
