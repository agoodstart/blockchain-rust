use std::collections::LinkedList;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;

mod server;

pub struct Time {
    current_time: Duration,
}

impl Time {
    fn new() -> Time {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        Time {
            current_time: since_epoch,
        }
    }
}

pub struct Blockchain {
    chain: LinkedList<Block>,
}

pub struct Block {
    index: i32,
    timestamp: Time,
    proof: i32,
    prev_hash: i32,
}

impl Blockchain {
    fn create_block(self, proof: i32, prev_hash: i32) {
        let new_block = Block {
            index: self.chain.len() as i32,
            timestamp: Time::new(),
            proof: 8,
            prev_hash: 9,
        };
    }
}



fn main() {
    server::run();
}