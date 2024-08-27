#![cfg_attr(target_arch="wasm32",no_std)]

extern crate alloc;
extern crate fluentbase_sdk;
extern crate heapless;


use alloc;
use fluentbase_sdk::{basic_entrypoint, derive::{router, signatur}, SharedAPI};


#[derive(Default)]
pub struct VickereyAuction{
    bids: Vec<u32,16>
}


impl VickreyAuction {
    pub fn new() -> Self {
        VickreyAuction {
            bids: Vec::new(),
        }
    }

    pub fn add_bid(&mut self, bid: u32) -> Result<(), &'static str> {
        self.bids.push(bid).map_err(|_| "Max bids reached")
    }

    pub fn determine_winner(&self) -> Result<(u32, u32), &'static str> {
        if self.bids.len() < 2 {
            return Err("Not enough bids to determine a winner");
        }

        let mut sorted_bids = self.bids.clone();
        sorted_bids.sort_unstable();

        // The highest bid wins, but the winner pays the second-highest bid
        let highest = *sorted_bids.iter().rev().nth(0).unwrap();
        let second_highest = *sorted_bids.iter().rev().nth(1).unwrap();

        Ok((highest, second_highest))
    }

    //pub fn deploy<SDK: SharedAPI>()
}

basic_entrypoint!(VickereyAuction);


