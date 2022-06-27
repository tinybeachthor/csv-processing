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

        (client.total().integer == model)
            && (client.available().integer == model)
            && (client.held().integer == 0)
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

        (client.total().integer == model)
            && (client.available().integer == model)
            && (client.held().integer == 0)
    }

    fn deposits_disputes(rounds: u8) -> bool {
        let mut tx = 0;
        let client_id = 0;

        let mut client = Client::new(client_id);
        let mut model_available = 0;
        let mut model_held = 0;
        let mut model_total = 0;

        for i in 0..rounds {
            let deposit = new_deposit(client_id, tx);
            let dispute = Transaction {
                r#type: TransactionType::Dispute,
                client: client_id, tx, amount: None,
            };
            tx += 1;

            let amount = deposit.amount.unwrap().integer;
            model_available += amount;
            model_total += amount;
            client.apply(deposit).unwrap();

            if i % 2 == 0 {
                client.apply(dispute).unwrap();
                model_available -= amount;
                model_held += amount;
            }
        }

        (client.available().integer == model_available)
            && (client.held().integer == model_held)
            && (client.total().integer == model_total)
    }
    fn deposits_disputes_resolves(rounds: u8) -> bool {
        let mut tx = 0;
        let client_id = 0;

        let mut client = Client::new(client_id);
        let mut model_available = 0;
        let mut model_held = 0;
        let mut model_total = 0;

        for i in 0..rounds {
            let deposit = new_deposit(client_id, tx);
            let dispute = Transaction {
                r#type: TransactionType::Dispute,
                client: client_id, tx, amount: None,
            };
            let resolve = Transaction {
                r#type: TransactionType::Resolve,
                client: client_id, tx, amount: None,
            };
            tx += 1;

            let amount = deposit.amount.unwrap().integer;
            model_available += amount;
            model_total += amount;
            client.apply(deposit).unwrap();

            if i % 2 == 0 {
                client.apply(dispute).unwrap();
                model_available -= amount;
                model_held += amount;

                if i % 3 == 0 {
                    client.apply(resolve).unwrap();
                    model_available += amount;
                    model_held -= amount;
                }
            }
        }

        (client.available().integer == model_available)
            && (client.held().integer == model_held)
            && (client.total().integer == model_total)
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

        (client.total().integer == 0)
            && (client.available().integer == 0)
            && (client.held().integer == 0)
    }
}
