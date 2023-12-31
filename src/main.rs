use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Info {
    /// Size of a file in bytes for a single-file record
    length: i64,
    name: String,
    piece_length: usize,
    pieces: serde_bytes::ByteBuf,
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value: Torrent = serde_bencode::from_str(&encoded_value).unwrap();
        println!("{:#?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
