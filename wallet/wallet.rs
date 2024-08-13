pub(crate) fn wallet_function() {
    println!("This is a function in the wallet module.");
}

use lnd_grpc::lnrpc::{wallet_unlocker_client::WalletUnlockerClient, WalletUnlocker, WalletUnlockerClient, PreImage};

impl WalletUnlockerClient {
    pub async fn unlock_wallet(&mut self, wallet_password: &str) -> Result<(), Box<dyn std::error::Error>> {
        let request = WalletUnlocker::UnlockWalletRequest {
            wallet_password: wallet_password.to_string(),
        };
    let response = self.unlock_wallet(request).await?;
    Ok(())
    }
}

impl PreImage {
    pub fn new(preimage: &[u8]) -> Self {
        PreImage {
            preimage: preimage.to_vec(),
        }
    }
}

