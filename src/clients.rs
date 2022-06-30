use std::collections::HashMap;

use crate::{Client, Transaction};

/// Container of [Client]s.
#[derive(Debug)]
pub struct Clients {
    clients: HashMap<u16, Client>,
}
impl Clients {
    /// Create new [Clients].
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    /// Apply a [Transaction].
    pub fn apply(&mut self, transaction: Transaction) {
        let client_id = transaction.client;

        let client = self.clients
            .entry(client_id)
            .or_insert(Client::new(client_id));

        client.apply(transaction)
    }
}
impl IntoIterator for Clients {
    type Item = Client;
    type IntoIter = std::collections::hash_map::IntoValues<u16, Client>;
    fn into_iter(self) -> std::collections::hash_map::IntoValues<u16, Client> {
        self.clients.into_values()
    }
}
