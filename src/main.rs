mod client;
mod config;
mod api;
mod json;

use json::blockchaininfo::BlockChainInfo;
use serde_json::Value;
use crate::json::simple::{ConnectionCount, BlockCount, BestBlockHash, Difficulty};

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPC::new();

    let listaddressgroupings: Value = rpc
        .send_raw("listaddressgroupings", Vec::new())
        .expect("I didn't get value!")
        .json()?;
    println!("listaddressgroupings {:#?}", listaddressgroupings);

    let echo_json: Value = rpc.client
        .post(config::TEST_URL)
        .json(
            &serde_json::json!({
               "jsonrpc": "1.0",
               "id": "curltest",
               "method": "listunspent",
               "params": [
                    1,
                    99999999,
                    ["n1uPfAqcF5vh8gCH4zpXcJy2vXPnK9dEqr"]
                ]
            })
        )
        .send()?
        .json()?;
    println!("{:#?}", echo_json);
    Ok(())
}