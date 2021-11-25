use md5;
use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

#[derive(Debug, PartialEq, Eq)]
pub enum HashType {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    MD5,
}

pub fn get_hash_type(hash: String) -> HashType {
    match hash.len() {
        32 => HashType::MD5,
        40 => HashType::SHA1,
        56 => HashType::SHA224,
        64 => HashType::SHA256,
        96 => HashType::SHA384,
        128 => HashType::SHA512,
        _ => {
            panic!("Error, invalid/unrecognized hash");
        }
    }
}

fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("").to_ascii_lowercase().to_string()
}

pub fn hash_sha1(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    hasher.digest().to_string()
}

pub fn hash_sha224(bytes: &[u8]) -> String {
    let mut hasher = Sha224::new();
    hasher.update(bytes);
    to_hex_string(hasher.finalize().to_vec())
}
pub fn hash_sha256(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    to_hex_string(hasher.finalize().to_vec())
}
pub fn hash_sha384(bytes: &[u8]) -> String {
    let mut hasher = Sha384::new();
    hasher.update(bytes);
    to_hex_string(hasher.finalize().to_vec())
}
pub fn hash_sha512(bytes: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(bytes);
    to_hex_string(hasher.finalize().to_vec())
}
pub fn hash_md5(bytes: &[u8]) -> String {
    let digest = md5::compute(bytes);
    format!("{:x}", digest)
}
