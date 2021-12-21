use std::io;
use std::io::Write;

use crate::services;
use crate::BlockChain;

const DEFAULT_SHOW_TRANSACTIONS_SIZE: usize = 10;
const DEFAULT_SHOW_BLOCKS_SIZE: usize = 10;

pub fn process_command(command: &str, blockchain: &mut BlockChain) {
    match command {
        "exit" => std::process::exit(0),
        "help" => show_commands(),
        "create transaction" => create_transaction(blockchain),
        "show transactions" => show_transactions(blockchain),
        "confirm transactions" => services::confirm_transactions(blockchain),
        "show blocks" => show_blocks(blockchain),
        "mining block" => show_block_to_mine(blockchain),
        _ => eprintln!("Unrecognized command!"),
    }
}

pub fn show_commands() {
    println!("\nThis is an experimental toy blockchain.\n");
    println!("List of all commands:");
    println!("  `help` - Some info about the blockchain and commands.");
    println!("  `exit` - Exits the session, blochain will be lost.");
    println!("  `create transaction` - Create new dummy transaction.");
    println!("  `show transactions` - Show all transactions in the blockchain pool.");
    println!("  `confirm transactions` - Take last most weighted transactions, verify them and put into block.");
    println!("  `show blocks` - Show all the blocks in the blockchain.");
    println!("  `mining block` - Show current mining block, which should be mined to confirm transactions.\n\t\t\tMay be empty, if noone started mining it yet.");
    println!("\n");
}

fn get_trimmed_input(message: Option<&str>) -> String {
    let mut input = String::new();
    print!("{}", message.unwrap_or("> "));
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    input.trim().to_string()
}

fn create_transaction(blockchain: &mut BlockChain) {
    let data = get_trimmed_input(Some("Write a transaction message: "));

    let weight: f32;
    loop {
        let weight_input = get_trimmed_input(Some(
            "Give some weight to your transaction [min 0.0, max 1.0]: ",
        ));
        match weight_input.parse::<f32>() {
            Ok(value) => {
                if value < 0.0 || value > 1.0 {
                    eprintln!("Value should be > 0.0 and < 1.0");
                    continue;
                }
                weight = value;
                break;
            }
            Err(err) => eprintln!("Incorrect value, please try again. {}", err),
        }
    }

    services::create_transaction(data, weight, blockchain);
}

fn process_size_input(message: Option<&str>) -> usize {
    let n_input = get_trimmed_input(message);

    loop {
        if n_input.is_empty() {
            return DEFAULT_SHOW_TRANSACTIONS_SIZE;
        } else {
            match n_input.parse::<usize>() {
                Ok(value) => return value,
                Err(err) => eprintln!("Incorrect value, please try again. {}", err),
            }
        }
    }
}

fn show_transactions(blockchain: &BlockChain) {
    let n = process_size_input(Some(
        &format!(
            "How many transactions you want to display? [{}]: ",
            DEFAULT_SHOW_TRANSACTIONS_SIZE
        )[..],
    ));

    let transactions = services::get_last_transactions(blockchain, n);
    if transactions.len() == 0 {
        println!("No transactions in the pool yet.");
        return;
    }

    println!("Showing last {} (or less) transactions.", n);
    for (i, transaction) in transactions.iter().enumerate() {
        println!("{}: {}. Weight {}", i, transaction.data, transaction.weight);
    }
}

fn show_blocks(blockchain: &BlockChain) {
    let n = process_size_input(Some(
        &format!(
            "How many transactions you want to display? [{}]: ",
            DEFAULT_SHOW_BLOCKS_SIZE
        )[..],
    ));

    let blocks = services::get_last_blocks(blockchain, n);
    if blocks.len() == 0 {
        println!("No blocks yet.");
        return;
    }

    println!("Showing last {} (or less) blocks.", n);
    for (i, block) in blocks.iter().enumerate() {
        println!("{}", i);
        println!("  -> Timestamp: {:?}", block.timestamp);
        println!("  -> Hash: {:?}", block.hash);
        println!("  -> Previous hash: {:?}", block.previous_hash);
        println!("  -> Nonce: {:?}", block.nonce);
        println!("  -> Transactions:");
        for (i, transaction) in block.transactions.iter().enumerate() {
            println!(
                "    -> {}: {}. Weight {}",
                i, transaction.data, transaction.weight
            );
        }
        println!("")
    }
}

fn show_block_to_mine(blockchain: &BlockChain) {
    let block_to_mine = services::get_block_to_mine(blockchain);
    match block_to_mine {
        Some(block) => {
            println!("Showing current mining block:");
            println!("  Timestamp: {:?}", block.timestamp);
            println!("  Hash of previous block: {:?}", block.previous_hash);
            for (i, transaction) in block.transactions.iter().enumerate() {
                println!(
                    "  -> {}: {}. Weight {}",
                    i, transaction.data, transaction.weight
                );
            }
        }
        None => println!("No block to mine yet."),
    }
}
