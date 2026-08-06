# Structured Request Engine

## Contract 014 — Canonical Request Identity and Issuance

**Version:** v0.1.0 Draft  
**Architectural status:** Frozen  
**Contract family:** Issuance and Handoff Family  
**Constitutional act:** Issue

---

## Normative requirement identifiers

Normative requirements in this contract use the prefix:

```text
SRE-014-
```

The key words **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **MAY**, and **MUST** are to be interpreted as normative requirement terms.

---

# 1. Architectural context

## SRE-014-CONTEXT-001

Contract 014 governs the constitutional transition by which one immutable Construction Publication Set acquires deterministic issued-request identity and initial constitutional standing.

## SRE-014-CONTEXT-002

Contract 014 is downstream of Contract 013 — Canonical Request Construction.

```text
Construction Publication Set
        ├── ConstructedCanonicalRequest
        └── ConstructionManifest
                    +
        Applicable Issuance Governance
                    ↓
        Canonical Request Identity and Issuance
                    ↓
        Issuance Publication Set
        ├── CanonicalStructuredRequest
        └── IssuanceManifest
```

## SRE-014-CONTEXT-003

Contract 014 SHALL remain constitutionally distinct from:

- semantic interpretation and proposal formation under Contracts 001–008;
- semantic reconciliation under Contract 009;
- semantic normalization under Contract 010;
- canonical ordering under Contract 011;
- structural validation under Contract 012;
- canonical request construction under Contract 013;
- downstream custody transfer under Contract 015;
- provider selection, routing, planning, execution, generation, release, and tool invocation;
- transport serialization, storage encoding, and wire-format publication.

## SRE-014-CONTEXT-004

The terminal Structured Request Engine lifecycle boundary SHALL be understood as:

```text
013 Canonical Request Construction
    creates one immutable Construction Publication Set
        ↓
014 Canonical Request Identity and Issuance
    assigns issued identity and initial constitutional standing
        ↓
015 Downstream Handoff
    transfers custody or responsibility to a declared downstream authority
```

## SRE-014-CONTEXT-005

The constitutional distinctions SHALL remain:

```text
Artifact existence
        ≠
Issued standing
        ≠
Transferred custody
        ≠
Execution authorization
```

---

# 2. Purpose

## SRE-014-PURPOSE-001

The purpose of Contract 014 is to define how one exact immutable Construction Publication Set receives deterministic issued identity and formal initial standing as a `CanonicalStructuredRequest` without modifying request content or granting downstream authority.

## SRE-014-PURPOSE-002

Contract 014 SHALL establish:

- the exact binding between issuance and one Construction Publication Set;
- the Issuance Context within which identity, uniqueness, replay, and standing are determined;
- the authority and limitations of Canonical Request Issuance;
- deterministic identity derivation under an applicable Identity Policy;
- the distinction among artifact identity, issuance-activity identity, issued-request identity, manifest identity, and later handoff identity;
- context-distinct multiple issuance;
- same-context uniqueness and idempotent replay by default;
- initial standing assignment;
- atomic publication of the `CanonicalStructuredRequest` and `IssuanceManifest`;
- immutable failure representation;
- the boundary between issuance and downstream handoff.

## SRE-014-PURPOSE-003

Contract 014 SHALL NOT establish:

- semantic meaning or interpretation;
- request content, normalization, ordering, or validation;
- construction eligibility or construction correctness;
- execution, provider, tool, routing, or resource authority;
- recipient selection or custody transfer;
- later revocation, suspension, expiration, withdrawal, supersession, replacement, renewal, or correction of an issued request;
- transport or storage serialization as constitutional request meaning.

---

# 3. Governing constitutional elements

## SRE-014-GOVERNING-001 — Governing question

> **How does one immutable constructed canonical request acquire deterministic issued identity and formal constitutional standing?**

## SRE-014-GOVERNING-002 — Constitutional subject

The constitutional subject is:

> **The issued identity and initial constitutional standing assigned to one immutable Construction Publication Set.**

## SRE-014-GOVERNING-003 — Constitutional act

The constitutional act is:

```text
Issue
```

## SRE-014-GOVERNING-004 — Organizing doctrine

> **Issuance grants constitutional standing. It does not alter constitutional content.**

## SRE-014-GOVERNING-005 — Binding doctrine

> **A CanonicalStructuredRequest SHALL establish constitutional identity and standing by immutable reference to exactly one Construction Publication Set.**

## SRE-014-GOVERNING-006 — Context doctrine

> **Issued identity, uniqueness, replay, and initial standing SHALL be determined within a governed Issuance Context.**

## SRE-014-GOVERNING-007 — Publication doctrine

Successful issuance SHALL produce one constitutionally inseparable Issuance Publication Set containing:

```text
CanonicalStructuredRequest
IssuanceManifest
```

The set is a constitutional grouping and SHALL NOT require an independent wrapper identity unless a later adopted contract expressly requires one.

---

# 4. Architectural identity

## SRE-014-IDENTITY-001

Contract 014 is the identity-and-initial-standing contract of the Structured Request Engine.

## SRE-014-IDENTITY-002

Contract 014 consumes the exact immutable Construction Publication Set produced by Contract 013.

## SRE-014-IDENTITY-003

The constitutional relationship SHALL be:

```text
Construction supplies the immutable artifact and construction account.
Issuance supplies issued identity and initial standing.
Handoff supplies downstream custody transition.
Authorization, if any, is supplied elsewhere.
```

## SRE-014-IDENTITY-004

A `CanonicalStructuredRequest` SHALL be an immutable issuance-layer publication bound to one exact Construction Publication Set.

## SRE-014-IDENTITY-005

A `CanonicalStructuredRequest` SHALL NOT be a copied, regenerated, reconstructed, substituted, mutated, or merely equivalent replacement for the `ConstructedCanonicalRequest`.

## SRE-014-IDENTITY-006

A `CanonicalStructuredRequest` SHALL NOT independently duplicate the complete request body unless an adopted representation standard requires embedded representation while preserving exact upstream identity and non-substitution.

