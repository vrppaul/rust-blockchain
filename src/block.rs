use std::convert::TryInto;
use std::fmt::Debug;
use std::mem;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

extern crate hex;

const BLOCK_TRANSACTIONS_SIZE: usize = 10;

#[derive(Debug)]
pub struct Block {
    pub timestamp: Duration,
    pub hash: [u8; 32],
    pub previous_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        hash: [u8; 32],
        previous_hash: [u8; 32],
        transactions: Vec<Transaction>,
        nonce: u64,
    ) -> Self {
        Block {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            hash,
            previous_hash,
            transactions,
            nonce,
        }
    }

    pub fn new_from_block_to_mine(block_to_mine: &mut Block) -> Self {
        Block {
            transactions: mem::take(&mut block_to_mine.transactions),
            ..*block_to_mine
        }
    }

    pub fn new_from_transactions(transactions: Vec<Transaction>) -> Self {
        Block {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            hash: [0; 32],
            previous_hash: [0; 32],
            transactions,
            nonce: 0,
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
    pub fn new(complexity: usize) -> BlockChain {
        BlockChain {
            blocks: vec![],
            complexity,
            transaction_pool: vec![],
            block_to_mine: None,
        }
    }

    fn append_block(&mut self, block: Block) {
        self.blocks.push(block)
    }

    pub fn confirm_transactions(&mut self) {
        let start = SystemTime::now();
        let complexity = self.complexity;
        let mut block = self.get_block_to_mine();

        let mut bytes: Vec<u8> = vec![];
        bytes.extend(block.previous_hash);
        bytes.extend(block.timestamp.as_secs().to_be_bytes());
        for transaction in &block.transactions {
            bytes.extend(transaction.data.bytes());
            bytes.extend(transaction.weight.to_be_bytes());
        }

        for try_nonce in 0..u64::MAX {
            let mut new_vec = bytes.to_vec();
            new_vec.extend(try_nonce.to_be_bytes());

            let mut hasher = Sha256::new();
            hasher.update(&new_vec);
            let result = hasher.finalize();

            if try_nonce % 10000 == 0 {
                println!("Nonce tried: {}", try_nonce);
            }

            if result[..complexity].eq(&vec![0; complexity]) {
                println!("Time elapsed: {:?}", start.elapsed().unwrap());
                println!("Result nonce: {}", try_nonce);
                block.nonce = try_nonce;
                block.hash = result
                    .to_vec()
                    .try_into()
                    .unwrap_or_else(|_: Vec<u8>| panic!("Cannot happen."));
                self.new_block_from_block_to_mine();
                return;
            }
        }
    }

    pub fn schedule_transaction(&mut self, transaction: Transaction) {
        self.transaction_pool.push(transaction);
        self.transaction_pool
            .sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    }

    fn get_block_to_mine(&mut self) -> &mut Block {
        if self.block_to_mine.is_none() {
            let block_transactions: Vec<Transaction> =
                if self.transaction_pool.len() < BLOCK_TRANSACTIONS_SIZE {
                    mem::take(&mut self.transaction_pool)
                } else {
                    self.transaction_pool
                        .split_off(self.transaction_pool.len() - BLOCK_TRANSACTIONS_SIZE)
                };
            self.block_to_mine = Some(self.new_block_from_transactions(block_transactions));
        };
        return self.block_to_mine.as_mut().unwrap();
    }

    fn new_block_from_transactions(&self, transactions: Vec<Transaction>) -> Block {
        let mut block = Block::new_from_transactions(transactions);
        block.previous_hash = match self.blocks.last() {
            None => [0; 32],
            Some(block) => block.hash,
        };
        block
    }

    fn new_block_from_block_to_mine(&mut self) {
        let block = Block::new_from_block_to_mine(self.get_block_to_mine());
        self.append_block(block);
        self.block_to_mine = None;
    }
}
