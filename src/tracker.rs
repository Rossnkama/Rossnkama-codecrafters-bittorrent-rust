use reqwest::blocking;

use crate::bencode;
use crate::torrent::Torrent;

pub fn discover(torrent: &Torrent) -> Result<(), reqwest::Error> {
    let body = blocking::get(&torrent.announce)?.text()?;
    let decoded_value = bencode::decode_value(&body);
    let val = bencode::render_value(&decoded_value);

    println!("body = {:?}", val);
    Ok(())
}
