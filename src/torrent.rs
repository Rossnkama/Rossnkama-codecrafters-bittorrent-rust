use crate::torrent_pieces;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub length: usize,
    pub name: String,   
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    // #[serde(with = "serde_bytes")]
    pub pieces: torrent_pieces::Pieces,
}

pub fn read_from_file(file_path: &str) -> Torrent {
    let file = fs::read(file_path).unwrap();
    serde_bencode::from_bytes(&file).unwrap()
}