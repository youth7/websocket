use std::{convert::TryInto, io::Read, net::TcpStream};

#[derive(Debug)]
pub enum TypeOfPayloadLength {
    OneByte,
    TwoByte,
    EightByte,
}

#[derive(Debug, Clone, Copy)]
pub enum OperationCode {
    Continue,
    Text,
    Binary,
    NonContorlPreserve(u8),
    Close,
    Ping,
    Pong,
    PreserveControl(u8),
}

pub struct WebSocketHeader {
    pub payload_length: u64,
    pub bytes: Vec<u8>,
    mask: u32,
}

impl WebSocketHeader {
    pub fn create_from_stream(stream: &mut TcpStream) -> WebSocketHeader {
        let mut buf: [u8; 14] = [0; 14];
        stream.read_exact(&mut buf[0..2]).unwrap();
        let mut payload_length = (buf[1] & 0b01111111) as u64;
        let mask_start: usize;
        match payload_length {
            0..=125 => {
                // println!("header长度类型为1");
                mask_start = 2;
            }
            126 => {
                // println!("header长度类型为2");
                mask_start = 2 + 2;
                stream.read_exact(&mut buf[2..4]).unwrap(); //读取真实长度
                payload_length = u16::from_be_bytes(buf[2..mask_start].try_into().unwrap()) as u64;
            }
            127 => {
                // println!("header长度类型为3");
                mask_start = 2 + 8;
                stream.read_exact(&mut buf[2..10]).unwrap(); //读取真实长度
                payload_length = u64::from_be_bytes(buf[2..mask_start].try_into().unwrap()) as u64;
            }
            _ => {
                panic!("不支持的长度{}", payload_length);
            }
        }
        let mask_end = mask_start + 4;
        stream.read_exact(&mut buf[mask_start..mask_end]).unwrap(); //读取mask

        WebSocketHeader {
            mask: u32::from_be_bytes(buf[mask_start..mask_end].try_into().unwrap()),
            bytes: buf[0..mask_end].to_vec(),
            payload_length,
        }
    }

    pub fn new(fin:bool, op_code:OperationCode, payload_length: u64)->WebSocketHeader{
        let op_code_value:u8;
        op_code_value = match op_code {
            OperationCode::Continue => 0,
            OperationCode::Text => 1,
            OperationCode::Binary => 2,
            OperationCode::NonContorlPreserve(value) => value,
            OperationCode::Close => 8,
            OperationCode::Ping => 9,
            OperationCode::Pong => 10,
            OperationCode::PreserveControl(value) => value,
        };
        let mut first_byte = 0b0000_0000 | (op_code_value & 0b00001111);
        if fin {
            first_byte |= 0b1000_0000;
        }
        let mut bytes:Vec<u8> = vec![first_byte];
        match payload_length {
            0..=125 => bytes.push(payload_length as u8),
            126..=0xFFFF => {
                bytes.push(126);
                bytes.append(&mut Vec::from((payload_length as u16).to_be_bytes()));
            }
            _=>{
                bytes.push(127);
                bytes.append(&mut Vec::from((payload_length as u64).to_be_bytes()));
            }
        };
        
        let header = WebSocketHeader { payload_length,  bytes, mask: 0 };
        // header.display();
        header
    }

    pub fn get_mask(&self) -> [u8; 4] {
        self.mask.to_be_bytes()
    }

    pub fn get_payload_type(&self) -> TypeOfPayloadLength {
        let length = self.bytes[1] & 0b01111111;
        match length {
            0..=125 => TypeOfPayloadLength::OneByte,
            126 => TypeOfPayloadLength::TwoByte,
            127 => TypeOfPayloadLength::EightByte,
            _ => panic!("不可能的长度：{}", length),
        }
    }

    pub fn display(&self) {
        println!(
            "Fin={}, 帧类型{:?} 长度类型{:?} 长度是{}, 掩码是{:?}",
            self.get_fin(),
            self.get_opcode(),
            self.get_payload_type(),
            self.payload_length,
            self.get_mask()
        );
    }

    pub fn get_opcode(&self) -> OperationCode {
        let code = self.bytes[0] & 0b0000_1111;
        match code {
            0x0 => OperationCode::Continue,
            0x1 => OperationCode::Text,
            0x2 => OperationCode::Binary,
            0x3..=0x7 => OperationCode::NonContorlPreserve(code),
            0x8 => OperationCode::Close,
            0x9 => OperationCode::Ping,
            0xa => OperationCode::Pong,
            _ => OperationCode::PreserveControl(code),
        }
    }

    pub fn get_fin(&self) -> u8 {
        (self.bytes[0] & 0b1000_0000) >> 7
    }
}
