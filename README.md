# Structured Request Engine

<p align="center">
  <img
    src="docs/assets/SRE-image.png"
    alt="Structured Request Engine transforming unstructured sources through fifteen constitutional processing stages into a canonical structured request, then handing it to a separate downstream authority boundary."
    width="100%"
  />
</p>

## Constitutional transformation. Deterministic order. Trust by design.

The Structured Request Engine is Mineard AI's portable request-structuring boundary. It converts supplied source material and explicitly declared interpretation material into an immutable, provenance-preserving, machine-readable representation of what was requested.

SRE creates constitutional clarity before operational authority. It represents a request; it does not decide whether that request should be authorized, planned, executed, generated, or released.

> **Mineard AI - Architecting Trust into Intelligence**

Repository: <https://github.com/MineardAI/Structured-Request-Engine>

## Why SRE exists

AI systems often collapse several distinct acts into one opaque operation:

- interpreting a request;
- deciding what it means;
- selecting what proceeds;
- determining whether it is permitted;
- planning execution;
- invoking models or tools; and
- producing or releasing output.

SRE separates request representation from every later authority. It creates a deterministic, inspectable request artifact before governance, authorization, planning, generation, tool use, or execution begins.

## Constitutional boundary

```text
Source Input
     |
     v
Structured Request Engine
     |
     v
CanonicalStructuredRequest
     |
     v
Declared Downstream Authority
     |
     v
Governance / Planning / Generation / Tools / Execution
```

SRE ends at governed handoff. It does not:

- authorize requests;
- select providers, models, tools, or connectors;
- establish credential or resource availability;
- plan work;
- execute tasks;
- generate responses; or
- release outputs.

Representation is not authorization. Construction is not issuance. Issuance is not handoff. Handoff is not execution.

## Request lifecycle

| Contract | Constitutional act | Principal result |
|---|---|---|
| 000 | Establish | Constitutional foundation and authority boundary |
| 001 | Admit | `SourceIntakeRecord` |
| 002 | Bound | `AdmittedInterpretationProposalSet` |
| 003 | Represent | `DeclaredObjectiveSet` |
| 004 | Represent | `DeclaredConstraintSet` |
| 005 | Represent | `CapabilityRequirementSet` |
| 006 | Qualify | `MeaningQualificationSet` |
| 007 | Ground | `InterpretationEvidenceSet` |
| 008 | Trace | `ProvenanceRecordSet` |
| 009 | Reconcile | `SemanticReconciliationSet` |
| 010 | Normalize | `NormalizedRequestRepresentation` |
| 011 | Order | `CanonicallyOrderedRequestRepresentation` |
| 012 | Validate | `StructuralValidationResult` |
| 013 | Construct | `ConstructedCanonicalRequest` + `ConstructionManifest` |
| 014 | Issue | `CanonicalStructuredRequest` + `IssuanceManifest` |
| 015 | Transfer | Handoff record or failure record |

The stages are separately bounded. In particular, reconciliation, normalization, ordering, validation, construction, issuance, and handoff are not interchangeable operations.

## Core guarantees

### Deterministic processing

Equivalent declared inputs, profiles, registries, schemas, rules, and version bindings produce equivalent governed outcomes.

### Immutable publications

Successful and failed operations publish immutable artifacts. Corrections require new publications rather than silent mutation.

### Explicit authority

Each transformation has one bounded authority. A later stage cannot silently assume the authority of an earlier or downstream stage.

### Provenance and evidence continuity

Evidence, grounding relationships, lineage, and provenance remain explicit and independently inspectable.

### Atomic outcomes

A completed operation produces one committed success result or one committed failure result, never an ambiguous partial constitutional result.

### Representation is not authorization

Increasing structural precision does not create operational authority.

## What SRE is - and is not

| SRE is | SRE is not |
|---|---|
| A deterministic request-representation engine | An autonomous agent |
| A constitutional boundary | An authorization authority |
| A provenance-preserving processor | A model runtime |
| A canonical request constructor | A planner or task executor |
| A handoff producer | A downstream governance system |
| Transport-neutral at the constitutional layer | A provider request format |
| Inspectable and replay-oriented | A hidden prompt pipeline |

