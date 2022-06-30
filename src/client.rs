//! Type representation of a client.

use std::collections::{HashMap, HashSet};
use serde::Serialize;

use crate::four_decimals::FourDecimals;
use crate::{Transaction, TransactionType};
use crate::MyError;

#[derive(Debug, Serialize, PartialEq, Eq)]
struct ClientRaw {
    client: u16,
    available: FourDecimals,
    held: FourDecimals,
    total: FourDecimals,
    locked: bool,
}

/// Type representation of a client.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(into = "ClientRaw")]
pub struct Client {
    id: u16,
    available: FourDecimals,
    held: FourDecimals,
    locked: bool,
    tx_amounts: HashMap<u32, FourDecimals>,
    disputes: HashSet<u32>,
}
impl Client {
    /// Create a new [Client].
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: FourDecimals::default(),
            held: FourDecimals::default(),
            locked: false,
            tx_amounts: HashMap::new(),
            disputes: HashSet::new(),
        }
    }
    /// Get the [Client] total.
    pub fn total(&self) -> FourDecimals {
        self.available + self.held
    }
    /// Get the [Client] available balance.
    pub fn available(&self) -> FourDecimals {
        self.available
    }
    /// Get the [Client] held balance.
    pub fn held(&self) -> FourDecimals {
        self.held
    }

    /// Apply a [Transaction].
    pub fn apply(&mut self, transaction: Transaction) -> Result<(), MyError> {
        if self.locked {
            return Ok(())
        }

        let amount = transaction.amount.unwrap_or_default();

        match transaction.r#type {
            TransactionType::Deposit => {
                self.tx_amounts.insert(transaction.tx, amount);
                self.available = self.available + amount;
            },
            TransactionType::Withdrawal => {
                if amount > self.available {
                    return Err(
                        MyError::BalanceLowForWithdrawal(self.id, transaction.tx))
                }
                self.tx_amounts.insert(transaction.tx, amount);
                self.available = self.available - amount;
            },
            TransactionType::Dispute => {
                if !self.disputes.insert(transaction.tx) {
                    return Ok(())
                };
                let amount = match self.tx_amounts.get(&transaction.tx) {
                    None => return Ok(()),
                    Some(amount) => amount,
                };
                self.available = self.available - *amount;
                self.held = self.held + *amount;
            },
            TransactionType::Resolve => {
                if self.disputes.take(&transaction.tx).is_none() {
                    return Ok(())
                }
                let amount = match self.tx_amounts.get(&transaction.tx) {
                    None => return Ok(()),
                    Some(amount) => amount,
                };
                self.held = self.held - *amount;
                self.available = self.available + *amount;
            },
            TransactionType::Chargeback => {
                if self.disputes.take(&transaction.tx).is_none() {
                    return Ok(())
                }
                let amount = match self.tx_amounts.get(&transaction.tx) {
                    None => return Ok(()),
                    Some(amount) => amount,
                };
                self.held = self.held - *amount;
                self.locked = true;
            },
        };

        Ok(())
    }
}
impl Into<ClientRaw> for Client {
    fn into(self) -> ClientRaw {
        ClientRaw {
            client: self.id,
            available: self.available,
            held: self.held,
            total: self.available + self.held,
            locked: self.locked,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    use csv::Writer;

    #[test]
    pub fn serialize() {
        let available = FourDecimals { integer: 1, decimal: 0 };
        let held = FourDecimals { integer: 2, decimal: 2 };

        let client = Client {
            id: 1, available, held, locked: false,
            tx_amounts: HashMap::new(), disputes: HashSet::new(),
        };

        let output = Vec::new();
        let mut wtr = Writer::from_writer(output);
        wtr.serialize(client).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output),
           "client,available,held,total,locked\n1,1.0000,2.0002,3.0002,false\n");
    }
}
