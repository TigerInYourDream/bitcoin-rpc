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

//    let mut txins = Vec::new();
//    txins.push(
//        TxIn::new("df376b0bab6bac1ab61b2592ca684ed5b6c20c72a4173cde8f344c13e3c74c01".to_string(), 0)
//    );
//    txins.push(
//        TxIn::new("a1857e2b5553f14c953ad4b9a7a2959d3f1805db107f4a8febc9f7bef61a1601".to_string(), 0)
//    );
//
//    let mut to_address = HashMap::new();
//    to_address.insert("2N3r1he7kMwfxgENDMZskHHiyyF9TsrpCWp".to_string(), 49.9);
//    to_address.insert("2N1tdFAcJc42vHEKZa2V9yq84obTbEDKWx2".to_string(), 49.9);
//
//
//    let create_raw_transaction = rpc.create_raw_transaction(
//        txins,
//        to_address,
//    ).unwrap();
//
//    println!("{:#?}", create_raw_transaction);

//    let fund_rawtransaction = rpc.fund_rawtransaction(
//        "0200000002014cc7e3134c348fde3c17a4720cc2b6d54e68ca92251bb61aac6bab0b6b37df0000000000ffffffff01161af6bef7c9eb8f4a7f10db05183f9d95a2a7b9d43a954cf153552b7e85a10000000000ffffffff02805b6d290100000017a9145ed3cf7beabdd188892046c3c8378192cfe2a98187805b6d290100000017a91474458f539dde43a5cf9991823bc9ec6163c27ba08700000000".to_string(),
//        "2N4z2aVuFFdVk5rwKmFckBTyrLFvGDFNvEs".to_string(),
//    ).unwrap();
//
//    println!("{:#?}", fund_rawtransaction);

//    let dumpprivkey = rpc.dumpprivkey("2Mvxq9KRvzsRTpHstSmSZYgLbWXTrEKNbDL".to_string()).unwrap();
//    println!("{:#?}", dumpprivkey);

//    let sign_rawtransaction_with_key = rpc.sign_rawtransaction_with_key(
//        "02000000000102014cc7e3134c348fde3c17a4720cc2b6d54e68ca92251bb61aac6bab0b6b37df0000000017160014be36f4664c9abf0ff4ace505abd4a419d6c43728ffffffff01161af6bef7c9eb8f4a7f10db05183f9d95a2a7b9d43a954cf153552b7e85a10000000000ffffffff03805b6d290100000017a9145ed3cf7beabdd188892046c3c8378192cfe2a98187805b6d290100000017a91474458f539dde43a5cf9991823bc9ec6163c27ba087801631010000000017a91480c1becd51535a816f2338fdda180941ad236ff5870247304402205e2c57f9d166c9f3dbae7e231783f7d7c60d43ccd2cf7ce116ddfb4b1128c10202205d904df08891f30c30a46f2f7a5c23cb18190e19b3c9b6788ee2516842358e600121032959a851be46f5ce57c4c73494aceb8ceb819a1156e8dfc0324b6a729030e1de0000000000".to_string(),
//        "cVo2GTZEDiBpRAorHmhHNaXjLv9fyAiVeQvoJeYPNBdfNGQ4UhXq".to_string(),
//    ).unwrap();
//
//    println!("{:#?}", sign_rawtransaction_with_key);

    let decode_raw_transaction = rpc.decode_raw_transaction(
        "02000000000102014cc7e3134c348fde3c17a4720cc2b6d54e68ca92251bb61aac6bab0b6b37df0000000017160014be36f4664c9abf0ff4ace505abd4a419d6c43728ffffffff01161af6bef7c9eb8f4a7f10db05183f9d95a2a7b9d43a954cf153552b7e85a10000000000ffffffff03805b6d290100000017a9145ed3cf7beabdd188892046c3c8378192cfe2a98187805b6d290100000017a91474458f539dde43a5cf9991823bc9ec6163c27ba087801631010000000017a91480c1becd51535a816f2338fdda180941ad236ff5870247304402205e2c57f9d166c9f3dbae7e231783f7d7c60d43ccd2cf7ce116ddfb4b1128c10202205d904df08891f30c30a46f2f7a5c23cb18190e19b3c9b6788ee2516842358e600121032959a851be46f5ce57c4c73494aceb8ceb819a1156e8dfc0324b6a729030e1de0000000000".to_string()
    ).unwrap();
    println!("{:#?}", decode_raw_transaction);

    Ok(())
}
