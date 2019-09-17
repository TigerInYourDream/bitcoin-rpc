mod client;
mod config;
mod api;
mod json;

use json::blockchaininfo::BlockChainInfo;
use serde_json::Value;
use crate::json::simple::{ConnectionCount, BlockCount, BestBlockHash, Difficulty, Unspent};
use crate::client::BitcoinRPC;
use crate::json::simple;

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPC::new();

//    let echo_json: simple::Unspent = rpc.client
//        .post(config::TEST_URL)
//        .json(
//            &serde_json::json!({
//               "jsonrpc": "1.0",
//               "id": 10,
//               "method": "listunspent",
//               "params": [
//                    1,
//                    99999999,
//                    ["n1uPfAqcF5vh8gCH4zpXcJy2vXPnK9dEqr"]
//                ]
//            })
//        )
//        .send()?
//        .json()?;
//    println!("{:#?}", echo_json);
//    Ok(())

    let mut address = Vec::new();
    address.push("n1uPfAqcF5vh8gCH4zpXcJy2vXPnK9dEqr");
    let list_unspent: Unspent = rpc.list_unspent(Some(1), Some(99999999), address).unwrap();
    println!("{:#?}", list_unspent);

    Ok(())
}



