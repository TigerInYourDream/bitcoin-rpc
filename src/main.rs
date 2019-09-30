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

//
//    let decode_raw_transaction = rpc.decode_raw_transaction(
//        "02000000000102014cc7e3134c348fde3c17a4720cc2b6d54e68ca92251bb61aac6bab0b6b37df0000000017160014be36f4664c9abf0ff4ace505abd4a419d6c43728ffffffff01161af6bef7c9eb8f4a7f10db05183f9d95a2a7b9d43a954cf153552b7e85a10000000000ffffffff03805b6d290100000017a9145ed3cf7beabdd188892046c3c8378192cfe2a98187805b6d290100000017a91474458f539dde43a5cf9991823bc9ec6163c27ba087801631010000000017a91480c1becd51535a816f2338fdda180941ad236ff5870247304402205e2c57f9d166c9f3dbae7e231783f7d7c60d43ccd2cf7ce116ddfb4b1128c10202205d904df08891f30c30a46f2f7a5c23cb18190e19b3c9b6788ee2516842358e600121032959a851be46f5ce57c4c73494aceb8ceb819a1156e8dfc0324b6a729030e1de0000000000".to_string()
//    ).unwrap();
//    println!("{:#?}", decode_raw_transaction);

//    let new_address = rpc.get_new_address(
//        None
//    ).unwrap();
//    println!("{:#?}", new_address);

//    let mut txins = Vec::new();
//    txins.push(
//        TxIn::new("ba06ab6238073d7ee539a0f04fb859881955c255dbaf6485aaa02ce4bd5ee24c".to_string(), 1),
//    );
//    let mut to_address = HashMap::new();
//    to_address.insert("2N2mS3GuCdYFippvXXW47Ms8SWotyqMHUJV".to_string(), 49.89);
//
//    let rawtransaction = rpc.create_raw_transaction(
//        txins,
//        to_address,
//    ).unwrap();
//    println!("{:#?}", rawtransaction);

//    let sign_rawtransaction_offline = rpc.sign_rawtransaction_offline(
//        "02000000014ce25ebde42ca0aa8564afdb55c255198859b84ff0a039e57e3d073862ab06ba0100000000ffffffff0140195e290100000017a914686fabc9fd35360a70ee04f32f9b0c3cf0444d858700000000".to_string(),
//        "ba06ab6238073d7ee539a0f04fb859881955c255dbaf6485aaa02ce4bd5ee24c".to_string(),
//        1,
//        "a91474458f539dde43a5cf9991823bc9ec6163c27ba087".to_string(),
//        49.9,
//    ).unwrap();
//    println!("{:#?}", sign_rawtransaction_offline);

//    let send_rawtransaction = rpc.send_rawtransaction(
//        "02000000000102014cc7e3134c348fde3c17a4720cc2b6d54e68ca92251bb61aac6bab0b6b37df0000000017160014be36f4664c9abf0ff4ace505abd4a419d6c43728ffffffff01161af6bef7c9eb8f4a7f10db05183f9d95a2a7b9d43a954cf153552b7e85a10000000017160014a437d4e35ce09293ee32758be79722399708492dffffffff03805b6d290100000017a9145ed3cf7beabdd188892046c3c8378192cfe2a98187805b6d290100000017a91474458f539dde43a5cf9991823bc9ec6163c27ba087801631010000000017a91480c1becd51535a816f2338fdda180941ad236ff5870247304402205e2c57f9d166c9f3dbae7e231783f7d7c60d43ccd2cf7ce116ddfb4b1128c10202205d904df08891f30c30a46f2f7a5c23cb18190e19b3c9b6788ee2516842358e600121032959a851be46f5ce57c4c73494aceb8ceb819a1156e8dfc0324b6a729030e1de02473044022000880e9c06c6c1e74416d7a5efafe53031ab1bdd30f7c7e9b432e8b269d375cf02202022ee2bd75e60e895b19375e4aa8597e1afb94761225f0e7f2e143aa602b26d0121025507f5daf66c9af6924b7155411cf52e4417568efc48650af6935807734470f800000000".to_string(),
//        false,
//    ).unwrap();
//    println!("{:#?}", send_rawtransaction);


    let send_rawtransaction = rpc.send_rawtransaction(
        "020000000001014ce25ebde42ca0aa8564afdb55c255198859b84ff0a039e57e3d073862ab06ba01000000171600144b91e2cb9a15f66e91b3214b36ddec11409c51a7ffffffff0140195e290100000017a914686fabc9fd35360a70ee04f32f9b0c3cf0444d8587024730440220680a7aaa520d9986a2f2efa2fe2008356e2cb58183d5623d8a019e6b5664f3ed0220626c9a360a55d4259ce4c88c4e65e6ef9322a5417d90ab03c6dbba70999b39ff012103d1df5e099ccc06a1cff20d7a083d46c89aa0cbede72ce90ad31d9539b1646e0300000000".to_string(),
        true,
    ).unwrap();
    println!("{:#?}", send_rawtransaction);

    Ok(())
}
