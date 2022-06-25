//! Type representation of a client.

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
}
impl Client {
    /// Create a new [Client].
    pub fn new(id: u16) -> Self {
        Self {
            id,
            available: FourDecimals::default(),
            held: FourDecimals::default(),
            locked: false,
        }
    }
    /// Get the [Client] total.
    pub fn total(&self) -> FourDecimals {
        self.available + self.held
    }

    /// Apply a [Transaction].
    pub fn apply(&mut self, transaction: Transaction) -> Result<(), MyError> {
        let amount = transaction.amount.unwrap_or_default();

        match transaction.r#type {
            TransactionType::Deposit => {
                self.available = self.available + amount;
            },
            TransactionType::Withdrawal => {
                if amount > self.available {
                    return Err(
                        MyError::BalanceLowForWithdrawal(self.id, transaction.tx))
                }
                self.available = self.available - amount;
            },
            TransactionType::Dispute => unimplemented!(),
            TransactionType::Resolve => unimplemented!(),
            TransactionType::Chargeback => unimplemented!(),
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

        let client = Client { id: 1, available, held, locked: false };

        let output = Vec::new();
        let mut wtr = Writer::from_writer(output);
        wtr.serialize(client).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output),
           "client,available,held,total,locked\n1,1.0000,2.0002,3.0002,false\n");
    }
}
