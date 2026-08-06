# Contract 015 — Canonical Request Handoff Evidence

Date: 2026-08-04

## Scope and exact Contract 014 seam

Contract 015 is implemented as the terminal Structured Request Engine boundary. It consumes exactly one verified Contract 014 `CanonicalStructuredRequest` plus `IssuanceManifest` pair, one active versioned `DownstreamBoundaryDeclaration`, and one exact `HandoffContext`. It rejects mismatched, substituted, incomplete, or integrity-invalid issuance publications before creating the handoff package or using transfer observations.

The issued request is never copied, regenerated, normalized, summarized, or mutated. The fixture package is reference-only and binds the exact issuance publication integrity binding, provenance continuity references, recipient declaration, purpose, context, profile, rules, registries, transport binding, and configuration.

## Handoff stages and authority

The runtime keeps the stages distinct:

- `HandoffPackageManifest` is the sole package presented to transport.
- `TransferAttempt` and `OperationalHandoffFact` are immutable operational records.
- `ReceiptAcknowledgment` is accepted only when attributable to the declared recipient, bound to the exact handoff/package/attempt, and integrity-matched.
- `HandoffRuleApplication` is immutable, replay-bound, and non-authoritative.
- `TransferDecision`, `CustodyDecision`, and `ResponsibilityDecision` are independently identified and separately justified.
- `CanonicalRequestHandoffRecord` and `CanonicalRequestHandoffFailureRecord` are mutually exclusive terminal alternatives.

Transport success is not acknowledgment. Delivery is not custody. Acknowledgment is not responsibility transfer. No endpoint, queue, process, or adapter identifier substitutes for constitutional boundary identity.

## Fixture authorities and rules

The fixture binds `fixture-boundary-declaration-v1`, `fixture-handoff-context-v1`, `fixture-handoff-v1`, separate fixture transfer/custody/responsibility/acknowledgment/retry/duplicate/expiration rule sets, `fixture-handoff-schema-v1`, `fixture-handoff-registry-v1`, `fixture-handoff-config-v1`, `fixture-transport-binding-v1`, and implementation `sre-runtime-fixture-v1`.

The closed fixture supports acknowledged, rejected, delivered-unacknowledged, recipient-unavailable after the declared attempt threshold, duplicate-recognized with prior constitutional correlation, and context-only expiration outcomes. Custody and responsibility remain independent; the fixture allows receipt to transfer custody while responsibility remains with SRE.

Retry continuity requires the same handoff, context, exact package, and a new sequential attempt. Duplicate recognition requires a prior constitutional reference and creates no second transition. Expiration affects only the handoff context and never expires or rewrites the issued request.

## Focused tests

The 35 focused tests in `contract_015_tests` cover exact issuance and boundary binding, context-before-package resolution, reference-only packaging, operational-versus-constitutional separation, acknowledgment attribution and wrong-attempt rejection, non-authoritative rule applications, independent decisions, completed negative outcomes, recipient-unavailable qualification, duplicate correlation, expiration, retry continuity, terminal alternatives, replay, and the absence of downstream authority.

Verification passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` — 150 unit tests, 0 failures
- `cargo test --doc` — 0 doctests, 0 failures

## Limitations and hard stop

Production downstream boundary and handoff profile/rule/registry ownership, durable persistence and crash recovery, and production conformance remain unresolved. Contract 015 does not implement downstream intake, governance acceptance, authorization, routing, planning, execution, generation, release, request mutation, or lifecycle management. No Contract 016 authority was introduced. No commit or push was performed.
