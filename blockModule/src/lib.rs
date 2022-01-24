use transactionModule::Transaction;
use std::collections::HashMap;
use cryptho;
use node;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use bank::Wallet;


#[derive(Serialize, Deserialize)]
pub struct Head{
    pub index:usize,
    pub timestamp:u128,
    pub precius_hash:String,
    pub nigbore_hash:String,
    pub proof:u128
    
    



}

impl Head {
    pub fn new(index:usize,timestamp:u128,precius_hash:String,nigbore_hash:String,proof:u128 )->Head{
        Head{index,timestamp,precius_hash,nigbore_hash,proof}
    }
    
    
}


#[derive(Serialize, Deserialize)]
pub struct Body{
    pub transactionMap:HashMap<String, Transaction>,
    pub value:usize,
    pub utvalue:usize,
    pub transactoinKey:String

}
impl Body {
    pub fn new(key:String,transactions:Vec<Transaction>,ut:&mut Wallet)->Body{
        let mut map:HashMap<String,Transaction> = HashMap::new();
        let mut value=0;
        let mut utvalue=0;
        let mut tkey=key;
        let utwallet = ut;
        for mut ta in transactions{
            value+= &ta.value;
            let newkey= &ta.sign_transaction(tkey, utwallet );
            map.insert(newkey.clone(), ta);
            tkey= newkey.clone();
        }
        tkey = cryptho::hash_funktion_sha256(format!("{}", &tkey));
        Body{
            transactionMap:map,
            value:value,
            utvalue:utvalue,
            transactoinKey:tkey
        }
    }

   
    
}

#[derive(Serialize, Deserialize)]
pub struct Block{
    pub block_hash:String,
    pub head:Head,
    pub body:Body,

    

    

}


impl Block {
    pub fn new( head:Head,body:Body, key:String)->Block{
        let mut data =String::new();
        for (key, value) in &body.transactionMap {
            data.push_str(key);
        }
        data.push_str(&head.precius_hash.clone());
        data.push_str(&head.nigbore_hash.clone());
        data.push_str(&head.timestamp.clone().to_string());
        Block{
            block_hash:cryptho::hash_funktion_sha512(format!("{}{}",&head.proof,&key.clone())),
            head:head,
            body:body
            
        }
    }

}






#[cfg(test)]
mod tests {
   
    #[test]
    fn test() {
    }
      
    
}
