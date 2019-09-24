pub fn import_private_key(rpc: &BitcoinRPC) -> Result<(), reqwest::Error> {
    let mut params = Vec::new();
    params.push("cMwCtdnswWkhWvSx7FvJzm2nJdmpN8dkn1pmDW5ioUHve6UtvfU9".to_string());
    params.push("alvin".to_string());

    let importprivkey: Value = rpc.send_raw("importprivkey", params)
        .expect("I didn't get value!")
        .json()?;
    println!("importprivkey {:#?}", importprivkey);
    Ok(())
}


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


//    let mut address = Vec::new();
//    address.push("n1uPfAqcF5vh8gCH4zpXcJy2vXPnK9dEqr");
//    let list_unspent: Unspent = rpc.list_unspent(Some(1), Some(99999999), address).unwrap();
//    println!("{:#?}", list_unspent);

//    let echo_json: Value= rpc.client.post(config::TEST_URL)
//        .json(
//            &serde_json::json!(
//
//                {"jsonrpc": "1.0",
//                "id":100,
//                "method": "createrawtransaction",
//                "params": [ [{"txid":"a14c0e7393b762fb71406f97f18422965d3360a218e5c9b33d699b0c921b6e0c","vout":0}], [{"mwRJfknvd3y65tC1Ckd5MNizj9StHadaJ1":0.001}]] }
//
//            )
//        )
//        .send()?
//        .json()?;
//
//    println!("createrawtransaction }{:#?", echo_json);



// sign_rawtransaction_with_wallet
let echo_json: Value = rpc.client
.post(config::TEST_URL)
.json(
& serde_json::json ! (
{
"jsonrpc": "1.0",
"id": 10,
"method": "signrawtransactionwithwallet",
"params": [
"02000000010c6e1b920c9b693db3c9e518a260335d962284f1976f4071fb62b793730e4ca10000000000ffffffff01a0860100000000001976a914ae7086368c8983429953dd2c2b3c79b1b3a0b60a88ac00000000",
[
{
"txid": "a14c0e7393b762fb71406f97f18422965d3360a218e5c9b33d699b0c921b6e0c",
"vout": 0,
"scriptPubKey": "76a914dfa0d6f68ada97595dac908c2d9821650e385a1088ac",
}
]
]
}
)
)
.send() ?
.json() ?;