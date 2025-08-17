# Solana Helpers

A collection of utility functions and helpers for Solana program development. This crate provides common functionality for account management, transfers, conversions, and validation operations.

## Features

- **Account Management**: Utilities for closing accounts and transferring lamports
- **Token Operations**: SPL token transfers and burns with PDA support
- **Data Conversions**: Convert between different data formats (BigInt, byte arrays, endianness)
- **Validation**: ATA validation and other common checks
- **Generic Utilities**: PDA creation, discriminator derivation, and logging helpers

## Modules

- `closers`: Utilities for closing accounts and transferring their lamports
- `conversions`: Utilities for converting between different data formats
- `generic`: Generic utility functions for Solana programs
- `transfers`: Utilities for transferring SOL and SPL tokens
- `validators`: Validation utilities for Solana programs

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
solana-helpers = "0.1.0"
```

## License

MIT