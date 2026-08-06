# Structured Request Engine

## Contract 013 — Canonical Request Construction

**Version:** v0.1.0 Draft  
**Architectural status:** Frozen  
**Contract family:** Deterministic Processing Family  
**Constitutional act:** Construct

---

## Normative requirement identifiers

Normative requirements in this contract use the prefix:

```text
SRE-013-
```

The key words **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **MAY**, and **MUST** are to be interpreted as normative requirement terms.

---

# 1. Architectural context

## SRE-013-CONTEXT-001

Contract 013 governs the constitutional transition from a structurally eligible canonically ordered request representation to an immutable constructed canonical request artifact.

## SRE-013-CONTEXT-002

Contract 013 is downstream of Contract 011 — Canonical Ordering and Contract 012 — Structural Validation.

```text
CanonicallyOrderedRequestRepresentation
        +
Applicable StructuralValidationResult
        ↓
Canonical Request Construction
        ↓
ConstructedCanonicalRequest
        +
ConstructionManifest
```

## SRE-013-CONTEXT-003

Contract 013 SHALL remain constitutionally distinct from:

- semantic reconciliation under Contract 009;
- semantic normalization under Contract 010;
- canonical ordering under Contract 011;
- structural eligibility determination under Contract 012;
- identity and issuance under Contract 014;
- downstream custody transfer under Contract 015;
- transport serialization, provider binding, planning, execution, generation, release, and tool invocation.

## SRE-013-CONTEXT-004

The canonical lifecycle boundary SHALL be understood as:

```text
012 Structural Validation
    determines whether an exact ordered representation
    is eligible to enter construction
        ↓
013 Canonical Request Construction
    mechanically assembles and publishes the immutable artifact
    and its bound construction manifest
        ↓
014 Identity and Issuance
    assigns issued constitutional request identity
    and establishes issuance standing
```

---

# 2. Purpose

## SRE-013-PURPOSE-001

The purpose of Contract 013 is to define the deterministic, identity-preserving, non-interpretive construction of one immutable logical request artifact from one exact canonically ordered representation whose applicable Structural Validation Result permits construction.

## SRE-013-PURPOSE-002

Contract 013 SHALL establish:

- the exact dual-input binding required for construction;
- the constitutional authority and limitations of Canonical Request Construction;
- the runtime roles of construction activities, decisions, manifests, artifacts, and failure records;
- the construction profile, schema, rules, registries, and configuration context;
- the mechanical assembly and preservation requirements;
- atomic publication of the constructed artifact and its manifest;
- deterministic replay and immutable correction doctrine;
- failure taxonomy for invocation, resolution, and mechanical failures;
- the boundary between constructed artifact identity and issued request identity.

## SRE-013-PURPOSE-003

Contract 013 SHALL NOT establish:

- what semantic content proceeds;
- how semantic content is canonically expressed;
- how content is ordered;
- whether content is structurally eligible;
- final canonical request identity;
- issuance standing;
- downstream custody;
- execution or use authority;
- provider selection or binding;
- transport or wire-format serialization.

---

# 3. Governing constitutional elements

## SRE-013-GOVERNING-001 — Governing question

> **What immutable canonical request artifact is mechanically constructed from the structurally eligible canonically ordered representation?**

## SRE-013-GOVERNING-002 — Constitutional subject

The constitutional subject is:

> **The immutable canonical request artifact mechanically assembled from one structurally eligible canonically ordered request representation.**

## SRE-013-GOVERNING-003 — Constitutional act

The constitutional act is:

```text
Construct
```

## SRE-013-GOVERNING-004 — Organizing doctrine

> **Construction assembles. It does not interpret.**

## SRE-013-GOVERNING-005 — Dependency doctrine

> **Canonical Request Construction is constitutionally downstream of Structural Validation and possesses no independent authority to establish construction eligibility.**

## SRE-013-GOVERNING-006 — Publication doctrine

Successful construction SHALL produce one constitutionally inseparable publication set containing:

```text
ConstructedCanonicalRequest
ConstructionManifest
```

The set is a constitutional grouping and SHALL NOT require an independent wrapper identity unless a later adopted contract expressly requires one.

---

# 4. Architectural identity

## SRE-013-IDENTITY-001

Contract 013 is the artifact-construction contract of the Structured Request Engine.

## SRE-013-IDENTITY-002

Contract 013 receives governed content from Contract 011 and construction eligibility from Contract 012.

## SRE-013-IDENTITY-003

The constitutional relationship SHALL be:

```text
Representation supplies content.
Validation supplies eligibility to enter construction.
Construction supplies the artifact.
Issuance supplies constitutional request standing.
```

## SRE-013-IDENTITY-004

A `ConstructedCanonicalRequest` SHALL be an immutable logical artifact independent of any specific transport encoding, wire format, storage representation, or implementation language.

## SRE-013-IDENTITY-005

A `ConstructedCanonicalRequest` SHALL remain constitutionally inert until Contract 014 establishes issuance standing.

---

# 5. Organizing constitutional doctrines

## 5.1 Construction assembles; it does not interpret

### SRE-013-DOCTRINE-001

Construction SHALL instantiate governed content and SHALL NOT reinterpret, supplement, reconcile, normalize, reorder, validate, repair, issue, authorize, or transfer that content.

## 5.2 Exact representation doctrine

### SRE-013-DOCTRINE-002

Construction SHALL instantiate the exact `CanonicallyOrderedRequestRepresentation` authorized by the applicable `StructuralValidationResult`.

It SHALL NOT construct from an alternative, substituted, reconstructed, regenerated, or merely equivalent representation.

## 5.3 Eligibility dependency

### SRE-013-DOCTRINE-003