## SRE-014-IDENTITY-007

A successful issuance SHALL establish initial constitutional request standing only.

---

# 5. Organizing constitutional doctrines

## 5.1 Construction is not issuance

### SRE-014-DOCTRINE-001

Completion of Contract 013 construction SHALL NOT, by itself, establish issued-request identity or standing.

```text
ConstructedCanonicalRequest
        ≠
CanonicalStructuredRequest
```

## 5.2 Exact publication-set binding

### SRE-014-DOCTRINE-002

Issuance SHALL reference the exact immutable Construction Publication Set presented to the issuance activity.

It SHALL NOT issue a copied, regenerated, reconstructed, substituted, embedded-as-replacement, or merely equivalent artifact.

## 5.3 No content modification

### SRE-014-DOCTRINE-003

Issuance SHALL NOT alter, reconcile, normalize, reorder, validate, supplement, repair, reinterpret, or reconstruct request content.

## 5.4 Publication-set integrity verification is not revalidation

### SRE-014-DOCTRINE-004

Contract 014 MAY verify the identity, completeness, mutual binding, version compatibility, applicability, and integrity of the Construction Publication Set.

It SHALL NOT repeat Contract 012 structural validation or Contract 013 construction invariant evaluation.

## 5.5 Issuance Context doctrine

### SRE-014-DOCTRINE-005

An `IssuanceContext` SHALL be the governed constitutional scope within which one Construction Publication Set is evaluated for issuance, identity derivation, uniqueness, replay, and initial standing.

## 5.6 Context-distinct multiple issuance

### SRE-014-DOCTRINE-006

One Construction Publication Set MAY support zero or more issuances only where each issuance possesses a constitutionally distinct Issuance Context.

## 5.7 Same-context uniqueness

### SRE-014-DOCTRINE-007

The same Construction Publication Set SHALL NOT receive more than one unrelated initial issuance within the same Issuance Context.

## 5.8 Idempotent replay default

### SRE-014-DOCTRINE-008

Equivalent issuance attempts involving the same Construction Publication Set, Issuance Context, governing Identity Policy, Issuance Profile, rule set, registries, and version snapshot SHALL be idempotent by default.

## 5.9 Event-unique issuance exception

### SRE-014-DOCTRINE-009

Event-unique issuance within one Issuance Context MAY occur only where the applicable Identity Policy explicitly authorizes it and defines:

- the authorized basis;
- the identity-bearing event input;
- the sequence scope;
- the uniqueness rule;
- the relationship to any earlier issuance;
- the resulting standing.

## 5.10 Timestamp limitation

### SRE-014-DOCTRINE-010

An Issuance Timestamp SHALL record when initial standing was established.

It SHALL NOT contribute to issued identity unless the applicable Identity Policy explicitly declares it identity-bearing.

## 5.11 Sequence scope doctrine

### SRE-014-DOCTRINE-011

An Issuance Sequence SHALL NOT be constitutionally meaningful unless its namespace, authority, scope, value, and allocation policy are declared.

## 5.12 Identity separation

### SRE-014-DOCTRINE-012

Artifact identity, construction-manifest identity, issuance-activity identity, issuance-decision identity, issued-request identity, issuance-manifest identity, failure-record identity, and later handoff identity SHALL remain distinct.

## 5.13 No execution authority

### SRE-014-DOCTRINE-013

Issuance SHALL NOT authorize execution, provider action, tool invocation, resource consumption, generation, release, or downstream processing.

## 5.14 No transfer

### SRE-014-DOCTRINE-014

Issuance SHALL NOT select a recipient, establish recipient custody, or complete downstream handoff.

## 5.15 Initial-standing limitation

### SRE-014-DOCTRINE-015

Contract 014 SHALL establish initial standing only.

It SHALL NOT revoke, suspend, expire, withdraw, supersede, replace, renew, or correct an already issued request.

## 5.16 Manifest non-semantic status

### SRE-014-DOCTRINE-016

The `IssuanceManifest` SHALL explain the issuance activity, identity derivation, and standing assignment.

It SHALL NOT contribute semantic request content.

## 5.17 Atomic publication

### SRE-014-DOCTRINE-017

Successful issuance SHALL atomically publish exactly one immutable `CanonicalStructuredRequest` and exactly one immutable bound `IssuanceManifest`.

Neither publication SHALL independently possess successful issuance standing without the other.

## 5.18 No hidden identity inputs

### SRE-014-DOCTRINE-018

Identity derivation SHALL NOT depend on undeclared runtime state, wall-clock values, processing nodes, storage locations, implementation-specific diagnostics, or hidden configuration.

---

# 6. Constitutional authority

## 6.1 Canonical Request Issuance Authority

### SRE-014-AUTHORITY-001

A conforming implementation SHALL recognize one bounded `CanonicalRequestIssuanceAuthority` for each issuance activity.

### SRE-014-AUTHORITY-002

The Canonical Request Issuance Authority MAY:

- receive one exact Construction Publication Set;
- verify publication-set identity, completeness, binding, integrity, applicability, and version compatibility;
- resolve the applicable Issuance Profile, Identity Policy, issuance rules, registries, and version snapshot;
- establish the applicable Issuance Context from authoritative declared inputs;
- determine same-context duplicate or replay status;
- allocate a governed issuance sequence where required;
- derive issued-request identity;
- assign an initial standing class supplied or permitted by applicable governance;
- produce Issuance Decisions;
- atomically publish the Issuance Publication Set;
- produce an `IssuanceFailureRecord` where issuance cannot complete.

### SRE-014-AUTHORITY-003

The Canonical Request Issuance Authority SHALL NOT:

- alter request content;
- reinterpret purpose from request content;
- reconcile, normalize, order, validate, or reconstruct;
- waive Contract 013 publication requirements;
- select providers, tools, routes, recipients, or execution strategies;
- grant execution or use authority;
- transfer custody;
- alter an already issued publication;
- infer undeclared issuance context from semantic content.

## 6.2 Exclusive publication authority

