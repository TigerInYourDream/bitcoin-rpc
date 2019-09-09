///This mod is used for build rpc client
use reqwest::Client as ReqClient;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use crate::config;

pub struct BitcoinRPC {
    pub(crate) client: ReqClient
}

impl BitcoinRPC {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        let authorization = config::authorize();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(AUTHORIZATION,
                       HeaderValue::from_str(authorization.as_str()).unwrap());
        BitcoinRPC {
            client: reqwest::Client::builder().default_headers(headers).build().unwrap()
        }
    }
}