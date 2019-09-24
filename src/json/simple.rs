use super::*;

///bitcoind Error object
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Error {
    code: i64,
    message: String,
}

///block count object
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlockCount {
    error: Option<Error>,
    id: u64,
    result: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BestBlockHash {
    error: Option<Error>,
    id: u64,
    result: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConnectionCount {
    error: Option<Error>,
    id: u64,
    result: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Difficulty {
    error: Option<String>,
    id: u64,
    result: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WalletInfo {
    error: Option<Error>,
    id: u64,
    result: Option<WalletInfoObject>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct WalletInfoObject {
    balance: f64,
    hdseedid: String,
    immature_balance: f64,
    keypoololdest: f64,
    keypoolsize: f64,
    keypoolsize_hd_internal: f64,
    paytxfee: f64,
    private_keys_enabled: bool,
    txcount: u64,
    unconfirmed_balance: f64,
    walletname: String,
    walletversion: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ListWallets {
    error: Option<Error>,
    id: u64,
    result: Option<Vec<String>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AddressGroupings {
    error: Option<Error>,
    id: u64,
    result: Option<Vec<Vec<Address>>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Address(String, f64, String);

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Unspent {
    error: Option<Error>,
    id: u64,
    result: Option<Vec<UnspentOutput>>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct UnspentOutput {
    address: String,
    amount: f64,
    confirmations: u64,
    desc: String,
    label: String,
    safe: bool,
    scriptPubKey: String,
    solvable: bool,
    spendable: bool,
    txid: String,
    vout: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RawTransaction {
    error: Option<Error>,
    id: u64,
    result: Option<String>,
}

/// use this struct for RawTransaction , use it in [client::BitcoinRPC::create_raw_transaction()]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct TxIn {
    txid: String,
    vout: u64,
}

impl TxIn {
    pub fn new(txid: String, vout: u64) -> Self {
        Self { txid, vout }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DecodeRawTransaction {
    error: Option<Error>,
    id: u64,
    result: Option<DecodeRawTransactionOutput>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DecodeRawTransactionOutput {
    hash: String,
    locktime: u64,
    size: u64,
    txid: String,
    version: u64,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    vsize: u64,
    weight: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Vin {
    scriptSig: ScriptSig,
    sequence: u64,
    txid: String,
    vout: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ScriptSig {
    asm: Option<String>,
    hex: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Vout {
    n: u64,
    scriptPubKey: ScriptPubKey,
    value: f64,

}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ScriptPubKey {
    addresses: Vec<String>,
    asm: Option<String>,
    hex: Option<String>,
    reqSigs: u64,
    #[serde(rename = "type")]
    ty_pe: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SignRawTransactionWithWallet {
    error: Option<Error>,
    id: u64,
    result: Option<SignRawTransactionWithWalletOutput>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SignRawTransactionWithWalletOutput {
    complete: bool,
    errors: Option<Vec<SignRawTransactionWithWalletError>>,
    hex: String,
}


#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SignRawTransactionWithWalletError {
    error: Option<String>,
    scriptSig: String,
    sequence: u64,
    txid: String,
    vout: u64,
    witness: Vec<String>,
}

///use for sign rawtransactions use it in [client::BitcoinRPC::create_raw_transaction]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct PrevTX {
    txid: String,
    vout: u64,
    scriptPubKey: String,
}

impl PrevTX {
    #[allow(non_snake_case)]
    pub fn new(txid: String, vout: u64, scriptPubKey: String) -> PrevTX {
        Self {
            txid,
            vout,
            scriptPubKey,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Sendrawtransaction {
    error: Option<Error>,
    id: u64,
    result: Option<String>,
}

