pub struct Block {
    hash: String,
    nonce: i32,
    blocknumber: i32,
}

impl Block {
    pub fn new_block(hash: String, nonce: i32, blocknumber: i32) -> Self {
        Self {
            hash,
            nonce,
            blocknumber,
        }
    }
}

pub struct Transaction {
    pub txn_hash: String,
    pub fees: f32,
}

impl Transaction {
    pub fn new_txn(txn_hash: String, fees: f32) -> Self {
        Self { txn_hash, fees }
    }
}

pub struct Chain {
    blocks: Vec<Block>,
    txns: Vec<Transaction>,
}
