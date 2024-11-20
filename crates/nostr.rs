use crate::Nostr;

pub (crate) struct NostrClient {
    nostr: Nostr,
}

fn new(nostr: Nostr) -> Self {
    Self {
        nostr,
    }
}

impl NostrClient {
    pub fn new(nostr: Nostr) -> Self {
        Self {
            nostr,
        }
    }
}

impl Client for NostrClient {
    fn send(&self, message: Message) -> Result<(), Error> {
        // TODO: Implement the send method
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Nostr, Message};
    use crate::Error;
    use crate::Message;
    use crate::Client;
    use l402::*;

    fn test_nostr_client() {
        let nostr = Nostr::new();
        let client = NostrClient::new(nostr);
        let message = Message::new("Hello, world!");
        let result = client.send(message);
        assert!(result.is_ok());
    }
    fn test_nostr_client_send() {
        let nostr = Nostr::new();
        let client = NostrClient::new(nostr);
        let message = Message::new("Hello, world!");
        let result = client.send(message);
        assert!(result.is_ok());
    }
    fn test_nostr_client_send_message() {
        let nostr = Nostr::new();
        let client = NostrClient::new(nostr);
        let message = Message::new("Hello, world!");
        let result = client.send(message);
        assert!(result.is_ok());
    }
    fn test_nostr_client_send_message_to_nostr() {
        let nostr = Nostr::new();
        let client = NostrClient::new(nostr);
        let message = Message::new("Hello, world!");
        let result = client.send(message);
        assert!(result.is_ok());
    }
}
