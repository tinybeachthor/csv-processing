# csv-processing

rustc 1.61.0 (fe5b13d68 2022-05-18)
cargo 1.61.0 (a028ae4 2022-04-29)

## assumptions

- all amounts and balances are positive
- all amounts and balances fit in u64
- disputes of deposits and withdrawals are handled the same way
- client account is locked after a chargeback and no further
  transactions are applied to it

## evaluation

### completeness

- read transactions from an input csv file
- apply deposits/withdraws/disputes/resolves/chargebacks to clients
- output the list of all clients final state to stdout

### correctness

- unit tests for serialization/deserialization
- unit tests for balance arithmetic
- isolation of concern between modules (most of the logic is in
  Client#apply)

### robustness

- no negative amounts/balances, u64
- arithmetic overflow results in a panic
- duplicate disputes or resolves are silently ignored
- quickcheck model tests for client properties

### efficiency

- streaming processing
- transaction amounts are stored by clients, for a long running process
  generational garbage collections would be cool
- no concurrency implemented - a good fitting model would be a threadpool
  with tasks split by client id (so chronological order of client
  transactions is preserved without extra blocking)

### maintainability

- yes