Contract 013 MAY verify the authenticity, applicability, completion, eligibility state, version compatibility, and binding of a supplied Structural Validation Result.

It SHALL NOT establish, revise, waive, infer, replace, or reinterpret construction eligibility.

## 5.4 Construction is not revalidation

### SRE-013-DOCTRINE-004

Eligibility binding verification SHALL NOT constitute structural revalidation.

Contract 013 SHALL NOT repeat Contract 012 validation rules or repair conditions identified by Contract 012.

## 5.5 Construction instantiates upstream decisions

### SRE-013-DOCTRINE-005

Construction SHALL instantiate upstream decisions and SHALL NOT reproduce, replace, or silently recreate semantic, normalization, ordering, or validation decisions.

## 5.6 Preservation doctrine

### SRE-013-DOCTRINE-006

Construction SHALL preserve:

- upstream identities;
- canonical semantic expressions;
- canonical ordering assignments;
- reconciliation standing;
- ambiguity, assumption, and uncertainty representation;
- evidence references;
- provenance references;
- validation lineage;
- profile, schema, registry, rule-set, and configuration context.

## 5.7 No upstream substitution

### SRE-013-DOCTRINE-007

Construction SHALL NOT replace an upstream constitutional object with a merely equivalent value, identifier, reference, expression, collection, or arrangement.

## 5.8 Mechanical decision limitation

### SRE-013-DOCTRINE-008

A Construction Decision SHALL resolve only a mechanically authorized assembly determination.

It SHALL NOT determine:

- what content means;
- whether content proceeds;
- how content is normalized;
- where content is ordered;
- whether a defect is acceptable;
- whether the artifact should be issued;
- whether any downstream authority may use it.

## 5.9 Manifest non-semantic status

### SRE-013-DOCTRINE-009

A Construction Manifest SHALL describe how a constructed artifact was produced.

It SHALL NOT independently contribute semantic request content.

## 5.10 Abstract artifact doctrine

### SRE-013-DOCTRINE-010

A `ConstructedCanonicalRequest` SHALL NOT be constitutionally defined by JSON, XML, Protocol Buffers, byte layout, whitespace, compression, transport framing, or any other wire-format concern.

## 5.11 Artifact identity is not issued request identity

### SRE-013-DOCTRINE-011

```text
ConstructedCanonicalRequestId
        ≠
Issued Canonical Request Identity
```

The first identifies an immutable pre-issuance artifact publication.

The second is governed by Contract 014.

## 5.12 Construction does not authorize

### SRE-013-DOCTRINE-012

Successful construction SHALL NOT establish:

- issuance standing;
- downstream custody;
- provider binding;
- planning authority;
- execution permission;
- generation authority;
- release authority;
- tool invocation authority;
- semantic truth, policy approval, safety approval, or feasibility.

## 5.13 No hidden construction inputs

### SRE-013-DOCTRINE-013

No hidden prompt, ambient runtime state, mutable global, undocumented session context, provider default, live tool inventory, operator preference, wall-clock value, or unrecorded assumption SHALL materially influence construction.

## 5.14 Correction requires reconstruction

### SRE-013-DOCTRINE-014

Correction of a Construction Decision, manifest entry, source mapping, omission, or artifact component SHALL require a new governed construction activity and new immutable publications.

Committed construction records SHALL NOT be edited in place.

---

# 6. Constitutional authority

## 6.1 Canonical Request Construction Authority

### SRE-013-AUTHORITY-001

The Canonical Request Construction Authority SHALL be the exclusive constitutional owner of the construction act governed by this contract.

### SRE-013-AUTHORITY-002

The Canonical Request Construction Authority MAY:

- identify one exact canonically ordered representation;
- identify one applicable completed Structural Validation Result;
- verify eligibility binding without revalidation;
- resolve the applicable Construction Profile;
- resolve the applicable Construction Schema;
- resolve applicable construction rule sets and registries;
- bind a Construction Configuration Snapshot;
- create immutable Construction Decisions;
- perform mechanical source-to-artifact mapping;
- instantiate required artifact fields, sections, collections, and references;
- record profile-authorized optional omissions;
- evaluate construction-operation invariants;
- publish one immutable `ConstructedCanonicalRequest`;
- publish one bound immutable `ConstructionManifest`;
- publish one immutable `ConstructionFailureRecord` when successful construction cannot lawfully complete.

### SRE-013-AUTHORITY-003

The Canonical Request Construction Authority SHALL NOT:

- reinterpret source meaning;
- select among semantic alternatives;
- modify reconciliation standing;
- normalize expression;
- sort, reorder, linearize, or revise canonical arrangement;
- perform or revise structural validation;
- waive construction eligibility requirements;
- invent missing semantic content;
- create missing evidence or provenance;
- repair upstream identifiers or relationships;
- issue canonical request identity;
- establish issuance standing;
- serialize a transport payload as a constitutional act;
- transfer custody;
- select or bind a provider;
- authorize planning, execution, generation, release, or tool use.

## 6.2 Exclusive publication authority

### SRE-013-AUTHORITY-004

Only the Canonical Request Construction Authority MAY commit a successful Contract 013 publication set or a `ConstructionFailureRecord`.

### SRE-013-AUTHORITY-005

Profiles, schemas, registries, rule sets, applications, upstream authorities, and downstream consumers MAY supply constitutional inputs.

They SHALL NOT independently publish Contract 013 outcomes unless they are themselves the recognized Canonical Request Construction Authority for the activity.

---

# 7. Major concept and artifact classification

**Specification level:** Constitutional concept

