#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::path::Path;
use node::Node;
use std::fs::File;
use std::fs;
use std::thread;

mod chain;


#[get("/index")]
fn index() -> &'static str {
"Hello, world!"
}

#[get("/get_chain")]
fn get_chain() -> String {
    let mut ch=chain::Chain::new(bank::Wallet::new(String::from("ut"),String::from("ut")));
    let j =ch.to_json();
    return j;
}

#[get("/proof/<proof>")]
fn mining(proof:String) -> String {
    let mut ch=chain::Chain::new(bank::Wallet::new(String::from("ut"),String::from("ut")));
    ch.create_block(proof.parse().unwrap());
    let data = ch.to_json().clone();
    write_to_file(&data);
    return format!("{}",&ch.proof);
}







fn main() {
    let ut= bank::Wallet::new(String::from("ut"),String::from("ut"));
    let mut chain = chain::Chain::new(ut);
    let data = chain.to_json().clone();
    write_to_file(&data);
    rocket::ignite().mount("/", routes![index,get_chain, mining]).launch();

}


pub fn write_to_file(data:&String)->std::io::Result<()>{
    if !Path::new("chain.json").is_file(){
        let mut file = File::create("chain.json")?;
        fs::write("chain.json", data)?;
    }else{
        fs::remove_file("chain.json")?;
        let mut file = File::create("chain.json")?;
        fs::write("chain.json", data)?;
    }
    Ok(())
}



