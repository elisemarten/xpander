# Wallet Portfolio Snapshot

Wallet Portfolio Snapshot is a Rust command-line tool that stores daily portfolio snapshots.

Instead of querying historical balances every time, the application saves lightweight JSON snapshots that can later be visualized or exported.

## Features

- Save daily portfolio snapshots
- Multiple assets
- JSON storage
- CSV export
- Portfolio value calculation

## Example

```bash
cargo run -- snapshot
```

```
Snapshot saved successfully.
```

```bash
cargo run -- export
```

Produces

```
portfolio.csv
```

## Future Ideas

- CoinGecko API
- Wallet address lookup
- Charts
- SQLite support
