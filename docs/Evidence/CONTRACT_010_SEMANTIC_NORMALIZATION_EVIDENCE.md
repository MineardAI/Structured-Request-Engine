# Contract 010 Semantic Normalization Evidence

## Disposition

**CONTRACT 010: IMPLEMENTED — VERIFIED WITH DECLARED FIXTURE PROFILE, CANONICAL REGISTRY, MAPPING-RULE REGISTRY, SCHEMA, CONFIGURATION, IDENTITY, REPLAY, AND PERSISTENCE DEPENDENCIES**

This is a bounded, fixture-profile-driven, closed-mapping, in-memory reference slice. Contract 010 remains governed by the draft Contract 010 source; implementation does not canonize or promote that source.

## Bounded scope and sole input

`SemanticNormalizationInput` is the sole Contract 010 entry point. It binds one exact committed `SemanticReconciliationSet`, the derived upstream content binding, profile and registry identities/versions, schema/configuration/implementation versions, and replay context. The operation rejects a foreign, copied, reconstructed, or substituted reconciliation publication before any mapping rule is considered.

`ReconciledSemanticElement` now carries the explicit normalization-source expression and semantic metadata supplied at the Contract 009 boundary: semantic domain, semantic class, represented scope, reconciliation references, standing references, evidence/grounding/provenance references, and unresolved conditions. Contract 010 does not inspect unrelated Contract 002–008 publications to reconstruct these values.

## Normalized relationship prerequisite

Contract 009 now exposes exact `ReconciledRelationship` artifacts. `NormalizedRelationship` is created only from one exact relationship in the bound publication. The endpoint rule is exact Contract 009 subject identity preservation: `source_subject_id` and `target_subject_id` remain `ReconciliationSubjectId` values because Contract 009 does not provide a lawful one-to-one normalized-element endpoint binding. Contract 011 may create an explicit profile-authorized bridge to normalized element subjects; Contract 010 does not infer or repair endpoints.

Relationship identity includes source relationship identity, relationship class, canonical expression, both endpoints, represented scope, normalization disposition, normalization/reconciliation decision references, standing, evidence, grounding, provenance, unresolved conditions, and applicable version bindings. Relationship membership participates in publication identity/content; relationship collection order is non-authoritative. No relationship is invented when Contract 009 contains none, duplicate-looking relationships remain distinct when continuity differs, and foreign/orphaned endpoints fail atomically.

## Fixture authorities

- Profile: `contract-010-fixture`, version `fixture-normalization-v1`, authority `TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT`.
- Canonical registry: `fixture-canonical-registry-v1`, identity `CanonicalRegistryId::derive(["contract-010-fixture"])`.
- Mapping-rule registry: `fixture-mapping-rules-v1`, identity `MappingRuleRegistryId::derive(["contract-010-fixture"])`.
- Schema: `fixture-normalization-schema-v1`.
- Configuration: `fixture-normalization-config-v1`.
- Implementation: `sre-runtime-fixture-v1`.

The canonical registry recognizes only these fixture values:

| Domain | Class | Expression class | Canonical value |
| --- | --- | --- | --- |
| Objective | RequestedOutcome | expression | `achieve-outcome` |
| Constraint | Content | expression | `do-not-disclose` |
| CapabilityRequirement | LanguageUnderstanding | expression | `understanding` |

The closed mapping-rule registry declares only:

| Rule | Version | Source domain/class | Exact source | Exact target | Kind |
| --- | --- | --- | --- | --- | --- |
| `ObjectiveRequestedOutcomeExactAliasRule` | `fixture-rule-v1` | Objective / RequestedOutcome | `achieve outcome` | `achieve-outcome` | `ExactAlias` |
| `ConstraintContentExactAliasRule` | `fixture-rule-v1` | Constraint / Content | `do not disclose` | `do-not-disclose` | `ExactAlias` |
| `CapabilityLanguageUnderstandingExactAliasRule` | `fixture-rule-v1` | CapabilityRequirement / LanguageUnderstanding | `understand` | `understanding` | `ExactAlias` |

Canonical registry membership never authorizes a source-form mapping. No fuzzy matching, synonym discovery, generic case/whitespace cleanup, locale handling, numeric conversion, decomposition, or collection deduplication is implemented.

## Runtime artifacts and authority

The runtime exposes `SemanticNormalizationInput`, `NormalizationOperation`, `FixtureNormalizationProfile`, `FixtureCanonicalRegistry`, `FixtureMappingRuleRegistry`, `MappingRuleDefinition`, `NormalizationOperationOutcome`, `MappingApplicationRecord`, `NormalizationDecision`, `NormalizedSemanticElement`, `NormalizedRelationship`, `NormalizedRequestRepresentation`, and `NormalizationFailureRecord`.

`MappingApplicationRecord` is immutable, independently identified, non-authoritative implementation-control evidence. `NormalizationDecision` is authoritative only for the resulting canonical expression. The operation publishes exactly one successful `NormalizedRequestRepresentation` or one `NormalizationFailureRecord`; no partial authoritative success survives failure.

Eligibility is evaluated by `evaluate_normalization_eligibility` from one exact reconciled element, its exact standing assignment, and the bound profile. Missing, foreign, duplicate, contradictory, or orphaned standing fails. `Standing::Included` and `Standing::Contributing` are eligible in the fixture; explicit `profile-deferred` conditions produce `ProfileDeferred`.

Element dispositions are exact:

- `Mapped`: one explicit, versioned, profile-authorized, domain/class-scoped rule changes the expression to a registered target.
- `Identity`: the expression is already registered canonical and identity authority is explicitly enabled; no unchanged expression is treated as identity by default.
- `Preserved`: the profile explicitly permits unchanged expression without asserting canonicality; it is not a missing-rule fallback.
- `Deferred`: explicit profile-deferred eligibility maps through the declared disposition path.

All dispositions preserve semantic identity, domain, class, scope, standing, reconciliation decisions, contributing subjects, preserved alternatives, evidence, grounding, provenance, and unresolved conditions. Different source elements may produce the same canonical expression without identity collapse. Internal `stable_iteration_order` is non-authoritative and is not Contract 011 ordering.

## Tests and verification

Focused Contract 010 tests in `src/lib.rs`:

- `contract_010_exact_alias_is_mapped_and_replay_is_deterministic`
- `contract_010_wrong_upstream_binding_fails_atomically`
- `contract_010_missing_standing_cannot_create_authority`
- `contract_010_no_mapping_is_not_an_identity_or_preservation_fallback`
- `contract_010_deferred_profile_condition_is_explicit`
- `contract_010_relationship_publication_preserves_exact_continuity`
- `contract_010_relationship_identity_separation_is_preserved`
- `contract_010_relationship_collection_order_is_non_authoritative`
- `contract_010_foreign_relationship_endpoint_fails_atomically`

Verification passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace` — 67 passed, 0 failed
- `cargo test --doc` — 0 doctests, 0 failed

## Limitations and boundaries

Production profile and registry ownership, broader mapping families, expanded semantic classes, production schemas/configuration, evidence breadth, durable persistence/crash recovery, and production conformance remain dependencies. Contract 010 does not own Contract 011 ordering authority; the separate Contract 011 fixture slice consumes its exact normalized elements and relationships. Contract 012 validation, Contract 013 construction, Contract 014 identity/issuance, and Contract 015 handoff remain outside the implementation. It introduces no authorization, provider/model/tool, planning, execution, generation, or release API.

No commit or push was performed.
