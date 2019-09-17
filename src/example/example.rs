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