use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};
use std::fmt;

use serde::de::{self, Visitor};

#[derive(Debug)]
pub struct Pieces(Vec<[u8; 20]>);
impl Pieces {
    pub fn get(&self) -> &Vec<[u8; 20]> {
        &self.0
    }
}

struct PiecesVisitor;

impl<'de> Visitor<'de> for PiecesVisitor {
    type Value = Pieces;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A byte string where it's len % 20 == 0")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!(
                "Length is not a multiple of 20 at {}",
                v.len()
            )));
        }
        let pieces = v
            .chunks_exact(20)
            .map(|piece| piece.try_into().expect("Bad chunk size"))
            .collect();
        Ok(Pieces(pieces))
    }
}

impl<'de> Deserialize<'de> for Pieces {
    fn deserialize<D>(deserializer: D) -> Result<Pieces, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PiecesVisitor)
    }
}

impl Serialize for Pieces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let flat_slice = self.0.concat();
        serializer.serialize_bytes(&flat_slice)
    }
}