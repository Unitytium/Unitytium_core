
use cryptho;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Clone, Serialize, Deserialize)]
pub struct Token{
    id:Uuid,
    index:usize,
    name:String,
    symbol:String,
    signature:String,
    creater:String,
}

impl Token {
    pub fn new(index:usize, name:String, symbol:String, creater:String)->Self{
        let id =Uuid::new_v4();
        let signature=Token::signature( &name, &symbol, &id, &creater, &index);
        Self{
            id:id,
            index:index,
            name:name,
            symbol:symbol,
            signature:signature,
            creater:creater
        }
        

    }
    fn signature(name:&String, symbol:&String, id:&Uuid, creater:&String, index:&usize)->String{
        cryptho::hash_funktion_sha512(String::from(format!("Token/{}{}{}{}{}",name,symbol,id,creater,index)))
    }
    pub fn to_string(&self)->String{
        String::from(format!("[ID:{},INDEX:{},NAME:{},SYMBOL:{},CREATER:{},SIGNATURE:{}",&self.id,&self.index,&self.name,&self.symbol,&self.creater, &self.signature))
    }
   
    
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet{
    pub tokens:Vec<Token>, 
    pub publickey:String,
    pub privatkey:String
}


impl Wallet {
    pub fn new(privatekey:String, mail:String)->Self{
        Self{
            tokens:Vec::new(),
            publickey:cryptho::hash_funktion_sha512(mail),
            privatkey:cryptho::hash_funktion_sha512(privatekey),
        }
        
    }

    pub fn resive(&mut self, tokens:Vec<Token>){
        for i in tokens{
            self.tokens.push(i);
        }
    }
    pub fn send(&mut self, amount:usize, feet:usize)->Vec<Token>{
        let mut sendamount:Vec<Token> = Vec::new();
        if amount>self.tokens.len()-feet{
            for i in 0..amount{
                sendamount.push(self.tokens.swap_remove(0));
            }
            println!("Widtdra {}UT",amount);
        }
        return sendamount
    }

    pub fn amount(&self)->usize{
        self.tokens.len()
    }

    
}




pub trait IContract {
    fn create()->Contract;
    fn get_signature(&self)->String;
    fn get_total_supply(&self)->String;
    fn get_owner(&self)->String;
    fn get_id(&self)->Uuid;
    fn burn(&self, amount:usize);
    fn mint(&self, amount:usize);
    fn state(&self);
      
    
}
#[derive(Serialize, Deserialize)]
pub struct Contract{
    pub id:Uuid,
    pub total_supply: usize,
    pub signature:String,
    pub owner:String,
}






