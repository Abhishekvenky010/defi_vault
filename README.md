# DeFi Vault

A simple Rust implementation of a DeFi vault with deposit and redeem functionality. The vault uses a share-based accounting system where users deposit assets to receive shares, and can redeem those shares to withdraw assets.

## Features

- **Deposit**: Users deposit assets and receive vault shares proportional to their contribution
- **Redeem**: Users redeem shares to withdraw their proportional share of assets
- **Safe Math**: Uses `u128` for intermediate calculations to prevent overflow
- **Error Handling**: Comprehensive error handling for edge cases (division by zero, overflow, insufficient shares)

## Architecture

The project is organized into three modules:

- `main.rs` - Entry point and usage example
- `vault.rs` - Core vault logic with deposit/redeem operations and share calculation
- `error.rs` - Error types for accounting operations

## How It Works

### Share Calculation

When the vault is empty or has no shares, deposits are 1:1 (1 asset = 1 share). Otherwise, shares are calculated as:

```
shares = (deposit_amount * total_shares) / total_assets
```

This ensures users receive shares proportional to the current exchange rate.

### Redemption

When redeeming, assets received are calculated as:

```
assets = (shares * total_assets) / total_shares
```

## Usage

```rust
let mut vault = vault::Vault::new();
vault.deposit("alice".to_string(), 100).unwrap();
let redeemed = vault.redeem("alice".to_string(), 50).unwrap();
```

## Running

```bash
cargo run
```

## Error Types

| Error | Description |
|-------|-------------|
| `Overflow` | Value exceeds `u64` range during calculation |
| `DivisionByZero` | Attempted division by zero |
| `InsufficientShares` | User tried to redeem more shares than they own |

