# paytments engine

A simple toy payments engine that reads a series of transactions from a CSV, updates client accounts, handles disputes and chargebacks, and then outputs the state of clients accounts as a CSV.

## Usage

```bash
$ cargo run -- transactions.csv > accounts.csv
```