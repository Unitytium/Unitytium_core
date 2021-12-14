use chrono::prelude::*;

#[derive(Clone)]
pub struct Transaction {
    pub index: i64,
    pub timestamp: String,
    pub sender: String,
    pub resiver: String,
    pub amount: f64,
    pub feet: f64,
}

impl Transaction {

    pub fn send_amount(&self, _amount: f64, _feet: f64) {}


    pub fn information(&self)->String{
        return format!("Transaction --> Index: {0}, Sender: {1} Resiver: {2}, Amount: {3}, Feet {4} Timestamp: {5},"
                 , self.index, self.sender, self.resiver, self.amount, self.feet, self.timestamp);
    }

}
pub fn timestamp() -> String {
    let utc: DateTime<Utc> = Utc::now();
    let format = "%s %6f";
    let string = utc.format(format).to_string();
    return  string;
}