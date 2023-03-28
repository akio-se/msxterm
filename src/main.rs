use std::cmp::Ordering;
use std::io::BufReader;
use std::io::{BufRead, Read, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;
use std::time::Duration;
//use std::fmt::Display::{OsStr};
//use rand::{thread_rng, Rng};
use rustyline::{DefaultEditor, ExternalPrinter, Result, error::ReadlineError};
use clap::{App, Arg};
use std::ffi::OsStr;


const C_TAB: char = '\u{0009}';
const C_CR: char = '\u{000d}';
const C_LF: char = '\u{000a}';

const U_BREAK:u8 = 0x03;
const U_BEL:u8 = 0x07;
const U_BS:u8 = 0x08;
const U_TAB:u8 = 0x09;
const U_LF:u8 = 0x0A;



fn main() -> Result<()> {
    // Clap
/*
    let matches = App::new("msxterm")
        .about("MSX0 Terminal")
        .bin_name("kiro")
        .arg(Arg::with_name("host").required(true))
        .arg(Arg::with_name("file").required(false))
        .arg(Arg::with_name("tty").required(false))
        .get_matches();
    
    let file_path = matches.value_of_os("file").unwrap().to_string();
    let host = matches.value_of_os("host").unwrap().to_string();
*/
    let file_path = "history.txt";
    let host = "192.168.128.7:2223";

    // エディタを生成
    let mut rl = DefaultEditor::new()?;
    let mut printer = rl.create_external_printer()?;

    if rl.load_history(file_path).is_err() {
        println!("No previous history.");
    }

    // ソケットを接続
    let server_address = host;
    let mut stream = TcpStream::connect(server_address).expect("Failed to connect to server");
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    let mut last_line: String = String::new();

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
        let mut readline = rl.readline("> ");

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
                    if line == "#^G" {
                        let buf = vec![U_BEL];
                        stream.write(&buf)?;
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
    rl.save_history(file_path).unwrap();
    Ok(())
}
