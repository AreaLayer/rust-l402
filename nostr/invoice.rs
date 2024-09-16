use nostr::{Event, EventKind};
use nostr::{ClientMessage, EventBuilder, Keys, Relay};
use nostr::nwc::{NostrWalletURI, Subscribe, PayInvoice, PayRequest , PaySend, CreateInvoice, LookupInvoice, GetBalance};
use nostr::Zapper;
use nostr::ZapperBackend::{NWC, WebLN};
use rust_l402::client::L402Client;
use tonic::transport::{Channel, ClientTlsConfig};
use lnd_grpc::rpc::invoice::Invoice;
use lnd_grpc::rpc::invoice::{InvoiceState, InvoiceSettlement, InvoiceType};

// Pub crate to Nostr
pub(crate) use nostr::invoice_request::invoice_request_client::InvoiceRequestClient;

// Pub struct

pub struct NostrWalletURI {
    relay: Relay,
    keys: Keys,
    client: NostrWalletURI,
}
// Pub mod to Nostr
pub mod nostr {
    pub mod invoice_request {
        tonic::include_proto!("invoice_request");
    }
}

fn main() {
    let keys = Keys::generate();
    let relay = Relay::new("wss://relay.damus.io");
    let mut relay = relay.connect().unwrap();
    let mut client = L402Client::new("http://localhost:8080"); // Replace for API endpoint
    let invoice = Invoice {
        memo: "test".to_string(),
        value: 1000,
        description_hash: vec![0; 32],
        r_preimage: vec![0; 32],
        invoice_state: lnd_grpc::rpc::invoice::InvoiceState::Open as i32,
        r_hash: vec![0; 32],
        payment_addr: vec![0; 32],
        payment_request: "test".to_string(),
        creation_date: 1234567890,
        settle_date: 1234567890,
        nwc: None,
        webln: None,
        invoice_type: lnd_grpc::rpc::invoice::InvoiceType::Unknown as i32,
        settle_index: 0,
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keys = Keys::generate();
    let relay = Relay::new("wss://relay.damus.io");
    let mut relay = relay.connect().unwrap();
    let mut client = L402Client::new("http://localhost:8080"); // Replace for API endpoint
    let invoice = Invoice {
        memo: "test".to_string(),
        value: 1000,
        description_hash: vec![0; 32],
        r_preimage: vec![0; 32],
        invoice_state: lnd_grpc::rpc::invoice::InvoiceState::Open as i32,
        r_hash: vec![0; 32],
        payment_addr: vec![0; 32],
        payment_request: "test".to_string(),
        creation_date: 1234567890,
        settle_date: 1234567890,
        invoice_type: lnd_grpc::rpc::invoice::InvoiceType::Unknown as i32,
        settle_index: 0,
    };
}
// Connect to LND
async fn connect_to_lnd(
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Invoice request
async fn invoice_request(
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Invoice State
async fn invoice_state(
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Invoice Type
async fn invoice_type(
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Invoice Settlement
async fn invoice_setllement(
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Nostr Wallet URI
async fn NostrWalletURI(
    relay: &str,
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
    invoice_state: lnd_grpc::rpc::invoice::InvoiceState,

) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Subscribe
async fn Subscribe(
    kind: &str,
    author: &str,
    filter: &str,
    
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Pay Request
async fn PayRequest(
    relay: &str,
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
    invoice_state: lnd_grpc::rpc::invoice::InvoiceState,
    )
    -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
    }
// Create Invoice
async fn CreateInvoice(
    relay: &str,
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
    invoice_state: lnd_grpc::rpc::invoice::InvoiceState,
    )
    -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = NostrWalletURI::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
    }
// Pay Invoice
async fn PayInvoice(
    relay: &str,
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
    invoice_state: lnd_grpc::rpc::invoice::InvoiceState,
    ) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Lookup Invoice
async fn LookupInvoice(
    relay: &str,
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
    invoice_state: lnd_grpc::rpc::invoice::InvoiceState,
    )
    ->Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(host)
        .tls_config(tls_config.unwrap())
        .connect()
        .await?;
    let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
}
// Get Balance
async fn GetBalance(
    relay: &str,
    host: &str,
    tls_config: Option<ClientTlsConfig>,
    macaroon: Option<Vec<u8>>,
    invoice: Invoice,
    invoice_state: lnd_grpc::rpc::invoice::InvoiceState,
) -> Result<(), Box<dyn std::error::Error>> {
  let mut client = nostr::Client::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    Ok(())
}
#[cfg(test)]
mod tests {
    use lnd_grpc::Invoice::{InvoiceType, InvoiceSettlement, InvoiceState};
    use nostr::nwc::{PayRequest, PaySend};
    fn main () {
        let mut client = NostrWalletURI::new(relay);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    let zapper = Zapper::new(keys, client);
    let event = EventBuilder::new(EventKind::TextNote, &keys);
    Ok(())
    }
}