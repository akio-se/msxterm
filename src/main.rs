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
use std::collections::{BTreeMap, HashMap};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, Write, BufReader,BufWriter};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

const C_CR: char = '\u{000d}';
const C_LF: char = '\u{000a}';

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
    dump_mode: bool,
    lower_mode: bool,
    prog_buff:BTreeMap<u16, String>,
    t_com: HashMap<String, String>,
}

impl Msxterm {
    pub fn new() -> Msxterm {
        Msxterm { 
            dump_mode: false, 
            lower_mode: true,
            prog_buff: BTreeMap::new(), 
            t_com: HashMap::new(),
        }
    }
    fn init(&mut self) {
        self.t_com.insert("#list".to_string(), "list Program".to_string());
    }

    pub fn parse_basic(&mut self, line:&str) {
        let mut iter = line.splitn(2,' ');
        if let Some(number) = iter.next() {
            if let Ok(number) = number.parse::<u16>() {
                if let Some(instruction) = iter.next() {
                    self.prog_buff.insert(number, instruction.trim().to_owned());
                } else {
                    self.prog_buff.remove(&number);
                }
            }
        }
    }

    pub fn print_basic(&mut self, start:u16, end:u16) -> Vec<String> {
        // println!("list {} {}", start, end);
        let mut history = Vec::new(); 
        if let Some(maxline) = self.prog_buff.iter().max() {
            let maxlen = maxline.0.to_string().len();
            let iter = self.prog_buff.range(start..=end);
            //for (num, inst) in &self.prog_buff {
            for (num ,inst) in iter {
                let padding = " ".repeat(maxlen - num.to_string().len());
                println!("{}\x1b[36m{}\x1b[0m {}",padding, num, inst);
                history.push(std::format!("{} {}", num, inst ));
            }
        }
        history
    }

    pub fn clear_basic(&mut self) {
        self.prog_buff.clear();
    }

    pub fn save_program(&self, command_line:&str) {
        let tokens: Vec<&str> = command_line.split(' ').collect();
        let path_str = tokens[1];
        // ファイルのパス
        let path = PathBuf::from(path_str.trim_matches('\"'));
        // ファイルを作成する
        let file = File::create(path).expect("Failed to create file");
        // ファイルに書き込むためのBufWriterを作成する
        let mut writer = BufWriter::new(file);

        // BTreeMapを文字列に変換してファイルに書き込む
        for (line_number, program) in self.prog_buff.iter() {
            let line = format!("{} {}\n", line_number, program);
            writer.write(line.as_bytes()).expect("Failed to write to file");
        }
        // ファイルをクローズする
        writer.flush().expect("Failed to flush buffer");
    }    


}

pub fn parse_command(command: &str) -> (Option<u16>, Option<u16>) {
    let mut parts = command.trim().split(" ");
    let _ = parts.next(); // Skip the command name
    let range = parts.next();
    match range {
        None => (None, None),
        Some(range) => {
            let mut range_parts = range.split("-");
            let start = range_parts.next().and_then(|x| x.parse().ok());
            let end = range_parts.next().and_then(|x| x.parse().ok());
            (start, end)
        }
    }
}

enum Command {
    DumpModeOn,
    DumpModeOff,
}


#[test]
fn test_msxterm () {
    let mut mt = Msxterm::new();
    mt.init();

    let basfile = load("#load ./src/test.bas").unwrap();
    for s in basfile {
        mt.parse_basic(&s);
    }
    mt.print_basic(0,65530);

    let (st,ed) = parse_command("#list 10-20");
    println!("list {}-{}", st.unwrap_or(0), ed.unwrap_or(65530));

    let (st,ed) = parse_command("#list 40-");
    println!("list {}-{}", st.unwrap_or(0), ed.unwrap_or(65530));

    let (st,ed) = parse_command("#list -50");
    println!("list {}-{}", st.unwrap_or(0), ed.unwrap_or(65530));

    let (st,ed) = parse_command("#list 50");
    println!("list {}-{}", st.unwrap_or(0), ed.unwrap_or(65530));

    /*
    mt.parse_basic("1000 print 10 + 20");
    mt.parse_basic("100 cls");
    mt.parse_basic("1010 goto 1000");
    mt.parse_basic("20010 gosub 1000");
    mt.print_basic(0,65530);
    mt.parse_basic("100");
    mt.parse_basic("#list");
    mt.print_basic(0,65530);

    mt.parse_basic("1010 for i=0 to 100");
    mt.parse_basic("1020 PRINT I");
    mt.parse_basic("1030 NEXT I");
    mt.parse_basic("20010 gosub 1000");
    mt.print_basic(2000, 65530);
    */
}