### SRE-014-AUTHORITY-004

Only a conforming Canonical Request Issuance Authority MAY publish a `CanonicalStructuredRequest` under this contract.

### SRE-014-AUTHORITY-005

A downstream system SHALL NOT self-declare a constructed artifact to possess issued standing in the absence of the complete Issuance Publication Set.

---

# 7. Major concept and artifact classification

| Concept or artifact | Classification | Constitutional role |
|---|---|---|
| `ConstructionPublicationSet` | Upstream constitutional publication set | Exact immutable artifact and construction account presented for issuance |
| `CanonicalRequestIssuance` | Constitutional activity | Governed act of assigning issued identity and initial standing |
| `IssuanceContext` | Governed constitutional scope | Defines uniqueness, replay, policy application, and standing scope |
| `IssuanceDecision` | Constitutional decision record | Records one bounded issuance determination |
| `CanonicalStructuredRequest` | Principal successful publication | Issuance-layer publication with issued identity and standing |
| `IssuanceManifest` | Mandatory supporting publication | Immutable account of issuance inputs, policies, decisions, derivation, and standing |
| `IssuanceFailureRecord` | Failure publication | Immutable account of issuance non-completion |
| `IssuanceProfile` | Referenced governance object | Supplies admissibility and initial-standing requirements |
| `IdentityPolicy` | Referenced governance object | Supplies identity-bearing inputs and derivation semantics |
| `IssuanceRuleSet` | Referenced governance object | Supplies issuance-specific constitutional rules |
| `IssuanceSequence` | Governed allocation object | Supplies a scoped value where identity or audit ordering requires one |

## SRE-014-CLASSIFICATION-001

Activity, context, decision, publication, governance object, and failure record SHALL remain distinct classifications.

## SRE-014-CLASSIFICATION-002

An Issuance Profile or Identity Policy SHALL NOT be represented as though it were created by the issuance activity unless a separate adopted authority expressly assigns that creation responsibility.

## SRE-014-CLASSIFICATION-003

The Issuance Publication Set SHALL reference the Construction Publication Set as a whole through an unambiguous binding to both constituent publications or through an adopted publication-set identity mechanism.

---

# 8. Canonical inputs

## SRE-014-INPUT-001

A Canonical Request Issuance activity SHALL receive exactly one Construction Publication Set containing:

```text
ConstructedCanonicalRequest
ConstructionManifest
```

## SRE-014-INPUT-002

The Construction Publication Set SHALL be complete, immutable, mutually bound, and attributable to one successful Contract 013 construction activity.

## SRE-014-INPUT-003

The issuance activity SHALL receive or resolve authoritative references to:

- an applicable `IssuanceProfile`;
- an applicable `IdentityPolicy`;
- applicable issuance rules;
- applicable contract versions;
- applicable registry versions;
- the issuing constitutional authority;
- the issuance namespace;
- any declared issuance purpose or scope classification;
- any required sequence allocation policy;
- any required initial-standing class.

## SRE-014-INPUT-004

Issuance-purpose or scope values SHALL be supplied from an authoritative declared classification or registry.

Contract 014 SHALL NOT infer issuance purpose from request content.

## SRE-014-INPUT-005

Any identity-bearing input SHALL be explicitly declared by the applicable Identity Policy.

## SRE-014-INPUT-006

Any optional input omitted from identity derivation SHALL remain non-identity-bearing unless the applicable Identity Policy expressly states otherwise.

## SRE-014-INPUT-007

A runtime implementation MAY receive transport or storage metadata, but such metadata SHALL NOT acquire constitutional meaning merely because it is present.

---

# 9. Issuance Context

## SRE-014-CONTEXT-MODEL-001

An `IssuanceContext` SHALL identify the constitutional scope in which issuance occurs.

## SRE-014-CONTEXT-MODEL-002

At minimum, an Issuance Context SHALL bind:

```text
ConstructionPublicationSetReference
IssuanceAuthorityReference
IssuanceNamespace
IssuanceProfileReference
IssuancePurposeOrScope
```

## SRE-014-CONTEXT-MODEL-003

An Issuance Context MAY additionally bind:

- jurisdiction;
- program or organizational domain;
- standing class;
- identity-policy family;
- registry domain;
- temporal validity class;
- other governed context dimensions expressly declared by the Issuance Profile.

## SRE-014-CONTEXT-MODEL-004

No context dimension SHALL be inferred from request semantics unless an upstream constitutional artifact already declares that dimension and the Issuance Profile authorizes its use.

## SRE-014-CONTEXT-MODEL-005

Two issuances SHALL be context-distinct only where at least one identity- or uniqueness-relevant context dimension differs under applicable governance.

## SRE-014-CONTEXT-MODEL-006

Cosmetic metadata, runtime location, processing node, retry count, log correlation identifier, or storage path SHALL NOT create a distinct Issuance Context.

---

# 10. Canonical Request Issuance activity

## SRE-014-OPERATION-001

Each invocation SHALL create or identify one `CanonicalRequestIssuance` activity.

## SRE-014-OPERATION-002

The activity SHALL proceed through the following bounded sequence:

```text
Receive exact Construction Publication Set
        ↓
Verify publication-set binding and applicability
        ↓
Resolve Issuance Context
        ↓
Resolve Identity Policy and Issuance Profile
        ↓
Determine duplicate or replay status
        ↓
Allocate scoped sequence where required
        ↓
Derive issued-request identity
        ↓
Assign initial standing
        ↓
Record Issuance Decision(s)
        ↓
Atomically publish Issuance Publication Set
```

## SRE-014-OPERATION-003

A conforming implementation MAY combine mechanical steps operationally, but it SHALL preserve each constitutional distinction and produce sufficient records to demonstrate conformance.

## SRE-014-OPERATION-004

The activity SHALL terminate in exactly one constitutional outcome:

```text
Issued
or
Failed
```

## SRE-014-OPERATION-005

An idempotent replay that resolves to an existing valid Issuance Publication Set SHALL NOT be treated as a second unrelated initial issuance.

---

# 11. Runtime object model

