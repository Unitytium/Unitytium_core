use std::time::{SystemTime, UNIX_EPOCH};
use cryptho;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
pub struct Node {
    pub index:i128,
    pub addr:String,
    pub timestamp:u128,
    pub key:String,
    pub power:f32,
    pub wallet:String
    }
    



impl Node {
    pub fn create(index:i128, addr:String, wallet:String) -> Node{
        let sys_time = get_epoch_ms();
        let data= format!("Node{}{}{}{}",&index.clone() ,&addr.clone(),&sys_time.clone(),&wallet.clone());
        let key =cryptho::hash_funktion_sha512(data);
        let power = 100.0;
        Node{
            index:index,
            addr:addr,
            timestamp:sys_time,
            key:key,
            power:power,
            wallet:wallet
        }
    }

    pub fn to_String(&self)->String{
        format!("Node[index:{}, addr:{}, timestamp:{}, key:{}, power:{}, wallet:{}]",self.index, self.addr,self.timestamp,self.key, self.power,self.wallet)
    }


    
    

    
}
fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}