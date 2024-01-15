use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use sha2::{Sha256, Digest};

pub fn encode_url(url: &str) -> String {
    URL_SAFE.encode(url)
}

pub fn decode_url(url: &str) -> Vec<u8> {
    URL_SAFE.decode(url).unwrap()
}

pub fn hash_url(url: &str, len: u8) {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let result = hasher.finalize();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding() {
        let result = encode_url("www.google.com");
        assert_eq!(result, "d3d3Lmdvb2dsZS5jb20=");
    }

    fn decoding() {

    }

    fn hashing() {

    }
}