fn lower_program(input:&str) -> String {
    let mut output = String::new();
    let mut is_quoted = false;

    for line in input.lines() {
        let mut tmp="".to_string();
        let tline = line.trim();
        for c in tline.chars() { 
            if c == '"' {
                is_quoted = !is_quoted;
            }
            if is_quoted {
                tmp.push(c);
            } else {
                let lowercase = c.to_lowercase().next().unwrap();
                tmp.push(lowercase);
            }
        }
        if tmp.starts_with("rem") {
            output.push_str(line.trim());
            output.push(C_CR);
        } else {
            output.push_str(&tmp);
            output.push(C_CR);
        }
    }
    output
}

#[test]
fn test_lower_program() {
    let text = "input text \"PrintHello\"
                        REM Akio Setsumasa
                        Print A$ + B$
                        REM This Program is Free
                      END";
    println!("{}", text);
    let result = lower_program(text);
    println!("{}", result);
}

fn main() -> Result<()> {
    // 変数初期化
    let mut msxterm = Msxterm::new();
    msxterm.init();

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

    // 通信スレッドとメインスレッド間でやりとりするチャンネルを作成する
    let (tx, rx): (Sender<Command>, Receiver<Command>) = channel();
    // 受信用スレッドを作成
    let stream_clone = stream.try_clone().expect("Failed to clone stream");
    let receive_thread = thread::spawn(move || {
        let mut dump_mode = false;
        let mut reader = std::io::BufReader::new(&stream_clone);
        loop {
            if let Ok(command) = rx.recv_timeout(Duration::from_millis(1)) {
                match command {
                    Command::DumpModeOn => dump_mode = true,
                    Command::DumpModeOff => dump_mode = false,
                }
            }
            let mut byte_buff: Vec<u8> = [0x00_u8; 0].to_vec();
            let size = reader.read_until(U_LF, &mut byte_buff).unwrap();
            if size == 0 {
                break;
            }
            if dump_mode {
                let recv_buff = dump_hex(byte_buff);
                printer.print(recv_buff).expect("External print failure");
            } else {
                let recv_buff = msxcode::msx_ascii_to_string(byte_buff);
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
                        tx.send(Command::DumpModeOn).expect("Thread sync Error");
                        println!("Output dump mode On");
                        continue;             
                    }
                    if line.starts_with("#dump_off") {
                        tx.send(Command::DumpModeOff).expect("Thread sync Error");
                        println!("Output dump mode Off");
                        continue;             
                    }
                    if line.starts_with("#lowsend_on") {
                        msxterm.lower_mode = true;
                        println!("Lower Case send mode On");
                        continue;
                    }
                    if line.starts_with("#lowsend_off") {
                        msxterm.lower_mode = false;
                        println!("Lower Case send mode Off");
                        continue;
                    }
                    if line.starts_with("#clear_history") {
                        rl.clear_history().unwrap();
                        println!("History is cleared.");
                        continue;
                    }
                    if line.starts_with("#new") {
                        msxterm.prog_buff.clear();
                        println!("Program Buffer is cleared.");
                        continue;
                    }
                    if line.starts_with("#load") {
                        match load(line) {
                            Ok(basic) => {
                                let mut ld_program = "".to_string();
                                for bl in basic {
                                    let mut tmp = bl.trim().to_string();
                                    msxterm.parse_basic(tmp.as_str());
                                    rl.add_history_entry(tmp.as_str())?;
                                    tmp.push(C_CR);
                                    ld_program.push_str(&tmp);
                                }
                                if msxterm.lower_mode {
                                    ld_program = lower_program(&ld_program);
                                }
                                stream
                                .write_all(ld_program.as_bytes())
                                .expect("Failed to write to server");
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                        println!("Ok");
                        continue;
                    }
                    if line.starts_with("#list") {
                        let cols = line.split(' ');
                        for history in msxterm.print_basic(0, 65530) {
                            rl.add_history_entry(history)?;
                        }
                        continue;
                    }
                    if line.starts_with("#save") {
                        msxterm.save_program(&line);
                        println!("Ok");
                        continue;
                    }

                    msxterm.parse_basic(line);

                    let mut tmp2 = line.to_string();
                    tmp2.push(C_CR);
                    if msxterm.lower_mode {
                        tmp2 = lower_program(&tmp2);
                    }
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
