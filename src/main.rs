
use transaction_service;
use block;
use Chain_service::Chain;
use transaction_service::Transaction;
use rand::Rng;
use block::Block;


fn main() {
    println!("Starter Chain");
    let mut rng = rand::thread_rng();
    let feet = 0.0002;

    let mut main_chain = Chain{
        blocks: Vec::new(),
        transactions: Vec::new(),
        nodes: Vec::new(),
        transactions_id: 0
    };


    println!("1 block add to  Chain");
    if main_chain.create_block(0,"0".to_string()) {
        println!("Ready for Chain");
    }



    let mut amout=0.0;
    let mut amount_feet=0.0;
    for x in 0..10 {
        for i in 0..20 {
            amout = rng.gen_range(0.0000001..100000.00);
            amount_feet = amout*feet;

            let trans = transaction_service::Transaction {
                index: main_chain.transactions_id,
                sender: "hello".parse().unwrap(),
                resiver: "world".parse().unwrap(),
                amount: amout as f64,
                feet: amount_feet as f64,
                timestamp: transaction_service::timestamp(),
            };
            trans.information();
            if main_chain.add_transaction(trans){
                println!("Transaction added successfully");
            }
        }

        if main_chain.create_block(main_chain.proof_of_truth(main_chain.get_prev_block().proof),main_chain.hash(main_chain.get_prev_block()).to_string()){
            println!("Block created successfully");
        }




    }
    for b in main_chain.blocks.iter() {
        println!("----------------------------------------");
        println!("{}", b.display());
        println!("Block {0} feet {1}", b.index, b.feet);
    }
    println!("Ending Chain");

}
