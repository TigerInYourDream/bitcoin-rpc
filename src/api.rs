///build request params

use serde::{Serialize, Deserialize};

///json request object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Param {
    method: &'static str,
    //id for the request, it's will show in the response
    id: u64,
    params: Vec<String>,
    jsonrpc: &'static str,
}

impl Param {
    pub fn new(method: &'static str, id: u64, params: Vec<String>, jsonrpc: &'static str) -> Self {
        Param {
            method,
            id,
            params,
            jsonrpc,
        }
    }
}


