use crate::torrent_pieces_model;
use serde::{Deserialize, Serialize};

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
    pub pieces: torrent_pieces_model::Pieces,
}
