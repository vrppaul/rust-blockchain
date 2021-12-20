use blockchain::{Block, BlockChain};
use blockchain::transaction::Transaction;

fn main() {
    let initial_block = Block::new(vec![]);

    let mut blockchain = BlockChain {
        blocks: vec![initial_block],
        complexity: 3,
        transaction_pool: Default::default(),
        block_to_mine: None
    };

    for i in 0..=10 {
        let new_transaction = Transaction {
            data: "Initial data".to_string(),
            weight: 1.0 * (i as f32 / 10.0),
        };
        blockchain.schedule_transaction(new_transaction);
    }

    blockchain.confirm_transactions();
}
