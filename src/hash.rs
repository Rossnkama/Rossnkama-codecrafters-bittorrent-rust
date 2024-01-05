use sha1::{Digest, Sha1};

pub fn calculate_hash(info: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(info);
    let result = hasher.finalize();
    let array: [u8; 20] = match result.try_into() {
        Ok(arr) => arr,
        Err(_) => panic!("Expected a Vec of length 20, but it was different")
    };
    array
}