| Concept or artifact | Classification |
|---|---|
| Canonical Request Construction doctrine | Constitutional concept |
| Canonical Request Construction Authority | Constitutional authority |
| `CanonicalRequestConstruction` | Logical constitutional activity |
| `ConstructionDecision` | Required runtime artifact and immutable trace record |
| `ConstructionProfile` | Required runtime artifact or externally supplied normative profile |
| `ConstructionSchema` | Required runtime artifact or externally supplied normative schema |
| Construction Rule Set | Required runtime artifact or externally supplied normative rule set |
| Construction Registries | Required runtime artifacts or externally supplied normative registries |
| `ConstructionConfigurationSnapshot` | Required runtime reference when configuration materially affects construction |
| `ConstructedCanonicalRequest` | Required runtime artifact and principal artifact publication |
| `ConstructionManifest` | Required runtime artifact and principal companion publication |
| Contract 013 publication set | Constitutional grouping, not necessarily a runtime wrapper |
| `ConstructionFailureRecord` | Required runtime artifact |
| Construction section or field builder | Implementation convenience unless separately required |
| Serialized request representation | Outside Contract 013 constitutional authority |
| Transport payload | Outside Contract 013 constitutional authority |

## SRE-013-CLASSIFICATION-001

A named concept SHALL NOT automatically require a dedicated Rust type, module, service, database table, or top-level serialized artifact unless this contract classifies it as a required runtime artifact.

## SRE-013-CLASSIFICATION-002

A `ConstructionDecision` SHALL remain independently identifiable and auditable even when physically embedded within a Construction Manifest.

## SRE-013-CLASSIFICATION-003

Artifact sections and components SHALL remain internal components of the `ConstructedCanonicalRequest` unless a later adopted contract establishes independent identity, lifecycle, publication, replacement, or authority for them.

---

# 8. Canonical inputs

## SRE-013-INPUT-001

A conforming Canonical Request Construction activity SHALL consume exactly:

- one committed `CanonicallyOrderedRequestRepresentation`;
- one applicable completed `StructuralValidationResult` governing eligibility of that exact representation;
- one applicable `ConstructionProfileId` and exact profile version;
- one applicable `ConstructionSchemaId` and exact schema version;
- applicable Construction Rule Set identity and version;
- applicable Construction Registry identities and versions;
- one Construction Configuration Snapshot reference when configuration materially affects construction;
- applicable contract and implementation version context.

## SRE-013-INPUT-002

The `StructuralValidationResult.ValidatedRepresentationReference` SHALL identify the exact `CanonicallyOrderedRequestRepresentationId` supplied to construction.

## SRE-013-INPUT-003

The Structural Validation Result SHALL have:

```text
ValidationCompletion = Completed
```

and an eligibility state expressly permitted to enter construction.

## SRE-013-INPUT-004

Unless the applicable adopted construction requirements expressly provide otherwise, only:

```text
Eligible
EligibleWithWarnings
```

SHALL permit construction.

## SRE-013-INPUT-005

A `Deferred` eligibility state SHALL permit construction only where the applicable adopted construction requirements expressly define the permitted artifact shape and do not allow an incomplete required artifact to masquerade as a completed `ConstructedCanonicalRequest`.

## SRE-013-INPUT-006

An `Ineligible` result or a `StructuralValidationFailureRecord` SHALL NOT permit construction.

## SRE-013-INPUT-007

Contract 013 SHALL NOT independently retrieve semantic publications from Contracts 003–010 where the governed content is already represented through the supplied `CanonicallyOrderedRequestRepresentation`.

## SRE-013-INPUT-008

Every input that materially affects construction SHALL be declared, version-bound, integrity-protected, traceable, and replayable.

---

# 9. Canonical Request Construction activity

## SRE-013-OPERATION-001

Every Canonical Request Construction activity SHALL possess exactly one immutable `CanonicalRequestConstructionId`.

## SRE-013-OPERATION-002

The activity identity SHALL correlate:

```text
CanonicalRequestConstructionId
├── CanonicallyOrderedRequestRepresentationId
├── StructuralValidationResultId
├── ConstructionProfileId and Version
├── ConstructionSchemaId and Version
├── ConstructionRuleSetId and Version
├── ConstructionRegistry Identities and Versions
├── ConstructionConfigurationSnapshotId, when applicable
├── ConstructionDecisionIds
├── ConstructedCanonicalRequestId, on success
├── ConstructionManifestId, on success
└── ConstructionFailureRecordId, on failure
```

## SRE-013-OPERATION-003

The activity identity SHALL NOT replace the identity of any consumed or produced artifact.

## SRE-013-OPERATION-004

A Canonical Request Construction activity SHALL have exactly one terminal constitutional outcome:

```text
Successful atomic publication set
```

or:

```text
ConstructionFailureRecord
```

## SRE-013-OPERATION-005

A single activity SHALL NOT publish both a successful publication set and a Construction Failure Record.

---

# 10. Runtime object model

## SRE-013-OBJECT-001

Contract 013 defines the following primary runtime roles:

1. `CanonicalRequestConstruction` owns one governed construction activity;
2. `ConstructionDecision` owns one mechanically governed assembly determination;
3. `ConstructedCanonicalRequest` owns one immutable logical pre-issuance request artifact;
4. `ConstructionManifest` owns one immutable account of construction inputs, decisions, mappings, omissions, versions, and outcome;
5. `ConstructionFailureRecord` owns one unsuccessful construction-activity publication.

No runtime role SHALL assume the constitutional responsibility of another.

## 10.1 Canonical Request Construction

### SRE-013-OBJECT-002

A `CanonicalRequestConstruction` is one governed activity that mechanically assembles one exact eligible ordered representation under one declared construction context.

## 10.2 Construction Decision

### SRE-013-OBJECT-003

A `ConstructionDecision` is an immutable replayable record of one mechanically governed assembly determination.

### SRE-013-OBJECT-004

Each Construction Decision SHALL identify at least:

