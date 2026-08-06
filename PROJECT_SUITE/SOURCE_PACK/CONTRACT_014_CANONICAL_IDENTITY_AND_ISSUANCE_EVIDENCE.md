# Contract 014 — Canonical Identity and Issuance Evidence

Date: 2026-08-04

## Scope

Contract 014 is implemented as a fixture-bounded, in-memory initial-issuance slice. It consumes exactly one `ConstructedCanonicalRequest` plus `ConstructionManifest` pair from Contract 013 and an explicit `IssuanceContext`, `IdentityPolicy`, fixture issuance profile, schema, rule set, registries, configuration, implementation version, and replay context.

The operation does only the following:

- verifies the exact Contract 013 request/manifest publication binding;
- resolves the declared issuance authorities and versions;
- derives `CanonicalStructuredRequestId` from the Identity Policy-selected, explicitly recorded inputs;
- records the derivation and assigns initial `Issued` standing; and
- publishes `CanonicalStructuredRequest` and `IssuanceManifest` atomically, or one `IssuanceFailureRecord`.

The constructed request is carried forward unchanged. Identity and construction integrity remain distinct. No normalization, reordering, validation rerun, reconstruction, authorization, lifecycle mutation, handoff, custody, execution, generation, or release authority is introduced.

## Fixture authority bindings

The focused fixture binds `fixture-issuance-context-v1`, `fixture-identity-policy-v1`, `contract-014-fixture` / `fixture-issuance-v1`, `fixture-issuance-schema-v1`, `fixture-issuance-rules-v1`, `fixture-issuance-registry-v1`, `fixture-issuance-config-v1`, `fixture-standing-rules-v1`, and implementation `sre-runtime-fixture-v1`. The initial standing rule is `AssignInitialIssuedStanding`.

## Focused tests

The 15 focused tests in `contract_014_tests` cover exact pair issuance, missing or mismatched publication failure, same-context replay and identity stability, context distinction, policy-controlled identity inputs, derivation-record presence, construction immutability, explicit initial standing, issued publication binding, no partial publication, identity/integrity separation, and absence of Contract 015 authority.

Verification passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` — 115 unit tests, 0 failures
- `cargo test --doc` — 0 doctests, 0 failures

## Limitations and stop boundary

Production identity policy/profile and registry ownership, broader mapping coverage, durable persistence and crash recovery, and production conformance remain unresolved. Contract 014 assigns only initial `Issued` standing; later lifecycle authority is not implemented. Contract 015 consumes the exact issuance pair as the terminal handoff boundary; no downstream authority is introduced. No commit or push was performed.
