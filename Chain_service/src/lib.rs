use std::ops::Add;
use transaction_service::Transaction;
use block::Block;
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};



pub struct  Chain {
    pub blocks : Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub nodes: Vec<String>,
    pub transactions_id:i64,



}
 impl Chain {

     pub fn get_prev_block(&self) -> Block{
         let block_size = self.blocks.len();
         let block:Block = self.blocks[block_size-1].clone();
         println!("{}", block.display());
         return block;
     }

     pub fn create_block(&mut self, _proof:i128, _previus_hash:String)->bool{
         println!("{}",_previus_hash);
         let mut _feet:f64=0.0;
         for f in &self.transactions{
             _feet+=f.feet;
         }
         let  ready_transactions = &self.transactions;
         let  _block = block::Block{
             index:self.blocks.len() as i128,
             timestamp:block::timestamp(),
             proof:_proof,
             previus_hash:_previus_hash,
             transactions: ready_transactions.to_vec(),
             feet:_feet,
         };
         self.transactions.clear();
         if !self.transactions.is_empty(){
             println!("Transaction are not empty ");
             return false;
         }
         self.blocks.push(_block);
         return true;

    }



     pub fn shahash( data:String)-> String{
         let mut hasher = Sha256::new();
         hasher.update(data);
         return format!("{:X}", hasher.finalize())
     }

     pub fn proof_of_truth(&self, prev_proof:i128) ->i128{
         let mut new_proof:i128 =1;
         let mut chech_proof = false;
         while !chech_proof{
             let mut form = i128::pow(new_proof, 2) - i128::pow(prev_proof ,2);
             let mut hash_opration = Chain::shahash(format!("{:X}",form));
             if hash_opration[..2].eq("00"){
                 chech_proof=true;
                 println!("new proof hash: {}",hash_opration);
             }
             else {
                 new_proof+=1
             }
         }
         println!("proof found {}",new_proof);
         return new_proof;
     }


     pub fn hash(&self, block: Block) ->String{
         let shahash5121 = Chain::shahash( block.display());
         return shahash5121;

     }
     pub fn chain_validation(&self, _blocks:Vec<Block>) ->bool{
         return false;
     }

     pub fn add_transaction(&mut self, _transaction:Transaction)->bool{
         &mut self.transactions_id.add(1);
         self.transactions.push(_transaction);
         return true;
     }

     pub fn add_Nodes(&mut self, adress:String){
         self.nodes.push(adress);
     }
     pub fn replace_chain(&self,)-> bool{
         return false;
     }

 }