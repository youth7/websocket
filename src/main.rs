use base64;
use sha1_smol;
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

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:3333").expect("绑定失败");
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

fn echo(stream: &mut TcpStream) {
    loop {
        let header = ws_reader::read_header(stream);
        let op_code = header.get_opcode();
        match header.get_opcode() {
            OperationCode::Text => {
                let payload = ws_reader::read_payload(stream, &header);
                let header = WebSocketHeader::new(true, op_code, payload.len() as u64);
                ws_writer::write_ws_message(&header, payload, stream);
            }
            OperationCode::Close => {
                ws_reader::read_payload(stream, &header);
                close_websocket(stream);
                break;
            }
            _ => {
                close_websocket(stream);
                break;
            }
        }
    }
}

fn close_websocket(stream: &mut TcpStream) {
    let close_code = 1000u16;
    let reason = "这水厂要倒闭了，哎...".as_bytes();
    let header = WebSocketHeader::new(true, OperationCode::Close, 2 + reason.len() as u64);
    stream.write(&header.bytes).unwrap();
    stream.write(&close_code.to_be_bytes()).unwrap();
    stream.write(reason).unwrap();
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
    println!("响应的消息是\n{}", response);
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