## SRE-014-OBJECT-001

The normative runtime model SHALL include:

```text
CanonicalRequestIssuance
IssuanceContext
IssuanceDecision
CanonicalStructuredRequest
IssuanceManifest
IssuanceFailureRecord
```

## 11.1 Canonical Request Issuance

### SRE-014-OBJECT-002

A `CanonicalRequestIssuance` SHALL represent one governed attempt to assign issued identity and initial standing to one Construction Publication Set within one Issuance Context.

## 11.2 Issuance Context

### SRE-014-OBJECT-003

An `IssuanceContext` SHALL be immutable for the duration of one issuance activity.

### SRE-014-OBJECT-004

Changing a context-defining input SHALL create a different Issuance Context rather than silently mutating the existing one.

## 11.3 Issuance Decision

### SRE-014-OBJECT-005

An `IssuanceDecision` SHALL record one bounded determination concerning:

- publication-set admissibility;
- context resolution;
- identity-policy selection;
- namespace selection;
- duplicate or replay status;
- sequence allocation;
- identity derivation;
- initial-standing assignment;
- publication authorization under this contract.

### SRE-014-OBJECT-006

An Issuance Decision SHALL NOT determine semantic content, structural eligibility, construction correctness, execution authority, recipient selection, or custody.

### SRE-014-OBJECT-007

Multiple Issuance Decisions MAY exist within one issuance activity where each decision has a distinct bounded subject and all are referenced by the Issuance Manifest.

## 11.4 Canonical Structured Request

### SRE-014-OBJECT-008

A `CanonicalStructuredRequest` SHALL be the principal successful publication of Contract 014.

### SRE-014-OBJECT-009

It SHALL reference exactly one Construction Publication Set and exactly one Canonical Request Issuance activity.

### SRE-014-OBJECT-010

It SHALL carry issued-request identity and initial standing without modifying the referenced construction publications.

## 11.5 Issuance Manifest

### SRE-014-OBJECT-011

An `IssuanceManifest` SHALL be the mandatory immutable account of one successful issuance.

### SRE-014-OBJECT-012

The Issuance Manifest SHALL bind all identity-bearing inputs, governance references, decisions, derivation results, standing, sequence information, and timestamps required to audit the issuance.

## 11.6 Issuance Failure Record

### SRE-014-OBJECT-013

An `IssuanceFailureRecord` SHALL be the sole terminal publication for a failed issuance activity.

### SRE-014-OBJECT-014

A failure record SHALL NOT imply that an issued request exists.

---

# 12. Identity doctrine

## SRE-014-ID-001

The following identities SHALL remain distinct:

```text
ConstructedCanonicalRequestId
ConstructionManifestId
CanonicalRequestIssuanceId
IssuanceContextId or equivalent governed context identity
IssuanceDecisionId
CanonicalStructuredRequestId
IssuanceManifestId
IssuanceFailureRecordId
```

## SRE-014-ID-002

`ConstructedCanonicalRequestId` SHALL identify the immutable constructed artifact.

## SRE-014-ID-003

`ConstructionManifestId` SHALL identify the construction account.

## SRE-014-ID-004

`CanonicalRequestIssuanceId` SHALL identify the governed issuance activity.

## SRE-014-ID-005

`CanonicalStructuredRequestId` SHALL identify the issued constitutional request within its governed identity semantics.

## SRE-014-ID-006

`IssuanceManifestId` SHALL identify the issuance account.

## SRE-014-ID-007

No downstream identity SHALL replace, mutate, obscure, or collapse an upstream identity.

## SRE-014-ID-008

An identity SHALL identify one constitutional object or act and SHALL NOT be reused to identify a different constitutional category.

---

# 13. Identity Policy

## SRE-014-POLICY-001

Each issuance SHALL resolve exactly one applicable `IdentityPolicy` version.

## SRE-014-POLICY-002

The Identity Policy SHALL declare:

- the identity namespace;
- the identity-bearing input tuple;
- the derivation algorithm or algorithm identifier;
- the canonical encoding used for identity derivation, where applicable;
- replay behavior;
- collision handling;
- timestamp participation;
- sequence participation;
- version-binding rules;
- any event-unique issuance authorization;
- prohibited or ignored metadata fields.

## SRE-014-POLICY-003

A suitable identity-bearing tuple MAY include:

```text
IssuanceNamespace
IssuanceAuthorityReference
ConstructionPublicationSetReference
IssuanceProfileReference
IssuanceContext
IssuanceSequenceScope
IdentityPolicyVersion
```

## SRE-014-POLICY-004

The Identity Policy SHALL distinguish mandatory identity-bearing fields from non-identity-bearing publication metadata.

## SRE-014-POLICY-005

An implementation SHALL NOT silently promote a metadata field into an identity-bearing field.

## SRE-014-POLICY-006

Equivalent identity-bearing inputs under the same Identity Policy version SHALL produce the same identity result unless event-unique issuance is explicitly authorized.

---

# 14. Issuance Profile

## SRE-014-PROFILE-001

Each issuance SHALL resolve exactly one applicable `IssuanceProfile` version.

## SRE-014-PROFILE-002

The Issuance Profile SHALL declare or constrain:

- admissible Construction Publication Set classes;
- required context dimensions;
- permitted issuance authorities;
- applicable namespaces;
- permitted initial-standing classes;
- required registry and contract versions;
- duplicate and replay rules not delegated exclusively to the Identity Policy;
- required sequence scope, if any;
- required manifest fields;
- issuance-specific failure conditions.

## SRE-014-PROFILE-003

The Issuance Profile SHALL NOT redefine request semantics or construction content.

## SRE-014-PROFILE-004

A profile version change SHALL be constitutionally visible in the Issuance Manifest and SHALL affect identity only where the Identity Policy declares it identity-bearing.

---

# 15. Issuance rules and registries

## SRE-014-RULE-001

All issuance rules and registries used by an issuance activity SHALL be explicitly versioned or otherwise immutably identifiable.

## SRE-014-RULE-002

Applicable registries MAY include:

- issuance-authority registries;
- namespace registries;
- standing-class registries;
- issuance-purpose or scope registries;
- identity-algorithm registries;
- sequence-allocation registries;
- contract-version registries;
- profile registries.

## SRE-014-RULE-003

Registry resolution SHALL NOT infer or create constitutional authority that is absent from the registry or governing contract.

## SRE-014-RULE-004

Failure to resolve a required rule set or registry SHALL prevent successful issuance.

## SRE-014-RULE-005

A registry update SHALL NOT retroactively mutate an already published Issuance Publication Set.

---

# 16. Construction Publication Set binding verification

## SRE-014-BINDING-001

Before identity derivation, Contract 014 SHALL verify that the supplied `ConstructedCanonicalRequest` and `ConstructionManifest`:

- both exist;
- are immutable;
- identify one another or are bound by an adopted equivalent mechanism;
- arise from the same successful construction activity;
- have not been substituted;
- are complete under Contract 013;
- possess compatible contract, profile, schema, registry, and configuration references;
- are admissible under the applicable Issuance Profile.

## SRE-014-BINDING-002

Binding verification SHALL operate on identity, integrity, completeness, applicability, and version compatibility only.

## SRE-014-BINDING-003

Binding verification SHALL NOT repeat semantic, ordering, structural-validation, or construction-invariant determinations.

## SRE-014-BINDING-004

A mismatch between the constructed artifact and Construction Manifest SHALL produce failure and SHALL NOT be repaired by Contract 014.

---

# 17. Duplicate, replay, and reissuance determination

## SRE-014-REPLAY-001

Before publication, the issuance activity SHALL determine whether the same Construction Publication Set has already received an issuance within the same Issuance Context.

## SRE-014-REPLAY-002

Where a valid matching Issuance Publication Set already exists and all identity-bearing inputs match, the activity SHALL resolve as idempotent replay unless event-unique issuance is explicitly authorized.

## SRE-014-REPLAY-003

Idempotent replay SHALL return or reference the existing issued identity and publication set without creating a second unrelated initial issuance.

## SRE-014-REPLAY-004

A same-context issuance attempt with conflicting identity-bearing inputs SHALL fail as a duplicate issuance conflict or policy conflict.

## SRE-014-REPLAY-005

Cross-context issuance MAY create a distinct `CanonicalStructuredRequestId` where the applicable Identity Policy defines the context difference as identity-bearing.

## SRE-014-REPLAY-006

Contract 014 SHALL NOT independently authorize replacement, renewal, correction, supersession, or lifecycle reissuance of an already issued request.

## SRE-014-REPLAY-007

Any authorized reissuance mechanism SHALL be supplied by separately governed authority or an expressly adopted Identity Policy and SHALL remain distinguishable from ordinary initial issuance.

---

# 18. Issuance Sequence

## SRE-014-SEQUENCE-001

Where an Issuance Sequence is required, it SHALL identify:

```text
SequenceNamespace
SequenceAuthority
SequenceScope
SequenceValue
AllocationPolicy
```

## SRE-014-SEQUENCE-002

Permitted sequence scopes MAY include:

- authority-wide;
- namespace-wide;
- Construction Publication Set-specific;
- profile-specific;
- Issuance Context-specific.

## SRE-014-SEQUENCE-003

A sequence value SHALL be identity-bearing only where the Identity Policy explicitly declares it so.

## SRE-014-SEQUENCE-004

Sequence allocation SHALL be deterministic or uniquely governed under its allocation policy.

## SRE-014-SEQUENCE-005

Failure to allocate a required sequence SHALL prevent issuance.

---

# 19. Identity derivation

## SRE-014-DERIVATION-001

Identity derivation SHALL use only the identity-bearing inputs declared by the applicable Identity Policy.

## SRE-014-DERIVATION-002

The derivation process SHALL be deterministic for equivalent identity-bearing inputs unless event-unique issuance is expressly authorized.

## SRE-014-DERIVATION-003

Any canonical serialization used solely for identity calculation SHALL be governed, versioned, deterministic, and non-semantic.

## SRE-014-DERIVATION-004

Identity-calculation serialization SHALL NOT become the constitutional transport format of the issued request merely because it is used in derivation.

## SRE-014-DERIVATION-005

Identity collisions SHALL fail closed unless the Identity Policy defines a deterministic collision-resolution mechanism that preserves uniqueness and auditability.

## SRE-014-DERIVATION-006

The complete derivation basis and result SHALL be represented in the Issuance Manifest directly or through immutable references sufficient for independent verification.

---

# 20. Initial standing assignment

## SRE-014-STANDING-001

A successful issuance SHALL assign exactly one initial constitutional standing to the `CanonicalStructuredRequest`.

## SRE-014-STANDING-002

The standing class SHALL be supplied or permitted by the Issuance Profile and applicable standing registry.

## SRE-014-STANDING-003

Initial standing MAY include an initial validity period where supplied by governing issuance rules.

## SRE-014-STANDING-004

Contract 014 SHALL record initial standing but SHALL NOT later modify it.

## SRE-014-STANDING-005

Issued standing SHALL mean only that the exact bound Construction Publication Set is officially recognized as a Canonical Structured Request within the applicable Issuance Context.

## SRE-014-STANDING-006

Issued standing SHALL NOT imply:

- permission to execute;
- acceptance by a recipient;
- legal or operational approval;
- resource allocation;
- provider binding;
- tool permission;
- successful handoff.

---

# 21. Canonical Structured Request requirements

## SRE-014-ARTIFACT-001

A `CanonicalStructuredRequest` SHALL contain or immutably reference at least:

```text
CanonicalStructuredRequestId
ConstructionPublicationSetReference
ConstructedCanonicalRequestReference
ConstructionManifestReference
CanonicalRequestIssuanceReference
IssuanceManifestReference
IssuanceContextReference
IssuanceProfileReference
IdentityPolicyReference
IssuanceAuthorityReference
IssuanceNamespace
IssuanceTimestamp
IssuanceSequenceReference, where applicable
InitialConstitutionalStanding
ContractVersionSnapshot
RegistryVersionSnapshot
```