- `ConstructionDecisionId`;
- `CanonicalRequestConstructionId`;
- governing construction rule identity and version;
- target artifact component;
- source representation reference;
- mechanical determination;
- material basis;
- applicable profile, schema, registry, and configuration references;
- resulting placement, inclusion, mapping, or authorized omission;
- decision status.

### SRE-013-OBJECT-005

Construction Decisions MAY be referenced by applicable evidence or provenance publications.

Contract 013 SHALL govern them as construction trace records and SHALL NOT independently grant them constitutional evidence standing.

## 10.3 Constructed Canonical Request

### SRE-013-OBJECT-006

A `ConstructedCanonicalRequest` is the immutable logical request artifact mechanically instantiated from the exact authorized representation.

### SRE-013-OBJECT-007

The artifact SHALL contain only profile- and schema-authorized components derived from governed inputs or mechanically required constitutional metadata.

### SRE-013-OBJECT-008

The artifact SHALL NOT contain the Construction Manifest as semantic request content unless a later adopted profile expressly defines a non-semantic reference field for that purpose.

## 10.4 Construction Manifest

### SRE-013-OBJECT-009

A `ConstructionManifest` is the immutable companion publication that explains and enables reproduction of the construction activity.

### SRE-013-OBJECT-010

The Construction Manifest SHALL identify at least:

- `ConstructionManifestId`;
- `CanonicalRequestConstructionId`;
- `ConstructedCanonicalRequestId`;
- source `CanonicallyOrderedRequestRepresentationId`;
- applicable `StructuralValidationResultId`;
- Construction Profile identity and version;
- Construction Schema identity and version;
- Construction Rule Set identity and version;
- Construction Registry identities and versions;
- Construction Configuration Snapshot reference, when applicable;
- every material Construction Decision reference;
- source-to-artifact mappings;
- included required components;
- profile-authorized optional omissions;
- construction outcome;
- applicable contract and implementation versions.

### SRE-013-OBJECT-011

The Construction Manifest SHALL reference every Construction Decision necessary to explain or reproduce the published artifact.

## 10.5 Construction Failure Record

### SRE-013-OBJECT-012

A `ConstructionFailureRecord` is the immutable authoritative publication of an unsuccessful Contract 013 activity.

### SRE-013-OBJECT-013

A Construction Failure Record SHALL identify:

- activity identity;
- attempted input bindings;
- applicable construction context resolved before failure;
- failure category;
- failure code;
- failure basis;
- material diagnostics;
- whether any non-authoritative intermediate objects were produced;
- commitment and publication metadata.

### SRE-013-OBJECT-014

A Construction Failure Record SHALL NOT contain or imply a successful `ConstructedCanonicalRequest` or `ConstructionManifest`.

---

# 11. Identity doctrine

## SRE-013-ID-001

The following identities SHALL remain distinct:

```text
CanonicalRequestConstructionId
ConstructionDecisionId
ConstructedCanonicalRequestId
ConstructionManifestId
ConstructionFailureRecordId
Issued Canonical Request Identity
```

## SRE-013-ID-002

Every `ConstructedCanonicalRequestId` SHALL identify exactly one immutable pre-issuance artifact publication.

## SRE-013-ID-003

Every `ConstructionManifestId` SHALL identify exactly one immutable manifest bound to exactly one Constructed Canonical Request and one construction activity.

## SRE-013-ID-004

A Constructed Canonical Request identity SHALL NOT constitute issuance standing or the final identity governed by Contract 014.

## SRE-013-ID-005

A Construction Decision identity SHALL remain stable and externally referenceable after publication.

## SRE-013-ID-006

The constitutional publication set SHALL be bound through the artifact identity, manifest identity, and common construction activity reference.

## SRE-013-ID-007

An independent publication-set identity SHALL NOT be required unless later constitutional requirements establish independent set-level reference, lifecycle, supersession, signature, or transfer semantics.

---

# 12. Eligibility binding verification

## SRE-013-BINDING-001

Before mechanical assembly, Contract 013 SHALL verify that:

- the supplied Structural Validation Result is authentic and committed;
- validation completed;
- the result references the exact supplied ordered representation;
- the eligibility state permits construction;
- the Construction Profile and validation construction context are compatible;
- required schemas, registries, rules, contracts, and configuration versions are available and match their declared identities;
- the inputs are not superseded where supersession is constitutionally relevant.

## SRE-013-BINDING-002

Eligibility binding verification SHALL be limited to input acceptance and SHALL NOT repeat or revise structural validation.

## SRE-013-BINDING-003

A mismatch between the representation and validation result SHALL produce an Invocation Failure.

## SRE-013-BINDING-004

An ineligible, failed, missing, stale, or constitutionally inapplicable validation publication SHALL NOT be converted into a mechanical construction failure.

---

# 13. Construction Profile

## SRE-013-PROFILE-001

Every construction activity SHALL identify exactly one applicable Construction Profile and exact version.

## SRE-013-PROFILE-002

A Construction Profile MAY define:

- authorized artifact shape;
- required and optional components;
- component placement rules;
- source-to-artifact mapping rules;
- reference-preservation strategies;
- profile-authorized omissions;
- construction invariant requirements;
- applicable schemas, rule sets, and registries;
- profile compatibility requirements;
- construction-permitting eligibility states.

## SRE-013-PROFILE-003

A Construction Profile SHALL NOT:

- alter semantic meaning;
- resolve semantic conflict;
- normalize expression;
- reorder content;
- waive structural eligibility;
- issue identity;
- authorize downstream use.

## SRE-013-PROFILE-004

A change to the applicable Construction Profile or version SHALL require a new construction activity.

---

# 14. Construction Schema

## SRE-013-SCHEMA-001

Every successful construction activity SHALL instantiate exactly one applicable Construction Schema version.

