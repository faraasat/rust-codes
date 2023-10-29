use crate::types::*;
mod user;

use user::*;

pub fn start_node(chain: Chain) {
    let new_block = Block::new_block("hash".to_string(), 0, 0);
}

pub fn distribute_reward(txn_hash: Transaction) {}

pub fn mine_block(block: Block) {}

pub fn create_user(id: i32, name: String) -> User {
    User {
        id,
        name,
    }
}