## SRE-014-ARTIFACT-002

The `CanonicalStructuredRequest` SHALL NOT contain a second authoritative request body that can diverge from the referenced `ConstructedCanonicalRequest`.

## SRE-014-ARTIFACT-003

Any embedded view of request content SHALL be derivative, integrity-bound, and explicitly non-substitutive.

## SRE-014-ARTIFACT-004

The `CanonicalStructuredRequest` SHALL be immutable after successful publication.

---

# 22. Issuance Manifest requirements

## SRE-014-MANIFEST-001

The `IssuanceManifest` SHALL contain or immutably reference at least:

```text
IssuanceManifestId
CanonicalRequestIssuanceId
CanonicalStructuredRequestId
ConstructionPublicationSetReference
ConstructedCanonicalRequestId
ConstructionManifestId
IssuanceContext
IssuanceAuthorityReference
IssuanceProfileReference and version
IdentityPolicyReference and version
IssuanceRuleSet references and versions
Registry version snapshot
Identity-bearing inputs
Identity derivation algorithm reference
Identity derivation result
Duplicate or replay determination
Issuance sequence and scope, where applicable
Issuance timestamp
Initial standing assigned
Issuance Decision references
Publication commitment information
```

## SRE-014-MANIFEST-002

The manifest SHALL provide sufficient information to determine why the issued identity and standing were assigned.

## SRE-014-MANIFEST-003

The manifest SHALL NOT contribute semantic request content.

## SRE-014-MANIFEST-004

The manifest SHALL be immutable and inseparable from the successful issued request publication.

## SRE-014-MANIFEST-005

An Issuance Manifest SHALL NOT exist as evidence of successful issuance without its bound `CanonicalStructuredRequest`.

---

# 23. Successful issuance publication set

## SRE-014-PUBLICATION-001

A successful issuance SHALL produce exactly:

```text
CanonicalStructuredRequest
IssuanceManifest
```

## SRE-014-PUBLICATION-002

The two publications SHALL be mutually bound.

## SRE-014-PUBLICATION-003

The successful publication set SHALL reference exactly one Construction Publication Set.

## SRE-014-PUBLICATION-004

Successful issuance SHALL NOT publish an `IssuanceFailureRecord` for the same activity.

## SRE-014-PUBLICATION-005

The Issuance Publication Set SHALL be complete only when both constituent publications are committed.

## SRE-014-PUBLICATION-006

Neither constituent publication SHALL independently be treated as proof of completed issuance.

---

# 24. Failure taxonomy

## SRE-014-FAILURE-001

A failed issuance SHALL produce exactly one immutable `IssuanceFailureRecord` classified under one primary failure class:

```text
InvocationFailure
ResolutionFailure
IdentityFailure
StandingFailure
PublicationFailure
```

## 24.1 Invocation Failure

### SRE-014-FAILURE-002

Invocation failures MAY include:

```text
MissingConstructedArtifact
MissingConstructionManifest
ConstructionPublicationSetMismatch
ConstructionPublicationSetIncomplete
ArtifactNotIssuable
DuplicateIssuanceConflict
IssuanceNotPermitted
InvalidIssuanceAuthority
InvalidIssuanceContextInput
```

### SRE-014-FAILURE-003

An Invocation Failure SHALL mean that the issuance activity lacked a constitutionally admissible invocation basis.

## 24.2 Resolution Failure

### SRE-014-FAILURE-004

Resolution failures MAY include:

```text
IssuanceProfileResolutionFailure
IdentityPolicyResolutionFailure
IssuanceRuleSetResolutionFailure
RegistryResolutionFailure
VersionSnapshotUnavailable
IssuanceContextResolutionFailure
StandingClassResolutionFailure
```

## 24.3 Identity Failure

### SRE-014-FAILURE-005

Identity failures MAY include:

```text
IdentityDerivationFailure
IdentityCollision
IdentityNamespaceUnavailable
IssuanceSequenceAllocationFailure
NonDeterministicIdentityPolicy
IdentityInputMismatch
ReplayConflict
UnauthorizedEventUniqueIssuance
```

## 24.4 Standing Failure

### SRE-014-FAILURE-006

Standing failures MAY include:

```text
StandingAssignmentFailure
UnsupportedStandingClass
StandingRuleConflict
StandingAuthorityUnavailable
```

## 24.5 Publication Failure

### SRE-014-FAILURE-007

Publication failures MAY include:

```text
AtomicPublicationFailure
IssuanceManifestCreationFailure
IssuedRequestPublicationFailure
PublicationBindingFailure
InternalIssuanceFailure
```

## SRE-014-FAILURE-008

A failure record SHALL identify:

- the failed issuance activity;
- the Construction Publication Set presented;
- the attempted Issuance Context;
- the primary failure class;
- the specific failure code;
- applicable governance references;
- whether retry is constitutionally permissible;
- whether an earlier valid issuance already exists;
- any non-sensitive diagnostic references.

## SRE-014-FAILURE-009

A failure record SHALL NOT create issued identity or standing.

---

# 25. Issued outcome versus failure

## SRE-014-BOUNDARY-001

An issuance activity SHALL be `Issued` only when the complete Issuance Publication Set is atomically committed.

## SRE-014-BOUNDARY-002

An issuance activity SHALL be `Failed` when any required invocation, resolution, identity, standing, or publication condition cannot be satisfied.

## SRE-014-BOUNDARY-003

A partially published request SHALL be treated as publication failure and SHALL NOT possess issued standing.

## SRE-014-BOUNDARY-004

A replay that returns an existing valid publication set SHALL remain a successful idempotent resolution and SHALL NOT create a second publication set.

---

# 26. Atomic commitment

## SRE-014-COMMIT-001

The `CanonicalStructuredRequest` and `IssuanceManifest` SHALL be committed atomically.

## SRE-014-COMMIT-002

The implementation SHALL prevent observers from treating one constituent publication as successfully issued before the other is committed.

