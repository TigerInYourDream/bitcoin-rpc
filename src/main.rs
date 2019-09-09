mod client;
mod config;

fn main() -> Result<(), reqwest::Error> {
    let rpc = client::BitcoinRPC::new().client;
    let echo_json: serde_json::Value = rpc
        .post(config::TEST_URL)
        .json(&serde_json::json!({
            "jsonrpc": "1.0",
            "method": "getblockchaininfo",
            "params":[],
            "id": "test_url"
        }))
        .send().expect("I didn't get value!")
        .json()?;

    println!("authorize {:?}", config::authorize());
    println!("{:#?}", echo_json);

    Ok(())
}
