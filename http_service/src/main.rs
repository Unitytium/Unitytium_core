#[macro_use] extern crate rocket;

use rocket::response::content::Json;
use Chain_service::Chain;
use transaction_service::Transaction;
use rocket::serde::{Deserialize, json::Json};

const  chain:Chain = Chain{
    blocks: Vec::new(),
    transactions: Vec::new(),
    nodes: Vec::new(),
    transactions_id: 0
};

#[derive(Deserialize)]
struct transactiondata<'r> {
    sender: &'r str,
    resiver: &'r str,
    amount: &'r f64,

}

#[get("/api_indo ")]
fn index() -> &'static str {
    "api information!"
}




#[post("/add_transaction", data = "<req>")]
fn add_transaction(req: Json<transactiondata<'_>>) -> String {
    let transaction:Transaction = Transaction{
        index: chain.transactions_id,
        timestamp: transaction_service::timestamp(),
        sender: req.sender,
        resiver: req.resiver,
        amount: req.amount,
        feet: req.amount*0.0002
    };
    if chain.add_transaction(transaction){
        return "200 OK".to_owned();
    }else{
        return "400 Bad Request".to_owned();
    }



}

#[post("/get_chain")]
fn getChain() -> Chain {
    return chain;
}

#[post("/add_node")]
fn add_Node() -> &'static str {
    "Hello, world!"
}




#[rocket::main]
async fn main() {


    rocket::build()
        .mount("/", routes![add_transaction,index,getChain])
        .launch()
        .await;
}