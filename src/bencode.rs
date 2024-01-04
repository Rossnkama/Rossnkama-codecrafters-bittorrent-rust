use serde_bencode::value::Value as SerdeBencodeValue;

pub fn decode_value(encoded_value: &str) -> SerdeBencodeValue {
    let serde_data: SerdeBencodeValue = serde_bencode::from_str(encoded_value).unwrap();
    serde_data
}

pub fn render_value(decoded_value: &SerdeBencodeValue) -> String {
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