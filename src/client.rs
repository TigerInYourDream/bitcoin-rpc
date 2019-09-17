///This mod is used for build rpc client
use reqwest::{Client as ReqClient, Response, Error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use crate::config;
use crate::api::Param;
use std::cell::Cell;
use crate::json::blockchaininfo::BlockChainInfo;
use crate::json::simple::{BlockCount, BestBlockHash, ConnectionCount, Difficulty, WalletInfo, ListWallets, AddressGroupings, Unspent};
use serde_json::{Value, to_value};

pub struct BitcoinRPC {
    pub client: ReqClient,
    // use for id, It's related to api::Param::id
    nonce: Cell<u64>,
}


impl BitcoinRPC {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        let authorization = config::authorize();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(AUTHORIZATION,
                       HeaderValue::from_str(authorization.as_str()).unwrap());
        BitcoinRPC {
            client: reqwest::Client::builder().default_headers(headers).build().unwrap(),
            nonce: Cell::new(0),
        }
    }

    fn build_request(&self, method: &'static str, params: Vec<Value>) -> Param {
        let id = self.nonce.get() + 1;
        self.nonce.set(id);
        //jsonrpc must be '1.0' or '2.0'
        Param::new(method, id, params, "1.0")
    }

    /// use reqwest::client::post  to post json object to rpc server
    /// rpc send from here
    pub fn send_raw(&self, method: &'static str, params: Vec<Value>) -> Result<Response, Error> {
        let params = self.build_request(method, params);
        self
            .client.post(config::TEST_URL)
            .json(&params)
            .send()
    }

    pub fn blockchaininfo(&self) -> Result<BlockChainInfo, Error> {
        self
            .send_raw("getblockchaininfo", Vec::new())
            .expect("I didn't get blockchaininfo!")
            .json()
    }

    pub fn block_count(&self) -> Result<BlockCount, Error> {
        self
            .send_raw("getblockcount", Vec::new())
            .expect("I didn't get count!")
            .json()
    }

    pub fn best_block_hash(&self) -> Result<BestBlockHash, Error> {
        self
            .send_raw("getbestblockhash", Vec::new())
            .expect("I didn't get best block hash!")
            .json()
    }

    pub fn connection_count(&self) -> Result<ConnectionCount, Error> {
        self
            .send_raw("getconnectioncount", Vec::new())
            .expect("I didn't get info!")
            .json()
    }

    pub fn difficulty(&self) -> Result<Difficulty, Error> {
        self
            .send_raw("getdifficulty", Vec::new())
            .expect("I didn't get info!")
            .json()
    }

    pub fn wallet_info(&self) -> Result<WalletInfo, Error> {
        self
            .send_raw("getwalletinfo", Vec::new())
            .expect("I didn't get wallet_info!")
            .json()
    }

    pub fn list_wallet(&self) -> Result<ListWallets, Error> {
        self
            .send_raw("listwallets", Vec::new())
            .expect("I didn't get listwallets!")
            .json()
    }

    pub fn list_address_groupings(&self) -> Result<AddressGroupings, Error> {
        self
            .send_raw("listaddressgroupings", Vec::new())
            .expect("I didn't get listaddressgroupings!")
            .json()
    }

    /// !use it carefully
    /// import privkey to wallet
    /// label is optional
    /// in default bitcoin core will rescan ,it will take hours to do it, when in rescan you will not get the right info about the address until rescan finished
    fn import_private_key(&self, private_key: String, label: Option<String>) -> Result<Value, Error> {
        let params = [
            to_value(private_key).unwrap(),
            to_value(label).unwrap(),
        ];

        self
            .send_raw("importprivkey", params.to_owned().to_vec())
            .expect("I didn't import_private_key")
            .json()
    }

    /// it still have another optional arguments, see detail at [https://bitcoincore.org/en/doc/0.18.0/rpc/wallet/listunspent/]
    /// for simple i don't use it. maybe it will add in future
    /// consider using Option<HashMap<&str, &str>>
    pub fn list_unspent(&self, minconf: Option<u64>, maxconf: Option<u64>, address: Vec<&'static str>) -> Result<Unspent, Error> {
        let params = [
            to_value(minconf).unwrap(),
            to_value(maxconf).unwrap(),
            to_value(address).unwrap(),
        ];

        self
            .send_raw("listunspent", params.to_owned().to_vec())
            .expect("I didn't get listunspent")
            .json()
    }
}
