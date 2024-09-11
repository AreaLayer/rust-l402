use nostr::{ClientMessage, EventBuilder, Keys, Relay};
use rust_l402::client::L402Client;
use tonic::transport::{Channel, ClientTlsConfig};
use tonic::Request;
use lnd_grpc::rpc::invoice::Invoice;
use lnd_grpc::rpc::lightning_client::LightningClient;
use lnd_grpc::rpc::AddInvoiceResponse;
use tokio::time::{self, Duration};
use std::env;
use std::error::Error;
use tracing::{info, warn};

pub mod lnd_grpc {
    tonic::include_proto!("lnrpc"); // Proto path for LND gRPC
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables or defaults
    let lnd_address = env::var("LND_ADDRESS").unwrap_or_else(|_| "https://localhost:10009".to_string());
    let relay_url = env::var("NOSTR_RELAY_URL").unwrap_or_else(|_| "wss://relay.your-nostr-server.com".to_string());
    let invoice_amount_sat: i64 = env::var("INVOICE_AMOUNT_SAT").unwrap_or_else(|_| "1000".to_string()).parse()?;

    // Initialize L402 client (optionally, depending on your use case)
    let l402_client = L402Client::new("https://your-l402-server-url").await.unwrap();

    // Set up Nostr keys and relay
    let keys = Keys::generate();
    let relay = setup_nostr_relay(&relay_url).await?;

    // Generate a Nostr event to request an invoice
    let invoice_request_content = "Requesting invoice for service X"; // Example content
    let invoice_event = EventBuilder::new_text_event(&keys, invoice_request_content)?;
    
    // Send the event to the relay and log the result
    if let Err(e) = relay.send(ClientMessage::new_event(invoice_event)).await {
        warn!("Failed to send Nostr event: {}", e);
    } else {
        info!("Successfully sent Nostr invoice request");
    }

    // Connect to LND node and generate an invoice
    let lnd_client = match connect_to_lnd(&lnd_address).await {
        Ok(client) => client,
        Err(e) => {
            warn!("Failed to connect to LND: {}", e);
            return Err(Box::new(e));
        }
    };

    let invoice = match generate_lnd_invoice(&lnd_client, invoice_amount_sat).await {
        Ok(inv) => inv,
        Err(e) => {
            warn!("Failed to generate LND invoice: {}", e);
            return Err(Box::new(e));
        }
    };

    // Create Nostr event with the generated invoice and log success/failure
    let invoice_content = format!("Invoice generated: {}", invoice.payment_request);
    let invoice_event = EventBuilder::new_text_event(&keys, &invoice_content)?;
    if let Err(e) = relay.send(ClientMessage::new_event(invoice_event)).await {
        warn!("Failed to send invoice via Nostr: {}", e);
    } else {
        info!("Successfully sent Nostr invoice response");
    }

    // Disconnect relay after sending invoice
    if let Err(e) = relay.disconnect().await {
        warn!("Failed to disconnect Nostr relay: {}", e);
    }

    Ok(())
}

// Helper function to set up Nostr relay with retries
async fn setup_nostr_relay(relay_url: &str) -> Result<Relay, Box<dyn Error>> {
    for _ in 0..3 {
        match Relay::new(relay_url).await {
            Ok(relay) => {
                relay.connect().await?;
                info!("Connected to Nostr relay at {}", relay_url);
                return Ok(relay);
            }
            Err(e) => {
                warn!("Failed to connect to Nostr relay: {}", e);
                time::sleep(Duration::from_secs(5)).await; // Retry after delay
            }
        }
    }
    Err("Failed to connect to Nostr relay after 3 attempts".into())
}

// Connect to LND node using gRPC with TLS config and macaroon authentication
async fn connect_to_lnd(lnd_address: &str) -> Result<LightningClient<Channel>, Box<dyn Error>> {
    let tls_config = ClientTlsConfig::new();
    let channel = Channel::from_static(lnd_address)
        .tls_config(tls_config)?
        .connect()
        .await?;
    let client = LightningClient::new(channel);
    info!("Connected to LND at {}", lnd_address);
    Ok(client)
}

// Function to generate an invoice using LND with retries
async fn generate_lnd_invoice(client: &LightningClient<Channel>, amount_sat: i64) -> Result<AddInvoiceResponse, Box<dyn Error>> {
    let invoice = Invoice {
        value: amount_sat, // Amount in satoshis
        memo: Some("Service X payment".into()),
        expiry: Some(3600), // 1-hour expiry
        ..Default::default()
    };

    for _ in 0..3 {
        match client.add_invoice(Request::new(invoice.clone())).await {
            Ok(response) => {
                info!("Successfully generated invoice with payment request: {}", response.get_ref().payment_request);
                return Ok(response.into_inner());
            }
            Err(e) => {
                warn!("Failed to generate LND invoice: {}", e);
                time::sleep(Duration::from_secs(5)).await; // Retry after delay
            }
        }
    }

    Err("Failed to generate LND invoice after 3 attempts".into())
}
