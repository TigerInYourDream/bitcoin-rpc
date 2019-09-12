use super::*;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlockChainInfo {
    error: Option<String>,
    id: u64,
    result: Option<BlockChainObject> ,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Softfork {
    /// Name of softfork
    pub id: String,
    /// Block version
    pub version: u64,
    /// Progress toward rejecting pre-softfork blocks
    pub reject: RejectStatus,
}

/// Progress toward rejecting pre-softfork blocks
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct RejectStatus {
    /// `true` if threshold reached
    pub status: bool,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlockChainObject {
    bestblockhash: String,
    //todo build bip9_softforks
    bip9_softforks: Value,
    blocks: u64,
    chain: String,
    chainwork: String,
    difficulty: f64,
    headers: u64,
    initialblockdownload: bool,
    mediantime: u64,
    pruned: bool,
    size_on_disk: u64,
    softforks: Vec<Softfork>,
    verificationprogress: f64,
    warnings: String,
}