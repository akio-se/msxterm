// MSX Term
// Copyright (c) 2023 Akio Setsumasa 
// Released under the MIT license
// https://github.com/akio-se/msxterm
//

// 初期開発中は Warnning 抑制
#![allow(unused_variables)]
#![allow(dead_code)]
//
mod msxcode;

use std::net::{Shutdown, TcpStream};
use std::thread;
//use std::sync::Arc;
use rustyline::{DefaultEditor, ExternalPrinter, Result, error::ReadlineError};

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, Write, BufReader};
use std::path::PathBuf;

const C_CR: char = '\u{000d}';

const U_BREAK:u8 = 0x03;
const U_BS:u8 = 0x08;
const U_LF:u8 = 0x0a;
const U_PAUSE:u8 = 0x7b;

fn dump_hex(uv: Vec<u8>) -> String
{
    let mut cv:String = "".to_string();
    for u in uv {
        let tmp = format!("{:02X} ", u);
        cv.push_str(tmp.as_str());
    }
    cv
}

#[test]
fn test_dump_hex() {
    let uv: Vec<u8> = [0x41,0x51,0x61,0x71,0x80,0x81,0x8A,0xB3,0xC4,0x55].to_vec();
    let s = dump_hex(uv);
    println!("{}",s);
    assert!(s == "41 51 61 71 80 81 8A B3 C4 55 ");
}


#[test]
fn test_hex () {
    let s = "#HEX 40 41 42 43 44";
    let v = hex2u8(s);
    println!("{:?}", v);
}

fn hex2u8(hex: &str) -> Vec<u8> {
    let mut hex_vec: Vec<u8> = Vec::new();
    let tokens: Vec<&str> = hex.split(' ').collect();
    for token in &tokens[1..] {
        if let Ok(val) = u8::from_str_radix(token, 16) {
            hex_vec.push(val)   
        }
    }
    hex_vec
}

//
// 指定されたファイルをロードしてvec<String>を返す
//
fn load(command_line: &str) -> Result<Vec<String>> {
    let tokens: Vec<&str> = command_line.split(' ').collect();
    let path_str = tokens[1];
    // ファイルのパス
    let path = PathBuf::from(path_str.trim_matches('\"'));
    let file = File::open(path)?;
    let reader = BufReader::new(file);       
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

//
// コマンドラインオプションの設定
//
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    host: String,

    #[arg(short, long, value_name = "history file", default_value = "history.txt")]
    file: String,
}

struct Msxterm {
    dump_mode:bool,
}


fn main() -> Result<()> {
    // 変数初期化
    let mut msxterm = Msxterm { 
        dump_mode: false
    };

    // コマンドライン引数取得
    let args = Args::parse();
    println!("{}", args.host);
    println!("{}", args.file);
 
    // エディタを生成
    let mut rl = DefaultEditor::new()?;
    let mut printer = rl.create_external_printer()?;
    if rl.load_history(&args.file).is_err() {
        println!("No previous history.");
    }

    // ソケットを接続
    let server_address = args.host;
    println!("Connecting... {}", server_address);
    let mut stream;
    let r = TcpStream::connect(server_address);
    match r {
        Ok(s) => {
            stream = s;
            println!("connected.");
        },
        Err(_) => {
            eprintln!("Failed to connect.");
            return Ok(());
        }, 
    }

    // 受信用スレッドを作成
    let stream_clone = stream.try_clone().expect("Failed to clone stream");
    let receive_thread = thread::spawn(move || {
        let mut reader = std::io::BufReader::new(&stream_clone);
        loop {
            let mut byte_buff: Vec<u8> = [0x00_u8; 0].to_vec();
            let size = reader.read_until(U_LF, &mut byte_buff).unwrap();
            if size == 0 {
                break;
            }
            if msxterm.dump_mode {
                let recv_buff = dump_hex(byte_buff);
                printer.print(recv_buff).expect("External print failure");
            } else {
                let recv_buff = msxcode::msx_ascii_to_string(byte_buff).trim().to_string();
                printer.print(recv_buff).expect("External print failure");    
            }
        }
    });

    // エディタ入力とコマンド送信のメインループ
    'input:loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(tmpl) => {
                //let mut line_tmp: &str = line.as_str();
                let b = tmpl.as_str().replace("\r\n","\r").replace('\n',"\r");
                let lines: Vec<&str> = b.split(C_CR).collect();
                for line in lines {
                    rl.add_history_entry(line)?;

                    if line.starts_with("#quit") {
                        // TCP 接続終了
                        stream.shutdown(Shutdown::Both)?;
                        break 'input;
                    }
                    if line.starts_with("#hex") {
                        let hex = hex2u8(line);
                        stream.write_all(&hex)?;
                        continue;
                    }
                    if line.starts_with("#dump_on") {
                        msxterm.dump_mode = true;
                        continue;             
                    }
                    if line.starts_with("#dump_off") {
                        msxterm.dump_mode = false;
                        continue;             
                    }
                    if line.starts_with("#clear_history") {
                        rl.clear_history().unwrap();
                        continue;
                    }
                    if line.starts_with("#load") {
                        match load(line) {
                            Ok(basic) => {
                                for bl in basic {
                                    let mut tmp = bl.trim().to_string();
                                    rl.add_history_entry(tmp.as_str())?;
                                    tmp.push(C_CR);
                                    stream
                                    .write_all(tmp.as_bytes())
                                    .expect("Failed to write to server");
                                }
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                        continue;
                    }

                    let mut tmp2 = line.to_string();
                    tmp2.push(C_CR);
                    let faces_code = msxcode::str_to_faces_code(tmp2.as_str());
                    stream
                        .write_all(&faces_code)
                        .expect("Failed to write to server");
                }
            }
            Err(ReadlineError::Interrupted) => {
                // break 送信
                let buf = vec![U_BREAK];
                stream.write_all(&buf)?;
                continue;
            }
            Err(ReadlineError::Eof) => {
                // BS 送信
                let buf = vec![U_PAUSE];
                stream.write_all(&buf)?;
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
    rl.save_history(& args.file).unwrap();
    Ok(())
}
