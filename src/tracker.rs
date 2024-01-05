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
            ("peer_id", "00112233445566778899".to_owned()),
            ("port", "6881".to_owned()),
            ("uploaded", "0".to_owned()),
            ("downloaded", "0".to_owned()),
            ("left", torrent.info.length.to_string()),
            ("compact", "1".to_owned()),
        ])
        .send()?;

    // let decoded_value = bencode::decode_value(&body);
    // let val = bencode::render_value(&decoded_value);
    println!("body = {:?}", res);
    Ok(())
}
