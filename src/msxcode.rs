// MSX0 ASCII Code Convert Module
// Copyright (c) 2023 Akio Setsumasa 
// Released under the MIT license
// https://github.com/akio-se/msxterm
//
use std::collections::HashMap;

const U_KANA:u8 = 0xF3;
const U_CAPS:u8 = 0xF2;

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
    'み','む','め','も','や','ゆ','よ','ら', 'り','る','れ','ろ','わ','ん','　','■',
];

const MSX_TO_GRAPH: [char;256] = [
    '\u{0000}','\u{0001}','\u{0002}','\u{0003}','\u{0004}','\u{0005}','\u{0006}','\u{0007}',
    '\u{0008}','\u{0009}','\u{000a}','\u{000b}','\u{000c}','\u{000d}','\u{000e}','\u{000f}',
    '\u{0010}','\u{0011}','\u{0012}','\u{0013}','\u{0014}','\u{0015}','\u{0016}','\u{0017}',
    '\u{0018}','\u{0019}','\u{001a}','\u{001b}','\u{001c}','\u{001d}','\u{001e}','\u{001f}',

    ' ', '!', '"', '#', '$', '%', '&', '\u{0027}', '(', ')', '*', '+', ',', '-', '.', '/',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?',

    '\u{3000}', '月', '火','水','木','金','土','日','年','円','時','分','秒','百','千','万',
    'π',  '┻', '┳', '┫', '┣', '╋','┃', '━', '┏','┓', '┗', '┛','\u{2715}', '大','中','小',

    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g',  'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w',  'x', 'y', 'z', '{', '|', '}', '~', ' ',

    '\u{2660}','\u{2665}','\u{2663}','\u{2666}','\u{25CB}','\u{25CF}','を','ぁ',
     'ぃ','ぅ','ぇ','ぉ','ゃ','ゅ','ょ','っ',
    '\u{3000}','あ','い','う','え','お','か','き', 'く','け','こ','さ','し','す','せ','そ',
    '\u{3000}','。','「','」','、','・','ヲ','ァ', 'ィ','ゥ','ェ','ォ','ャ','ュ','ョ','ッ',
    'ー','ア','イ','ウ','エ','オ','カ','キ','ク','ケ','コ','サ','シ','ス','セ','ソ',
    'タ','チ','ツ','テ','ト','ナ','ニ','ヌ','ネ','ノ','ハ','ヒ','フ','ヘ','ホ','マ',
    'ミ','ム','メ','モ','ヤ','ユ','ヨ','ラ', 'リ','ル','レ','ロ','ワ','ン','゛','゜',
    'た','ち','つ','て','と','な','に','ぬ', 'ね','の','は','ひ','ふ','へ','ほ','ま',
    'み','む','め','も','や','ゆ','よ','ら', 'り','る','れ','ろ','わ','ん','\u{3000}','■',
];

// MSX ASCII のベクターから UTF-8 文字列に変換
pub fn msx_ascii_to_string(uv: Vec<u8>) -> String
{
    let mut graph = false;
    let mut cv:String = "".to_string();
    for u in uv {
        let s = u as usize;
        if u == 1 {
            graph = true;
        } else {
            if graph {
                let c = MSX_TO_GRAPH[s];
                cv.push(c);
                graph = false;
            } else {
                let c =  MSX_TO_UTF8[s];
                cv.push(c);
            }
        }
    }
    cv
}

#[test]
fn msx_asci_test()
{
    let uv: Vec<u8> = vec![0x41,0x51,0x61,0x71,0x80,0x81,0x82,0x83,0x84,0x85];
    let s = msx_ascii_to_string(uv);
    println!("{}", s);
    assert!(s == "AQaq♠♥♣♦○●");
}

enum FacesCode {
    Ascii(u8),
    Hiragana(Vec<u8>),
    Katakana(Vec<u8>),
}


