//! Type representation of a transaction.

use serde::{Deserialize, Serialize};

use crate::four_decimals::FourDecimals;

/// Type representation of a transaction type.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    /// Deposit.
    Deposit,
    /// Withdrawal.
    Withdrawal,
    /// Dispute.
    Dispute,
    /// Resolve.
    Resolve,
    /// Chargeback.
    Chargeback,
}

/// Type representation of a transaction.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Transaction {
    /// [TransactionType].
    pub r#type: TransactionType,
    /// Client id.
    pub client: u16,
    /// Transaction number.
    pub tx: u32,
    /// Amount or None.
    pub amount: Option<FourDecimals>
}

#[cfg(test)]
pub mod test {
    use super::*;

    use std::io::Cursor;
    use csv::{ReaderBuilder, Writer, Trim};

    #[test]
    pub fn deserialize_single() {
        let input = "deposit, 2, 2, 2.0";

        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .trim(Trim::All)
            .from_reader(Cursor::new(input));
        let result = rdr.deserialize().next().unwrap();
        let output: Transaction = result.unwrap();
        let amount = FourDecimals { integer: 2, decimal: 0 };

        assert_eq!(output, Transaction {
            r#type: TransactionType::Deposit, client: 2, tx: 2, amount: Some(amount) })
    }

    #[test]
    pub fn deserialize_multiple() {
        let input = "type, client, tx, amount\ndeposit, 2, 2, 2.0\ndispute, 2, 2\n";

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .flexible(true)
            .trim(Trim::All)
            .from_reader(Cursor::new(input));

        let result = rdr.deserialize().next().unwrap();
        let output: Transaction = result.unwrap();
        let amount = FourDecimals { integer: 2, decimal: 0 };
        assert_eq!(output, Transaction {
            r#type: TransactionType::Deposit, client: 2, tx: 2, amount: Some(amount) });

        let result = rdr.deserialize().next().unwrap();
        let output: Transaction = result.unwrap();
        assert_eq!(output, Transaction {
            r#type: TransactionType::Dispute, client: 2, tx: 2, amount: None });
    }

    #[test]
    fn serialize_single() {
        let amount = FourDecimals { integer: 1, decimal: 0 };
        let transaction = Transaction {
            r#type: TransactionType::Withdrawal,
            client: 2, tx: 10, amount: Some(amount),
        };

        let output = Vec::new();
        let mut wtr = Writer::from_writer(output);
        wtr.serialize(transaction).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output),
          "type,client,tx,amount\nwithdrawal,2,10,1.0000\n");
    }
    #[test]
    fn serialize_multiple() {
        let amount = FourDecimals { integer: 1, decimal: 0 };
        let transaction1 = Transaction {
            r#type: TransactionType::Withdrawal,
            client: 2, tx: 10, amount: Some(amount),
        };
        let transaction2 = Transaction {
            r#type: TransactionType::Dispute,
            client: 2, tx: 10, amount: None,
        };

        let output = Vec::new();
        let mut wtr = Writer::from_writer(output);
        wtr.serialize(transaction1).unwrap();
        wtr.serialize(transaction2).unwrap();
        wtr.flush().unwrap();
        let output = wtr.into_inner().unwrap();

        assert_eq!(String::from_utf8_lossy(&output),
          "type,client,tx,amount\nwithdrawal,2,10,1.0000\ndispute,2,10,\n");
    }
}
