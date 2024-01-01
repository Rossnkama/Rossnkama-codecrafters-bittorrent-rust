use serde_bencode::value::Value as SerdeBencodeValue;
use std::env;

fn decode_value(encoded_value: &str) -> SerdeBencodeValue {
    let serde_data: SerdeBencodeValue = serde_bencode::from_str(encoded_value).unwrap();
    serde_data
}

fn render_value(decoded_value: SerdeBencodeValue) {
    match decoded_value {
        SerdeBencodeValue::Bytes(b) => {
            println!("{:?}", String::from_utf8_lossy(&b));
        }
        SerdeBencodeValue::Int(i) => {
            println!("{:?}", i);
        }
        SerdeBencodeValue::List(l) => {
            println!("{:?}", l);
        }
        SerdeBencodeValue::Dict(d) => {
            println!("{:?}", d);
        }
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_value(encoded_value);
        render_value(decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
