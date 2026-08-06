# Contract 005 Capability Requirements Evidence

## Disposition

**CONTRACT 005: VERIFIED WITH DECLARED PROFILE AND REGISTRY DEPENDENCIES**

This record documents the bounded in-memory implementation of draft Contract 005. Contract status remains Draft — Constitutional Development; this evidence does not adopt a production capability ontology, inventory, availability model, or downstream authority.

## Scope and proposal prerequisite

`represent_capabilities` answers only what abstract capability requirements are represented in an admitted submission context. It does not inspect or establish capability inventory, availability, reachability, licensing, quotas, authorization, binding, provider/model/tool/connector selection, credential state, invocation, execution, generation, or release.

The narrow Contract 002 prerequisite adds typed `ProposalContent.capability_elements` and `ProposalCapabilityElement` / `ProposalCapabilityRelationship`. Capability-bearing requests bind proposal schema `proposal-fixture-v3`. Proposal identity already fingerprints complete `ProposalContent`, so materially different typed capability content derives a new `InterpretationProposalId`. Existing Contract 002–004 surfaces remain semantically unchanged and prior fixtures retain their original schema bindings.

Capability generation uses only `capability_elements`. Generic narrative, `proposed_elements`, objective expressions, constraint expressions, assumptions, evidence text, source content, metadata, and application/runtime state cannot create capability requirements. Objectives and constraints are consumed only as exact lineage/context publications and attachment-reference domains.

## Profile, registries, and closed rules

The fixture profile is `contract-005-fixture` / `fixture-005-v1` with authority `TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT`. Eight fixture registries are explicitly bound at `fixture-005-registry-v1`: class, form, origin, basis, necessity, relationship, status, and scope-status. No ambient, global, provider, inventory, plugin, tool, model, credential, environment, or live registry lookup exists.

Abstract classes are functional and mechanism-neutral, such as `LanguageUnderstanding`, `LanguageGeneration`, `ExternalInformationRetrieval`, and `StructuredDataProcessing`. Provider/product/model/tool/database names are not accepted as capability classes. The closed fixture abstraction rule `fixture-rule-retrieve-v1` / `fixture-005-rule-v1` maps the supplied method `retrieve` to `ExternalInformationRetrieval` while preserving the supplied method, source class, expression, rule identity/version, origin, basis, and lineage. Unknown or open rules fail; no heuristic, similarity, fallback, or open semantic reasoning is used.

## Lineage and preserved semantics

`validate_capability_lineage` validates exact `AdmittedInterpretationProposalSet`, `DeclaredObjectiveSet`, and `DeclaredConstraintSet` bindings, including proposal-set identity, objective/constraint publication identity, interpretation operation, source intake, and proposal/decision context before candidate construction. Foreign-lineage composition fails without substitute search, repair, reconstruction, or mutation.

Capability requirements preserve logical and representation identities, exact expression, abstract class, supplied method/class, abstraction rule identity/version, form, origin, basis, necessity, scope and scope-resolution status, objective/constraint association IDs, access dependency, `classification_support_status`, evidence observations, relationship declarations, proposal/element lineage, profile, registries, schema, configuration, and implementation version.

Necessity (`Required`, `Optional`, `Conditional`, `Supporting`, `Alternative`, `Unresolved`, `Unknown`) is representational only. Access dependencies such as `credential-dependent` are preserved without checking credentials, licenses, subscriptions, reachability, quotas, or authorization. `classification_support_status` means representation/classification support only; it is not runtime or provider support. Relationships such as `SupportsAsDeclared` remain epistemically declared and do not establish dependency truth, substitutability, priority, binding, or execution order.

Profile-authorized composite decomposition preserves parent/child identities and `ComponentOf` relationships. Unauthorized or open-ended decomposition fails. No objective or constraint wording creates a capability without a typed capability element.

## Terminal behavior

The terminal algebra is exactly one `CapabilityRequirementSet` or one `CapabilityRepresentationFailureRecord`. Empty capability content succeeds when the fixture profile permits it. Malformed/mixed fatal elements, unsupported concrete classes, invalid references, foreign lineage, incompatible profile/registry/schema/configuration, and unauthorized abstraction/decomposition produce failure with no partial authoritative set. In-memory API-level atomicity is verified; durable persistence and crash recovery are not implemented.

## Verification

The final workspace test run completed with 42 passing unit tests, zero failures, and zero doctests. Contract 005 focused coverage includes typed-only generation, empty output, closed abstraction and basis preservation, provider-specific class rejection, open abstraction rejection, necessity/access/scope/classification-support preservation, declared relationships, lineage/profile/registry failures, mixed fatal input, authorized/unauthorized decomposition, and replay equivalence. All prior Contract 000–004 tests pass.

Verification commands passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test --doc`

## Limitations and boundary

Production capability ontology, abstraction-rule ownership, registries, profile/schema/configuration ownership, durable persistence, and crash recovery remain dependencies. Contract 005 does not implement Contract 006 ambiguity/assumption/uncertainty/clarification representation or any later evidence, provenance, reconciliation, normalization, ordering, validation, construction, issuance, handoff, planning, execution, generation, or release authority.
