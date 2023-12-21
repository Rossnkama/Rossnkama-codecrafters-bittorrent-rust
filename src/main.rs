use serde_json;
use std::env;

fn decode_bencoded_string(encoded_value: &str) -> serde_json::Value {
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();

    let start_index = colon_index + 1;
    let end_index = start_index + number as usize;

    let string = &encoded_value[start_index..end_index];

    return serde_json::Value::String(string.to_string());
}

fn decode_bencoded_integer(encoded_value: &str) -> serde_json::Value {
    let integer = encoded_value[1..].chars().into_iter().take_while(|ch| ch != &'e').collect();
    serde_json::Value::String(integer)
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let first_char = encoded_value.chars().next().unwrap();
    
    // If encoded_value starts with a digit, it's we're dealing with a byte string
    if first_char.is_digit(10) {
        decode_bencoded_string(encoded_value)
    } else if first_char.eq(&'i') {
        decode_bencoded_integer(encoded_value)
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}

