use std::time::{SystemTime, UNIX_EPOCH};
use cryptho;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use bank::Wallet;

#[derive(Clone,Serialize, Deserialize)] 
pub struct Transaction{
    pub value: usize,
    pub to:Wallet,
    pub from:Wallet,
    pub isSignt:bool,
    pub signature:String,
    pub index:usize,
    pub timestamp:u128,
    pub blockid:usize,
    pub utfeet:usize

    
}

impl Transaction {
    fn new(index:usize,value: usize,to:Wallet,from:Wallet, blockid:usize,)->Self{
        Self{value:value,
            to:to,
            from:from,
            isSignt:false,
            signature:String::new(),
            timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            index:index,
            blockid:blockid,
            utfeet:1
        }
    }
    pub fn sign_transaction(&mut self,signature:String, ut:&mut Wallet)->String{
        let mut amount = self.from.send(self.value, self.utfeet);
        if &amount.len()>&self.utfeet{
            let mut feet = Vec::new();
            self.signature= cryptho::hash_funktion_sha512(format!("{}{}{}{}{}", signature, self.to.publickey, self.from.publickey, amount.len(),self.timestamp));
            self.isSignt=true;
            for i in 0..self.utfeet{
                feet.push(amount.swap_remove(0));
            }
            ut.resive(feet);
            self.to.resive(amount);
            

        }
        return self.signature.clone();

    }

}




#[cfg(test)]
mod tests {
    use crate::Transaction;
    
}
