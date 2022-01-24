use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};


pub fn hash_funktion_sha512(data:String)->String{
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn hash_funktion_sha256(data:String)->String{
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result)
}