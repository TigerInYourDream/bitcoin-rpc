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
        Self {
            txid,
            vout,
        }
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
pub struct SignRawTransaction {
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

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ValidateAddress {
    error: Option<Error>,
    id: u64,
    result: Option<ValidateAddressOutput>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ValidateAddressOutput {
    isvalid: bool,
    address: String,
    scriptPubKey: String,
    isscript: bool,
    iswitness: bool,
    witness_version: Option<f64>,
    witness_program: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DumpPrivkey {
    error: Option<Error>,
    id: u64,
    result: Option<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Balance {
    error: Option<Error>,
    id: u64,
    result: f64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct NewAddress {
    error: Option<Error>,
    id: u64,
    result: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GenerateToAddress {
    error: Option<Error>,
    id: u64,
    result: Vec<String>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FundRawTransaction {
    error: Option<Error>,
    id: u64,
    result: Option<FundRawTransactionOutput>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct FundRawTransactionOutput<> {
    changepos: u64,
    fee: f64,
    hex: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetTransaction {
    error: Option<Error>,
    id: u64,
    result: Option<GetTransactionOutput>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetTransactionOutput {
    amount: f64,
    #[serde(rename = "bip125-replaceable")]
    bip125_replaceable: String,
    confirmations: u64,
    details: Vec<GetTransactionDetail>,
    fee: f64,
    hex: String,
    time: u64,
    timereceived: u64,
    trusted: bool,
    txid: String,
    walletconflicts: Vec<String>,

}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetTransactionDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    abandoned: Option<bool>,
    address: String,
    amount: f64,
    category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    fee: Option<f64>,
    label: String,
    vout: u64,
}

/// This struct is build for sign_rawtransaction_offline, build for params
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SignOffline {
    txid: String,
    vout: u64,
    scriptPubKey: String,
    amount: f64,
}

impl SignOffline {
    pub fn new(txid: String, vout: u64, script_pub_key: String, amount: f64) -> SignOffline {
        Self {
            txid,
            vout,
            scriptPubKey: script_pub_key,
            amount,
        }
    }
}