## SRE-014-COMMIT-003

If atomic commitment cannot complete, the issuance activity SHALL fail and produce an `IssuanceFailureRecord`.

## SRE-014-COMMIT-004

A failed commitment attempt SHALL NOT mutate the Construction Publication Set.

## SRE-014-COMMIT-005

A retry after publication failure SHALL follow the applicable replay and identity policy and SHALL NOT create accidental duplicate standing.

---

# 27. Immutability and later lifecycle actions

## SRE-014-IMMUTABILITY-001

The successful Issuance Publication Set SHALL be immutable after publication.

## SRE-014-IMMUTABILITY-002

Correction of an issuance error SHALL NOT occur by editing either constituent publication.

## SRE-014-IMMUTABILITY-003

Any later revocation, suspension, expiration, withdrawal, replacement, renewal, correction, or supersession SHALL require separately governed authority and a new immutable lifecycle publication.

## SRE-014-IMMUTABILITY-004

An updated registry, policy, profile, or implementation SHALL NOT retroactively alter an earlier issuance.

## SRE-014-IMMUTABILITY-005

The Construction Publication Set referenced by an issued request SHALL remain the exact set originally issued.

---

# 28. Determinism and replay

## SRE-014-DETERMINISM-001

Given the same:

- Construction Publication Set;
- Issuance Context;
- Identity Policy version;
- Issuance Profile version;
- issuance rules;
- registry versions;
- contract versions;
- sequence state required by policy;
- identity-bearing inputs;

a conforming implementation SHALL produce the same issuance result under the policy's declared replay semantics.

## SRE-014-DETERMINISM-002

The default replay semantic SHALL be idempotent.

## SRE-014-DETERMINISM-003

Event-unique behavior SHALL be explicit, versioned, testable, and auditable.

## SRE-014-DETERMINISM-004

Implementation timing, thread scheduling, host identity, storage path, retry count, and diagnostic logging SHALL NOT alter issued identity unless expressly declared as governed identity inputs, which SHOULD be avoided.

## SRE-014-DETERMINISM-005

A conforming implementation SHALL support deterministic verification of the published identity derivation basis.

---

# 29. Serialization boundary

## SRE-014-SERIALIZATION-001

The logical `CanonicalStructuredRequest` and `IssuanceManifest` SHALL remain constitutionally independent of any specific transport encoding or storage representation.

## SRE-014-SERIALIZATION-002

A canonical byte representation MAY be required for identity calculation or integrity verification where governed by the Identity Policy.

## SRE-014-SERIALIZATION-003

Identity-calculation serialization SHALL be deterministic, versioned, and non-semantic.

## SRE-014-SERIALIZATION-004

Contract 014 SHALL NOT select the downstream transport protocol or handoff envelope used by Contract 015.

---

# 30. Downstream handoff to Contract 015

## SRE-014-HANDOFF-001

Contract 014 SHALL make the complete Issuance Publication Set available as the constitutional input to Contract 015.

## SRE-014-HANDOFF-002

The handoff input SHALL preserve:

- the Canonical Structured Request identity;
- the Construction Publication Set references;
- the Issuance Manifest;
- the Issuance Context;
- the initial standing;
- the identity and registry version snapshot;
- all relevant provenance and integrity bindings.

## SRE-014-HANDOFF-003

Contract 014 SHALL NOT identify the downstream recipient as part of issued-request meaning.

## SRE-014-HANDOFF-004

Making an Issuance Publication Set available to Contract 015 SHALL NOT itself complete transfer.

## SRE-014-HANDOFF-005

A later `CanonicalRequestHandoffRecord` SHALL identify the custody event and SHALL remain distinct from the identities established by this contract.

---

# 31. Neighbor boundaries

## SRE-014-BOUNDARY-005 — Contract 012 boundary

Contract 012 exclusively determines structural eligibility for construction. Contract 014 SHALL NOT revisit that determination.

## SRE-014-BOUNDARY-006 — Contract 013 boundary

Contract 013 exclusively constructs the immutable Construction Publication Set. Contract 014 SHALL bind identity and standing to that exact set and SHALL NOT reconstruct it.

## SRE-014-BOUNDARY-007 — Contract 015 boundary

Contract 015 exclusively governs downstream transfer, custody transition, acknowledgment, and handoff recording. Contract 014 SHALL NOT transfer custody.

## SRE-014-BOUNDARY-008 — Authorization boundary

No contract in the Structured Request Engine family SHALL treat issuance as execution authorization unless a later adopted constitutional authority explicitly grants such power outside this contract.

## SRE-014-BOUNDARY-009 — Provenance and evidence boundary

Contract 014 MAY preserve and reference evidence and provenance already represented upstream. It SHALL NOT recreate or reinterpret those records.

---

# 32. Deferred responsibilities

## SRE-014-DEFER-001

The following responsibilities are deferred outside Contract 014:

- downstream recipient declaration;
- transport-neutral custody transfer;
- acknowledgment and acceptance;
- transfer retry and delivery state;
- operational authorization;
- execution planning;
- provider or tool binding;
- lifecycle revocation, suspension, expiration, replacement, renewal, correction, or supersession;
- release of generated outputs.

## SRE-014-DEFER-002

Deferral SHALL NOT be interpreted as permission for a conforming implementation to perform those responsibilities within the issuance authority.

---

# 33. Security and integrity considerations

## SRE-014-SECURITY-001

A conforming implementation SHALL protect against:

- substitution of the constructed artifact or Construction Manifest;
- issuance under an unauthorized authority or namespace;
- hidden or volatile identity inputs;
- same-context duplicate initial issuance;
- identity collision;
- sequence reuse or allocation ambiguity;
- replay that creates duplicate standing;
- partial publication;
- manifest tampering;
- version or registry substitution;
- context inflation through cosmetic metadata;
- issuance being misrepresented as execution permission.

## SRE-014-SECURITY-002

Integrity verification SHALL fail closed where a required binding cannot be established.

## SRE-014-SECURITY-003

