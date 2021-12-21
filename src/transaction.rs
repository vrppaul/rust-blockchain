use std::fmt::Debug;

#[derive(Debug)]
pub struct Transaction {
    pub data: String,
    pub weight: f32,
}

impl Transaction {
    pub fn new(data: String, weight: f32) -> Transaction {
        Transaction { data, weight }
    }
}
