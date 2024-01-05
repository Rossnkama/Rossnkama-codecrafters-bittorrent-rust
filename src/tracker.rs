use reqwest::blocking;

use crate::torrent::Torrent;

pub fn discover(torrent: &Torrent) -> Result<(), reqwest::Error> {
    let body = blocking::get(&torrent.announce)?.text()?;

    println!("body = {:?}", body);
    Ok(())
}
