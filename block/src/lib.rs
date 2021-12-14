use chrono::prelude::*;
#[warn(unused_imports)]


#[derive(Clone)]
pub struct Block {
    pub index: i128,
    pub timestamp: String,
    pub proof: i128,
    pub previus_hash: String,
    pub transactions : Vec<transaction_service::Transaction>,
    pub feet : f64
}

impl Block{



    pub fn display(&self)->String{
        let mut s = format!("Block (index: {0}, proof: {1}, previus_hash: {2}, timestamp: {3}, feet {4} )",
                 self.index, self.proof, self.previus_hash, self.timestamp, self.feet);
        for t in &self.transactions{
            let e = format!("transaction {}",t.information());
            for i in e.chars(){
                s.push(i)
            }
        }
        return s
    }
}
pub fn timestamp() ->String {
    let utc: DateTime<Utc> = Utc::now();
    let format = "%s %6f";
    let string = utc.format(format).to_string();
    return  string;

}