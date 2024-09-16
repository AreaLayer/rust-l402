pub mod wallet {
    use lnd_grpc::lnrpc::{wallet_unlocker_client::WalletUnlockerClient, UnlockWalletRequest, Preimage};
    use lnd_grpc::Invoice::{InvoiceSettlement, InvoiceType, InvoiceState};
    use std::error::Error;
    use sha2::{Sha256, Digest};
    // Pub struct to wallet
    pub struct Wallet {
        pub(crate) client: WalletUnlockerClient,
        pub(crate) preimage: Preimage,
        pub(crate) wallet_password: String,
    }
    // Wallet module
    pub(crate) fn wallet_function() {
        let mut client = AlbyGreenlightClient::new("http://localhost:8080");
        let mut client = lnd_grpc::lnrpc::wallet_unlocker_client::WalletUnlockerClient::connect("http://localhost:10009").await?;
        let mut client = client.get_wallet_unlocker_client();
        let mut preimage = Preimage::new(b"preimage_data");
        let mut wallet_password = "wallet_password";
    
        println!("This is a function in the wallet module.");

        // Preimage
        let preimage = Preimage::new(b"preimage_data");

        // Use of the preimage
        println!("Created a new Preimage with data: {:?}", example_preimage.preimage);
    }
   // Unlock wallet function
    pub(crate) fn unlock_wallet(
        &mut self,
        wallet_password: &str,
    ) -> Result<(), Box<dyn Error>> {
        let request = UnlockWalletRequest {
            wallet_password: wallet_password.to_string(),
        };
        let _response = self.unlock_wallet(request).await?;
        Ok(())
    }
    // Preimage struct
    pub(crate) fn pre_image() -> PreImage {
        PreImage {
            preimage: vec![0; 32],       // Example: 32 bytes of zero
            hash: vec![0; 32],           // Example: 32 bytes of zero
            hash_type: 0,                // Example: SHA-256
            timestamp: 0,                // Example: current or default timestamp
            preimage_hash: vec![0; 32],  // Example: SHA-256 hash of the preimage
        }
    }
   // Preimage hash struct
    pub(crate) fn pre_image_hash() -> PreImageHash {
        PreImageHash {
            preimage_hash: vec![],
        }
    }
   // Implementation of the WalletUnlockerClient trait
    impl WalletUnlockerClient {
        pub async fn unlock_wallet(
            &mut self,
            wallet_password: &str,
        ) -> Result<(), Box<dyn Error>> {
            let request = UnlockWalletRequest {
                wallet_password: wallet_password.to_string(),
            };
            let _response = self.unlock_wallet(request).await?;
            Ok(())
        }
    }

    impl PreImage {
        // Function to create a new PreImage
        fn new(preimage: Vec<u8>) -> Self {
            let mut hasher = Sha256::new();
            hasher.update(&preimage);
            let hash = hasher.finalize().to_vec();
    
            Self {
                preimage: preimage.clone(),
                hash: hash.clone(),
                hash_type: 0, // SHA-256
                timestamp: Self::current_timestamp(),
                preimage_hash: hash, // Optional, could be removed if redundant
            }
        }
    
        // Function to get the current timestamp
        fn current_timestamp() -> u64 {
            // Example of getting the current time as a timestamp
            use std::time::{SystemTime, UNIX_EPOCH};
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
            since_the_epoch.as_secs()
        }
    }

         // Invoice struct
         fn Invoice() -> Invoice {
            Invoice {
                 r#type: InvoiceType::Invoice,
                 value: 1000,
                 memo: "invoice_memo".to_string(),
                 r#final: false,
                 creation_date: 0,
                 settle_date: 0,
                 payment_request: "payment_request".to_string(),
                 add_index: 0,
                 settle_index: 0,
                 amt_paid: 0,
                 amt_paid_sat: 0,
                 amt_paid_msat: 0,
                 state: InvoiceState::Open,
                 htlcs: vec![],
                 features: vec![],
                 is_keysend: false,
                 is_canceled: false,
                 is_replaceable: false,
                 cooperative_close_address: "cooperative_close_address".to_string(),
                 close_address: "close_address".to_string(),
                 settle_address: "settle_address".to_string(),
                 creation_date_str: "creation_date_str".to_string(),
                 settle_date_str: "settle_date_str".to_string(),
                 payment_request_str: "payment_request_str".to_string(),
                 add_index_str: "add_index_str".to_string(),
                 settle_index_str: "settle_index_str".to_string(),
                 amt_paid_str: "amt_paid_str".to_string(),
                 amt_paid_sat_str: "amt_paid_sat_str".to_string(),
                 amt_paid_msat_str: "amt_paid_msat_str".to_string(),
                 amt_paid_sat_str: "amt_paid_sat_str".to_string(),
                 amt_paid_msat_str: "amt_paid_msat_str".to_string(),
                 state_str: "state_str".to_string(),
                 r#final_str: "final_str".to_string(),
                 is_keysend_str: "is_keysend_str".to_string(),
                 is_canceled_str: "is_canceled_str".to_string(),
                 is_replaceable_str: "is_replaceable_str".to_string(),
                 settle_address_str: "settle_address_str".to_string(),
                 close_address_str: "close_address_str".to_string(),
                 cooperative_close_address_str: "cooperative_close_address_str".to_string(),
                 settle_address_str: "settle_address_str".to_string(),
            };

        // Invoice State, nvoice Settlement, Invoice Type struct

        fn InvoiceState() -> InvoiceState {
            InvoiceState {
                state: InvoiceState::Open,
                state_str: "Open".to_string(),
            }
        }

        fn InvoiceSettlement() -> InvoiceSettlement {
            InvoiceSettlement {
                settlement: InvoiceSettlement::Settled,
                settlement_str: "Settled".to_string(),
            }
        }

        fn InvoiceType() -> InvoiceType {
            InvoiceType {
                r#type: InvoiceType::Invoice,
                r#type_str: "Invoice".to_string(),
            }
        }

         }
    fn main() {
        // Example usage
        let preimage = vec![1, 2, 3, 4]; // Replace with a secure random 32-byte value
        let preimage_struct = PreImage::new(preimage);

    #[cfg(test)]
    mod tests {
        use super::*;
        use lnd_grpc::lnrpc::wallet_unlocker_client::WalletUnlockerClient;
        use tokio;

        #[tokio::test]
        async fn test_unlock_wallet() -> Result<(), Box<dyn Error>> {
            // Example test code for unlocking the wallet
            let mut client = WalletUnlockerClient::new("localhost:10009").await?;
            let result = client.unlock_wallet("testpassword").await?;
            assert!(result.is_ok());
            Ok(())
        }
    }
}

fn main() {
    wallet::wallet_function();
}
}
#[cfg(test)]
mod tests {
    use super::*;
    use lnd_grpc::lnrpc::wallet_unlocker_client::WalletUnlockerClient;
    use tokio;
    #[tokio::test]
    async fn test_unlock_wallet() -> Result<(), Box<dyn Error>> {
        // Example test code for unlocking the wallet
        let mut client = WalletUnlockerClient::new("localhost:10009").await?;
        let result = client.unlock_wallet("testpassword").await?;
        assert!(result.is_ok());
        Ok(())
        }
    }