use std::io;
use std::io::Write;

use blockchain::commands;
use blockchain::BlockChain;

fn main() {
    let mut blockhain = initiate_blockchain(3);

    run_command_loop(&mut blockhain);
}

fn initiate_blockchain(complexity: usize) -> BlockChain {
    BlockChain::new(complexity)
}

fn run_command_loop(blockhain: &mut BlockChain) {
    commands::show_commands();
    println!("Enter some command.");

    loop {
        let mut input = String::new();

        print!("\n> ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        let command = input.trim();
        commands::process_command(command, blockhain);
    }
}
