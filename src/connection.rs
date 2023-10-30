/// (TCP IP | Serial Port) Connection Module
/// Copyright (c) 2023 Akio Setsumasa 
/// Released under the MIT license
/// https://github.com/akio-se/msxterm

use std::io::{Read, Write};
//use serialport::SerialPort;
use serial2::SerialPort;

use std::net::{IpAddr};
use regex::Regex;
//use std::thread;
//use std::time::{Duration};
//use std::sync::{Arc, Mutex};


/// Connection のトレイト定義
pub trait Connection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    fn write(&mut self, buf: &[u8]);
    fn flush(&mut self) -> std::io::Result<()>;
    fn close(&mut self) -> std::io::Result<()>;
}

/// TCP/IP コネクション
pub struct TcpConnection {
    stream: std::net::TcpStream,
}

impl TcpConnection {

}


impl Connection for TcpConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
    fn write(&mut self, buf: &[u8]) {
        self.stream.write_all(buf).expect("err"); 
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush()
    }
    fn close(&mut self) -> std::io::Result<()> {
        self.stream.shutdown(std::net::Shutdown::Write)
    }
}

/// シリアルポート コネクション
pub struct SerialConnection {
    port: serial2::SerialPort,
}

impl Connection for SerialConnection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buf)
    }
    fn write(&mut self, buf: &[u8]) {
        self.port.write_all(buf).expect("err")
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.port.flush()
    }
    fn close(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub enum ConnectionType {
    Tcp(std::net::TcpStream),
    Serial(serial2::SerialPort),
    BadParam(String),
}


pub fn create_connection(param:&str) -> ConnectionType {
    if is_valid_ip_port(param) {
        let tp = std::net::TcpStream::connect(param);
        match tp {
            Ok(t) => ConnectionType::Tcp(t),
            Err(e) => ConnectionType::BadParam(e.to_string())
        }
    } else if is_varid_serial_port(param) {
        let sp = SerialPort::open(param,115_200 );
        match sp {
            Ok(s) => ConnectionType::Serial(s),
            Err(e) => ConnectionType::BadParam(e.to_string())
        }
    } else {
        ConnectionType::BadParam("ParamError".to_string())
    }
}



enum ConnectionError {

}

impl ConnectionType {

    pub fn write(&mut self, buff: &[u8]) -> Result<(), String> {
        match self {
            ConnectionType::Tcp(stream) => {
                match stream.write_all(buff) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            },
            ConnectionType::Serial(serialport) => {
                match serialport.write_all(buff) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            },
            ConnectionType::BadParam(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        match self {
            ConnectionType::Tcp(stream) => {
                match stream.read(buf) {
                    Ok(size) => Ok(size),
                    Err(e) => Err(e.to_string()),
                }
            },
            ConnectionType::Serial(serialport) => {
                match serialport.read(buf) {
                    Ok(size) => Ok(size),
                    Err(e) => Err(e.to_string()),
                }
            },
            ConnectionType::BadParam(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn flush(&mut self) -> Result<(), String>  {
        match self {
            ConnectionType::Tcp(stream) => {
                match stream.flush() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            },
            ConnectionType::Serial(sp) => {
                match sp.flush() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string()),
                }
            },
            ConnectionType::BadParam(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        match self {
            ConnectionType::Tcp(stream) => {
                match stream.shutdown(std::net::Shutdown::Write) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e.to_string())
                }
            },
            ConnectionType::Serial(sp) => {
                Ok(())
            },
            ConnectionType::BadParam(e) => {
                Err(e.to_string())
            }
        }
    }
}


// 入力された文字列が IPアドレス:Port を示す文字列かどうかチェック
fn is_valid_ip_port(input: &str) -> bool {
    let parts: Vec<&str> = input.split(':').collect();
    // Check that there are two parts
    if parts.len() != 2 {
        return false;
    }
    // Check that the IP address part is valid
    match parts[0].parse::<IpAddr>() {
        Ok(IpAddr::V4(_)) => (),
        Ok(IpAddr::V6(_)) => (),
        _ => return false,
    }
    // Check that the port number part is valid
    match parts[1].parse::<u16>() {
        Ok(_) => (),
        _ => return false,
    }
    true
}

// 入力された文字列がシリアルポートを示す文字列かどうかチェック
// OS 毎に判定が変わる。
fn is_varid_serial_port(path: &str) -> bool {
    #[cfg(target_os = "windows")]
    const SERIAL_PORT_REGEX: &str = r"^COM\d+$";
    
    #[cfg(target_os = "linux")]
    const SERIAL_PORT_REGEX: &str = r"^/dev/ttyS\d+$|^/dev/ttyUSB\d+$";
    
    #[cfg(target_os = "macos")]
    const SERIAL_PORT_REGEX: &str = r"^/dev/cu\.usbserial-\w+$|^/dev/tty\..+$";

    let serial_port_regex = Regex::new(SERIAL_PORT_REGEX).unwrap();
    serial_port_regex.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_port() {
        assert_eq!(is_valid_ip_port("192.168.128.7:2223"), true);
        assert_eq!(is_valid_ip_port("192.234.456.122:2223"), false);
        assert_eq!(is_valid_ip_port("192.234.156.122:70223"), false);
        assert_eq!(is_valid_ip_port("localhot:2223"), false);
    }

    #[test]
    fn test_serial_path() {
        #[cfg(target_os = "windows")]
        {
            assert_eq!(is_varid_serial_port("COM3"), true);
            assert_eq!(is_varid_serial_port("com1"), false);
        }
        #[cfg(target_os = "linux")]
        {
            assert_eq!(is_varid_serial_port("/dev/ttyS1"), true);
            assert_eq!(is_varid_serial_port("/dev/ttyUSB1"), true);
        }
        #[cfg(target_os = "macos")]
        {
            assert_eq!(is_varid_serial_port("/dev/tty.usbserial-559B0204231"), true);
            assert_eq!(is_varid_serial_port("/dev/cu.usbserial-559B0204231"), true);    
        }
    }

    #[test]
    fn test_create_con() {
//      let mut contype = create_connection("/dev/cu.usbserial-569C0128081");
        let mut contype = create_connection("192.168.128.13:2223");
        match contype {
            ConnectionType::Tcp(mut ts) => {
                //println!("TCP {:?}", ts);
                ts.write(b"Test TCP\r");

            },
            ConnectionType::Serial(mut sr) => {
                //println!("Serial {:?}", sr);
                sr.write(b"Test Serial\r");
            },
            ConnectionType::BadParam(mut e)=> {
                println!("error {}", e);
            }
        }
    }

}
