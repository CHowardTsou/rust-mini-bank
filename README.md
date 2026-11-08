# Mini Bank — Rust Token Ledger

A simple in-memory token ledger built in Rust to practice ownership, borrowing, error handling, and security-oriented testing.

This is an educational project, not a production banking application or deployed smart contract.

## Features

- Create accounts with unique IDs
- Deposit and withdraw tokens
- Transfer tokens between accounts
- Query account balances
- Track total token supply
- Handle invalid accounts and arithmetic errors using `Result`

## Security Properties

**1. Total Supply Consistency**

The total supply must equal the sum of all account balances.

**2. Failed Operation Atomicity**

Failed deposits, withdrawals, and transfers must leave account balances and total supply unchanged.

**3. Transfer Conservation**

Successful transfers between accounts must not change total token supply.

**4. Arithmetic Safety**

Use checked arithmetic to prevent integer overflow and underflow.

**5. Account Identity**

Account IDs must remain unique, and operations must validate account existence.

## Testing

The project includes:

- Unit tests for individual operations
- Regression tests for error handling and boundary conditions
- Invariant checks after state transitions
- Property-based testing using `proptest`
- Generated sequences combining deposits, withdrawals, and transfers

Run tests:

```bash
cargo test
```

Run additional property test cases:

```bash
PROPTEST_CASES=1000 cargo test
```

Check formatting and static analysis:

```bash
cargo fmt --check
cargo clippy --all-targets
```

## What I Learned

- Rust ownership and exclusive mutable borrowing
- Using `split_at_mut()` to safely update two accounts
- Using `Result`, `Option`, and `?` for error handling
- Preventing partial state updates by validating before mutation
- Designing accounting invariants
- Writing property-based tests with independent expected outcomes

## Limitations

- In-memory state only; no persistence
- No authentication or authorization model
- No actual token transfers or external assets
- Not audited or production-ready

## Purpose

This project is part of my progression from Rust fundamentals toward reviewing and auditing Rust-based smart-contract protocols.