// Import necessary modules from the crate and external libraries
use crate::Paywall;
use lnd_grpc_rust::LightningClient;
use lnd_grpc_rust::LND;
use lnd_grpc_rust::Macroon;
use nostr::Event;
use nostr::Zapper;
use nostr::WebLN;

/// Represents a paywall structure with various payment and metadata-related fields.
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
    /// Creates a new instance of the Paywall struct.
    /// 
    /// # Parameters
    /// - `client`: A LightningClient instance for payment handling.
    /// - `l402`: A boolean indicating if L402 authentication is enabled.
    /// - `url`: The URL associated with the paywall.
    /// - `paywall_id`: The unique identifier for the paywall.
    /// - `paywall_secret`: The secret key for the paywall.
    /// - `event`: The associated event for the paywall.
    /// - `zapper`: The zapper associated with the paywall.
    /// - `web_ln`: WebLN instance for web-based lightning interactions.
    /// - `paywall_secret_hash`: A hash of the paywall secret.
    /// 
    /// # Returns
    /// A new Paywall instance.
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
            macroon: String::new(),
            event,
            paywall_id,
            paywall_secret,
            paywall_secret_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Fetches an environment variable as a string.
    /// 
    /// # Parameters
    /// - `key`: The name of the environment variable.
    /// 
    /// # Returns
    /// The value of the environment variable.
    fn get_env_var(key: &str) -> String {
        env::var(key).expect(&format!("{} must be set", key))
    }

    /// Fetches an environment variable as a boolean.
    /// 
    /// # Parameters
    /// - `key`: The name of the environment variable.
    /// 
    /// # Returns
    /// The boolean value of the environment variable.
    fn get_env_var_bool(key: &str) -> bool {
        env::var(key).expect(&format!("{} must be set", key)).parse::<bool>().unwrap()
    }
}
