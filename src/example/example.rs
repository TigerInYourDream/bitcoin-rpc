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
//    println!("createrawtransaction {:#?}", echo_json);