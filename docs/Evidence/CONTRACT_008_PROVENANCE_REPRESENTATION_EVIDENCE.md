# Contract 008 Provenance Representation Evidence

## Disposition

**CONTRACT 008: VERIFIED WITH DECLARED FIXTURE PROFILE, REGISTRY, SCHEMA, CONFIGURATION, RULE, IDENTITY, REPLAY, AND PERSISTENCE DEPENDENCIES**

This record documents the bounded in-memory implementation of draft Contract 008. Contract status remains Draft — Constitutional Development; no production provenance ontology, historical-truth authority, completeness assessment, custody verification, trust, identity federation, durable graph persistence, or Contract 009 authority is adopted.

## Scope and sole semantic entry point

`ProvenanceRepresentationInput` is the only Contract 008 semantic entry point. It is immutable, identity-bearing, schema-bound, profile/registry/configuration-bound, replay-bound, and contains explicit subject, event, lineage, lifecycle, external-mapping, conflict, upstream-publication, and temporal declarations. Earlier artifacts and technical records are used only for exact identity, subject, participant, publication, operation, and lineage validation.

Artifact existence, proposal content, evidence representations, grounding assertions, operation/publication records, version order, filesystem/Git history, timestamps, logs, telemetry, environment state, and matching strings do not generate provenance. No scanning, mining, inference, reconstruction, substitution, newest-version selection, or ambient history lookup is performed.

## Fixture origins, profile, and registries

The fixture profile is `FixtureProvenanceProfile::fixture()` with explicit identity, version `fixture-provenance-v1`, and authority `TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT`. Accepted declaration origins are `ApplicationSupplied`, `AdmittedProposalSupplied`, `ImportedDeclaration`, `PriorGovernedDeclaration`, `ExternalIdentityMappingDeclaration`, and `TestFixture`.

`FixtureProvenanceRegistries::fixture()` binds subject classes, event classes, lineage relationships, lifecycle relationships, external namespaces, conflict classes, and temporal variants under `fixture-provenance-registry-v1`. Schema, configuration, implementation, input-schema, operation, and exact Contract 001–007 publication bindings are explicit. No production default or ambient fallback exists.

## Subjects, events, lineage, lifecycle, mapping, and conflicts

Subject declarations preserve exact existing subject identities and class/origin/basis without reissuing, aliasing, repairing, or replacing upstream identities. Provenance representations and immutable instances retain profile, registry, schema, configuration, implementation, operation, input, and basis metadata.

Events are a closed typed family: transformation, custody, publication, and lifecycle. Events retain independent identities, declaration identities, class/schema, typed payload, participant/subject references, basis, and event time. Event existence does not create lineage; lineage assertions are independently declared and independently identified. A lineage assertion does not create an event.

Relationship behavior is registry-specific. The bounded implementation validates supported relationship membership, exact subject references, and prohibited ancestry self-cycles without imposing a global graph/transitivity rule. Lifecycle relations, external identity mappings, and conflict associations are separately represented. External mappings preserve only an explicitly declared namespace/identifier-to-subject relation and do not establish authenticity, ownership, trust, authority continuity, or universal equivalence.

## Temporal and non-success behavior

Event time is a closed representation with exact, range, approximate, relative, unknown, unavailable, and conflicting variants. Unknown time remains unknown; the current clock, filesystem timestamps, Git timestamps, representation time, and commitment time never replace declared event time. Conflicting claims remain separately representable through conflict declarations without selection or adjudication.

Empty provenance may commit as `ProvenanceStatus::Empty` when the fixture profile permits it. It means only that no provenance was represented by that operation; it does not mean the subject has no history. Construction decisions are immutable, inspectable, non-authoritative implementation-control records and never replace the terminal publication.

## Exact lineage and terminal behavior

The operation validates exact Contracts 001–007 bindings: admitted proposal set, objective set, constraint set, capability set, meaning-qualification set, evidence set, source intake, interpretation operation, proposal IDs, admission decision IDs, and evidence publication lineage. Foreign or mismatched bindings fail before candidate publication.

The terminal algebra is exactly one `ProvenanceRecordSet` or one `ProvenanceRepresentationFailureRecord`. Failed operations publish no partial authoritative set. Corrections, replacements, and supersession require new declared inputs and new immutable identities; prior artifacts are not mutated.

## Tests and verification

Contract 008 focused tests cover declared-only subject/event/lineage/mapping/time representation, event-versus-lineage separation, deterministic replay, permitted empty publication, foreign-lineage failure, prohibited ancestry cycle failure, subject identity preservation, and atomic outcomes. The final workspace run completed with **48 passing unit tests**, zero failures, and zero doctests.

Verification commands passed:

- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo test --doc`

## Limitations and boundary

Production provenance ontology/profile/registry ownership, temporal authority, external identity federation, participant authentication, historical truth, completeness, custody verification, trust, retrieval, durable persistence, graph databases, reconciliation, normalization, ordering, validation, construction, issuance, handoff, planning, execution, communication, and release remain outside this slice. Contract 009 was not implemented or modified. No commit or push was performed.
