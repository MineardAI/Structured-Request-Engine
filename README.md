# Structured Request Engine

Structured Request Engine (SRE) is a Rust reference implementation for deterministic, provenance-preserving request representation. Its bounded implementation covers Contracts 000–015 as explicit in-memory publication boundaries.

Repository: <https://github.com/MineardAI/Structured-Request-Engine>

The crate represents and validates request artifacts. It does not authorize, execute, route, generate, release, or persist production work. Representation is not authorization; construction is not issuance; issuance is not downstream handoff; handoff is not execution.

## Current status

Contracts 000–015 have fixture-bounded implementations with immutable values, explicit success/failure alternatives, deterministic replay inputs, and contract-specific evidence. The current verification baseline is 150 unit tests and zero doctests, with the standard formatting, check, Clippy, and test gates passing.

This is not a claim of constitutional adoption, production conformance, or operational readiness. Production profile and registry ownership, broader mappings, durable persistence and recovery, and production conformance remain unresolved limitations. Formal contract and architecture standings remain authoritative and are not changed by implementation evidence.

## Repository layout

- `src/lib.rs` — bounded runtime surfaces and tests.
- `docs/Contracts/` — contract specifications.
- `docs/Implementation/` — implementation mappings and dependency records.
- `docs/Evidence/` — verification evidence.
- `PROJECT_SUITE/` — project control material and the bounded Source Pack.

## Verification

Install a current stable Rust toolchain, then run:

```text
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --doc
```

## Contributions and security

See `CONTRIBUTING.md` for development expectations. Do not disclose security-sensitive information in public issues. A private security reporting channel must be configured when the eventual GitHub repository is created; `SECURITY.md` records this limitation.

## License

Licensed under the Apache License, Version 2.0. See `LICENSE` and `LICENSE-APACHE`.
