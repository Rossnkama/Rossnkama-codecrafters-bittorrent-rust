use std::env;

mod bencode;
mod hash;
mod torrent;
mod torrent_pieces;
mod tracker;

fn handle_decode(encoded_value: &str) {
    let decoded_value = bencode::decode_value(encoded_value);
    println!("{}", bencode::render_value(&decoded_value));
}

fn handle_info(file_path: &str) {
    let decoded_value = torrent::read_from_file(file_path);
    let info = serde_bencode::to_bytes(&decoded_value.info).unwrap();
    let hex_encoded_data = hash::calculate_hash(&info);
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
            "peers" => {
                let torrent = torrent::read_from_file(&args[2]);
                tracker::discover(&torrent).unwrap()
            }
            _ => println!("unknown command: {}", args[1]),
        }
    } else {
        println!("Usage: your_bittorrent.sh decode \"<encoded_value>\"");
    }
}