## SRE-013-SCHEMA-002

The Construction Schema SHALL define the logical artifact structure, including applicable:

- fields;
- sections;
- collections;
- references;
- component types;
- cardinalities;
- logical relationships;
- required constitutional metadata.

## SRE-013-SCHEMA-003

The Construction Schema SHALL be representation-independent and SHALL NOT define transport encoding as constitutional meaning.

## SRE-013-SCHEMA-004

Schema resolution failure SHALL be classified as a Resolution Failure.

---

# 15. Construction rules and registries

## SRE-013-RULE-001

Every material assembly determination SHALL be governed by an identified authoritative construction rule or by an unambiguous schema requirement.

## SRE-013-RULE-002

Construction rules and registries SHALL be externalized, version-bound, integrity-protected, and replayable.

## SRE-013-RULE-003

A construction rule MAY govern:

- target component selection;
- profile-required section creation;
- field placement;
- source-reference mapping;
- identity-preserving embedding or referencing;
- profile-authorized optional omission;
- mechanically predetermined schema-instantiation choices.

## SRE-013-RULE-004

A construction rule SHALL NOT govern semantic survival, normalization, ordering, eligibility, issuance, authorization, or transfer.

## SRE-013-RULE-005

A non-deterministic or ambiguous rule without an adopted deterministic resolution SHALL produce a Mechanical Failure.

---

# 16. Mechanical assembly

## SRE-013-ASSEMBLY-001

Mechanical assembly SHALL instantiate the applicable Construction Schema using the exact governed content and arrangement supplied by the authorized representation.

## SRE-013-ASSEMBLY-002

Mechanical assembly SHALL preserve each upstream element and relationship identity where the Construction Schema represents that subject directly or by reference.

## SRE-013-ASSEMBLY-003

Mechanical assembly SHALL preserve canonical ordering assignments and SHALL NOT independently sort, reorder, linearize, or apply a new tie-breaker.

## SRE-013-ASSEMBLY-004

Mechanical assembly SHALL preserve evidence and provenance references without collapsing them into untraceable embedded values.

## SRE-013-ASSEMBLY-005

Mechanical assembly SHALL NOT regenerate identifiers merely because an implementation-specific builder or serializer would otherwise do so.

## SRE-013-ASSEMBLY-006

Mechanical assembly SHALL NOT copy or transform canonical expression through a formatter that can materially change meaning or canonical form.

## SRE-013-ASSEMBLY-007

Every material source-to-artifact mapping SHALL be recorded in the Construction Manifest directly or through referenced Construction Decisions.

---

# 17. Optional omissions and completeness

## SRE-013-OMISSION-001

The sole successful top-level outcome is:

```text
Constructed
```

## SRE-013-OMISSION-002

A profile-authorized optional omission MAY occur without changing the successful outcome, provided that the omission:

- is expressly permitted by the applicable Construction Profile;
- does not remove a required component;
- does not alter represented meaning;
- is recorded in the Construction Manifest;
- is traceable to its governing rule or schema condition.

## SRE-013-OMISSION-003

A required unresolved, absent, or deferred component SHALL NOT be represented as a complete `ConstructedCanonicalRequest`.

## SRE-013-OMISSION-004

Contract 013 SHALL NOT publish `ConstructedWithOptionalOmissions` or `ConstructedDeferredSections` as separate successful top-level outcomes.

## SRE-013-OMISSION-005

A partial construction artifact SHALL NOT receive successful Contract 013 standing unless a later adopted contract expressly defines a distinct partial artifact type and lifecycle.

---

# 18. Construction invariant evaluation

## SRE-013-INVARIANT-EVAL-001

Construction invariant evaluation SHALL be limited to invariants of the construction operation and publication set.

## SRE-013-INVARIANT-EVAL-002

Construction invariant evaluation MAY verify:

- exact input binding;
- source-to-artifact mapping completeness;
- reference preservation;
- identity preservation;
- canonical arrangement preservation;
- required artifact component assembly;
- manifest completeness;
- atomic publication readiness;
- determinism requirements.

## SRE-013-INVARIANT-EVAL-003

Construction invariant evaluation SHALL NOT repeat structural validation or create a new structural eligibility determination.

---

# 19. Constructed Canonical Request requirements

## SRE-013-ARTIFACT-001

A successful `ConstructedCanonicalRequest` SHALL:

- identify its `ConstructedCanonicalRequestId`;
- reference its `CanonicalRequestConstructionId`;
- reference or preserve the source ordered representation identity;
- preserve governed canonical content and arrangement;
- conform to the applicable Construction Profile and Construction Schema;
- contain every required artifact component;
- preserve required evidence, provenance, reconciliation, normalization, ordering, and validation lineage references;
- remain independent of transport encoding;
- remain immutable after publication.

## SRE-013-ARTIFACT-002

A Constructed Canonical Request SHALL NOT claim:

- issued constitutional request identity;
- issuance standing;
- downstream acceptance;
- execution authority;
- provider binding;
- tool eligibility;
- policy, truth, safety, or feasibility approval.

## SRE-013-ARTIFACT-003

The artifact SHALL be externally referenceable by Contract 014 together with its bound Construction Manifest.

---

# 20. Construction Manifest requirements

## SRE-013-MANIFEST-001

Every successful construction activity SHALL publish exactly one bound Construction Manifest.

## SRE-013-MANIFEST-002

The manifest SHALL contain sufficient information to explain and reproduce the material construction process.

## SRE-013-MANIFEST-003

Volatile metadata such as wall-clock timestamps SHALL:

- be excluded from constitutional equivalence;
- be supplied through the frozen configuration snapshot;
- or be explicitly classified as non-canonical diagnostic metadata.

## SRE-013-MANIFEST-004