The implementation SHALL preserve sufficient immutable evidence to verify identity derivation, context uniqueness, standing assignment, and atomic publication.

## SRE-014-SECURITY-004

Sensitive issuance metadata MAY be access-controlled, but access control SHALL NOT erase the existence or integrity of the constitutional publication.

## SRE-014-SECURITY-005

The issuance authority SHALL not accept implementation-local defaults as constitutional inputs unless those defaults are explicitly adopted by the applicable profile or policy.

---

# 34. Conformance requirements

## SRE-014-CONFORM-001

A conforming implementation SHALL demonstrate that it:

- consumes exactly one immutable Construction Publication Set;
- verifies publication-set binding without revalidation;
- resolves one explicit Issuance Context;
- resolves one applicable Issuance Profile and Identity Policy;
- exposes all identity-bearing inputs;
- distinguishes identity categories;
- prevents unrelated same-context duplicate initial issuance;
- applies idempotent replay by default;
- scopes all issuance sequences;
- treats timestamp as metadata unless expressly identity-bearing;
- assigns initial standing only;
- publishes the issued request and manifest atomically;
- produces a failure record on non-completion;
- preserves upstream identity and content immutability;
- grants no execution or transfer authority.

## SRE-014-CONFORM-002

Conformance tests SHALL include at least:

- valid first issuance;
- idempotent same-context replay;
- context-distinct issuance;
- same-context duplicate conflict;
- unauthorized event-unique issuance;
- missing or mismatched Construction Manifest;
- unauthorized issuance authority;
- unresolved profile or policy;
- identity collision;
- sequence allocation failure;
- timestamp variance under non-time-bearing identity policy;
- atomic publication failure;
- attempted content modification;
- attempted revalidation;
- attempted custody transfer;
- attempted execution authorization.

## SRE-014-CONFORM-003

A system SHALL NOT claim Contract 014 conformance if it can publish an issued request without a bound Issuance Manifest.

---

# 35. Normative dependencies and downstream consumers

## SRE-014-DEPENDENCY-001

Contract 014 normatively depends on:

- Contract 000 — governing constitutional doctrine;
- Contracts 001–008 — upstream represented artifacts preserved through construction;
- Contract 009 — semantic reconciliation standing;
- Contract 010 — normalized request representation;
- Contract 011 — canonical ordering;
- Contract 012 — structural eligibility;
- Contract 013 — Construction Publication Set;
- adopted identity, issuance, registry, profile, and versioning standards.

## SRE-014-DEPENDENCY-002

Contract 014 SHALL not reinterpret or expand the authority of any dependency.

## SRE-014-CONSUMER-001

The direct downstream constitutional consumer is Contract 015 — Downstream Handoff.

## SRE-014-CONSUMER-002

Audit, provenance, registry, conformance, and downstream governance systems MAY consume the Issuance Publication Set without receiving authority to alter it.

---

# 36. Expected implementation responsibility

## SRE-014-IMPLEMENTATION-001

A conforming implementation is expected to provide:

- a Construction Publication Set binding verifier;
- an Issuance Context resolver;
- an Identity Policy resolver and deterministic derivation component;
- an Issuance Profile resolver;
- a duplicate and replay detector;
- a governed sequence allocator where required;
- an initial-standing assigner;
- an Issuance Decision recorder;
- an atomic Issuance Publication Set publisher;
- an immutable failure recorder;
- deterministic replay and collision tests.

## SRE-014-IMPLEMENTATION-002

Implementation decomposition MAY vary, but it SHALL preserve all constitutional boundaries established by this contract.

## SRE-014-IMPLEMENTATION-003

A runtime MAY cache policy, registry, or prior-issuance data, but cache use SHALL preserve version identity, determinism, and auditability.

---

# 37. Fundamental identity and issuance invariants

## SRE-014-INVARIANT-001

For every successful issuance:

```text
Exactly one Construction Publication Set
Exactly one Canonical Request Issuance activity
Exactly one Issuance Context
Exactly one CanonicalStructuredRequest
Exactly one IssuanceManifest
Exactly one initial standing assignment
```

SHALL be constitutionally bound.

## SRE-014-INVARIANT-002

The following invariants SHALL always hold:

```text
Construction content is unchanged.
Issued identity is distinct from construction identity.
The Issuance Manifest is mandatory.
Publication is atomic.
Same-context replay is idempotent by default.
Multiple issuance requires context distinction.
Issuance grants no execution authority.
Issuance transfers no custody.
```

## SRE-014-INVARIANT-003

No successful issuance SHALL exist without sufficient immutable information to verify:

- what exact Construction Publication Set was issued;
- under which Issuance Context;
- by which authority;
- under which profile, policy, rules, registries, and versions;
- from which identity-bearing inputs;
- with which derivation result;
- with which initial standing;
- through which atomic publication commitment.

## SRE-014-INVARIANT-004

One Construction Publication Set MAY support multiple issued requests only where each issuance is constitutionally context-distinct or separately authorized as governed event-unique issuance.

---

# 38. Adoption and review status

## SRE-014-STATUS-001

The architecture of Contract 014 is frozen around the following adopted model:

```text
One immutable Construction Publication Set
        ↓
Zero or more constitutionally context-distinct issuances
        ↓
At most one unrelated initial issuance per context by default
        ↓
Atomic Issuance Publication Set
```

## SRE-014-STATUS-002

The following architectural decisions are closed:

- binding rather than copying or mutation;
- issuance of the full Construction Publication Set;
- mandatory Issuance Manifest;
- atomic publication;
- context-distinct multiple issuance;
- same-context uniqueness;
- idempotent replay by default;
- policy-controlled event-unique issuance exception;
- scoped sequence doctrine;
- timestamp non-participation by default;
- initial-standing-only limitation;
- strict separation from execution and handoff.

## SRE-014-STATUS-003

This document is a normative draft prepared against the adopted SRE Contract Plan v2.1.0 and the frozen Contract 014 architectural closure.

It is ready for Candidate Review, cross-contract terminology harmonization, normative dependency verification, and subsequent canonization.
