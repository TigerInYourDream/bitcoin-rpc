///!This mod is used for build rpc client
use reqwest::{Client as ReqClient, Response, Error};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use crate::config;
use crate::api::Param;
use std::cell::Cell;
use crate::json::blockchaininfo::BlockChainInfo;
use crate::json::simple::{BlockCount, BestBlockHash, ConnectionCount, Difficulty, WalletInfo, ListWallets, AddressGroupings, Unspent, RawTransaction, TxIn, DecodeRawTransaction, SignRawTransaction, PrevTX, Sendrawtransaction, SignRawTransactionWithWalletOutput, ValidateAddress, DumpPrivkey, Balance, NewAddress, GenerateToAddress, FundRawTransaction, GetTransaction};
use serde_json::{Value, to_value};
use std::collections::HashMap;
use crate::json::simple;

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
    /// import private_key to wallet
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
    ///
    pub fn list_unspent(&self, minconf: Option<u64>, maxconf: Option<u64>, address: Option<Vec<&'static str>>) -> Result<Unspent, Error> {
        let mut params = Vec::new();

        if let Some(min) = minconf {
            params.push(to_value(min).unwrap());
        } else {
            params.push(to_value(0).unwrap());
        }

        if let Some(max) = maxconf {
            params.push(to_value(maxconf).unwrap());
        } else {
            params.push(to_value(9999999).unwrap());
        }

        if let Some(addr) = address {
            params.push(to_value(addr).unwrap());
        }

        self
            .send_raw("listunspent", params)
            .expect("I didn't get listunspent")
            .json()
    }

    /// see more default at [https://bitcoincore.org/en/doc/0.18.0/rpc/rawtransactions/createrawtransaction/]
    pub fn create_raw_transaction(&self, txin: Vec<TxIn>, to_address: HashMap<String, f64>) -> Result<RawTransaction, Error> {
        let params: [Value; 2] = [
            to_value(&txin).unwrap(),
            to_value(to_address).unwrap(),
        ];

        self
            .send_raw("createrawtransaction", params.to_owned().to_vec())
            .expect("I didn't create_raw_transaction")
            .json()
    }

    pub fn decode_raw_transaction(&self, hexstring: String) -> Result<DecodeRawTransaction, Error> {
        let params = [
            to_value(hexstring).unwrap(),
        ];

        self.
            send_raw("decoderawtransaction", params.to_owned().to_vec())
            .expect("I didn't get decode_raw_transaction")
            .json()
    }

    ///see more detail at [https://bitcoincore.org/en/doc/0.18.0/rpc/wallet/signrawtransactionwithwallet/]
    /// hex is hexstring of raw transaction
    /// you can also sign raw transaction with key ,see detail at [https://bitcoincore.org/en/doc/0.18.0/rpc/rawtransactions/signrawtransactionwithkey/]
    ///
    /// rawtransaction is a raw transaction hex string, it comes from create_raw_transaction's result
    ///
    /// txid vout and script_pub_key build prevtxs ( It's all comes from prevtxs you can find it in Result Value of function "list_unspent"
    ///
    pub fn sign_rawtransaction_with_wallet(&self, rawtransaction: String) -> Result<SignRawTransaction, Error> {
        let params = [
            to_value(&rawtransaction).unwrap(),
        ];

        self
            .send_raw("signrawtransactionwithwallet", params.to_owned().to_vec())
            .expect("I didn't signrawtransactionwithwallet")
            .json()
    }

    pub fn sign_rawtransaction_with_key(&self, rawtransaction: String, private_key: String) -> Result<SignRawTransaction, Error> {
        let params = [
            to_value(&rawtransaction).unwrap(),
            to_value(&[private_key]).unwrap()
        ];

        self
            .send_raw("signrawtransactionwithkey", params.to_owned().to_vec())
            .expect("I didn't signrawtransactionwithkey")
            .json()
    }

    /// The pram allow_high_fee is set for allow high tx fee ,default = false.
    pub fn send_rawtransaction(&self, signed_rawtransaction: String, allow_high_fee: bool) -> Result<Sendrawtransaction, Error> {
        let params = [
            to_value(&signed_rawtransaction).unwrap(),
            to_value(allow_high_fee).unwrap(),
        ];

        self
            .send_raw("sendrawtransaction", params.to_owned().to_vec())
            .expect("I didn't send the transaction")
            .json()
    }

    pub fn validate_address(&self, address: String) -> Result<ValidateAddress, Error> {
        let params = [
            to_value(&address).unwrap(),
        ];

        self
            .send_raw("validateaddress", params.to_owned().to_vec())
            .expect("I didn't validate the address")
            .json()
    }

    pub fn dumpprivkey(&self, address: String) -> Result<DumpPrivkey, Error> {
        let params = [
            to_value(&address).unwrap(),
        ];

        self
            .send_raw("dumpprivkey", params.to_owned().to_vec())
            .expect("I didn't validate the address")
            .json()
    }

    pub fn get_balance(&self) -> Result<Balance, Error> {
        self
            .send_raw("getbalance", Vec::new())
            .expect("I didn't validate the address")
            .json()
    }

    /// see more details at [https://bitcoincore.org/en/doc/0.18.0/rpc/wallet/getnewaddress/]
    /// you can add label in it
    pub fn get_new_address(&self, label: Option<String>) -> Result<NewAddress, Error> {
        let mut params = Vec::new();
        if let Some(label) = label {
            params.push(to_value(label).unwrap());
        }

        self
            .send_raw("getnewaddress", params)
            .expect("I didn't validate the address")
            .json()
    }

    /// see more details at [https://bitcoincore.org/en/doc/0.18.0/rpc/generating/generatetoaddress/]
    pub fn generate_to_address(&self, nblocks: u64, address: String) -> Result<GenerateToAddress, Error> {
        let params = [
            to_value(&nblocks).unwrap(),
            to_value(&address).unwrap(),
        ];

        self
            .send_raw("generatetoaddress", params.to_owned().to_vec())
            .expect("I didn't generate to address")
            .json()
    }

    /// FundRawTransaction with change
    /// Add inputs to a transaction until it has enough in value to meet its out value.
    /// There also have some other option in this function
    /// See more details at [https://bitcoincore.org/en/doc/0.18.0/rpc/rawtransactions/fundrawtransaction/]
    pub fn fund_rawtransaction(&self, hex: String, change_address: String) -> Result<FundRawTransaction, Error> {
        let mut addresses_map = HashMap::new();
        addresses_map.insert("changeAddress".to_string(), change_address);

        let params = [
            to_value(&hex).unwrap(),
            to_value(&addresses_map).unwrap(),
        ];

        self
            .send_raw("fundrawtransaction", params.to_owned().to_vec())
            .expect("I didn't fund raw transaction")
            .json()
    }

    ///Get detailed information about in-wallet transaction <txid>
    /// There is also have a simlar function called "getrawtransaction"
    /// see more details at [https://bitcoincore.org/en/doc/0.18.0/rpc/wallet/gettransaction/]
    pub fn get_transaction(&self, txid: String, include_watchonly: Option<bool>) -> Result<GetTransaction, Error> {
        let mut params = Vec::new();
        params.push(to_value(&txid).unwrap());

        if let Some(include_watchonly) = include_watchonly {
            params.push(to_value(&include_watchonly).unwrap());
        }

        self
            .send_raw("gettransaction", params)
            .expect("I didn't get transaction")
            .json()
    }
}

