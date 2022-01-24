pub mod util;

pub fn hash_funktion_sha512(data:String)->String{
    util::hash_funktion_sha512(data)
}
pub fn hash_funktion_sha256(data:String)->String{
    util::hash_funktion_sha256(data)
}
#[cfg(test)]
mod tests {
    use crate::util;

    #[test]
    fn hash_sha521_test() {
        let data = "tester ".to_string();
        let hash = util::hash_funktion_sha512(data);
        println!("hash521_test value:{}",hash);
    }

    #[test]
    fn hash_sha256_test() {
        let data = "tester ".to_string();
        let hash = util::hash_funktion_sha256(data);
        println!("hash256_test value: {}",hash);
    }
}
