use reqwest::blocking::Response;

use crate::hash::calculate_hash;
use crate::torrent::Torrent;

pub fn discover(torrent: &Torrent) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let info = serde_bencode::to_bytes(&torrent.info);

    // TODO: Handle this error gracefully
    let res: Response = client
        .get(&torrent.announce)
        .query(&[
            (
                "info_hash",
                calculate_hash(&info.expect("Info did not unwrap!")),
            ),
            ("peer_id", "00112233445566778899".as_bytes().to_vec()),
            ("port", "6881".as_bytes().to_vec()),
            ("uploaded", "0".as_bytes().to_vec()),
            ("downloaded", "0".as_bytes().to_vec()),
            ("left", torrent.info.length.to_string().as_bytes().to_vec()),
            ("compact", "1".as_bytes().to_vec()),
        ])
        .send()?;

    // let decoded_value = bencode::decode_value(&body);
    // let val = bencode::render_value(&decoded_value);
    println!("body = {:?}", res);
    Ok(())
}
