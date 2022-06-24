//! Type representation of a client.

use serde::Serialize;

use crate::four_decimals::FourDecimals;

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
    /// Get the client total.
    pub fn total(&self) -> FourDecimals {
        self.available.clone() + self.held.clone()
    }
}
impl Into<ClientRaw> for Client {
    fn into(self) -> ClientRaw {
        ClientRaw {
            client: self.id,
            available: self.available.clone(),
            held: self.held.clone(),
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