//
// UTF-8 ひらがな から FacesKeyCode への変換
//
pub fn str_to_faces_code(input: &str) -> Vec<u8> {

    // 変換用ハッシュテーブル
    let hash_map = HashMap::from([
        // ASCII 入力不能文字
        ('!', FacesCode::Ascii(0x20)),
        ('%', FacesCode::Ascii(0x20)),
        ('?', FacesCode::Ascii(0x20)),
        ('`', FacesCode::Ascii(0x20)),
        ('{', FacesCode::Ascii(0x20)),
        ('|', FacesCode::Ascii(0x20)),
        ('}', FacesCode::Ascii(0x20)),
        ('~', FacesCode::Ascii(0x20)),

        // ASCII コード変換文字
        ('@', FacesCode::Ascii(0xC0)),
        ('[', FacesCode::Ascii(0xDB)),
        (']', FacesCode::Ascii(0xDD)),
        ('^', FacesCode::Ascii(0xDE)),

        // ひらがな
        ('ぁ', FacesCode::Hiragana(vec![0x23])),
        ('ぃ', FacesCode::Hiragana(vec![0x45])),
        ('ぅ', FacesCode::Hiragana(vec![0x24])),
        ('ぇ', FacesCode::Hiragana(vec![0x35])), // 大きい「え」で代用
        ('ぉ', FacesCode::Hiragana(vec![0x26])),
        ('ゃ', FacesCode::Hiragana(vec![0x27])),
        ('ゅ', FacesCode::Hiragana(vec![0x28])),
        ('ょ', FacesCode::Hiragana(vec![0x29])),
        ('っ', FacesCode::Hiragana(vec![0x5a])),
  
        ('あ', FacesCode::Hiragana(vec![0x33])),
        ('い', FacesCode::Hiragana(vec![0x65])),
        ('う', FacesCode::Hiragana(vec![0x34])),
        ('え', FacesCode::Hiragana(vec![0x35])),
        ('お', FacesCode::Hiragana(vec![0x36])),

        ('か', FacesCode::Hiragana(vec![0x54])),
        ('き', FacesCode::Hiragana(vec![0x47])),
        ('く', FacesCode::Hiragana(vec![0x48])),
        ('け', FacesCode::Hiragana(vec![0x2a])),
        ('こ', FacesCode::Hiragana(vec![0x42])),
        
        ('さ', FacesCode::Hiragana(vec![0x58])),
        ('し', FacesCode::Hiragana(vec![0x44])),
        ('す', FacesCode::Hiragana(vec![0x52])),
        ('せ', FacesCode::Hiragana(vec![0x50])),
        ('そ', FacesCode::Hiragana(vec![0x43])),

        ('た', FacesCode::Hiragana(vec![0x51])),
        ('ち', FacesCode::Hiragana(vec![0x41])),
        ('つ', FacesCode::Hiragana(vec![0x7A])),
        ('て', FacesCode::Hiragana(vec![0x57])),
        ('と', FacesCode::Hiragana(vec![0x53])),

        ('な', FacesCode::Hiragana(vec![0x55])),
        ('に', FacesCode::Hiragana(vec![0x49])),
        ('ぬ', FacesCode::Hiragana(vec![0x31])), 
        ('ね', FacesCode::Hiragana(vec![0x3b])), // 「ね」に似ている「れ」
        ('の', FacesCode::Hiragana(vec![0x4b])),

        ('は', FacesCode::Hiragana(vec![0x46])),
        ('ひ', FacesCode::Hiragana(vec![0x56])),
        ('ふ', FacesCode::Hiragana(vec![0x32])),
        ('へ', FacesCode::Hiragana(vec![0xDE])),
        ('ほ', FacesCode::Hiragana(vec![0x2D])),

        ('ま', FacesCode::Hiragana(vec![0x4a])),
        ('み', FacesCode::Hiragana(vec![0x4e])),
        ('む', FacesCode::Hiragana(vec![0xdd])),
        ('め', FacesCode::Hiragana(vec![0x2f])),
        ('も', FacesCode::Hiragana(vec![0x4d])),

        ('や', FacesCode::Hiragana(vec![0x37])),
        ('ゆ', FacesCode::Hiragana(vec![0x38])),
        ('よ', FacesCode::Hiragana(vec![0x39])),

        ('ら', FacesCode::Hiragana(vec![0x4f])),
        ('り', FacesCode::Hiragana(vec![0x4c])),
        ('る', FacesCode::Hiragana(vec![0x2e])),
        ('れ', FacesCode::Hiragana(vec![0x2b])),
        ('ろ', FacesCode::Hiragana(vec![0x5f])),

        ('わ', FacesCode::Hiragana(vec![0x30])), 
        ('を', FacesCode::Hiragana(vec![0x36])), // とりあえず「お」に変換
        ('ん', FacesCode::Hiragana(vec![0x59])),
        ('　', FacesCode::Hiragana(vec![0x20])),
        ('・', FacesCode::Hiragana(vec![0x20])),
        ('゛', FacesCode::Hiragana(vec![0xc0])),
        ('゜', FacesCode::Hiragana(vec![0xdb])),
        ('ー', FacesCode::Hiragana(vec![0x5c])),
        ('、', FacesCode::Hiragana(vec![0x3c])),
        ('。', FacesCode::Hiragana(vec![0x3e])),

        ('が', FacesCode::Hiragana(vec![0x54,0xc0])),
        ('ぎ', FacesCode::Hiragana(vec![0x47,0xc0])),
        ('ぐ', FacesCode::Hiragana(vec![0x48,0xc0])),
        ('げ', FacesCode::Hiragana(vec![0x2a,0xc0])),
        ('ご', FacesCode::Hiragana(vec![0x42,0xc0])),

        ('ざ', FacesCode::Hiragana(vec![0x58,0xc0])),
        ('じ', FacesCode::Hiragana(vec![0x44,0xc0])),
        ('ず', FacesCode::Hiragana(vec![0x52,0xc0])),
        ('ぜ', FacesCode::Hiragana(vec![0x50,0xc0])),
        ('ぞ', FacesCode::Hiragana(vec![0x43,0xc0])),

        ('だ', FacesCode::Hiragana(vec![0x51,0xc0])),
        ('ぢ', FacesCode::Hiragana(vec![0x41,0xc0])),
        ('づ', FacesCode::Hiragana(vec![0x7A,0xc0])),
        ('で', FacesCode::Hiragana(vec![0x57,0xc0])),
        ('ど', FacesCode::Hiragana(vec![0x53,0xc0])),

        ('ば', FacesCode::Hiragana(vec![0x46,0xc0])),
        ('び', FacesCode::Hiragana(vec![0x56,0xc0])),
        ('ぶ', FacesCode::Hiragana(vec![0x32,0xc0])),
        ('べ', FacesCode::Hiragana(vec![0xDE,0xc0])),
        ('ぼ', FacesCode::Hiragana(vec![0x2D,0xc0])),

        ('ぱ', FacesCode::Hiragana(vec![0x46,0xdb])),
        ('ぴ', FacesCode::Hiragana(vec![0x56,0xdb])),
        ('ぷ', FacesCode::Hiragana(vec![0x32,0xdb])),
        ('ぺ', FacesCode::Hiragana(vec![0xDE,0xdb])),
        ('ぽ', FacesCode::Hiragana(vec![0x2D,0xdb])),

        // カタカナ
        ('ァ', FacesCode::Katakana(vec![0x23])),
        ('ィ', FacesCode::Katakana(vec![0x45])),
        ('ゥ', FacesCode::Katakana(vec![0x24])),
        ('ェ', FacesCode::Katakana(vec![0x35])), // 大きい「エ」で代用
        ('ォ', FacesCode::Katakana(vec![0x26])),
   
        ('ャ', FacesCode::Katakana(vec![0x27])),
        ('ュ', FacesCode::Katakana(vec![0x28])),
        ('ョ', FacesCode::Katakana(vec![0x29])),
        ('ッ', FacesCode::Katakana(vec![0x5a])),
  
        ('ア', FacesCode::Katakana(vec![0x33])),
        ('イ', FacesCode::Katakana(vec![0x65])),
        ('ウ', FacesCode::Katakana(vec![0x34])),
        ('エ', FacesCode::Katakana(vec![0x35])),
        ('オ', FacesCode::Katakana(vec![0x36])),

        ('カ', FacesCode::Katakana(vec![0x54])),
        ('キ', FacesCode::Katakana(vec![0x47])),
        ('ク', FacesCode::Katakana(vec![0x48])),
        ('ケ', FacesCode::Katakana(vec![0x2a])),
        ('コ', FacesCode::Katakana(vec![0x42])),
        
        ('サ', FacesCode::Katakana(vec![0x58])),
        ('シ', FacesCode::Katakana(vec![0x44])),
        ('ス', FacesCode::Katakana(vec![0x52])),
        ('セ', FacesCode::Katakana(vec![0x50])),
        ('ソ', FacesCode::Katakana(vec![0x43])),

        ('タ', FacesCode::Katakana(vec![0x51])),
        ('チ', FacesCode::Katakana(vec![0x41])),
        ('ツ', FacesCode::Katakana(vec![0x7A])),
        ('テ', FacesCode::Katakana(vec![0x57])),
        ('ト', FacesCode::Katakana(vec![0x53])),

        ('ナ', FacesCode::Katakana(vec![0x55])),
        ('ニ', FacesCode::Katakana(vec![0x49])),
        ('ヌ', FacesCode::Katakana(vec![0x31])), 
        ('ネ', FacesCode::Katakana(vec![0x3b])), // 「ネ」に似ている「ヌ」
        ('ノ', FacesCode::Katakana(vec![0x4b])),

        ('ハ', FacesCode::Katakana(vec![0x46])),
        ('ヒ', FacesCode::Katakana(vec![0x56])),
        ('フ', FacesCode::Katakana(vec![0x32])),
        ('ヘ', FacesCode::Katakana(vec![0xDE])),
        ('ホ', FacesCode::Katakana(vec![0x2D])),

        ('マ', FacesCode::Katakana(vec![0x4a])),
        ('ミ', FacesCode::Katakana(vec![0x4e])),
        ('ム', FacesCode::Katakana(vec![0xdd])),
        ('メ', FacesCode::Katakana(vec![0x2f])),
        ('モ', FacesCode::Katakana(vec![0x4d])),

        ('ヤ', FacesCode::Katakana(vec![0x37])),
        ('ユ', FacesCode::Katakana(vec![0x38])),
        ('ヨ', FacesCode::Katakana(vec![0x39])),

        ('ラ', FacesCode::Katakana(vec![0x4f])),
        ('リ', FacesCode::Katakana(vec![0x4c])),
        ('ル', FacesCode::Katakana(vec![0x2e])),
        ('レ', FacesCode::Katakana(vec![0x2b])),
        ('ロ', FacesCode::Katakana(vec![0x5f])),

        ('ワ', FacesCode::Katakana(vec![0x30])), 
        ('ヲ', FacesCode::Katakana(vec![0x36])), // とりあえず「オ」に変換
        ('ン', FacesCode::Katakana(vec![0x59])),

        ('ガ', FacesCode::Katakana(vec![0x54,0xc0])),
        ('ギ', FacesCode::Katakana(vec![0x47,0xc0])),
        ('グ', FacesCode::Katakana(vec![0x48,0xc0])),
        ('ゲ', FacesCode::Katakana(vec![0x2a,0xc0])),
        ('ゴ', FacesCode::Katakana(vec![0x42,0xc0])),

        ('ザ', FacesCode::Katakana(vec![0x58,0xc0])),
        ('ジ', FacesCode::Katakana(vec![0x44,0xc0])),
        ('ズ', FacesCode::Katakana(vec![0x52,0xc0])),
        ('ゼ', FacesCode::Katakana(vec![0x50,0xc0])),
        ('ゾ', FacesCode::Katakana(vec![0x43,0xc0])),

        ('ダ', FacesCode::Katakana(vec![0x51,0xc0])),
        ('ヂ', FacesCode::Katakana(vec![0x41,0xc0])),
        ('ヅ', FacesCode::Katakana(vec![0x7A,0xc0])),
        ('デ', FacesCode::Katakana(vec![0x57,0xc0])),
        ('ド', FacesCode::Katakana(vec![0x53,0xc0])),

        ('バ', FacesCode::Katakana(vec![0x46,0xc0])),
        ('ビ', FacesCode::Katakana(vec![0x56,0xc0])),
        ('ブ', FacesCode::Katakana(vec![0x32,0xc0])),
        ('ベ', FacesCode::Katakana(vec![0xDE,0xc0])),
        ('ボ', FacesCode::Katakana(vec![0x2D,0xc0])),

        ('パ', FacesCode::Katakana(vec![0x46,0xdb])),
        ('ピ', FacesCode::Katakana(vec![0x56,0xdb])),
        ('プ', FacesCode::Katakana(vec![0x32,0xdb])),
        ('ペ', FacesCode::Katakana(vec![0xDE,0xdb])),
        ('ポ', FacesCode::Katakana(vec![0x2D,0xdb]))
    ]);

    let mut result = Vec::new();
    let mut caps_on = false;
    let mut kana_on = false;
    for c in input.chars() {
        if c.is_ascii() {
            if let Some(FacesCode::Ascii(code)) = hash_map.get(&c) {
                if kana_on {
                    result.push(U_KANA);
                    kana_on = !kana_on;
                }
                if caps_on {
                    result.push(U_CAPS);
                    caps_on = !caps_on;
                }
                result.push(*code);
            } else {
                if kana_on {
                    result.push(U_KANA);
                    kana_on = !kana_on;
                }
                if caps_on {
                    result.push(U_CAPS);
                    caps_on = !caps_on;
                }
                result.push(c as u8);
            }
        } else if let Some(keycode) = hash_map.get(&c) {
            match keycode {
                FacesCode::Hiragana(codes) => {
                    if !kana_on {
                        result.push(U_KANA);
                        kana_on = !kana_on;    
                    }
                    if caps_on {
                        result.push(U_CAPS);
                        caps_on = !caps_on;
                    }
                    for v in codes {
                        result.push(*v);
                    }
                 },
                FacesCode::Katakana(codes) => {
                    if !kana_on {
                        result.push(U_KANA);
                        kana_on = !kana_on;    
                    }
                    if !caps_on {
                        result.push(U_CAPS);
                        caps_on = !caps_on;
                    }
                    for v in codes {
                        result.push(*v);
                    }
                },
                FacesCode::Ascii(code) => {
                    if kana_on {
                        result.push(U_KANA);
                        kana_on = !kana_on;
                    }
                    if caps_on {
                        result.push(U_CAPS);
                        caps_on = !caps_on;
                    }
                    result.push(*code);
                }
            }
        }
    }
    if kana_on {
        result.push(U_KANA);
    }
    if caps_on {
        result.push(U_CAPS);
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
    assert!(s == "48 65 6C 6C 6F F3 42 59 49 41 46 20 35 3B 36 20 3E F3 20 20 20 20 20 20 20 20 ");

    let v=str_to_faces_code("ワタシはもうMSX0をてにいれました");
    let s = dump_hex(v);
    println!("{}",s);
    assert!(s == "F3 F2 30 51 44 F2 46 4D 34 F3 4D 53 58 30 F3 36 57 49 65 2B 4A 44 51 F3 ");
}
