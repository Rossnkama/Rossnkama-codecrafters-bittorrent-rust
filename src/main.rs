use sha1::{Digest, Sha1};
use std::{env, fs};

mod torrent;
mod torrent_pieces;
mod bencode;

fn calculate_hash(info: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(info);
    let result = hasher.finalize();
    hex::encode(result)
}

fn handle_decode(encoded_value: &str) {
    let decoded_value = bencode::decode_value(encoded_value);
    println!("{}", bencode::render_value(&decoded_value));
}

fn handle_info(file_path: &str) {
    let file = fs::read(file_path).unwrap();
    let decoded_value: torrent::Torrent = serde_bencode::from_bytes(&file).unwrap();
    let info = serde_bencode::to_bytes(&decoded_value.info).unwrap();
    let hex_encoded_data = calculate_hash(&info);
    println!("Tracker URL: {}", decoded_value.announce);
    println!("Length: {}", decoded_value.info.length);
    println!("Info Hash: {}", hex_encoded_data);
    println!("Piece Length: {}", decoded_value.info.piece_length);
    println!("Piece Hashes: ");
    for piece in decoded_value.info.pieces.get() {
        print!("{} ", hex::encode(piece));
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let command = &args[1];

        match command.as_str() {
            "decode" => handle_decode(&args[2]),
            "info" => handle_info(&args[2]),
            _ => println!("unknown command: {}", args[1]),
        }
    } else {
        println!("Usage: your_bittorrent.sh decode \"<encoded_value>\"");
    }
}
