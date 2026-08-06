# Contract 013 Canonical Request Construction Evidence

**Disposition:** **CONTRACT 013: IMPLEMENTED — VERIFIED WITH DECLARED FIXTURE PROFILE, SCHEMA, RULE-SET, REGISTRY, CONFIGURATION, IDENTITY, REPLAY, AND PERSISTENCE DEPENDENCIES**

This is a bounded fixture-profile-driven, in-memory logical construction slice. It does not canonize or promote the Contract 013 draft.

## Exact dual-input boundary

`CanonicalRequestConstructionInput` binds one exact immutable `CanonicallyOrderedRequestRepresentation` and one exact completed `StructuralValidationResult`. It verifies the Contract 011 publication identity/content binding, Contract 012 result identity and validation subject binding, completion state, accepted eligibility, and all construction profile/schema/rule-set/registry/configuration/implementation/replay bindings before assembly.

Only `Eligible` and `EligibleWithWarnings` are accepted. `Ineligible`, `Deferred`, and `NotDetermined` are terminal invocation failures. Construction does not reconsider Contract 012 findings, severity, decisions, or warnings.

Contract 011 carries immutable normalized element and relationship snapshots alongside its orderable identities. Contract 013 uses those exact snapshots exclusively; it does not inspect unrelated upstream publications, reinterpret expressions, or recreate semantic relationships.

## Fixture authorities

| Authority | Fixture identity/version |
| --- | --- |
| Construction profile | `contract-013-fixture` / `fixture-construction-v1` |
| Construction schema | `fixture-construction-schema-v1` |
| Construction rule set | `fixture-construction-rules-v1` |
| Construction registry | `fixture-construction-registry-v1` |
| Construction configuration | `fixture-construction-config-v1` |
| Implementation | `sre-runtime-fixture-v1` |

The profile authority is explicitly `TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT`.

The closed schema supports `Container`, `ReferenceHolder`, `StructuralGrouping`, `ValidationLineageHolder`, `ConstructionMetadataHolder`, `SectionHeader`, and `IndexHolder`. Semantic classes such as `SemanticSummary` are rejected. Relationships remain explicit relationship components.

## Runtime and accounting model

The implementation exposes distinct `ConstructionDecision`, `ConstructionPlacement`, `SourceToArtifactMapping`, `ConstructedRequestComponent`, `ConstructionExecutionRecord`, `ConstructedCanonicalRequest`, `ConstructionManifest`, `ConstructionFailureRecord`, and `CanonicalRequestConstructionOutcome` artifacts.

Decisions, placements, and mappings are emitted during construction and the manifest is assembled from those records. Every Contract 011 orderable subject receives exactly one `Included` disposition and governed-source basis. Components preserve normalized identities, canonical expressions, semantic/relationship facts, standing, evidence, grounding, provenance, unresolved conditions, exact ordering assignment identity, and scope-relative canonical position.

The fixture authorizes one narrow omission: an empty optional `IndexHolder` structural component. The omission is recorded in a decision, mapping, and manifest and removes no Contract 011 subject. No semantic or relationship content is omitted.

## Atomicity and replay

Construction accumulates private candidate state, validates source coverage, target bases, placements, mappings, positions, identities, and request/manifest bindings, then publishes exactly one successful request-plus-manifest outcome or one failure record. A request without a manifest, manifest without a request, incomplete mapping, orphan placement, or missing basis cannot receive successful standing.

Equivalent declared inputs replay to equivalent request and manifest artifacts, including decisions, placements, mappings, omissions, bindings, and execution evidence. The artifact is logical and pre-issuance; no transport encoding is defined.

## Focused tests

The 15 focused tests in `contract_013_tests` cover exact dual binding, substituted and mismatched validation inputs, warning acceptance, rejected eligibility states, identity and order preservation, source/mapping/position failures, closed schema classes, authorized and unauthorized omission, manifest record coverage, deterministic replay, and the absence of Contract 014/015 authority.

Full verification passed: 100 unit tests, 0 failures; 0 doctests, 0 failures.

## Limitations and stop boundary

Production construction profile/schema/rule-set/registry/configuration ownership, broader schema coverage, durable persistence, crash recovery, and production conformance remain unresolved dependencies. No semantic interpretation, normalization, reordering, or validation rerun is implemented. Contract 014 consumes this exact request/manifest pair for initial issuance; Contract 015 handoff remains unimplemented and unchanged. No commit or push was performed.
