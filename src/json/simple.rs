use super::*;

///block count object
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BlockCount {
    error: Option<String>,
    id: u64,
    result: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct BestBlockHash {
    error: Option<String>,
    id: u64,
    result: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ConnectionCount {
    error: Option<String>,
    id: u64,
    result: u64,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Difficulty {
    error: Option<String>,
    id: u64,
    result: u64,
}