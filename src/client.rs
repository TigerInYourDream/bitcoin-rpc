///This mod is used for build rpc client
use reqwest::{Client as ReqClient, Response, Error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use crate::config;
use crate::api::Param;
use std::cell::Cell;
use crate::json::blockchaininfo::BlockChainInfo;
use crate::json::simple::{BlockCount, BestBlockHash, ConnectionCount, Difficulty};

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

    fn build_request(&self, method: &'static str, params: Vec<String>) -> Param {
        let id = self.nonce.get() + 1;
        self.nonce.set(id);
        //jsonrpc must be '1.0' or '2.0'
        Param::new(method, id, params, "1.0")
    }

    /// use reqwest::client::post  to post json object to rpc server
    /// rpc send from here
    pub fn send_raw(&self, method: &'static str, params: Vec<String>) -> Result<Response, Error> {
        let params = self.build_request(method, params);
        self.client.post(config::TEST_URL)
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

    pub fn connectioncount(&self) -> Result<ConnectionCount, Error> {
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
}

