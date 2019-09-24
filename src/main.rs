mod client;
mod config;
mod api;
mod json;

use json::blockchaininfo::BlockChainInfo;
use serde_json::Value;
use crate::client::BitcoinRPC;
use crate::json::simple;
use crate::json::simple::{SignRawTransactionWithWallet, DecodeRawTransaction, RawTransaction, Sendrawtransaction};

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPC::new();


    //signrawtransactionwithwallet
    // sign_rawtransaction_with_wallet
//    let echo_json: Value = rpc.client
//        .post(config::TEST_URL)
//        .json(
//            &serde_json::json!(
//            {
//  "jsonrpc": "1.0",
//  "id": 10,
//  "method": "signrawtransactionwithwallet",
//  "params": [
//    "02000000010c6e1b920c9b693db3c9e518a260335d962284f1976f4071fb62b793730e4ca10100000000ffffffff0120a10700000000001976a914ae7086368c8983429953dd2c2b3c79b1b3a0b60a88ac00000000",
//  ]
//}
//            )
//        )
//        .send()?
//        .json()?;
//    println!("{:#?}", echo_json);

    let sign_rawtransaction_with_wallet: SignRawTransactionWithWallet = rpc.sign_rawtransaction_with_wallet(
        "02000000010c6e1b920c9b693db3c9e518a260335d962284f1976f4071fb62b793730e4ca10100000000ffffffff0120a10700000000001976a914ae7086368c8983429953dd2c2b3c79b1b3a0b60a88ac00000000".to_string()
    ).unwrap();
    println!("{:#?}", sign_rawtransaction_with_wallet);


//    let send_rawtransaction: Sendrawtransaction = rpc.send_rawtransaction(
//        "02000000010c6e1b920c9b693db3c9e518a260335d962284f1976f4071fb62b793730e4ca1010000006a473044022034e8abe331ccd87b7bb02a29ff67aa8469e61b2ec3704632f206199b0faf4be602201f72f18ecb044528b3debfad0abe623b2dffeee136b0706383dde0ecb183e8db01210291ee52a0e0c22db9772f237f4271ea6f9330d92b242fb3c621928774c560b699ffffffff0120a10700000000001976a914ae7086368c8983429953dd2c2b3c79b1b3a0b60a88ac00000000".to_string(),
//        Some(true),
//    ).unwrap();
//    println!("sendrawtransaction {:#?}", send_rawtransaction);

    Ok(())
}



