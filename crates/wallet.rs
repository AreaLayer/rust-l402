use crate::Wallet;

pub (crate) struct Wallets {
    wallets: Vec<Wallet>
}
impl Wallets {
    pub fn new() -> Self {
        Wallets {
            wallets: Vec::new()
        }
    }
}
impl Wallets {
    pub fn add_wallet(&mut self, wallet: Wallet) {
        self.wallets.push(wallet);
    }
}