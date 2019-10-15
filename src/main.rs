mod client;
mod config;
mod api;
mod json;

use json::blockchaininfo::BlockChainInfo;
use serde_json::Value;
use crate::client::BitcoinRPC;
use crate::json::simple;
use crate::json::simple::{SignRawTransaction, DecodeRawTransaction, RawTransaction,
                          Sendrawtransaction, ValidateAddress, Balance, NewAddress, GenerateToAddress, Unspent, TxIn, DumpPrivkey};
use std::collections::HashMap;

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPC::new();
    let balance =rpc.get_balance().unwrap();
    println!("{:?}", balance);

    let new_address:NewAddress = rpc.get_new_address(None).unwrap();
    println!("{:?}", new_address);

    Ok(())
}
