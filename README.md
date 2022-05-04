# payments engine

A simple toy payments engine that reads a series of transactions from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs the state of clients accounts to `stdout` in CSV format.

## Usage

```bash
$ cargo run -- transactions.csv > accounts.csv
```

## Testing
Unit tests in `src/client.rs` for transaction methods (withdrawal, deposit, dispute, resolve, and chargeback).

Manual testing was conducted using CSV files located in `tests/CSVs` for a variety of scenarios.
