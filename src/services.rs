use std::cmp::Ordering;

use crate::{transaction::Transaction, Block, BlockChain};

pub fn create_transaction(data: String, weight: f32, blockchain: &mut BlockChain) {
    blockchain.schedule_transaction(Transaction::new(data, weight));
}

pub fn get_last_transactions(blockchain: &BlockChain, n: usize) -> &[Transaction] {
    let pool_size = &blockchain.transaction_pool.len();
    match n.cmp(pool_size) {
        Ordering::Less => &blockchain.transaction_pool[pool_size - n..],
        _ => &blockchain.transaction_pool,
    }
}

pub fn get_last_blocks(blockchain: &BlockChain, n: usize) -> &[Block] {
    let blocks_size = &blockchain.blocks.len();
    match n.cmp(blocks_size) {
        Ordering::Less => &blockchain.blocks[blocks_size - n..],
        _ => &blockchain.blocks,
    }
}

pub fn confirm_transactions(blockchain: &mut BlockChain) {
    blockchain.confirm_transactions();
}

pub fn get_block_to_mine(blockchain: &BlockChain) -> Option<&Block> {
    blockchain.block_to_mine.as_ref()
}
