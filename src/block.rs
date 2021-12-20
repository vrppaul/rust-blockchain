use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{Debug};
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use sha2::{Sha256, Digest};

use crate::transaction::Transaction;
use sha2::digest::ExtendableOutput;

extern crate hex;

const BLOCK_TRANSACTIONS_SIZE: usize = 10;


#[derive(Debug)]
pub struct Block {
    pub timestamp: Duration,
    pub hash: [u8; 32],
    pub previous_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub nonce: i64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>) -> Self {
        Block {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            hash: [0; 32],
            previous_hash: [0; 32],
            transactions,
            nonce: 0
        }
    }
}

#[derive(Debug)]
pub struct BlockChain {
    pub blocks: Vec<Block>,
    pub complexity: usize,
    pub transaction_pool: Vec<Transaction>,
    pub block_to_mine: Option<Block>,
}

impl BlockChain {
    fn append_block(&mut self, block: Block) {
        self.blocks.push(block)
    }

    pub fn confirm_transactions(&mut self) {
        let start = SystemTime::now();
        let block = self.get_block_to_mine();

        let mut bytes: Vec<u8> = vec![];
        bytes.extend(block.previous_hash);
        bytes.extend(block.timestamp.as_secs().to_be_bytes());
        for transaction in &block.transactions {
            bytes.extend(transaction.data.bytes());
            bytes.extend(transaction.weight.to_be_bytes());
        }

        for try_nonce in 0..u32::MAX {
            let mut new_vec = bytes.to_vec();
            new_vec.extend(try_nonce.to_be_bytes());

            let mut hasher = Sha256::new();
            hasher.update(&new_vec);
            let result = hasher.finalize();

            if try_nonce % 10000 == 0 {
                println!("Nonce tried: {}", try_nonce);
            }

            if result[..self.complexity].eq(&vec![0; self.complexity]) {
                println!("Time elapsed: {:?}", start.elapsed());
                println!("{}", try_nonce);
                println!("{:?}", result);
                return
            }
        }
    }

    pub fn schedule_transaction(&mut self, transaction: Transaction) {
        self.transaction_pool.push(transaction);
        self.transaction_pool.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    }

    fn get_block_to_mine(&mut self) -> &Block {
        if self.block_to_mine.is_none() {
            let block_transactions: Vec<Transaction> = if self.transaction_pool.len() < BLOCK_TRANSACTIONS_SIZE {
                mem::take(&mut self.transaction_pool)
            } else {
                self.transaction_pool.split_off(self.transaction_pool.len() - BLOCK_TRANSACTIONS_SIZE)
            };
            self.block_to_mine = Some(self.new_block(block_transactions));
        };
        return &self.block_to_mine.as_ref().unwrap();
    }

    fn new_block(&self, transactions: Vec<Transaction>) -> Block {
        let mut block = Block::new(transactions);
        block.previous_hash = match self.blocks.last() {
            None => [0; 32],
            Some(block) => block.hash,
        };
        block
    }
}
