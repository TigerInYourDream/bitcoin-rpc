///This mod is used for build rpc client
use reqwest::{Client as ReqClient, Response, Error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use crate::config;
use std::sync::{Arc, Mutex};
use crate::api::Param;

pub struct BitcoinRPC {
    pub(crate) client: ReqClient,
    // use for id, It's related to api::Param::id
    nonce: Arc<Mutex<u64>>,
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
            nonce: Arc::new(Mutex::new(0)),
        }
    }

    fn last_nonce(&self) -> u64 {
        *self.nonce.lock().unwrap()
    }

    fn build_request(&self, method: &'static str, params: Vec<String>) -> Param {
        let mut id = self.last_nonce();
        id += 1;
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
}

///Warper for BitcoinRPC, use for simplify code
pub struct BitcoinRPCWarper(pub BitcoinRPC);

impl BitcoinRPCWarper {
    pub fn new() -> Self {
        BitcoinRPCWarper(BitcoinRPC::new())
    }
}
