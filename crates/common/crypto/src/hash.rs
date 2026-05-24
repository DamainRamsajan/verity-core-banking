/// Extension trait for BLAKE3 hashing.
use hex;
pub trait HashExt {
    fn blake3_hex(&self) -> String;
    fn blake3_bytes(&self) -> [u8; 32];
}

impl HashExt for [u8] {
    fn blake3_hex(&self) -> String {
        let hash = blake3::hash(self);
        hex::encode(hash.as_bytes())
    }
    fn blake3_bytes(&self) -> [u8; 32] {
        *blake3::hash(self).as_bytes()
    }
}

impl HashExt for str {
    fn blake3_hex(&self) -> String { self.as_bytes().blake3_hex() }
    fn blake3_bytes(&self) -> [u8; 32] { self.as_bytes().blake3_bytes() }
}