The manifest SHALL NOT omit a material Construction Decision that affected artifact content, placement, mapping, omission, identity preservation, or reproducibility.

## SRE-013-MANIFEST-005

The manifest SHALL NOT be interpreted as request meaning or execution instruction.

---

# 21. Successful publication set

## SRE-013-PUBLICATION-001

Successful construction SHALL atomically publish exactly:

```text
one immutable ConstructedCanonicalRequest
and
one immutable bound ConstructionManifest
```

## SRE-013-PUBLICATION-002

Neither publication SHALL independently possess successful Contract 013 standing without the other.

## SRE-013-PUBLICATION-003

Publication of either object without the other SHALL NOT constitute successful construction.

## SRE-013-PUBLICATION-004

The successful publication set SHALL be constitutionally inseparable while the artifact and manifest remain distinct publications with distinct identities.

## SRE-013-PUBLICATION-005

The manifest SHALL describe construction but SHALL NOT be incorporated into semantic request content by virtue of publication-set membership.

---

# 22. Failure taxonomy

## SRE-013-FAILURE-001

Every unsuccessful construction activity SHALL publish exactly one `ConstructionFailureRecord` classified as:

```text
InvocationFailure
ResolutionFailure
MechanicalFailure
```

## 22.1 Invocation Failure

### SRE-013-FAILURE-002

Invocation Failure SHALL include conditions such as:

- `MissingApplicableValidationResult`;
- `ValidationResultNotApplicable`;
- `ValidationBindingMismatch`;
- `ConstructionNotPermitted`;
- `ConstructionProfileMismatch`;
- `StaleValidationBinding`;
- `IneligibleValidationState`;
- `FailedValidationPublicationSupplied`.

### SRE-013-FAILURE-003

Invocation Failure means the activity lacks constitutional permission or valid input binding to proceed.

It SHALL NOT be represented as a mechanical assembly defect.

## 22.2 Resolution Failure

### SRE-013-FAILURE-004

Resolution Failure SHALL include conditions such as:

- `ConstructionProfileResolutionFailure`;
- `ConstructionSchemaResolutionFailure`;
- `ConstructionRegistryResolutionFailure`;
- `ConstructionRuleSetResolutionFailure`;
- `ConfigurationSnapshotUnavailable`;
- `VersionCompatibilityFailure`;
- `IntegrityVerificationFailure`.

## 22.3 Mechanical Failure

### SRE-013-FAILURE-005

Mechanical Failure SHALL include conditions such as:

- `SourceMappingFailure`;
- `RequiredComponentAssemblyFailure`;
- `ReferencePreservationFailure`;
- `IdentityPreservationFailure`;
- `OrderingPreservationFailure`;
- `ConstructionInvariantViolation`;
- `NonDeterministicConstructionRule`;
- `DeterminismFailure`;
- `ManifestCompletenessFailure`;
- `AtomicPublicationFailure`;
- `InternalAssemblyFailure`.

## SRE-013-FAILURE-006

A Construction Failure Record SHALL preserve the attempted activity context without publishing a partial authoritative Constructed Canonical Request or Construction Manifest.

## SRE-013-FAILURE-007

Semantic conflict, normalization failure, ordering failure, structural ineligibility, issuance failure, or downstream rejection SHALL NOT be reclassified as Contract 013 Mechanical Failure.

---

# 23. Completed construction versus failure

## SRE-013-BOUNDARY-001

A successful completed construction activity SHALL publish the atomic artifact-and-manifest set.

## SRE-013-BOUNDARY-002

An unsuccessful activity SHALL publish one Construction Failure Record.

## SRE-013-BOUNDARY-003

A successful activity SHALL NOT publish a Construction Failure Record for the same construction attempt.

## SRE-013-BOUNDARY-004

An invocation rejected before mechanical assembly remains an unsuccessful governed construction activity and MAY be recorded through the same Construction Failure Record publication type using the `InvocationFailure` category.

---

# 24. Atomic commitment

## SRE-013-COMMIT-001

Every terminal Contract 013 activity SHALL commit exactly one authoritative outcome:

```text
ConstructedCanonicalRequest + ConstructionManifest
```

or:

```text
ConstructionFailureRecord
```

## SRE-013-COMMIT-002

A terminal activity SHALL NOT commit both outcomes.

## SRE-013-COMMIT-003

A terminal activity SHALL NOT commit neither outcome.

## SRE-013-COMMIT-004

Successful artifact-and-manifest publication SHALL be atomic at the constitutional level even where an implementation performs multiple physical storage writes.

## SRE-013-COMMIT-005

An implementation SHALL prevent any physically intermediate state from being exposed as successful construction standing.

---

# 25. Immutability and revision

## SRE-013-IMMUTABILITY-001

Committed Construction Decisions, Constructed Canonical Requests, Construction Manifests, and Construction Failure Records SHALL be immutable.

## SRE-013-IMMUTABILITY-002

A later change to any material:

- ordered representation;
- validation result;
- eligibility binding;
- Construction Profile;
- Construction Schema;
- Construction Rule Set;
- Construction Registry;
- configuration snapshot;
- Construction Decision;
- source mapping;
- optional omission;
- artifact component;
- manifest entry;

SHALL require a new Canonical Request Construction activity and new publication identities.

## SRE-013-IMMUTABILITY-003

A later construction activity SHALL NOT overwrite or retroactively revise a prior activity or publication.

## SRE-013-IMMUTABILITY-004

Supersession, where applicable, SHALL be represented through explicit later publications and SHALL NOT mutate historical construction records.

---

# 26. Determinism and replay

## SRE-013-DETERMINISM-001

Equivalent canonically ordered representations processed with equivalent applicable validation results and identical Construction Profiles, Schemas, Rule Sets, Registries, configuration snapshots, contract versions, and material implementation semantics SHALL produce:

- equivalent material Construction Decisions;
- equivalent Constructed Canonical Requests;
- equivalent Construction Manifests.

## SRE-013-DETERMINISM-002

Deterministic equivalence SHALL NOT require identical:

- memory layout;
- database row order;
- implementation language;
- process identity;
- storage location;
- transport encoding;
- non-canonical diagnostics.

## SRE-013-DETERMINISM-003

A conforming implementation SHALL preserve sufficient information to replay every material Construction Decision and reproduce the logical artifact and manifest.

## SRE-013-DETERMINISM-004

Nondeterministic material construction behavior SHALL produce a Construction Failure Record unless an adopted rule provides a deterministic constitutional resolution.

## SRE-013-DETERMINISM-005

Replay SHALL use the exact governing versions originally applied unless an explicitly separate reconstruction activity is initiated.

---

# 27. Serialization boundary

## SRE-013-SERIALIZATION-001

Contract 013 governs logical artifact construction, not transport encoding.

## SRE-013-SERIALIZATION-002

The following distinctions SHALL remain explicit:

```text
ConstructedCanonicalRequest
        ≠
Serialized Request Representation
        ≠
Transport Payload
```

## SRE-013-SERIALIZATION-003

Implementation serialization MAY be used to store or transmit Contract 013 artifacts, provided it does not redefine constitutional content, identity, ordering, or authority.

## SRE-013-SERIALIZATION-004

Canonical byte encoding, signing representation, or wire-level hashing SHALL require explicit constitutional authority and SHALL NOT be inferred from this contract.

---

# 28. Downstream handoff to Contract 014

## SRE-013-HANDOFF-001

A successful Contract 013 publication set MAY become an eligible input to Contract 014.

## SRE-013-HANDOFF-002

Contract 014 SHALL receive or reference:

- exactly one immutable `ConstructedCanonicalRequestId`;
- exactly one bound `ConstructionManifestId`;
- the common `CanonicalRequestConstructionId`;
- applicable lineage and version context.

## SRE-013-HANDOFF-003

Successful Contract 013 construction SHALL authorize only consideration under Contract 014.

## SRE-013-HANDOFF-004

Contract 014 SHALL NOT treat construction success as preexisting issuance standing or as permission for execution or downstream use.

## SRE-013-HANDOFF-005

A Construction Failure Record SHALL NOT become an input for successful issuance.

---

# 29. Neighbor boundaries

## SRE-013-BOUNDARY-005 — Contract 011 boundary

Contract 011 establishes canonical arrangement.

Contract 013 SHALL instantiate that arrangement and SHALL NOT revise it.

## SRE-013-BOUNDARY-006 — Contract 012 boundary

Contract 012 establishes structural eligibility for the exact ordered representation.

Contract 013 SHALL verify binding and applicability only and SHALL NOT independently establish eligibility.

## SRE-013-BOUNDARY-007 — Contract 014 boundary

Contract 013 creates the immutable pre-issuance artifact and its construction account.

Contract 014 assigns issued constitutional request identity and establishes issuance standing.

## SRE-013-BOUNDARY-008 — Contract 015 boundary

Contract 013 SHALL NOT transfer custody or publish a downstream handoff record.

## SRE-013-BOUNDARY-009 — Contract 007 and Contract 008 boundary

Construction Decisions MAY support later evidence or provenance references.

Contract 013 SHALL NOT independently grant evidence standing or replace provenance authority.

---

# 30. Deferred responsibilities

## SRE-013-DEFER-001

Contract 013 explicitly defers:

- issued request identity and issuance standing to Contract 014;
- custody transfer and handoff recording to Contract 015;
- canonical transport encoding to an expressly authorized future contract or implementation layer;
- signing and canonical byte hashing to an expressly authorized authority;
- provider selection and binding to downstream governance;
- planning, execution, generation, release, and tool invocation to lawful downstream authorities;
- policy, safety, truth, feasibility, and authorization determinations to their governing systems.

## SRE-013-DEFER-002

No implementation convenience SHALL collapse these deferred responsibilities into Contract 013 authority.

---

# 31. Security and integrity considerations

## SRE-013-SECURITY-001

Construction Profiles, Schemas, Rule Sets, Registries, configuration snapshots, input artifacts, decisions, manifests, artifacts, and failure records SHALL be integrity-protected.

## SRE-013-SECURITY-002

A conforming implementation SHALL prevent unauthorized substitution of:

- ordered representations;
- validation publications;
- profiles;
- schemas;
- rules;
- registries;
- configuration snapshots;
- source mappings;
- Construction Decisions;
- artifact or manifest identities.

## SRE-013-SECURITY-003

Any substitution capable of changing material construction outcome SHALL be observable and SHALL require a new construction activity.

## SRE-013-SECURITY-004

Contract 013 SHALL not be used as a covert channel for semantic invention, policy adjudication, authorization, provider preference, execution routing, or hidden data injection.

## SRE-013-SECURITY-005

An implementation SHALL not expose a partially written artifact or manifest as a successfully published constitutional result.

---

# 32. Conformance requirements

## SRE-013-CONFORM-001

An implementation conforms to this contract only if it:

