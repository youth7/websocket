use std::{io::Read, net::TcpStream};

enum StatusMachine {
    Others,
    CR,
    CRLF,
    CRLFCR,
    CRLFCRLF,
}

struct Status {
    status: StatusMachine,
}

impl Status {
    fn new() -> Status {
        Status {
            status: StatusMachine::Others,
        }
    }
}
const LIMIT: usize = 1024 * 5; // header最大不能超过5k;

// 正常情况下当client发起ws请求的时候，必然是先发起一个请求头然后静待协议升级的响应。
// 如果client在完整的请求头后还夹带了私货，则可能会导致各种莫名错误，但这里不管，只期待客户端会传输期待的数据。
pub fn get_sec_key(stream: &mut TcpStream) -> Result<String, ()> {
    let mut raw_headers = Vec::<u8>::new();
    // let mut headers = HashMap::<String, String>::new();
    // let mut key = String::new();
    // let mut value = String::new();
    let mut current_status = Status::new();
    let mut buf: [u8; 1024] = [0; 1024];
    'outer: loop {
        stream.read(&mut buf[..]).unwrap(); //读取http请求头，当网络慢的时候这里需要持续去读，直到整个http请求头都被读完，暂时先这样实现。
        for byte in buf {
            print!("{}", byte as char);
            if raw_headers.len() >= LIMIT {
                println!("header超出限制");
                return Err(());
            }
            if let StatusMachine::CRLFCRLF = current_status.status {
                println!("header解析已经结束");
                break 'outer;
            }
            raw_headers.push(byte);
            match byte {
                0x0D => {
                    // \r
                    match current_status.status {
                        StatusMachine::CRLF => current_status.status = StatusMachine::CRLFCR,
                        _ => current_status.status = StatusMachine::CR,
                    }
                }
                0x0A => {
                    // \n
                    match current_status.status {
                        StatusMachine::CR => current_status.status = StatusMachine::CRLF,
                        StatusMachine::CRLFCR => current_status.status = StatusMachine::CRLFCRLF,
                        _ => current_status.status = StatusMachine::Others,
                    }
                }
                _ => {
                    current_status.status = StatusMachine::Others;
                }
            }
        }
    }
    let request_string = String::from_utf8(raw_headers).unwrap();
    for header in request_string.split("\r\n") {
        //这里之前用了\n导致踩了一个巨坑，必须要\r\n
        let kv_pair: Vec<&str> = header.split(":").collect();
        println!("{:?}", kv_pair);
        let key = kv_pair[0];
        if key.to_lowercase().eq("sec-websocket-key") {
            println!("websocket 握手信息 {}", header);
            let value = kv_pair[1].trim();
            return Ok(String::from(value));
        }
    }
    println!("不含websocket握手信息 {}\n", request_string);
    Err(())
}
