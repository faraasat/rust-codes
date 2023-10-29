mod Greet {
    // every function is private by default
    // to make public write pub
    pub fn hello() {
        println!("hello")
    }
    fn goodbye() {
        println!("goodbye")
    }
}

mod functions_chain;
mod types;

use Greet::*;
use functions_chain::*;
use types::*;

fn main() {
    // Greet::hello();

    // we have used "use Greet::*" so we can use that module's functions here
    hello();

    let txn = Transaction {
        txn_hash: "ksgjs".to_string(),
        fees: 100.0,
    };

    distribute_reward(txn);

    let user = create_user(1, "John".to_string());
}
