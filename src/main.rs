use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde::{Deserialize, Deserializer};
use serde_bencode::value::Value as SerdeBencodeValue;
use sha1::{Digest, Sha1};
use std::{env, fs};

use std::fmt;

use serde::de::{self, Visitor};

#[derive(Debug)]
struct Pieces(Vec<[u8; 20]>);
struct PiecesVisitor;

impl<'de> Visitor<'de> for PiecesVisitor {
    type Value = Pieces;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A byte string where it's len % 20 == 0")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!(
                "Length is not a multiple of 20 at {}",
                v.len()
            )));
        }
        let pieces = v
            .chunks(20)
            .map(|piece| piece.try_into().expect("Bad chunk size"))
            .collect();
        Ok(Pieces(pieces))
    }
}

impl<'de> Deserialize<'de> for Pieces {
    fn deserialize<D>(deserializer: D) -> Result<Pieces, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PiecesVisitor)
    }
}

impl Serialize for Pieces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()));
        for elem in self.0 {
            seq?.serialize_element(&elem).unwrap();
        }
        seq?.end()
    }
}

#[derive(serde::Serialize, Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(serde::Serialize, Deserialize, Debug)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
    // #[serde(with = "serde_bytes")]
    pieces: Pieces,
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

fn calculate_hash(info: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(info);
    let result = hasher.finalize();
    hex::encode(result)
}

fn handle_decode(encoded_value: &str) {
    let decoded_value = decode_value(encoded_value);
    println!("{}", render_value(&decoded_value));
}

fn handle_info(file_path: &str) {
    let file = fs::read(file_path).unwrap();
    let decoded_value: Torrent = serde_bencode::from_bytes(&file).unwrap();
    let info = serde_bencode::to_bytes(&decoded_value.info).unwrap();
    let hex_encoded_data = calculate_hash(&info);
    println!("Tracker URL: {}", decoded_value.announce);
    println!("Length: {}", decoded_value.info.length);
    println!("Info Hash: {}", hex_encoded_data);
    println!("Piece Length: {}", decoded_value.info.piece_length);
    println!("Piece Hashes: ");
    for piece in &decoded_value.info.pieces.0 {
        print!("{} ", calculate_hash(piece));
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
