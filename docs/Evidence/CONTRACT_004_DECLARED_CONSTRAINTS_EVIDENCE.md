# Contract 004 Declared Constraints Evidence

## Disposition

**CONTRACT 004: VERIFIED WITH DECLARED PROFILE AND REGISTRY DEPENDENCIES**

This record documents the bounded in-memory implementation of the draft Contract 004 source. Contract status remains Draft — Constitutional Development; this evidence does not promote the source or adopt production authorities.

## Scope and normative sources

The implementation is governed by `docs/Contracts/SRE-CONTRACT-000 v0.1.0`, the repaired/implemented Contract 002 and Contract 003 runtime boundaries, `docs/Contracts/SRE-CONTRACT-004_Declared_Constraints_v0.1.0_Draft.md`, and the bounded representation pattern already tracked by the implementation-control documents. The implementation answers only what typed constraint-domain content is represented in an admitted submission context. It does not create policy, enforcement, feasibility, precedence, authorization, redaction, capability requirements, reconciliation, planning, execution, generation, or release authority.

## Contract 002 prerequisite

The existing proposal model had no typed constraint-bearing field. The narrow prerequisite amendment adds `ProposalContent.constraint_elements` and the typed `ProposalConstraintElement` / `ProposalConstraintRelationship` structures. This is a versioned proposal-schema amendment in the fixture slice: proposal requests now bind `proposal-fixture-v2`. The proposal identity constructor already fingerprints complete `ProposalContent`, so materially different typed constraint content necessarily receives a different `InterpretationProposalId`; no committed publication is mutated. No Contract 005–008 typed collections were added.

Contract 002 still performs only request issuance, structural proposal admission, and admitted-set publication. Contract 003 still consumes only `objective_elements`; its tests remain unchanged and pass.

## Implemented authority and inputs

`represent_constraints` consumes exactly one committed `AdmittedInterpretationProposalSet`, exactly one committed `DeclaredObjectiveSet`, one explicit fixture profile, eight explicit fixture registry snapshots, schema/configuration bindings, and an implementation-version binding. The successful artifact is `DeclaredConstraintSet`; the failure artifact is `ConstraintRepresentationFailureRecord`.

The fixture profile is `contract-004-fixture` / `fixture-004-v1` with authority `TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT`. The eight fixture registries are class, form, origin, basis, relationship, status, scope-status, and conflict registries, all versioned `fixture-004-registry-v1` with the same fixture-only authority. There is no ambient, global, live, hidden, or production fallback.

## Lineage and representation behavior

`validate_constraint_lineage` runs before candidate construction and returns either a `ConstraintLineageProof` or a typed validation error. It checks the exact admitted proposal set, objective set, source intake, interpretation operation, proposal IDs, and admission-decision bindings. It never searches, substitutes, repairs, reconstructs, or mutates upstream artifacts.

Only explicitly typed `constraint_elements` are consumed. Generic proposal narrative, `proposed_elements`, objective expressions, capability-like material, evidence text, source text, metadata, and application state are not classified as constraints.

Constraint expression, class, form, origin, basis, scope, scope-resolution status, objective references, declared priority, evidence references/status, reference state, relationships, conflict state, source proposal/element bindings, profile, registry, schema, configuration, and distinct logical/representation identities are preserved. Declared priority does not become precedence. Explicit conflicts and `OverridesAsDeclared` relationships do not select winners. Confidentiality-class constraints do not redact, hide, suppress, or enforce access control.

Permitted unresolved references and unresolved scope remain represented as unresolved states. Malformed, prohibited, foreign-lineage, or structurally invalid references fail the whole operation. A profile cannot downgrade a fatal reference condition to unresolved. Mixed valid/invalid input produces only the failure artifact; no partial set survives.

Profile-authorized composite decomposition produces deterministic parent/child representations and `ComponentOf` relationships. Unauthorized decomposition fails. Decomposition does not imply preferred granularity, precedence, or execution order.

## Empty and terminal behavior

When no typed constraint elements are present and the fixture profile permits emptiness, a committed empty `DeclaredConstraintSet` is produced with full upstream, objective, profile, registry, schema, configuration, and replay bindings. The terminal algebra is exactly one set or one failure record; in-memory construction demonstrates API-level atomicity. Durable persistence and crash recovery are not implemented.

## Verification

The final workspace test run completed with 34 passing unit tests, zero failures, and zero doctests. Contract 004 focused coverage includes typed-surface admission, narrative non-inference, proposal identity distinction, empty sets, exact lineage, foreign lineage, missing decision bindings, unresolved/malformed/prohibited reference treatment, non-success statuses, priority/conflict/confidentiality preservation, composite decomposition, unauthorized decomposition, profile/registry failures, no partial set, replay equivalence, and lineage proof stability. All prior Contract 000–003 tests also pass.

Verification commands passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test --doc`

## Limitations and downstream boundary

The implementation is fixture-profile and fixture-registry bounded. Production profile, registry, schema, configuration, reference-resolution, implementation-version ownership, durable persistence, and crash recovery remain dependencies. Contract 004 does not construct capability requirements, ambiguity/assumption/uncertainty artifacts, evidence or provenance artifacts, reconciliation results, normalized or ordered requests, validation results, canonical requests, issuance artifacts, handoffs, or execution outputs. Contract 005 is the next separate downstream boundary and remains unimplemented.
