mod client;
mod config;
mod api;

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPCWarper::new();
    let echo_json: serde_json::Value = rpc.
        0.send_raw("getblockchaininfo", Vec::new())
        .expect("I didn't get value!")
        .json()?;

    println!("{:#?}", echo_json);

    let echo_json2: serde_json::Value = rpc
        .0.send_raw("getblockcount", Vec::new())
        .expect("I didn't get value!")
        .json()?;
    println!("{:#?}", echo_json2);

    let echo_json3: serde_json::Value = rpc
        .0.send_raw("getbestblockhash", Vec::new())
        .expect("I didn't get value!")
        .json()?;
    println!("{:#?}", echo_json3);

    Ok(())
    // client::BitcoinRPC::new().getblockchaininfo();
}
