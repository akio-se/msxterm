// MSX0 ASCII Code Convert Module
// Copyright (c) 2023 Akio Setsumasa 
// Released under the MIT license
// https://github.com/akio-se/msxterm
//
use std::collections::HashMap;

const U_KANA:u8 = 0xF3;

//
// MSX ASCII から UTF-8 への変換テーブル
//
const MSX_TO_UTF8: [char; 256] = [
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


// MSX ASCII のベクターから UTF-8 文字列に変換
pub fn msx_ascii_to_string(uv: Vec<u8>) -> String
{
    let mut cv:String = "".to_string();
    for u in uv {
        let s = u as usize;
        let c =  MSX_TO_UTF8[s];
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


//
// UTF-8 ひらがな から FacesKeyCode への変換
//
pub fn str_to_faces_code(input: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let hash_map: HashMap<char, Vec<u8>> = [
        ('ぁ', [0x23].to_vec()),
        ('ぃ', [0x45].to_vec()),
        ('ぅ', [0x24].to_vec()),
        ('ぇ', [0x35].to_vec()), // 大きい「え」で代用
        ('ぉ', [0x26].to_vec()),
   
        ('ゃ', [0x27].to_vec()),
        ('ゅ', [0x28].to_vec()),
        ('ょ', [0x29].to_vec()),
        ('っ', [0x5a].to_vec()),
  
        ('あ', [0x33].to_vec()),
        ('い', [0x65].to_vec()),
        ('う', [0x34].to_vec()),
        ('え', [0x35].to_vec()),
        ('お', [0x36].to_vec()),

        ('か', [0x54].to_vec()),
        ('き', [0x47].to_vec()),
        ('く', [0x48].to_vec()),
        ('け', [0x2a].to_vec()),
        ('こ', [0x42].to_vec()),
        
        ('さ', [0x58].to_vec()),
        ('し', [0x44].to_vec()),
        ('す', [0x52].to_vec()),
        ('せ', [0x50].to_vec()),
        ('そ', [0x43].to_vec()),

        ('た', [0x51].to_vec()),
        ('ち', [0x41].to_vec()),
        ('つ', [0x7A].to_vec()),
        ('て', [0x57].to_vec()),
        ('と', [0x53].to_vec()),

        ('な', [0x55].to_vec()),
        ('に', [0x49].to_vec()),
        ('ぬ', [0x31].to_vec()), 
        ('れ', [0x3b].to_vec()), // 「ね」に似ている「れ」
        ('の', [0x4b].to_vec()),

        ('は', [0x46].to_vec()),
        ('ひ', [0x56].to_vec()),
        ('ふ', [0x32].to_vec()),
        ('へ', [0xDE].to_vec()),
        ('ほ', [0x2D].to_vec()),

        ('ま', [0x4a].to_vec()),
        ('み', [0x4e].to_vec()),
        ('む', [0xdd].to_vec()),
        ('め', [0x2f].to_vec()),
        ('も', [0x4d].to_vec()),

        ('や', [0x37].to_vec()),
        ('ゆ', [0x38].to_vec()),
        ('よ', [0x39].to_vec()),

        ('ら', [0x4f].to_vec()),
        ('り', [0x4c].to_vec()),
        ('る', [0x2e].to_vec()),
        ('れ', [0x2b].to_vec()),
        ('ろ', [0x5f].to_vec()),

        ('わ', [0x30].to_vec()), 
        ('を', [0x36].to_vec()), // とりあえず「お」に変換
        ('ん', [0x59].to_vec()),
        ('　', [0x20].to_vec()),
        ('・', [0x20].to_vec()),
        ('゛', [0xc0].to_vec()),
        ('゜', [0xdb].to_vec()),
        ('ー', [0x5c].to_vec()),
        ('、', [0x3c].to_vec()),
        ('。', [0x3e].to_vec()),

        ('が', [0x54,0xc0].to_vec()),
        ('ぎ', [0x47,0xc0].to_vec()),
        ('ぐ', [0x48,0xc0].to_vec()),
        ('げ', [0x2a,0xc0].to_vec()),
        ('ご', [0x42,0xc0].to_vec()),

        ('ざ', [0x58,0xc0].to_vec()),
        ('じ', [0x44,0xc0].to_vec()),
        ('ず', [0x52,0xc0].to_vec()),
        ('ぜ', [0x50,0xc0].to_vec()),
        ('ぞ', [0x43,0xc0].to_vec()),

        ('だ', [0x51,0xc0].to_vec()),
        ('ぢ', [0x41,0xc0].to_vec()),
        ('づ', [0x7A,0xc0].to_vec()),
        ('で', [0x57,0xc0].to_vec()),
        ('ど', [0x53,0xc0].to_vec()),

        ('ば', [0x46,0xc0].to_vec()),
        ('び', [0x56,0xc0].to_vec()),
        ('ぶ', [0x32,0xc0].to_vec()),
        ('べ', [0xDE,0xc0].to_vec()),
        ('ぼ', [0x2D,0xc0].to_vec()),

        ('ぱ', [0x46,0xdb].to_vec()),
        ('ぴ', [0x56,0xdb].to_vec()),
        ('ぷ', [0x32,0xdb].to_vec()),
        ('ぺ', [0xDE,0xdb].to_vec()),
        ('ぽ', [0x2D,0xdb].to_vec()),

    ].iter().cloned().collect();

    for c in input.chars() {
        if c.is_ascii() {
            let u = match c {
                // 入力不能文字
                '!' => 0x20,
                '%' => 0x20,
                '?' => 0x20,
                '`' => 0x20,
                '{' => 0x20,
                '|' => 0x20,
                '}' => 0x20,
                '~' => 0x20,
                // コード変換
                '@' => 0xC0,
                '[' => 0xDB,
                ']' => 0xDD,
                '^' => 0xDE,
                _ => c as u8
            };
            result.push(u);
        } else {
            match hash_map.get(&c) {
                Some(value) => {
                    match result.last() {
                        Some(x) => {
                            if *x == U_KANA {
                                result.pop();
                            } else {
                                result.push(U_KANA);
                            }
                        },
                        None => {
                            result.push(U_KANA);
                        }
                    }
                    for v in value {
                        result.push(*v);
                    }
                    result.push(U_KANA);
                },
                None => ()
            }
        }
    }
    result
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
fn test_str_to_faces_code()
{
    let v=str_to_faces_code("Helloこんにちは・ぇねを・。!%?`{|}~");
    let s = dump_hex(v);
    println!("{}",s);
    assert!(s == "48 65 6C 6C 6F F3 42 59 49 41 46 20 35 36 20 3E F3 20 20 20 20 20 20 20 20 ");
}