## Implementation status

The complete Contract 000-015 reference lifecycle is implemented and verified as a fixture-bounded, in-memory Rust implementation.

Current verification baseline:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` - **150 passing tests**
- `cargo test --doc` - **0 failures**

The current reference slice is:

- fixture-profile driven;
- fixture-registry driven;
- in-memory;
- transport-neutral at the constitutional layer; and
- verified for deterministic replay and atomic API outcomes.

This does not claim constitutional adoption, production conformance, or operational readiness. Production profile ownership, production registries, broader mappings, durable persistence, crash recovery, and production conformance remain separate integration responsibilities.

## Quick start

### Requirements

- A current stable Rust toolchain
- Cargo

### Verify the repository

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --doc
cargo doc --workspace --no-deps
```

The public Rust API is exposed from `src/lib.rs`, beginning with source admission through `admit` and continuing through the bounded contract operations. The contract artifacts and evidence records are the authoritative guides for semantics and boundaries.

## Repository map

```text
.
├── src/
│   └── lib.rs                 # Reference implementation and tests
├── docs/
│   ├── assets/                # Public diagrams and project artwork
│   ├── Contracts/             # Contracts 000-015
│   ├── Evidence/              # Implementation evidence by contract
│   ├── Implementation/        # Matrix, readiness, dependencies, and traceability
│   └── archive/               # Superseded planning material retained for provenance
├── .github/workflows/         # Rust CI
├── Cargo.toml                 # Package metadata
└── README.md
```

Internal workspace instructions, prompts, status history, and derivative onboarding material are intentionally kept outside the public repository.

## Documentation

### Start here

- [Contract 000](docs/Contracts/SRE-CONTRACT%20000.txt) - constitutional identity, authority, and boundaries
- [Contract Plan](docs/Contracts/SRE-CONTRACT-PLAN_v2.1.0_Adopted_Planning_Baseline.md) - adopted planning baseline
- [Implementation matrix](docs/Implementation/CONTRACT_IMPLEMENTATION_MATRIX.md)
- [Implementation status and dependency model](docs/Implementation/IMPLEMENTATION_STATUS_AND_DEPENDENCY_MODEL.md)

### Architecture

- [ARCH-001](docs/Contracts/SRE-ARCH-001_Canonical_Representation_Pattern_v0.1.0_Draft.md) - canonical representation pattern
- [ARCH-002](docs/Contracts/SRE-ARCH-002-Constitutional-Grounding-Architecture.txt) - constitutional grounding architecture
- [ARCH-003](docs/Contracts/SRE-ARCH-003_Constitutional_Lifecycle_Pattern_v0.1.0_Provisional_Draft.md) - provisional, non-binding lifecycle pattern

### Evidence

Contract-specific evidence records are in [`docs/Evidence/`](docs/Evidence/). They document the verified scope, replay behavior, atomic outcomes, and declared limitations of the reference implementation.

## Design principles

1. Representation before authority.
2. Interpretation remains a proposal.
3. Meaning may be refined without being silently replaced.
4. Evidence, grounding, and provenance remain distinct.
5. Every material input is declared and version-bound.
6. Valid negative states remain distinct from operation failure.
7. Construction is not issuance.
8. Issuance is not handoff.
9. Handoff is not execution.
10. Greater precision does not imply greater authority.

## Production considerations

The reference implementation does not itself establish:

- production profile authorities;
- production registry ownership;
- durable storage or crash recovery;
- production identity federation;
- external historical-truth verification;
- provider or tool integrations;
- governance or authorization policy; or
- execution infrastructure.

Those responsibilities belong to declared production integrations or downstream constitutional systems.

## License and contribution

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) and [LICENSE-APACHE](LICENSE-APACHE).

See [CONTRIBUTING.md](CONTRIBUTING.md) for development expectations and [SECURITY.md](SECURITY.md) for the current security-reporting limitation.
