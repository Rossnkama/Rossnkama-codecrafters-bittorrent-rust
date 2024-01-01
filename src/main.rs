use serde_bencode::value::Value as SerdeBencodeValue;
use std::env;

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
            let mut entries = vec![];
            for (key, value) in d {
                let rendered_key = String::from_utf8_lossy(key);
                let rendered_value = render_value(value);
                entries.push(format!("{}: {}", rendered_key, rendered_value));
            }
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
        } else {
            println!("unknown command: {}", args[1])
        }
    } else {
        println!("Usage: your_bittorrent.sh decode \"<encoded_value>\"");
    }
}
