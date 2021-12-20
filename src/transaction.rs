use std::fmt::{Debug};

#[derive(Debug)]
pub struct Transaction {
    pub data: String,
    pub weight: f32,
}
