use serde::{Serialize, Deserialize};
use serde_bencode::value::Value as SerdeBencodeValue;
use std::{env, fs};

#[derive(Serialize, Deserialize)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Serialize, Deserialize)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename="piece length")]
    piece_length: usize,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}

fn decode_value(encoded_value: &str) -> SerdeBencodeValue {
    let serde_data: SerdeBencodeValue = serde_bencode::from_str(encoded_value).unwrap();
    serde_data
}

fn render_value(decoded_value: &SerdeBencodeValue) -> String {
    match decoded_value {
        SerdeBencodeValue::Bytes(b) => format!("\"{}\"", String::from_utf8_lossy(b)),
        SerdeBencodeValue::Int(i) => format!("{}", i),
        SerdeBencodeValue::List(l) => {
            let list_items: Vec<String> = l.iter().map(|item| render_value(item)).collect();
            format!("[{}]", list_items.join(", "))
        }
        SerdeBencodeValue::Dict(d) => {
            let mut entries: Vec<_> = d.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));
            let entries: Vec<String> = entries
                .iter()
                .map(|(key, value)| {
                    let rendered_key = String::from_utf8_lossy(key);
                    let rendered_value = render_value(value);
                    format!("\"{}\": {}", rendered_key, rendered_value)
                })
                .collect();
            format!("{{{}}}", entries.join(", "))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let command = &args[1];

        if command == "decode" {
            let encoded_value = &args[2];
            let decoded_value = decode_value(encoded_value);
            println!("{}", render_value(&decoded_value));
        } else if command == "info" {
            let file_path = &args[2];
            let file = fs::read(file_path).unwrap();
            let decoded_value: Torrent = serde_bencode::from_bytes(&file).unwrap();
            print!("Tracker URL: {}", decoded_value.announce);
            print!("Length: {}", std::mem::size_of_val(&decoded_value.info.pieces));
        } else {
            println!("unknown command: {}", args[1])
        }
    } else {
        println!("Usage: your_bittorrent.sh decode \"<encoded_value>\"");
    }
}
