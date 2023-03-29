// MSX Term
// Copyright (c) 2023 Akio Setsumasa 
// Released under the MIT license
// https://github.com/akio-se/msxterm
//

use std::cmp::Ordering;
//use std::io::BufReader;
use std::io::{BufRead, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;
//use std::time::Duration;
//use std::fmt::Display::{OsStr};
use rustyline::{DefaultEditor, ExternalPrinter, Result, error::ReadlineError};
use clap::Parser;
//use std::ffi::OsStr;


const C_TAB: char = '\u{0009}';
const C_CR: char = '\u{000d}';
const C_LF: char = '\u{000a}';

const U_BREAK:u8 = 0x03;
const U_BEL:u8 = 0x07;
const U_BS:u8 = 0x08;
const U_TAB:u8 = 0x09;
const U_LF:u8 = 0x0A;

#[test]
fn test_hex () {
    let s = "#HEX 40 41 42 43 44";
    let v = hex2u8(s);
    println!("{:?}", v);
}

fn hex2u8(hex: &str) -> Vec<u8> {
    let mut hex_vec: Vec<u8> = Vec::new();
    let tokens: Vec<&str> = hex.split(" ").collect();
    for token in &tokens[1..] {
        match u8::from_str_radix(token, 16) {
            Ok(val) => hex_vec.push(val),
            Err(_) => (),
        }
    }
    hex_vec
}

fn ascii_check() {
    let s = "Hello, こんにちは！";
    for c in s.chars() {
        if c.is_ascii() {
            println!("{} is ASCII", c);
        } else {
            println!("{} is multibyte", c);
        }
    }
}
#[test]
fn hiragana() {
    let s = "Hello, こんにちは！";
    let mut hiragana_chars: Vec<char> = Vec::new();

    for c in s.chars() {
        if c.is_ascii() {
            continue;
        }
        let k = c.to_string();
        let bytes= k.as_bytes();
        if bytes.len() == 3 && bytes[0] == 0xE3 && bytes[1] >= 0x81 && bytes[1] <= 0x82 && bytes[2] >= 0x80 && bytes[2] <= 0x9F {
            hiragana_chars.push(c);
        }
    }
    println!("Hiragana characters: {:?}", hiragana_chars);
}
#[test]
fn hiragana2() {
    let s = "こんにちは、世界！";
    let mut hiragana_chars: Vec<char> = Vec::new();

    for c in s.chars() {
        if c.is_ascii() {
            // ASCII文字は処理しない
            continue;
        } else {
            let bytes = c.to_string().into_bytes();
            if bytes.len() > 3 && bytes[0] == 0xE3 && bytes[1] >= 0x81 && bytes[1] <= 0x82 {
                // ひらがなかどうかを判定する
                if bytes[2] >= 0x81 && bytes[2] <= 0x9F || bytes[2] >= 0xE0 && bytes[2] <= 0xEF {
                    hiragana_chars.push(c);
                }
            }
        }
    }

    println!("Hiragana characters: {:?}", hiragana_chars);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    host: String,

    #[arg(short, long, value_name = "histort.txt")]
    filename: String,
}


fn main() -> Result<()> {
    // Clap
    let args = Args::parse();
    println!("{}", args.host);
    println!("{}", args.filename);
 /* 
    let matches = App::new("msxterm")
        .about("CUI Terminal for MSX0")
        .bin_name("kiro")
        .arg(Arg::with_name("host").required(true))
        .arg(Arg::with_name("file").required(false))
        .arg(Arg::with_name("tty").required(false))
        .get_matches();
    
        let host = matches.value_of("host").unwrap().to_string();
        let file_path = matches.value_of("file").unwrap().to_string();
 */
    //let file_path = "history.txt";
    //let host = "192.168.128.7:2223";

    // エディタを生成
    let mut rl = DefaultEditor::new()?;
    let mut printer = rl.create_external_printer()?;

    if rl.load_history(&args.filename).is_err() {
        println!("No previous history.");
    }

    // ソケットを接続
    let server_address = args.host;
    let mut stream = TcpStream::connect(server_address).expect("Failed to connect to server");
    let stream_clone = stream.try_clone().expect("Failed to clone stream");
    let last_line: String = String::new();

    // 受信用スレッドを作成
    let receive_thread = thread::spawn(move || {
        let mut reader = std::io::BufReader::new(&stream_clone);
        loop {
            let mut recv_buff = String::new();
            let size = reader.read_line(&mut recv_buff).unwrap();
            if size == 0 {
                break;
            }
            recv_buff = recv_buff.trim().to_string();
            //let tmp = rl.history().);
            if recv_buff.cmp(&last_line) == Ordering::Equal {
                printer
                    .print("--".to_string())
                    .expect("External print failure");
            } else {
                let s = format!("{} {}", recv_buff, last_line);
                printer.print(s).expect("External print failure");
            }
        }
    });

    // エディタ入力とコマンド送信のメインループ
    'input:loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(tmpl) => {
                //let mut line_tmp: &str = line.as_str();
                let b = tmpl.as_str().replace("\r\n","\r").replace("\n","\r");
                let lines: Vec<&str> = b.split(C_CR).collect();
                for line in lines {
                    rl.add_history_entry(line)?;

                    //println!("Line: {line}");
                    if line == "#quit" {
                        // TCP 接続終了
                        stream.shutdown(Shutdown::Both)?;
                        break 'input;
                    }
                    if line.starts_with("#hex") {
                        let hex = hex2u8(line);
                        stream.write(&hex)?;
                        continue;
                    }
                    if line == "#^J" {
                        let buf = vec![U_LF];
                        stream.write(&buf)?;
                        continue;
                    }
                    if line == "#^H" {
                        let buf = vec![U_BS];
                        stream.write(&buf)?;
                        continue;
                    }
                    if line == "#^I" {
                        let buf = vec![U_TAB];
                        stream.write(&buf)?;
                        continue;
                    }
                    let mut tmp2 = line.to_string();
                    tmp2.push(C_CR);
                    stream
                        .write(tmp2.as_bytes())
                        .expect("Failed to write to server");
                }
            }
            Err(ReadlineError::Interrupted) => {
                // break 送信
                let buf = vec![U_BREAK];
                stream.write(&buf)?;
                continue;
            }
            Err(ReadlineError::Eof) => {
                // BS 送信
                let buf = vec![U_BS];
                stream.write(&buf)?;
                continue;
            }
            Err(err) => {
                println!("Error: {err:?}");
                break;
            }
        }
    }
    // 受信スレッド終了
    receive_thread
        .join()
        .expect("Failed to join receive thread");

    // 履歴ファイル記録
    rl.save_history(& args.filename).unwrap();
    Ok(())
}
