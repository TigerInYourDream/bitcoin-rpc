mod client;
mod config;
mod api;
mod json;

use json::blockchaininfo::BlockChainInfo;
use serde_json::Value;
use crate::json::simple::{ConnectionCount, BlockCount, BestBlockHash, Difficulty, Unspent, RawTransaction};
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


    let echo_json: RawTransaction = rpc.create_raw_transaction(
        "a14c0e7393b762fb71406f97f18422965d3360a218e5c9b33d699b0c921b6e0c".to_string(),
        0,
        "mwRJfknvd3y65tC1Ckd5MNizj9StHadaJ1".to_string(),
        0.001,
    ).expect("I didn't ger create_raw_transaction");
    println!("createrawtransaction {:#?}", echo_json);

    Ok(())
}



