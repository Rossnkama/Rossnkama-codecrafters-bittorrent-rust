use sha1::{Digest, Sha1};

pub fn calculate_hash(info: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(info);
    let result = hasher.finalize();
    result.to_vec()
}