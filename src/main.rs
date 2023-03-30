// MSX Term
// Copyright (c) 2023 Akio Setsumasa 
// Released under the MIT license
// https://github.com/akio-se/msxterm
//

// 初期開発中は Warnning 抑制
#![allow(unused_variables)]
#![allow(dead_code)]
//

use std::net::{Shutdown, TcpStream};
use std::thread;
//use std::sync::Arc;
use rustyline::{DefaultEditor, ExternalPrinter, Result, error::ReadlineError};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, Write, BufReader};

const C_TAB: char = '\u{0009}';
const C_CR: char = '\u{000d}';
const C_LF: char = '\u{000a}';

const U_BREAK:u8 = 0x03;
const U_BS:u8 = 0x08;
const U_CR:u8 = 0x0d;
const U_LF:u8 = 0x0a;

const MSX_UTF8: [char; 256] = [
    '\u{0000}','\u{0001}','\u{0002}','\u{0003}','\u{0004}','\u{0005}','\u{0006}','\u{0007}',
    '\u{0008}','\u{0009}','\u{000a}','\u{000b}','\u{000c}','\u{000d}','\u{000e}','\u{000f}',
    '\u{0010}','\u{0011}','\u{0012}','\u{0013}','\u{0014}','\u{0015}','\u{0016}','\u{0017}',
    '\u{0018}','\u{0019}','\u{001a}','\u{001b}','\u{001c}','\u{001d}','\u{001e}','\u{001f}',

    ' ', '!', '"', '#', '$', '%', '&', '\u{0027}', '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',

    '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G',  'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',  'X', 'Y', 'Z', '[', '\u{005c}', ']', '^', '_',

    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g',  'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w',  'x', 'y', 'z', '{', '|', '}', '~', ' ',

    '\u{2660}','\u{2665}','\u{2663}','\u{2666}','\u{25CB}','\u{25CF}','を','ぁ',
     'ぃ','ぅ','ぇ','ぉ','ゃ','ゅ','ょ','っ',
    '　','あ','い','う','え','お','か','き', 'く','け','こ','さ','し','す','せ','そ',
    '　','。','「','」','、','・','ヲ','ァ', 'ィ','ゥ','ェ','ォ','ャ','ュ','ョ','ッ',
    'ー','ア','イ','ウ','エ','オ','カ','キ','ク','ケ','コ','サ','シ','ス','セ','ソ',
    'タ','チ','ツ','テ','ト','ナ','ニ','ヌ','ネ','ノ','ハ','ヒ','フ','ヘ','ホ','マ',
    'ミ','ム','メ','モ','ヤ','ユ','ヨ','ラ', 'リ','ル','レ','ロ','ワ','ン','゛','゜',
    'た','ち','つ','て','と','な','に','ぬ', 'ね','の','は','ひ','ふ','へ','ほ','ま',
    'み','む','め','も','や','ゆ','よ','ら', 'り','る','れ','ろ','わ','ん','　','　',
];

fn msx_ascii_to_string(uv: Vec<u8>) -> String
{
    let mut cv:String = "".to_string();
    for u in uv {
        let s = u as usize;
        let c =  MSX_UTF8[s];
        cv.push(c);
    }
    cv
}

#[test]
fn msx_asci_test()
{
    let uv: Vec<u8> = [0x41,0x51,0x61,0x71,0x80,0x81,0x82,0x83,0x84,0x85].to_vec();
    let s = msx_ascii_to_string(uv);
    println!("{}", s);
    assert!(s == "AQaq♠♥♣♦○●");
}

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

fn load(command_line: &str) -> Result<Vec<String>> {
    let tokens: Vec<&str> = command_line.split(" ").collect();
    let token = tokens[1];
    let file = File::open(token).unwrap();
    let reader = BufReader::new(file);       
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    host: String,

    #[arg(short, long, value_name = "histort.txt")]
    filename: String,
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
    println!("{}", args.filename);
 
    // エディタを生成
    let mut rl = DefaultEditor::new()?;
    let mut printer = rl.create_external_printer()?;

    if rl.load_history(&args.filename).is_err() {
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
        Err(e) => {
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
                let recv_buff = msx_ascii_to_string(byte_buff).trim().to_string();
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
                let b = tmpl.as_str().replace("\r\n","\r").replace("\n","\r");
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
                        stream.write(&hex)?;
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
                        let basic = load(line).unwrap();
                        for bl in basic {
                            let mut tmp = bl.trim().to_string();
                            rl.add_history_entry(tmp.as_str())?;
                            tmp.push(C_CR);
                            stream
                            .write(tmp.as_bytes())
                            .expect("Failed to write to server");
                        }
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
