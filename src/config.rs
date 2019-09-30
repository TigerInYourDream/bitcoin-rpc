///This mod is used for user config, just like username password authentication and the url of the bitcoind node
use base64::encode;

const USERNAME: &'static str = "bitcoinrpc";
const PASSWORD: &'static str = "password";
pub const TEST_URL: &'static str = "http://192.168.1.7:18332";
pub const MAIN_URL: &'static str = "http://192.168.1.7:8332";
const BASIC: &'static str = "Basic ";

fn base() -> String {
    let mut authorize = USERNAME.to_string();
    authorize.push_str(":");
    authorize.push_str(PASSWORD);
    encode(&authorize)
}

pub fn authorize() -> String {
    let mut authorization = BASIC.to_string();
    authorization.push_str(&base());
    authorization
}
