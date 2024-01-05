use reqwest::blocking::{self, Response};

use crate::bencode;
use crate::hash::calculate_hash;
use crate::torrent::Torrent;

pub fn discover(torrent: &Torrent) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let info = serde_bencode::to_bytes(&torrent.info);

    // TODO: Handle this error gracefully
    let res: Response = client
        .post(&torrent.announce)
        .query(&[(
            "info_hash",
            calculate_hash(&info.unwrap()),
            ("peer_id", "00112233445566778899"),
            ("port", "6881"),
            ("uploaded", "0"),
            ("downloaded", "0"),
            ("left", torrent.info.length.to_string()),
            ("compact", "1"),
        )])
        .send()?;

    // let decoded_value = bencode::decode_value(&body);
    // let val = bencode::render_value(&decoded_value);
    println!("body = {:?}", res.text());
    Ok(())
}
