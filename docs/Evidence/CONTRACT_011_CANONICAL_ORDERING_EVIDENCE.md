# Contract 011 Canonical Ordering Evidence

## Disposition

**CONTRACT 011: IMPLEMENTED — VERIFIED WITH DECLARED FIXTURE PROFILE, REGISTRIES, RULES, SCHEMA, CONFIGURATION, IDENTITY, REPLAY, AND PERSISTENCE DEPENDENCIES**

This is a bounded, fixture-profile-driven, in-memory reference slice. Contract 011 remains governed by the draft Contract 011 source; implementation does not canonize or promote that source.

## Exact input and subject boundary

`CanonicalOrderingInput` binds one exact immutable `NormalizedRequestRepresentation`, its content/integrity binding, `fixture-ordering-v1`, ordering and rule registries, scope/comparison/traversal/tie-break versions, schema/configuration/implementation versions, replay context, explicit scopes, and declared constraints. Foreign, copied, reconstructed, or substituted normalized publications fail before ordering.

`OrderableSubject` references exactly one existing `NormalizedSemanticElementId` or `NormalizedRelationshipId`. Normalized elements and relationships are never copied, reconstructed, or mutated. Contract 011 does not inspect Contract 009 or Contracts 002–008 to reconstruct semantics.

## Fixture authorities

- Profile: `contract-011-fixture`, version `fixture-ordering-v1`, authority `TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT`.
- Ordering registry: `fixture-ordering-registry-v1`.
- Ordering rule registry: `fixture-ordering-rules-v1`.
- Scope registry: `fixture-ordering-scopes-v1`.
- Comparison: `fixture-ordering-comparison-v1`.
- Traversal: `DeterministicKahnTraversal` / `fixture-traversal-v1`.
- Tie-break: `SubjectIdentityAscendingTieBreak` / `fixture-tie-break-v1`.
- Schema/configuration/implementation: `fixture-ordering-schema-v1`, `fixture-ordering-config-v1`, `sre-runtime-fixture-v1`.

The fixture relationship bridge authorizes only `DependsOn` with `target-before-source`. A normalized relationship never becomes an ordering constraint automatically; without the explicit bridge, no relationship edge is created.

## Runtime artifacts and authority

The runtime exposes `CanonicalOrderingInput`, `OrderableSubject`, `OrderingScope`, `OrderingConstraint`, `OrderingConstraintApplication`, `OrderingConstraintGraph`, `OrderingLinearizationRecord`, `OrderingDecision`, `CanonicalOrderingAssignment`, `CanonicallyOrderedRequestRepresentation`, and `CanonicalOrderingFailureRecord`.

`OrderingConstraintApplication`, `OrderingConstraintGraph`, and `OrderingLinearizationRecord` are immutable, identified, replay-bound, non-authoritative implementation-control evidence. Only `OrderingDecision` plus `CanonicalOrderingAssignment` establishes canonical position, and every assignment is scope-relative.

The operation validates exact subject references, scope membership, explicit bridge applicability, constraint integrity, contradiction/cycle conditions, deterministic traversal, and assignment coverage. It publishes exactly one `CanonicallyOrderedRequestRepresentation` or one `CanonicalOrderingFailureRecord`; no partial authoritative ordering publication survives failure.

Outcome classes are explicit: `DependencyLinearized`, `TieBroken`, `IdentityOrder`, `PreservedOrder`, and `Direct`. The fixture uses `TieBroken` for valid underdetermination and `DependencyLinearized` where explicit constraints fully determine arrangement. Identity/preserved order are not inferred as fallbacks.

Canonical position does not imply priority, importance, hierarchy, semantic precedence, dependency truth, execution order, eligibility, construction validity, issuance, authorization, or execution.

## Focused tests

- `contract_011_direct_element_and_relationship_ordering_is_explicit`
- `contract_011_relationship_bridge_is_explicit_not_automatic`
- `contract_011_authorized_tie_break_is_distinct_from_incidental_order`
- `contract_011_cycle_and_contradiction_fail_before_tie_break`
- `contract_011_wrong_normalized_binding_is_atomic`

These cover direct element/relationship subjects, explicit relationship bridge behavior, authorized tie-breaking, contradiction-before-tie-break, exact upstream binding, assignment coverage, and atomic failure. The full suite also retains all Contract 000–010 tests.

## Verification and limitations

Passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` — 67 passed, 0 failed
- `cargo test --doc` — 0 doctests, 0 failed

Production ordering profile and registry/rule ownership, broader scope and comparison coverage, durable persistence/crash recovery, and production conformance remain dependencies. Contract 012 consumes this exact immutable publication for separate structural validation; Contract 013 construction and Contract 014 identity/issuance consume later exact publications; Contract 015 handoff remains unimplemented. No commit or push was performed.
