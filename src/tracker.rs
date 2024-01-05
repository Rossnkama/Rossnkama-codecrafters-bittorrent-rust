use crate::hash::calculate_hash;
use crate::torrent::Torrent;
use reqwest::blocking::Response;
use serde::{Serialize, Serializer};

pub fn urlencode<S>(t: &[u8; 20], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut encoded = String::new();
    for &byte in t {
        if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.' || byte == b'~' {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{:02X}", byte));
        }
    }
    serializer.serialize_str(&encoded)
}

#[derive(Debug, Serialize)]
pub struct TrackerRequest {
    #[serde(serialize_with = "urlencode")]
    info_hash: [u8; 20],
    peer_id: String,
    port: u16,
    uploaded: usize,
    downloaded: usize,
    left: usize,
    compact: u8,
}

impl TrackerRequest {
    pub fn new(torrent: &Torrent) -> Self {
        let info = serde_bencode::to_bytes(&torrent.info).unwrap();
        let info_hash = calculate_hash(&info);
        Self {
            info_hash,
            peer_id: "00112233445566778899".to_owned(),
            port: 6881,
            uploaded: 0,
            downloaded: 0,
            left: torrent.info.length,
            compact: 1,
        }
    }
}

pub fn discover(torrent: &Torrent) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let request = TrackerRequest::new(torrent);
    let res: Response = client.get(&torrent.announce).query(&request).send()?;
    println!("body = {:?}", res);
    Ok(())
}
