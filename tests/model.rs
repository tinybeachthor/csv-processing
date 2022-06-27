//! Model testing for [Client] [Transaction] processing.
//!
//! - single client under test
//! - transactions have integral only values (decimal = 0)

use quickcheck::quickcheck;

use csv_processing::{FourDecimals, Client, Transaction, TransactionType};

fn new_deposit(id: u16, tx: u32) -> Transaction {
    Transaction {
        r#type: TransactionType::Deposit,
        client: id,
        tx,
        amount: Some(FourDecimals {
            integer: rand::random::<u8>() as u64,
            decimal: 0,
        }),
    }
}
fn new_withdrawal(id: u16, tx: u32, max: u64) -> Transaction {
    Transaction {
        r#type: TransactionType::Withdrawal,
        client: id,
        tx,
        amount: Some(FourDecimals {
            integer: (rand::random::<u8>() as u64) % (max + 1),
            decimal: 0,
        }),
    }
}

quickcheck! {
    fn deposits_withdrawals(rounds: u8) -> bool {
        let mut tx = 0;
        let client_id = 0;

        let mut client = Client::new(client_id);
        let mut model = 0;

        for _ in 0..rounds {
            let deposit = new_deposit(client_id, tx);
            tx += 1;
            model += deposit.amount.unwrap().integer;
            client.apply(deposit).unwrap();

            let withdrawal = new_withdrawal(client_id, tx, model);
            tx += 1;
            model -= withdrawal.amount.unwrap().integer;
            client.apply(withdrawal).unwrap();
        }

        client.total().integer == model
    }

    fn deposits_undisputed(rounds: u8) -> bool {
        let mut tx = 0;
        let client_id = 0;

        let mut client = Client::new(client_id);
        let mut model = 0;

        for i in 0..rounds {
            let deposit = new_deposit(client_id, tx);
            let resolve = Transaction {
                r#type: TransactionType::Resolve,
                client: client_id,
                tx,
                amount: None,
            };
            let chargeback = Transaction {
                r#type: TransactionType::Chargeback,
                client: client_id,
                tx,
                amount: None,
            };
            tx += 1;

            model += deposit.amount.unwrap().integer;
            client.apply(deposit).unwrap();
            if i % 2 == 0 {
                client.apply(resolve).unwrap();
                client.apply(chargeback).unwrap();
            }
            else {
                client.apply(chargeback).unwrap();
                client.apply(resolve).unwrap();
            }
        }

        client.total().integer == model
    }

    fn chargeback(rounds: u8) -> bool {
        let mut tx = 0;
        let client_id = 0;

        let mut client = Client::new(client_id);
        let mut model = 0;

        let deposit = new_deposit(client_id, tx);
        let dispute = Transaction {
            r#type: TransactionType::Dispute,
            client: client_id, tx, amount: None,
        };
        let chargeback = Transaction {
            r#type: TransactionType::Chargeback,
            client: client_id, tx, amount: None,
        };
        tx += 1;

        client.apply(deposit).unwrap();
        client.apply(dispute).unwrap();
        client.apply(chargeback).unwrap();

        for _ in 0..rounds {
            let deposit = new_deposit(client_id, tx);
            tx += 1;
            model += deposit.amount.unwrap().integer;
            client.apply(deposit).unwrap();

            let withdrawal = new_withdrawal(client_id, tx, model);
            tx += 1;
            model -= withdrawal.amount.unwrap().integer;
            client.apply(withdrawal).unwrap();
        }

        client.total().integer == 0
    }
}
