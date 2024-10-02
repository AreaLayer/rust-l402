use crate::Nostr;

pub (crate) struct NostrClient {
    nostr: Nostr,
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
        todo!()
    }
}