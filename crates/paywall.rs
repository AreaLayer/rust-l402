use crate::Paywall;
use lnd_grpc_rust::LightningClient;
use lnd_grpc_rust::LND;
use lnd_grpc_rust::Macroon;
use nostr::Event;
use nostr::Zapper;
use nostr::WebLN;
pub struct Paywall {
    pub client: LightningClient,
    pub l402: bool,
    pub url: String,
    pub event: String,
    pub zapper: String,
    pub web_ln: String,
    pub macroon: String,
    pub paywall_id: String,
    pub paywall_secret: String,
    pub paywall_secret_hash: String,
}

impl Paywall {
    pub fn new(
        client: LightningClient,
        l402: bool,
        url: String,
        paywall_id: String,
        paywall_secret: String,
        event: String,
        zapper: String,
        web_ln: String,
        paywall_secret_hash: String,
    ) -> Paywall {
        Paywall {
            client,
            l402,
            url,
            zapper,
            web_ln,
            macroon,
            event,
            paywall_id,
            paywall_secret,
            paywall_secret_hash,
            macroon: String::new(),
            
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use nostr::Event;
    use nostr::Zapper;
    use
    nostr::WebLN;
    use std::env;
    use std::str::FromStr;
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(&format!("{} must be set", key))
    }
    fn get_env_var_bool(key: &str) -> bool {
        env::var(key).expect(&format!("{} must be set", key)).parse::<bool>().unwrap()
    }
}