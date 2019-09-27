mod client;
mod config;
mod api;
mod json;

use json::blockchaininfo::BlockChainInfo;
use serde_json::Value;
use crate::client::BitcoinRPC;
use crate::json::simple;
use crate::json::simple::{SignRawTransactionWithWallet, DecodeRawTransaction, RawTransaction, Sendrawtransaction, ValidateAddress, Balance, NewAddress, GenerateToAddress, Unspent, TxIn};

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPC::new();

//    let echo_json: simple::GetTransaction = rpc.client.post(config::TEST_URL)
//        .json(
//            &serde_json::json!(
//                {
//                    "jsonrpc": "1.0",
//                    "id": 100,
//                    "method": "gettransaction",
//                    "params": [
//                    "668911f8a41113ffdbf65fd326e20c107dfd62cf3cc86959444f55be94ab78d7",
//                    true
//                    ]
//
//                }
//            )
//        )
//        .send()?
//        .json()?;
//
//    println!("{:#?}", echo_json);


    let echo_json = rpc.get_transaction(
        "668911f8a41113ffdbf65fd326e20c107dfd62cf3cc86959444f55be94ab78d7".to_string(),
        None,
    ).unwrap();

    println!("{:#?}", echo_json);

    Ok(())
}
