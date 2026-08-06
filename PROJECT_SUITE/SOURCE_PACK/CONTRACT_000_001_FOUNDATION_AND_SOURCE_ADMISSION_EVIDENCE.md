# Contract 000/001 Foundation and Source Admission Evidence

**Scope:** bounded Contract 000 foundation and complete default-policy Contract 001 source-admission slice.

**Implementation surface:** `Cargo.toml`, `src/lib.rs`.

**Status:** `Verified` for the implemented bounded slice. Contract status remains unchanged; the Draft Contract 001 source is not canonized by implementation activity.

## Implemented behavior

- Contract 000 representation/authority boundaries are represented through a source-only public API, opaque submission/source/operation identities, immutable committed values, and an explicit `IntakeOutcome::Success`/`IntakeOutcome::Failure` terminal algebra.
- Contract 001 enumerates every supplied component, preserves source category, origin, payload/reference, mechanically observable metadata, preservation facts, observed admission state, source identity, and policy mapping.
- The default Composite Admission Policy binds identity `policy-<stable value>` with version `contract-001-default-v1` and maps `Accepted` to success, qualifying `Redacted` to success, failure-requiring states to failure, and unresolved `Pending`/`Received` to failure at commitment.
- Optional profiles are validation-only at this stage. They must be fully versioned, authoritative, scoped, exhaustive, and compatible with Contract 001; no production optional profile is fabricated or adopted.
- A failure outcome preserves accepted components and every other enumerated component/state. No semantic interpretation, canonicalization, authorization, provider selection, planning, execution, generation, or release API is exposed.

## Verification evidence

Run from the repository root:

```text
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --doc
```

Results:

- formatting check passed;
- workspace check passed;
- Clippy with warnings denied passed;
- 9 focused unit tests passed;
- 0 documentation tests passed (no doctests were present).

The focused tests cover deterministic replay, receive-order equivalence, references and application-supplied distinction, redaction preservation, mixed-component failure preservation, all default-policy failure/unresolved states, state/outcome separation, and unauthorized/incomplete optional-profile rejection.

## Declared limitations

- The implementation uses an in-memory immutable value boundary; durable storage and crash recovery are not present because the repository had no existing persistence architecture.
- Stable IDs are opaque deterministic implementation identities. They are not constitutional canonical-request identities and are not presented as cryptographic digests.
- No optional Contract 001 profile is adopted. Profile validation rejects incomplete or incompatible profiles and does not fall back silently to the default policy.
- The public API exposes no Contract 002 proposal-admission implementation; successful intake is only a source-preservation result eligible for later Contract 002 evaluation.
