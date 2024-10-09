use crate::Paywall;
use lnd_grpc::LightningClient;

pub struct Paywall {
    pub client: LightningClient,
    pub l402: bool,
    pub url: String,
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
        paywall_secret_hash: String,
    ) -> Paywall {
        Paywall {
            client,
            l402,
            url,
            paywall_id,
            paywall_secret,
            paywall_secret_hash,
        }
    }
}