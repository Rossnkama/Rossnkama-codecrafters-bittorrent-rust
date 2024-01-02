use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Serialize, Deserializer, Serializer};
use serde_bencode::value::Value as SerdeBencodeValue;
use sha1::{Digest, Sha1};
use std::{env, fmt, fs};

struct ByteArrayVisitor;

impl<'de> Visitor<'de> for ByteArrayVisitor {
    type Value = [u8; 20];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of bytes")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut array = [0; 20];
        for i in 0..20 {
            eprintln!("{}", array[i]);
            array[i] = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
        }
        Ok(array)
    }
}

#[derive(Debug)]
pub struct ByteArray([u8; 20]);

impl<'de> Deserialize<'de> for ByteArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = deserializer.deserialize_seq(ByteArrayVisitor)?;
        Ok(ByteArray(b))
    }
}

impl Serialize for ByteArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert the ByteArray to a slice and serialize it
        serializer.serialize_bytes(&self.0)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
    // #[serde(with = "serde_bytes")]
    pieces: ByteArray,
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
    println!("Piece Hashes: {:?}", decoded_value.info)
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
