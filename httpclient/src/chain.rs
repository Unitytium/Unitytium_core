use blockModule::Block;
use blockModule::Head;
use blockModule::Body;
use transactionModule::Transaction;
use node::Node;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use cryptho;
use rand::Rng;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;
use bank::Wallet;





#[derive(Serialize, Deserialize)]
pub struct Chain{
    pub index:usize,
    pub blocks:Vec<Block>,
    pub nodes:Vec<Node>,
    pub transactions:Vec<Transaction>,
    pub transactionspool:Vec<Transaction>,
    pub proof:u128,
    pub wallets:Vec<Wallet>,
    


}

impl Chain {
    pub fn new(ut:Wallet)->Self{
        if !Path::new("chain.json").is_file(){
            let mut chain:Vec<Block> = Vec::new();
            let mut index:usize=0;
            for i in 0..9{
                let time = Chain::proof_of_time(String::from("0000"));
                let head = Head::new(i, time.clone(), String::from("0000"), String::from("0000"),time.clone());
                let transactions=Vec::new();
                let body = Body::new(cryptho::hash_funktion_sha512(format!("{}{}",time,&head.precius_hash.clone())), transactions, &mut ut.clone());
                let key =String::from(format!("{}",&head.precius_hash.clone()));
                let proof = 1;
                let head = Head::new(i, time, String::from("0000"), String::from("0000"),proof);
                let block = Block::new(head, body, key);
                chain.push(block);
                index+=1;
            }
            let mut wallets= Vec::new();
            wallets.push(ut);
        return Self{
            index:index,
            blocks:chain,
            nodes:Vec::new(),
            transactions:Vec::new(),
            proof:1,
            transactionspool:Vec::new(),
            wallets:wallets
            };
        }else{
        
            let file = fs::File::open("chain.json")
            .expect("file should open read only");
            let json: serde_json::Value = serde_json::from_reader(file)
            .expect("file should be proper JSON");
            let ch = serde_json::from_str(&json.to_string()).unwrap();
            return ch;
        
        }
        }
    pub fn create_block(&mut self, proof:u128){
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let prehash = self.blocks[self.blocks.len()-1].block_hash.clone();
        let nighash=self.blocks[self.blocks.len()-9].block_hash.clone();
        let body = Body::new(self.blocks[self.blocks.len()].body.transactoinKey.clone(),self.transactions.clone(), &mut self.wallets[0]);
        let key =String::from(format!("{}{}{}",body.transactoinKey.clone(),self.blocks[self.index-1].block_hash.clone(),self.blocks[self.index-9].block_hash.clone() ));
        self.transactions=Vec::new();
        let head = Head::new(self.index, time, prehash, nighash, proof);
        let block = Block::new(head, body, key.clone());
        self.proof=block.head.proof.clone();
        println!("Block index {} \nBH\t{} \nPH\t{} \nNH\t{} \nproof {}\n----------------------------------------------------------------------------------------------", &block.head.index, &block.block_hash, &block.head.precius_hash,&block.head.nigbore_hash,&block.head.proof);
        self.blocks.push(block);
        
        self.index+=1;
    }
    
    pub fn proof_of_time(key:String)->u128{
        let mut time= SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut check_proof=false;
        while !&check_proof {
            time= SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let hash_opration = cryptho::hash_funktion_sha512(format!("{}{}",time,&key.clone()));
            if &hash_opration[0..2]==String::from("0b"){
                check_proof=true;
                println!("[Proof of time {}]",&hash_opration);
            }
        };
        return time

    }

    pub fn to_json(&mut self)->String{
        let j = serde_json::to_string(self).unwrap();
        return j;

    }

   

}

