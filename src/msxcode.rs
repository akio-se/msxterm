// MSX0 ASCII Code Convert Module
// Copyright (c) 2023 Akio Setsumasa 
// Released under the MIT license
// https://github.com/akio-se/msxterm
//
use std::collections::HashMap;
use encoding_rs::SHIFT_JIS;

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
    'π',  '┻', '┳', '┫', '┣', '╋','┃', '━', '┏','┓', '┗', '┛','\u{2573}', '大','中','小',

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
        } else if graph {
            let c = MSX_TO_GRAPH[s];
            cv.push(c);
            graph = false;
        } else {
            let c =  MSX_TO_UTF8[s];
            cv.push(c);
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

// MSX KANJIから UTF-8 文字列へ変換
pub fn msx_kanji_to_string(uv: Vec<u8>) -> String
{
    let (res,_,_) = SHIFT_JIS.decode(&uv);
    res.into_owned()
}

pub fn utf8_to_msx_kanji(input: &str) -> Vec<u8>
{
    let (res,_,_) = SHIFT_JIS.encode(input);
    res.into_owned()
}


#[test]
fn msx_kanji_test()
{
    let uv: Vec<u8> = vec![0x82,0x6c,0x82,0x72,0x82,0x77,0x82,0xcc,0x8a,0xbf,0x8e,0x9a];
    let s = msx_kanji_to_string(uv);
    println!("{}", s);
    assert_eq!(s,"ＭＳＸの漢字");
}


enum FacesCode {
    Ascii(u8),
    Hiragana(Vec<u8>),
    Katakana(Vec<u8>),
}

enum MsxCode {
    Ascii(u8),
    Graph(u8),
    Kana(Vec<u8>),
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

//
// UTF8 から MSX_JP_CODE への変換
//
pub fn utf8_msx_jp_code(input: &str) -> Vec<u8> {

    // 変換用ハッシュテーブル
    let hash_map = HashMap::from([
        // ASCII コード変換文字
        ('\u{2660}', MsxCode::Ascii(0x80)), // スペード
        ('\u{2665}', MsxCode::Ascii(0x81)), // ハート
        ('\u{2663}', MsxCode::Ascii(0x82)), // クローバー
        ('\u{2666}', MsxCode::Ascii(0x83)), // ダイヤ
        ('\u{25CB}', MsxCode::Ascii(0x84)), // 中空丸 ◯
        ('\u{25CF}', MsxCode::Ascii(0x85)), // 塗潰丸 ●

        // Graph コード変換文字
        ('月', MsxCode::Graph(0x41)),
        ('火', MsxCode::Graph(0x42)),
        ('水', MsxCode::Graph(0x43)),
        ('木', MsxCode::Graph(0x44)),
        ('金', MsxCode::Graph(0x45)),
        ('土', MsxCode::Graph(0x46)),
        ('日', MsxCode::Graph(0x47)),

        ('年', MsxCode::Graph(0x48)),
        ('円', MsxCode::Graph(0x49)),
        ('時', MsxCode::Graph(0x4a)),
        ('分', MsxCode::Graph(0x4b)),
        ('秒', MsxCode::Graph(0x4c)),
        ('百', MsxCode::Graph(0x4d)),
        ('千', MsxCode::Graph(0x4e)),
        ('万', MsxCode::Graph(0x4f)),

        ('π',  MsxCode::Graph(0x50)),
        ('┻',  MsxCode::Graph(0x51)),
        ('┳',  MsxCode::Graph(0x52)),
        ('┫',  MsxCode::Graph(0x53)),
        ('┣',  MsxCode::Graph(0x54)),
        ('╋',  MsxCode::Graph(0x55)),
        ('┃',  MsxCode::Graph(0x56)),
        ('━',  MsxCode::Graph(0x57)),

        ('┏',  MsxCode::Graph(0x58)),
        ('┓',  MsxCode::Graph(0x59)),
        ('┗',  MsxCode::Graph(0x5a)),
        ('┛',  MsxCode::Graph(0x5b)),
        ('\u{2573}', MsxCode::Graph(0x5c)),
        ('大', MsxCode::Graph(0x5d)),
        ('中', MsxCode::Graph(0x5e)),
        ('小', MsxCode::Graph(0x5f)),

         


        // ひらがな
        ('を', MsxCode::Kana(vec![0x86])),
        ('ぁ', MsxCode::Kana(vec![0x87])),
        ('ぃ', MsxCode::Kana(vec![0x88])),
        ('ぅ', MsxCode::Kana(vec![0x89])),
        ('ぇ', MsxCode::Kana(vec![0x8a])),
        ('ぉ', MsxCode::Kana(vec![0x8b])),
        ('ゃ', MsxCode::Kana(vec![0x8c])),
        ('ゅ', MsxCode::Kana(vec![0x8d])),
        ('ょ', MsxCode::Kana(vec![0x8e])),
        ('っ', MsxCode::Kana(vec![0x8f])),
  
        ('あ', MsxCode::Kana(vec![0x91])),
        ('い', MsxCode::Kana(vec![0x92])),
        ('う', MsxCode::Kana(vec![0x93])),
        ('え', MsxCode::Kana(vec![0x94])),
        ('お', MsxCode::Kana(vec![0x95])),

        ('か', MsxCode::Kana(vec![0x96])),
        ('き', MsxCode::Kana(vec![0x97])),
        ('く', MsxCode::Kana(vec![0x98])),
        ('け', MsxCode::Kana(vec![0x99])),
        ('こ', MsxCode::Kana(vec![0x9a])),
        
        ('さ', MsxCode::Kana(vec![0x9b])),
        ('し', MsxCode::Kana(vec![0x9c])),
        ('す', MsxCode::Kana(vec![0x9d])),
        ('せ', MsxCode::Kana(vec![0x9e])),
        ('そ', MsxCode::Kana(vec![0x9f])),

        ('た', MsxCode::Kana(vec![0xe0])),
        ('ち', MsxCode::Kana(vec![0xe1])),
        ('つ', MsxCode::Kana(vec![0xe2])),
        ('て', MsxCode::Kana(vec![0xe3])),
        ('と', MsxCode::Kana(vec![0xe4])),

        ('な', MsxCode::Kana(vec![0xe5])),
        ('に', MsxCode::Kana(vec![0xe6])),
        ('ぬ', MsxCode::Kana(vec![0xe7])), 
        ('ね', MsxCode::Kana(vec![0xe8])),
        ('の', MsxCode::Kana(vec![0xe9])),

        ('は', MsxCode::Kana(vec![0xea])),
        ('ひ', MsxCode::Kana(vec![0xeb])),
        ('ふ', MsxCode::Kana(vec![0xec])),
        ('へ', MsxCode::Kana(vec![0xed])),
        ('ほ', MsxCode::Kana(vec![0xee])),

        ('ま', MsxCode::Kana(vec![0xef])),
        ('み', MsxCode::Kana(vec![0xf0])),
        ('む', MsxCode::Kana(vec![0xf1])),
        ('め', MsxCode::Kana(vec![0xf2])),
        ('も', MsxCode::Kana(vec![0xf3])),

        ('や', MsxCode::Kana(vec![0xf4])),
        ('ゆ', MsxCode::Kana(vec![0xf5])),
        ('よ', MsxCode::Kana(vec![0xf6])),

        ('ら', MsxCode::Kana(vec![0xf7])),
        ('り', MsxCode::Kana(vec![0xf8])),
        ('る', MsxCode::Kana(vec![0xf9])),
        ('れ', MsxCode::Kana(vec![0xfa])),
        ('ろ', MsxCode::Kana(vec![0xfb])),

        ('わ', MsxCode::Kana(vec![0xfc])), 
        ('ん', MsxCode::Kana(vec![0xfd])),

        ('　', MsxCode::Kana(vec![0x20])),
        ('。', MsxCode::Kana(vec![0xa1])),
        ('「', MsxCode::Kana(vec![0xa2])),
        ('」', MsxCode::Kana(vec![0xa3])),
        ('、', MsxCode::Kana(vec![0xa4])),
        ('・', MsxCode::Kana(vec![0xa5])),


        ('が', MsxCode::Kana(vec![0x96,0xde])),
        ('ぎ', MsxCode::Kana(vec![0x97,0xde])),
        ('ぐ', MsxCode::Kana(vec![0x98,0xde])),
        ('げ', MsxCode::Kana(vec![0x99,0xde])),
        ('ご', MsxCode::Kana(vec![0x9a,0xde])),

        ('ざ', MsxCode::Kana(vec![0x9b,0xde])),
        ('じ', MsxCode::Kana(vec![0x9c,0xde])),
        ('ず', MsxCode::Kana(vec![0x9d,0xde])),
        ('ぜ', MsxCode::Kana(vec![0x9e,0xde])),
        ('ぞ', MsxCode::Kana(vec![0x9f,0xde])),

        ('だ', MsxCode::Kana(vec![0xe0,0xde])),
        ('ぢ', MsxCode::Kana(vec![0xe1,0xde])),
        ('づ', MsxCode::Kana(vec![0xe2,0xde])),
        ('で', MsxCode::Kana(vec![0xe3,0xde])),
        ('ど', MsxCode::Kana(vec![0xe4,0xde])),

        ('ば', MsxCode::Kana(vec![0xea,0xde])),
        ('び', MsxCode::Kana(vec![0xeb,0xde])),
        ('ぶ', MsxCode::Kana(vec![0xec,0xde])),
        ('べ', MsxCode::Kana(vec![0xed,0xde])),
        ('ぼ', MsxCode::Kana(vec![0xee,0xde])),

        ('ぱ', MsxCode::Kana(vec![0xea,0xdf])),
        ('ぴ', MsxCode::Kana(vec![0xeb,0xdf])),
        ('ぷ', MsxCode::Kana(vec![0xec,0xdf])),
        ('ぺ', MsxCode::Kana(vec![0xed,0xdf])),
        ('ぽ', MsxCode::Kana(vec![0xee,0xdf])),

        // カタカナ
        ('ヲ', MsxCode::Kana(vec![0xa6])), 
        ('ァ', MsxCode::Kana(vec![0xa7])),
        ('ィ', MsxCode::Kana(vec![0xa8])),
        ('ゥ', MsxCode::Kana(vec![0xa9])),
        ('ェ', MsxCode::Kana(vec![0xaa])),
        ('ォ', MsxCode::Kana(vec![0xab])),
   
        ('ャ', MsxCode::Kana(vec![0xac])),
        ('ュ', MsxCode::Kana(vec![0xad])),
        ('ョ', MsxCode::Kana(vec![0xae])),
        ('ッ', MsxCode::Kana(vec![0xaf])),
        ('ー', MsxCode::Kana(vec![0xb0])),
  
        ('ア', MsxCode::Kana(vec![0xb1])),
        ('イ', MsxCode::Kana(vec![0xb2])),
        ('ウ', MsxCode::Kana(vec![0xb3])),
        ('エ', MsxCode::Kana(vec![0xb4])),
        ('オ', MsxCode::Kana(vec![0xb5])),

        ('カ', MsxCode::Kana(vec![0xb6])),
        ('キ', MsxCode::Kana(vec![0xb7])),
        ('ク', MsxCode::Kana(vec![0xb8])),
        ('ケ', MsxCode::Kana(vec![0xb9])),
        ('コ', MsxCode::Kana(vec![0xba])),
        
        ('サ', MsxCode::Kana(vec![0xbb])),
        ('シ', MsxCode::Kana(vec![0xbc])),
        ('ス', MsxCode::Kana(vec![0xbd])),
        ('セ', MsxCode::Kana(vec![0xbe])),
        ('ソ', MsxCode::Kana(vec![0xbf])),

        ('タ', MsxCode::Kana(vec![0xc0])),
        ('チ', MsxCode::Kana(vec![0xc1])),
        ('ツ', MsxCode::Kana(vec![0xc2])),
        ('テ', MsxCode::Kana(vec![0xc3])),
        ('ト', MsxCode::Kana(vec![0xc4])),

        ('ナ', MsxCode::Kana(vec![0xc5])),
        ('ニ', MsxCode::Kana(vec![0xc6])),
        ('ヌ', MsxCode::Kana(vec![0xc7])), 
        ('ネ', MsxCode::Kana(vec![0xc8])),
        ('ノ', MsxCode::Kana(vec![0xc9])),

        ('ハ', MsxCode::Kana(vec![0xca])),
        ('ヒ', MsxCode::Kana(vec![0xcb])),
        ('フ', MsxCode::Kana(vec![0xcc])),
        ('ヘ', MsxCode::Kana(vec![0xcd])),
        ('ホ', MsxCode::Kana(vec![0xce])),

        ('マ', MsxCode::Kana(vec![0xcf])),
        ('ミ', MsxCode::Kana(vec![0xd0])),
        ('ム', MsxCode::Kana(vec![0xd1])),
        ('メ', MsxCode::Kana(vec![0xd2])),
        ('モ', MsxCode::Kana(vec![0xd3])),

        ('ヤ', MsxCode::Kana(vec![0xd4])),
        ('ユ', MsxCode::Kana(vec![0xd5])),
        ('ヨ', MsxCode::Kana(vec![0xd6])),

        ('ラ', MsxCode::Kana(vec![0xd7])),
        ('リ', MsxCode::Kana(vec![0xd8])),
        ('ル', MsxCode::Kana(vec![0xd9])),
        ('レ', MsxCode::Kana(vec![0xda])),
        ('ロ', MsxCode::Kana(vec![0xdb])),

        ('ワ', MsxCode::Kana(vec![0xdc])), 
        ('ン', MsxCode::Kana(vec![0xdd])),

        ('゛', MsxCode::Kana(vec![0xde])),
        ('゜', MsxCode::Kana(vec![0xdf])),

        ('ガ', MsxCode::Kana(vec![0xb6,0xde])),
        ('ギ', MsxCode::Kana(vec![0xb7,0xde])),
        ('グ', MsxCode::Kana(vec![0xb8,0xde])),
        ('ゲ', MsxCode::Kana(vec![0xb9,0xde])),
        ('ゴ', MsxCode::Kana(vec![0xba,0xde])),

        ('ザ', MsxCode::Kana(vec![0xbb,0xde])),
        ('ジ', MsxCode::Kana(vec![0xbc,0xde])),
        ('ズ', MsxCode::Kana(vec![0xbd,0xde])),
        ('ゼ', MsxCode::Kana(vec![0xbe,0xde])),
        ('ゾ', MsxCode::Kana(vec![0xbf,0xde])),

        ('ダ', MsxCode::Kana(vec![0xc0,0xde])),
        ('ヂ', MsxCode::Kana(vec![0xc1,0xde])),
        ('ヅ', MsxCode::Kana(vec![0xc2,0xde])),
        ('デ', MsxCode::Kana(vec![0xc3,0xde])),
        ('ド', MsxCode::Kana(vec![0xc4,0xde])),

        ('バ', MsxCode::Kana(vec![0xca,0xde])),
        ('ビ', MsxCode::Kana(vec![0xcb,0xde])),
        ('ブ', MsxCode::Kana(vec![0xcc,0xde])),
        ('ベ', MsxCode::Kana(vec![0xcd,0xde])),
        ('ボ', MsxCode::Kana(vec![0xce,0xde])),

        ('パ', MsxCode::Kana(vec![0xca,0xdf])),
        ('ピ', MsxCode::Kana(vec![0xcb,0xdf])),
        ('プ', MsxCode::Kana(vec![0xcc,0xdf])),
        ('ペ', MsxCode::Kana(vec![0xcd,0xdf])),
        ('ポ', MsxCode::Kana(vec![0xce,0xdf]))
    ]);

    let mut result = Vec::new();
    for c in input.chars() {
        if c.is_ascii() {
            if let Some(MsxCode::Ascii(code)) = hash_map.get(&c) {
                result.push(*code);
            } else {
                result.push(c as u8);
            }
        } else if let Some(keycode) = hash_map.get(&c) {
            match keycode {
                MsxCode::Kana(codes) => {
                    for v in codes {
                        result.push(*v);
                    }
                 },
                MsxCode::Ascii(code) => {
                    result.push(*code);
                },
                MsxCode::Graph(code) => {
                    result.push(0x01);
                    result.push(*code);
                }
            }
        }
    }
    result
}

#[test]
fn test_str_to_msx_code()
{
    let v=utf8_msx_jp_code("Helloこんにちは・ぇねを・。!%?`{|}~");
    let s = dump_hex(v);
    println!("{}",s);
    assert!(s == "48 65 6C 6C 6F 9A FD E6 E1 EA A5 8A E8 86 A5 A1 21 25 3F 60 7B 7C 7D 7E ");

    let v=utf8_msx_jp_code("ワタシはもうMSX0をてにいれました");
    let s = dump_hex(v);
    println!("{}",s);
    assert!(s == "DC C0 BC EA F3 93 4D 53 58 30 86 E3 E6 92 FA EF 9C E0 ");
}