1. consumes exactly one immutable canonically ordered representation;
2. consumes exactly one applicable completed Structural Validation Result bound to that exact representation;
3. permits construction only under an eligible state recognized by the applicable construction context;
4. performs eligibility binding verification without revalidation;
5. binds all material profiles, schemas, rule sets, registries, configuration, contract, and implementation versions;
6. creates only mechanically governed Construction Decisions;
7. preserves upstream meaning, expression, identity, order, standing, evidence, provenance, and validation lineage;
8. performs no hidden repair, substitution, reordering, normalization, or semantic invention;
9. constructs one complete profile-conforming logical artifact;
10. publishes one complete bound Construction Manifest;
11. records every material Construction Decision necessary for explanation and replay;
12. records every authorized optional omission;
13. atomically commits the artifact-and-manifest publication set;
14. commits one Construction Failure Record when construction cannot lawfully complete;
15. keeps success and failure mutually exclusive;
16. preserves artifact identity separately from issued request identity;
17. remains serialization-independent;
18. supports deterministic replay and immutable historical records;
19. transfers only the successful publication set to Contract 014.

## SRE-013-CONFORM-002

An implementation SHALL be non-conforming if it:

- constructs from a validation result without the actual ordered representation;
- constructs from an alternative or merely equivalent representation;
- re-runs or revises structural validation;
- reorders content during assembly;
- regenerates upstream identities without authority;
- silently drops evidence, provenance, or lineage references;
- publishes an artifact without its manifest;
- publishes a manifest without its artifact;
- treats optional omission as permission to omit required content;
- publishes a partial artifact as a successful Constructed Canonical Request;
- grants evidence standing to Construction Decisions under Contract 013;
- assigns issued constitutional request identity;
- equates artifact construction with authorization or execution permission;
- defines the artifact by a particular wire format;
- mutates committed decisions, artifacts, manifests, or failure records;
- permits undeclared inputs to influence construction.

---

# 33. Normative dependencies and downstream consumers

## SRE-013-DEPENDENCY-001

Contract 013 normatively depends upon:

- Contract 000 — Architecture, Identity, and Authority;
- Contract 007 — Evidence Representation, for preserved evidence references;
- Contract 008 — Provenance Representation, for preserved provenance references;
- Contract 009 — Semantic Reconciliation, through preserved downstream standing;
- Contract 010 — Semantic Normalization, through preserved canonical expression;
- Contract 011 — Canonical Ordering, as the source of the ordered representation;
- Contract 012 — Structural Validation, as the exclusive source of construction eligibility.

## SRE-013-DEPENDENCY-002

Contract 013 SHALL NOT reinterpret responsibilities owned by its normative dependencies.

## SRE-013-CONSUMER-001

The principal downstream consumer is Contract 014 — Identity and Issuance.

## SRE-013-CONSUMER-002

Additional systems MAY inspect Contract 013 artifacts for audit, replay, evidence, provenance, testing, or assurance purposes, but such inspection SHALL NOT create issuance or operational authority.

---

# 34. Expected implementation responsibility

## SRE-013-IMPLEMENTATION-001

A conforming implementation is expected to provide:

- exact representation-and-validation binding verification;
- profile, schema, rule-set, registry, and configuration resolution;
- deterministic logical artifact builders;
- immutable Construction Decision capture;
- complete source-to-artifact mapping;
- manifest generation and completeness checks;
- atomic artifact-and-manifest publication semantics;
- immutable failure publication with three-part taxonomy;
- deterministic replay tests;
- authority-boundary tests proving no revalidation, reordering, issuance, serialization authority, or downstream authorization.

## SRE-013-IMPLEMENTATION-002

Implementation decomposition MAY use shared crates, modules, services, stores, or processes, provided constitutional roles, inputs, outputs, identities, atomicity, and prohibited authority remain explicit and independently testable.

---

# 35. Fundamental canonical request construction invariants

## SRE-013-INVARIANT-001

Every successful Canonical Request Construction activity SHALL:

- consume exactly one identified `CanonicallyOrderedRequestRepresentation`;
- consume exactly one applicable completed `StructuralValidationResult` governing that exact representation;
- verify eligibility binding without revalidation;
- apply only declared construction profiles, schemas, rules, registries, and configuration;
- record every material mechanical determination through immutable Construction Decisions;
- preserve exact upstream meaning, canonical expression, identities, arrangement, standing, evidence, provenance, and validation lineage;
- instantiate one complete profile-conforming logical artifact;
- produce one complete construction account;
- atomically publish exactly one immutable `ConstructedCanonicalRequest` and exactly one bound immutable `ConstructionManifest`;
- preserve the distinction between constructed artifact identity and issued request identity;

while creating no new semantic meaning, no normalization, no ordering, no structural eligibility, no issued request identity, no issuance standing, no transport payload authority, no downstream custody, and no execution authorization.

## SRE-013-INVARIANT-002

Every unsuccessful Canonical Request Construction activity SHALL:

- preserve the attempted input and construction context;
- classify failure as Invocation Failure, Resolution Failure, or Mechanical Failure;
- identify why lawful construction could not complete;
- commit exactly one immutable `ConstructionFailureRecord`;

while publishing no authoritative partial Constructed Canonical Request, no authoritative partial Construction Manifest, no issuance standing, and no downstream authorization.

## SRE-013-INVARIANT-003

A successful construction publication SHALL never exist constitutionally as an artifact without its bound manifest or as a manifest without its bound artifact.

---

# 36. Adoption and review status

## SRE-013-STATUS-001

The architecture of Contract 013 is frozen for normative drafting.

## SRE-013-STATUS-002

This v0.1.0 Draft incorporates the adopted architectural closures concerning:

- dual-input binding;
- abstract logical artifact construction;
- mechanical decision traceability;
- artifact-and-manifest publication-set atomicity;
- manifest non-semantic status;
- exact representation preservation;
- identity separation;
- failure taxonomy;
- construction downstream of structural validation;
- serialization exclusion;
- immutable correction through reconstruction.

## SRE-013-STATUS-003

Future changes that alter the governing question, constitutional subject, constitutional act, authority boundary, publication model, identity separation, or neighboring contract responsibilities SHALL require formal architectural amendment rather than editorial revision.
