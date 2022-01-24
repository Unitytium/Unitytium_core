use std::collections::HashMap;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;
use blockModule::Block;
use blockModule::Head;
use blockModule::Body;
use transactionModule::Transaction;
use node::Node;
use std::time::{SystemTime, UNIX_EPOCH};
use cryptho;
use rand::Rng;
use std::time::{Duration, Instant};
use std::thread;


pub struct Chain{
    pub index:usize,
    pub blocks:Vec<Block>,
    pub nodes:HashMap<String,String>,
    pub transactions:Vec<Transaction>,
    pub transactionspool:Vec<Transaction>,
    pub proof:u128


}

impl Chain {
    pub fn new()->Self{
        let mut chain:Vec<Block> = Vec::new();
        let mut index:usize=0;
        for i in 0..9{
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let head = Head::new(i, time, String::from("0000"), String::from("0000"),1);
            let transactions=Vec::new();
            let body = Body::new(transactions);
            let key =String::from(format!("{}","firstBlock"));
            let proof = 1;
            let head = Head::new(i, time, String::from("0000"), String::from("0000"),proof);
            let block = Block::new(head, body, key);
            chain.push(block);
            index+=1;
        }
        Self{
            index:index,
            blocks:chain,
            nodes:HashMap::new(),
            transactions:Vec::new(),
            proof:1,
            transactionspool:Vec::new(),
        }
    }
    pub fn create_block(&mut self){
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let prehash = self.blocks[self.index-1].block_hash.clone();
        let nighash=self.blocks[self.index-9].block_hash.clone();
        let body = Body::new(self.transactions.clone());
        let key =String::from(format!("{}{}{}",body.transactoinKey.clone(),self.blocks[self.index-1].block_hash.clone(),self.blocks[self.index-9].block_hash.clone() ));
        let proof = self.proof_of_time(key.clone());
        self.transactions=Vec::new();
        let head = Head::new(self.index, time, prehash, nighash, proof);
        let block = Block::new(head, body, key.clone());
        println!("Block index {} \nBH\t{} \nPH\t{} \nNH\t{} \nproof {}\n----------------------------------------------------------------------------------------------", &block.head.index, &block.block_hash, &block.head.precius_hash,&block.head.nigbore_hash,&block.head.proof);
        self.blocks.push(block);
        self.index+=1;
    }

    pub fn proof_of_word(&mut self, previus_proof:i128)->i128{
        let mut new_proof:i128=1;
        let mut check_proof=false;
        while !&check_proof {
            let p =new_proof.pow(2) - previus_proof.pow(2);
            let hash_opration = cryptho::hash_funktion_sha512(String::from(format!("{}",p)));
            if hash_opration[0..2]==String::from("00"){
                check_proof=true
            }else{
                new_proof+=1;
            }
        }
        return new_proof;

    }
    
    
    
    pub fn proof_of_time(&mut self, key:String)->u128{
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
    
}










fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            println!("[{}]: {}",stream.peer_addr().unwrap(),from_utf8(&data).unwrap());
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let mut chain = Chain::new();
    let mut pool:Vec<Transaction> = Vec::new();
    let mut index:usize=9;
    let mut blockid = chain.index.clone();



    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    
    // close the socket server
    drop(listener);
}