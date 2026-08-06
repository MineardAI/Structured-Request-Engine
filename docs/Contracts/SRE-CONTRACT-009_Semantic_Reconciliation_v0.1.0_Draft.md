# Structured Request Engine

## Contract 009 — Semantic Reconciliation

**Document ID:** `SRE-CONTRACT-009`  
**Version:** `v0.1.0`  
**Contract-set version:** `v0.1.0`  
**Status:** Draft — Constitutional Development  
**Project:** Structured-Request-Engine  
**Normative dependencies:**

- `SRE-CONTRACT-000 v0.1.0`
- `SRE-CONTRACT-001 v0.1.0`
- `SRE-CONTRACT-002 v0.1.0`
- `SRE-CONTRACT-003 v0.1.0`
- `SRE-CONTRACT-004 v0.1.0`
- `SRE-CONTRACT-005 v0.1.0`
- `SRE-CONTRACT-006 v0.1.0`
- `SRE-CONTRACT-007 v0.1.0`
- `SRE-CONTRACT-008 v0.1.0`
- `SRE-CONTRACT-PLAN v2.1.0`

---

## Normative requirement identifiers

Every normative `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, and `MAY` statement in this contract shall possess a stable requirement identifier.

Requirement identifiers use:

```text
SRE-009-<DOMAIN>-<SEQUENCE>
```

Examples:

```text
SRE-009-RECONCILE-001
SRE-009-DECISION-004
SRE-009-COMMIT-003
```

Requirement identifiers exist solely for traceability, implementation verification, conformance testing, amendment tracking, and cross-contract dependency analysis.

Requirement identifiers SHALL NOT alter the normative meaning of any requirement.

---

# 1. Architectural context

**Specification level:** Constitutional concept

## SRE-009-CONTEXT-001

Contracts 003 through 008 preserve admitted semantic representations, their grounding, and their provenance without determining which represented alternatives receive downstream standing.

## SRE-009-CONTEXT-002

This contract establishes the first Structured Request Engine authority permitted to assign governed downstream semantic standing among admitted represented alternatives.

## SRE-009-CONTEXT-003

The neighboring constitutional boundaries SHALL remain:

```text
Contract 008
Provenance Representation
        ↓
Contract 009
Semantic Reconciliation
        ↓
Contract 010
Semantic Normalization
```

## SRE-009-CONTEXT-004

Contract 009 SHALL determine what semantic content proceeds downstream.

Contract 010 SHALL determine how that reconciled semantic content is expressed in stable canonical form.

## SRE-009-CONTEXT-005

Contract 009 SHALL NOT perform structural construction-eligibility determination governed by Contract 012 or canonical request construction governed by Contract 013.

---

# 2. Purpose

**Specification level:** Constitutional concept

## SRE-009-PURPOSE-001

This contract establishes the constitutional rules by which admitted represented semantic alternatives may be compared, grouped, reconciled, assigned governed downstream standing, and published as one immutable `SemanticReconciliationSet`.

## SRE-009-PURPOSE-002

This contract defines:

- the Semantic Reconciliation Authority;
- Semantic Reconciliation Operations;
- Semantic Reconciliation logical continuity;
- immutable Semantic Reconciliation Representations;
- Reconciliation Subjects;
- Reconciliation Groups;
- Reconciliation Decisions;
- Downstream Standing;
- Reconciled Semantic Elements;
- within-domain reconciliation;
- bounded cross-domain reconciliation;
- comparison relationships;
- conflict classification;
- precedence and tie-breaking;
- merge, selection, preservation, split, deferral, exclusion, and unresolved dispositions;
- Reconciliation Profiles;
- reconciliation registries;
- decision basis and rule traceability;
- unresolved and blocked valid outcomes;
- `SemanticReconciliationSet` construction;
- failure outcomes;
- atomic commitment;
- immutable publication;
- downstream handoff;
- deferred responsibilities.

## SRE-009-PURPOSE-003

The organizing doctrine of this contract is:

> **Semantic Reconciliation determines governed downstream standing without erasing upstream constitutional plurality.**

## SRE-009-PURPOSE-004

This contract SHALL answer only:

> **What represented semantic alternatives receive what governed downstream standing?**

## SRE-009-PURPOSE-005

This contract SHALL NOT answer:

- what the requester truly intended;
- whether a representation is objectively true;
- whether evidence is sufficient;
- whether provenance is trustworthy;
- whether a request is permitted, safe, feasible, or executable;
- how reconciled meaning should be normalized;
- how request elements should be canonically ordered;
- whether the ordered representation is structurally eligible for construction;
- how the canonical request is mechanically constructed;
- whether downstream action is authorized.

---

# 3. Governing constitutional elements

**Specification level:** Constitutional concept

## SRE-009-GOVERNING-001

The governing constitutional question of this contract is:

> **What represented semantic alternatives receive what governed downstream standing?**

## SRE-009-GOVERNING-002

The constitutional subject of this contract is:

> **Admitted represented semantic alternatives and their governed downstream standing.**

## SRE-009-GOVERNING-003

The constitutional act of this contract is:

> **Issue an explicit Reconciliation Decision.**

## SRE-009-GOVERNING-004

The principal successful publication of this contract is:

```text
SemanticReconciliationSet
```

## SRE-009-GOVERNING-005

The principal failed publication of this contract is:

```text
SemanticReconciliationFailureRecord
```

---

# 4. Architectural identity

**Specification level:** Constitutional concept

## SRE-009-IDENTITY-001

Contract 009 establishes the Semantic Reconciliation domain of the Structured Request Engine.

## SRE-009-IDENTITY-002

The constitutional transformation governed by this contract is:

```text
Committed Semantic Representation Sets
        +
InterpretationEvidenceSet
        +
ProvenanceRecordSet
        +
ReconciliationProfile
        +
Applicable Reconciliation Registries
        │
        ▼
SemanticReconciliationAuthority
        │
        ├── success ──► SemanticReconciliationSet
        │
        └── failure ──► SemanticReconciliationFailureRecord
```

## SRE-009-IDENTITY-003

Contract 009 SHALL operate only over immutable committed upstream artifacts and declared reconciliation inputs.

## SRE-009-IDENTITY-004

Contract 009 SHALL NOT modify, replace, rewrite, erase, or reissue any upstream semantic, evidence, grounding, provenance, or proposal artifact.

## SRE-009-IDENTITY-005

A reconciliation decision SHALL affect downstream standing only.

It SHALL NOT alter upstream validity, identity, admission status, evidence, provenance, or historical standing.

---

# 5. Formal definitions

**Specification level:** Constitutional concept

## SRE-009-DEFINITION-001 — Semantic Reconciliation

A **Semantic Reconciliation** is the logical constitutional object identifying one governed reconciliation context in which admitted represented semantic alternatives are assigned downstream standing under declared rules.

## SRE-009-DEFINITION-002 — Semantic Reconciliation Representation

A **Semantic Reconciliation Representation** is one immutable represented state of a Semantic Reconciliation, including its subjects, groups, decisions, standing assignments, resulting semantic elements, profile context, registry context, and unresolved conditions.

## SRE-009-DEFINITION-003 — Reconciliation Subject

A **Reconciliation Subject** is an immutable reference to an upstream semantic representation eligible for comparison, grouping, or standing assignment under this contract.

## SRE-009-DEFINITION-004 — Reconciliation Group

A **Reconciliation Group** is a bounded collection of Reconciliation Subjects considered together because an applicable profile or rule establishes that they concern the same logical subject, semantic role, scope, relationship, or conflict context.

## SRE-009-DEFINITION-005 — Reconciliation Decision

A **Reconciliation Decision** is one immutable constitutional artifact recording that a defined Reconciliation Group received a defined reconciliation disposition under an identified authorized rule and decision basis.

## SRE-009-DEFINITION-006 — Downstream Standing

**Downstream Standing** is the governed status assigned by a Reconciliation Decision that determines whether and how an upstream representation participates in the reconciled semantic basis supplied to Contract 010.

## SRE-009-DEFINITION-007 — Reconciled Semantic Element

A **Reconciled Semantic Element** is a downstream semantic artifact produced by one or more Reconciliation Decisions from admitted upstream semantic meaning without unauthorized semantic invention.

## SRE-009-DEFINITION-008

A Reconciliation Decision answers:

> What constitutional resolution occurred?

A Reconciled Semantic Element answers:

> What semantic content proceeds downstream?

## SRE-009-DEFINITION-009

A Reconciliation Decision and a Reconciled Semantic Element SHALL remain distinct constitutional objects.

---

# 6. Organizing constitutional doctrines

**Specification level:** Constitutional concept

## 6.1 Downstream standing is not upstream validity

### SRE-009-DOCTRINE-001

A reconciliation disposition SHALL govern downstream standing only.

It SHALL NOT determine that an upstream representation is true, false, valid, invalid, admissible, inadmissible, supported, unsupported, or erased.

## 6.2 Reconciliation is not discovery of true intent

### SRE-009-DOCTRINE-002

Contract 009 SHALL NOT claim to discover the requesting party's true, complete, final, or exclusive intent.

## 6.3 Reconciliation is not normalization

### SRE-009-DOCTRINE-003

Contract 009 SHALL determine what represented semantic content proceeds downstream.

It SHALL NOT assign canonical wording, terminology, unit expression, field form, enum representation, or canonical serialization.

## 6.4 Reconciliation is not validation

### SRE-009-DOCTRINE-004

Contract 009 SHALL NOT determine structural construction eligibility.

It MAY identify semantic conflict and reconciliation disposition.

Contract 012 SHALL determine whether the resulting representation is structurally eligible for construction.

## 6.5 Reconciliation is not construction

### SRE-009-DOCTRINE-005

Contract 009 SHALL NOT construct a `CanonicalStructuredRequest`, `ConstructedCanonicalRequest`, or equivalent canonical request artifact.

## 6.6 Conflict may remain unresolved

### SRE-009-DOCTRINE-006

A valid reconciliation operation MAY publish unresolved semantic conflict when the applicable profile permits preserved unresolved state.

## 6.7 Unresolved is not failure

### SRE-009-DOCTRINE-007

The following distinction SHALL remain explicit:

```text
Unresolved Reconciliation
        ≠
Semantic Reconciliation Failure
```

## 6.8 Non-invention

### SRE-009-DOCTRINE-008

Contract 009 SHALL NOT invent new objectives, constraints, capability requirements, ambiguities, assumptions, uncertainty, evidence, provenance, semantic content, or authority merely to produce a reconciled result.

## 6.9 Upstream immutability

### SRE-009-DOCTRINE-009

Upstream committed artifacts SHALL remain immutable.

Reconciliation SHALL occur through new identified decisions and downstream artifacts.

## 6.10 Decision explicitness

### SRE-009-DOCTRINE-010

Every merge, selection, preservation, split, deferral, exclusion, or unresolved outcome SHALL be represented by an identified Reconciliation Decision.

## 6.11 Traceability

### SRE-009-DOCTRINE-011

Every Reconciled Semantic Element SHALL remain traceable to all contributing, competing, excluded, deferred, or unresolved upstream alternatives relevant to its formation or standing.

## 6.12 Authority non-expansion

### SRE-009-DOCTRINE-012

No reconciliation decision, downstream-standing assignment, conflict classification, precedence rule, tie-break result, or publication created under this contract SHALL expand downstream operational authority.

---

# 7. Constitutional authority

**Specification level:** Constitutional authority

## 7.1 Semantic Reconciliation Authority

### SRE-009-AUTHORITY-001

The Semantic Reconciliation Authority SHALL be the exclusive constitutional owner of the transformation governed by this contract.

### SRE-009-AUTHORITY-002

The Semantic Reconciliation Authority MAY:

- identify eligible Reconciliation Subjects;
- construct Reconciliation Groups;
- classify comparison relationships;
- identify duplicate, equivalent, compatible, complementary, alternative, competing, conflicting, unrelated, or indeterminate representations;
- apply declared reconciliation rules;
- apply declared precedence rules;
- apply declared deterministic tie-break rules;
- issue Reconciliation Decisions;
- assign Downstream Standing;
- construct Reconciled Semantic Elements;
- preserve unresolved and blocked valid states;
- preserve all upstream alternatives and decision bases;
- construct a `SemanticReconciliationSet`;
- construct a `SemanticReconciliationFailureRecord`.

### SRE-009-AUTHORITY-003

The Semantic Reconciliation Authority SHALL NOT:

- modify upstream artifacts;
- determine truth or correctness;
- evaluate evidence sufficiency;
- verify provenance claims;
- invent new semantic meaning;
- perform policy or safety adjudication;
- determine feasibility;
- select tools, providers, models, connectors, credentials, or execution methods;
- perform semantic normalization;
- establish canonical ordering;
- determine structural eligibility;
- construct a canonical request;
- issue canonical request identity;
- authorize planning, execution, generation, or release.

## 7.2 Exclusive decision authority

### SRE-009-AUTHORITY-004

Only the Semantic Reconciliation Authority MAY issue a Reconciliation Decision under this contract.

### SRE-009-AUTHORITY-005

Applications, interpreters, upstream contracts, evidence artifacts, provenance artifacts, and downstream consumers MAY supply constitutional inputs.

They SHALL NOT directly assign Contract 009 Downstream Standing unless they are themselves the recognized Semantic Reconciliation Authority for the operation.

---

# 8. Major concept and artifact classification

**Specification level:** Constitutional concept

| Concept or artifact | Classification |
|---|---|
| Semantic Reconciliation doctrine | Constitutional concept |
| Semantic Reconciliation Authority | Constitutional authority |
| Semantic Reconciliation | Logical constitutional object |
| Semantic Reconciliation Representation | Logical artifact with immutable runtime representation |
| Reconciliation Subject | Required runtime reference role |
| Reconciliation Group | Required runtime artifact |
| Reconciliation Decision | Required standalone constitutional runtime artifact |
| Downstream Standing | Required decision semantics |
| Reconciled Semantic Element | Required runtime artifact |
| Reconciled Semantic Representation | Required immutable runtime artifact |
| Reconciliation Profile | Required runtime artifact or externally supplied normative profile |
| `SemanticReconciliationSet` | Required runtime artifact |
| `SemanticReconciliationFailureRecord` | Required runtime artifact |
| Internal comparison indexes or graphs | Implementation convenience |

## SRE-009-CLASSIFICATION-001

`ReconciliationDecision` SHALL remain independently identifiable and auditable even when physically embedded within a `SemanticReconciliationSet`.

## SRE-009-CLASSIFICATION-002

A named concept SHALL NOT automatically require a dedicated Rust type, module, file, service, database table, or top-level serialized artifact unless this contract classifies it as a required runtime artifact.

---

# 9. Canonical inputs

**Specification level:** Required runtime behavior

## SRE-009-INPUT-001

A conforming Semantic Reconciliation Operation SHALL consume, as applicable:

- one admitted interpretation proposal set;
- one committed `DeclaredObjectiveSet`;
- one committed `DeclaredConstraintSet`;
- one committed `CapabilityRequirementSet`;
- one committed `SemanticClarificationSet`;
- one committed `InterpretationEvidenceSet`;
- one committed `ProvenanceRecordSet`;
- one applicable `ReconciliationProfileId` and exact profile version;
- applicable comparison, conflict, disposition, standing, precedence, decision-basis, semantic-domain, and status registry versions;
- applicable schema versions;
- configuration snapshot references when they materially affect reconciliation.

## SRE-009-INPUT-002

A required upstream artifact MAY be represented as a valid empty, non-applicable, unresolved, or blocked result where its governing contract and the applicable reconciliation profile permit that state.

## SRE-009-INPUT-003

Every consumed artifact SHALL possess immutable identity and SHALL be deterministically associated with the same reconciliation lineage or declared reconciliation context.

## SRE-009-INPUT-004

Contract 009 SHALL reject or fail any operation whose declared inputs cannot be deterministically associated with one reconciliation operation context.

## SRE-009-INPUT-005

No hidden prompt, ambient state, undocumented session context, provider default, mutable global, live tool inventory, operator preference, or unrecorded assumption SHALL influence reconciliation output.

## SRE-009-INPUT-006

Any context materially affecting reconciliation SHALL be declared, versioned, and traceable.

---

# 10. Semantic Reconciliation Operation

**Specification level:** Required runtime behavior

## SRE-009-OPERATION-001

Every Semantic Reconciliation Operation SHALL possess exactly one immutable `SemanticReconciliationOperationId`.

## SRE-009-OPERATION-002

The operation identity SHALL correlate:

```text
SemanticReconciliationOperationId
├── Upstream Artifact Identities
├── Upstream Representation Identities
├── Applicable Profile Identity and Version
├── Applicable Schema and Registry Versions
├── SemanticReconciliationSetId, on success
└── SemanticReconciliationFailureRecordId, on failure
```

## SRE-009-OPERATION-003

Operation identity SHALL NOT replace upstream artifact, reconciliation, decision, reconciled element, set, or failure identities.

## SRE-009-OPERATION-004

Equivalent declared inputs processed under equivalent contract, profile, schema, registry, and configuration versions SHALL produce equivalent constitutional outcomes.

---

# 11. Identity model

**Specification level:** Required runtime behavior

## SRE-009-ID-001

The following identities SHALL remain distinct:

```text
SemanticReconciliationId
        ≠
SemanticReconciliationRepresentationId
        ≠
ReconciliationGroupId
        ≠
ReconciliationDecisionId
        ≠
ReconciledSemanticElementId
        ≠
ReconciledSemanticRepresentationId
        ≠
SemanticReconciliationSetId
        ≠
SemanticReconciliationFailureRecordId
```

## SRE-009-ID-002

`SemanticReconciliationId` SHALL identify logical reconciliation continuity.

## SRE-009-ID-003

`SemanticReconciliationRepresentationId` SHALL identify one immutable represented state of that reconciliation.

## SRE-009-ID-004

`ReconciliationGroupId` SHALL identify one bounded comparison set.

## SRE-009-ID-005

`ReconciliationDecisionId` SHALL identify one immutable reconciliation act.

## SRE-009-ID-006

`ReconciledSemanticElementId` SHALL identify logical continuity of one downstream semantic result.

## SRE-009-ID-007

`ReconciledSemanticRepresentationId` SHALL identify one immutable represented state of one Reconciled Semantic Element.

## SRE-009-ID-008

`SemanticReconciliationSetId` SHALL identify one immutable successful publication.

## SRE-009-ID-009

`SemanticReconciliationFailureRecordId` SHALL identify one immutable failed publication outcome.

## SRE-009-ID-010

Identity equality SHALL NOT imply truth, correctness, equivalence beyond the governing decision, evidence sufficiency, provenance trustworthiness, authorization, or downstream execution eligibility.

---

# 12. Reconciliation Subjects

**Specification level:** Required runtime reference role

## SRE-009-SUBJECT-001

Every Reconciliation Subject SHALL reference exactly one immutable upstream semantic representation.

## SRE-009-SUBJECT-002

A Reconciliation Subject reference SHALL include at least:

- upstream artifact identity;
- upstream representation identity, when distinct;
- semantic domain;
- governing upstream contract;
- represented scope;
- representation status;
- evidence references;
- provenance references;
- source and proposal references required for traceability.

## SRE-009-SUBJECT-003

A Reconciliation Subject SHALL NOT replace, reissue, or mutate the identity of the upstream artifact it references.

## SRE-009-SUBJECT-004

Only semantic representations eligible under the applicable Reconciliation Profile MAY become Reconciliation Subjects.

---

# 13. Reconciliation Groups

**Specification level:** Required runtime artifact

## SRE-009-GROUP-001

Every reconciliation comparison SHALL occur within exactly one identified Reconciliation Group.

## SRE-009-GROUP-002

A Reconciliation Group SHALL contain one or more Reconciliation Subjects.

## SRE-009-GROUP-003

A Reconciliation Group SHALL identify:

- `ReconciliationGroupId`;
- semantic domain or permitted cross-domain interaction class;
- participating subject references;
- group-formation rule;
- applicable profile rule;
- represented scope;
- group status;
- unresolved group conditions, when applicable.

## SRE-009-GROUP-004

Group formation SHALL be deterministic, profile-governed, and traceable.

## SRE-009-GROUP-005

Contract 009 SHALL NOT group unrelated representations merely because they appear in the same submission, proposal, source, set, or implementation structure.

## SRE-009-GROUP-006

A subject MAY participate in more than one Reconciliation Group only when each group represents a distinct authorized comparison context and that participation is explicitly preserved.

---

# 14. Comparison Relationship registry

**Specification level:** Required runtime registry

## SRE-009-COMPARISON-001

Each Reconciliation Group SHALL possess one or more comparison relationship findings from the applicable registry.

## SRE-009-COMPARISON-002

The initial Comparison Relationship registry SHALL include at least:

```text
ExactDuplicate
ProfileEquivalent
Compatible
Complementary
Alternative
Competing
Conflicting
Unrelated
Indeterminate
```

## SRE-009-COMPARISON-003

`ExactDuplicate` SHALL indicate representations that are identical under the applicable comparison rule.

## SRE-009-COMPARISON-004

`ProfileEquivalent` SHALL indicate representations treated as semantically equivalent only under an explicit profile-authorized equivalence rule.

## SRE-009-COMPARISON-005

`Compatible` SHALL indicate representations that may coexist without contradiction under the applicable profile.

## SRE-009-COMPARISON-006

`Complementary` SHALL indicate representations that contribute distinct compatible meaning to one downstream basis.

## SRE-009-COMPARISON-007

`Alternative` SHALL indicate multiple represented semantic paths where one or more may proceed according to declared rules.

## SRE-009-COMPARISON-008

`Competing` SHALL indicate representations seeking overlapping downstream standing under limited or exclusive conditions.

## SRE-009-COMPARISON-009

`Conflicting` SHALL indicate represented semantic incompatibility that cannot be jointly given the same downstream standing under the applicable rule.

## SRE-009-COMPARISON-010

`Unrelated` SHALL indicate that the compared subjects do not belong in the same Reconciliation Group under the applicable rules.

## SRE-009-COMPARISON-011

`Indeterminate` SHALL indicate that the relationship cannot be lawfully classified more precisely under the declared inputs and applicable profile.

## SRE-009-COMPARISON-012

A comparison relationship SHALL NOT, by itself, assign Downstream Standing.

---

# 15. Reconciliation Decisions

**Specification level:** Required standalone constitutional runtime artifact

## SRE-009-DECISION-001

Every Reconciliation Decision SHALL possess exactly one immutable `ReconciliationDecisionId`.

## SRE-009-DECISION-002

Every Reconciliation Decision SHALL identify at least:

- one `ReconciliationDecisionId`;
- one `ReconciliationGroupId`;
- participating Reconciliation Subject references;
- semantic domain or permitted cross-domain interaction class;
- comparison relationship findings;
- conflict references, when applicable;
- decision disposition;
- Downstream Standing assignments;
- governing Reconciliation Profile identity and version;
- exact governing rule identity;
- decision basis;
- evidence references;
- provenance references;
- preserved non-selected alternatives;
- unresolved conditions;
- deterministic tie-break information, when applicable;
- resulting Reconciled Semantic Element references, when applicable;
- decision status;
- commitment metadata.

## SRE-009-DECISION-003

A Reconciliation Decision SHALL remain independently auditable and replayable.

## SRE-009-DECISION-004

A Reconciliation Decision SHALL NOT conceal participating alternatives, governing rules, or non-selected outcomes.

## SRE-009-DECISION-005

A Reconciliation Decision SHALL NOT claim authority beyond assigning Downstream Standing under this contract.

---

# 16. Reconciliation Disposition registry

**Specification level:** Required runtime registry

## SRE-009-DISPOSITION-001

Every Reconciliation Decision SHALL possess exactly one primary disposition from the applicable registry.

## SRE-009-DISPOSITION-002

The initial Reconciliation Disposition registry SHALL include:

```text
Merged
Selected
Preserved
Split
Deferred
ExcludedFromDownstreamBasis
Unresolved
```

## 16.1 Merged

### SRE-009-DISPOSITION-003

`Merged` SHALL indicate that two or more Reconciliation Subjects contribute to one Reconciled Semantic Element under an explicit equivalence or compatible-composition rule.

### SRE-009-DISPOSITION-004

Merge SHALL preserve every contributing subject identity and SHALL NOT erase contributor history.

## 16.2 Selected

### SRE-009-DISPOSITION-005

`Selected` SHALL indicate that one or more subjects receive affirmative downstream standing under an explicit precedence or selection rule.

### SRE-009-DISPOSITION-006

Selection SHALL preserve all non-selected alternatives and the basis for selection.

## 16.3 Preserved

### SRE-009-DISPOSITION-007

`Preserved` SHALL indicate that one or more subjects proceed downstream without merge, exclusive selection, or semantic modification.

## 16.4 Split

### SRE-009-DISPOSITION-008

`Split` SHALL indicate that a represented semantic structure is expressed as multiple downstream semantic elements only where the split is supported by admitted upstream structure or an explicit deterministic profile rule.

### SRE-009-DISPOSITION-009

Contract 009 SHALL NOT perform open-ended semantic decomposition.

## 16.5 Deferred

### SRE-009-DISPOSITION-010

`Deferred` SHALL indicate that Downstream Standing cannot yet be assigned because a declared condition, clarification, input, or profile requirement remains unresolved.

### SRE-009-DISPOSITION-011

Deferral SHALL be a valid reconciliation outcome where permitted by profile.

## 16.6 ExcludedFromDownstreamBasis

### SRE-009-DISPOSITION-012

`ExcludedFromDownstreamBasis` SHALL indicate that an admitted upstream representation does not participate in the current reconciled downstream semantic basis under an authorized rule.

### SRE-009-DISPOSITION-013

Exclusion SHALL NOT imply that the upstream representation is false, invalid, inadmissible, unsupported, or erased.

## 16.7 Unresolved

### SRE-009-DISPOSITION-014

`Unresolved` SHALL indicate that Contract 009 lacks lawful authority or sufficient declared basis to assign a more specific reconciled disposition.

### SRE-009-DISPOSITION-015

An unresolved decision SHALL preserve all affected alternatives and the reason reconciliation remained unresolved.

---

# 17. Downstream Standing registry

**Specification level:** Required decision semantics

## SRE-009-STANDING-001

Every participating Reconciliation Subject SHALL receive exactly one Downstream Standing assignment for each Reconciliation Decision in which it participates.

## SRE-009-STANDING-002

The initial Downstream Standing registry SHALL include at least:

```text
Included
Contributing
PreservedAsAlternative
Deferred
Excluded
Unresolved
```

## SRE-009-STANDING-003

`Included` SHALL indicate that the subject directly proceeds into the downstream reconciled semantic basis.

## SRE-009-STANDING-004

`Contributing` SHALL indicate that the subject contributes meaning to a merged or composed Reconciled Semantic Element.

## SRE-009-STANDING-005

`PreservedAsAlternative` SHALL indicate that the subject remains available downstream as an explicit alternative without exclusive selection.

## SRE-009-STANDING-006

`Deferred` SHALL indicate that downstream participation awaits a declared future condition or clarification.

## SRE-009-STANDING-007

`Excluded` SHALL indicate that the subject does not participate in the current downstream semantic basis while remaining preserved upstream.

## SRE-009-STANDING-008

`Unresolved` SHALL indicate that downstream participation remains constitutionally unsettled.

## SRE-009-STANDING-009

Downstream Standing SHALL NOT imply policy approval, authorization, feasibility, execution eligibility, or release eligibility.

---

# 18. Precedence

**Specification level:** Constitutional boundary and required runtime behavior

## SRE-009-PRECEDENCE-001

Precedence SHALL operate only through an explicit, versioned, profile-authorized rule.

## SRE-009-PRECEDENCE-002

Every precedence decision SHALL identify:

- the governing profile;
- the exact precedence rule;
- the affected alternatives;
- the selected disposition;
- the assigned Downstream Standing;
- the preserved non-selected alternatives;
- the decision basis.

## SRE-009-PRECEDENCE-003

Precedence SHALL belong to an explicitly governed relationship or context.

It SHALL NOT attach intrinsically to a source class merely because the source is user-supplied, application-supplied, interpreter-supplied, explicit, inferred, or referenced.

## SRE-009-PRECEDENCE-004

A Reconciliation Profile MAY define domain-specific or scope-specific precedence.

It SHALL NOT define a hidden global hierarchy.

## SRE-009-PRECEDENCE-005

Declared priority preserved by an upstream representation SHALL NOT automatically become Contract 009 precedence unless an applicable reconciliation rule expressly authorizes that use.

---

# 19. Deterministic tie-breaking

**Specification level:** Required runtime behavior

## SRE-009-TIEBREAK-001

A deterministic tie-break rule MAY be applied only where:

1. the applicable Reconciliation Profile expressly permits tie-breaking;
2. the tie-break rule is identified and versioned;
3. the tie-break does not invent semantic meaning;
4. all affected alternatives remain preserved;
5. the result remains replayable.

## SRE-009-TIEBREAK-002

Tie-breaking SHALL NOT be based on undocumented interpreter trust, hidden model confidence, implementation preference, runtime order, hash iteration order, arrival order, or mutable environment state.

## SRE-009-TIEBREAK-003

A tie-break result SHALL identify the exact comparison inputs, rule, and deterministic result basis.

---

# 20. Reconciled Semantic Elements

**Specification level:** Required runtime artifact

## SRE-009-ELEMENT-001

Every Reconciled Semantic Element SHALL possess one immutable `ReconciledSemanticElementId` identifying logical continuity.

## SRE-009-ELEMENT-002

Every immutable represented state of a Reconciled Semantic Element SHALL possess one unique `ReconciledSemanticRepresentationId`.

## SRE-009-ELEMENT-003

Every Reconciled Semantic Element SHALL identify:

- semantic domain;
- semantic expression or structured meaning preserved from admitted inputs;
- contributing Reconciliation Decision identities;
- contributing upstream subject identities;
- preserved alternative identities;
- evidence references;
- provenance references;
- represented scope;
- reconciliation status;
- unresolved conditions;
- applicable profile, schema, and registry versions.

## SRE-009-ELEMENT-004

A Reconciled Semantic Element SHALL contain only:

1. admitted upstream semantic meaning; or
2. meaning produced through an explicit profile-authorized equivalence, composition, or split rule.

## SRE-009-ELEMENT-005

A Reconciled Semantic Element SHALL NOT silently add, remove, strengthen, weaken, broaden, narrow, or reinterpret meaning beyond the governing decision and authorized rule.

## SRE-009-ELEMENT-006

A material change to semantic expression, scope, contributing subjects, decision basis, standing, unresolved state, evidence association, provenance association, profile, schema, or registry context SHALL produce a new `ReconciledSemanticRepresentationId`.

---

# 21. Within-domain reconciliation

**Specification level:** Required runtime behavior

## SRE-009-WITHIN-001

Within-domain reconciliation MAY compare and assign standing among representations governed by the same semantic domain.

Supported initial domains SHALL include at least:

```text
Objective
Constraint
CapabilityRequirement
Ambiguity
Assumption
Uncertainty
```

## SRE-009-WITHIN-002

Within-domain reconciliation MAY govern:

- exact duplicates;
- profile-authorized equivalence;
- compatible variants;
- complementary representations;
- competing representations;
- alternatives;
- conflicts;
- differing decompositions;
- differing designations;
- differing scope assignments.

## SRE-009-WITHIN-003

Within-domain reconciliation SHALL NOT alter the governing semantic domain of a subject.

---

# 22. Bounded cross-domain reconciliation

**Specification level:** Constitutional boundary and required runtime behavior

## SRE-009-CROSSDOMAIN-001

Cross-domain reconciliation MAY occur only when an applicable Reconciliation Profile expressly defines the permitted interaction class, participating domains, comparison rule, decision authority, and result constraints.

## SRE-009-CROSSDOMAIN-002

Permitted cross-domain reconciliation MAY determine downstream standing for interacting semantic representations, including interactions such as:

- Objective against Constraint;
- Objective against Capability Requirement;
- Constraint against Capability Requirement;
- Ambiguity, Assumption, or Uncertainty against another semantic-domain element where downstream standing depends on that interaction.

## SRE-009-CROSSDOMAIN-003

Cross-domain reconciliation SHALL NOT construct new semantic content not supported by admitted representations or profile-authorized deterministic composition rules.

## SRE-009-CROSSDOMAIN-004

Cross-domain reconciliation SHALL NOT convert:

```text
Objective
        ↓
New Constraint

Constraint
        ↓
New Capability Requirement

Capability Requirement
        ↓
New Assumption
```

unless the resulting semantic meaning already exists in admitted upstream representation or is expressly authorized by a deterministic rule consistent with the governing upstream contract boundaries.

## SRE-009-CROSSDOMAIN-005

A cross-domain conflict MAY remain unresolved and preserved.

## SRE-009-CROSSDOMAIN-006

Contract 009 SHALL NOT use cross-domain reconciliation to perform policy, safety, legal, feasibility, or execution adjudication.

---

# 23. Conflict classification

**Specification level:** Required runtime registry

## SRE-009-CONFLICT-001

Every identified reconciliation conflict SHALL possess one or more conflict classifications from the applicable registry.

## SRE-009-CONFLICT-002

The initial Conflict Class registry SHALL include at least:

```text
ExpressionConflict
ScopeConflict
DesignationConflict
ObjectiveConflict
ConstraintConflict
CapabilityConflict
AssumptionConflict
UncertaintyConflict
CrossDomainConflict
EvidenceAssociationConflict
ProvenanceAssociationConflict
```

## SRE-009-CONFLICT-003

`EvidenceAssociationConflict` SHALL indicate that competing semantic alternatives possess materially different grounding associations.

It SHALL NOT authorize Contract 009 to determine which evidence is true or sufficient.

## SRE-009-CONFLICT-004

`ProvenanceAssociationConflict` SHALL indicate that competing semantic alternatives possess materially different represented provenance contexts.

It SHALL NOT authorize Contract 009 to determine historical truth, trustworthiness, or semantic equivalence from lineage alone.

## SRE-009-CONFLICT-005

Conflict classification SHALL not itself determine a reconciliation disposition.

---

# 24. Decision Basis registry

**Specification level:** Required runtime registry

## SRE-009-BASIS-001

Every Reconciliation Decision SHALL identify at least one Decision Basis from the applicable registry.

## SRE-009-BASIS-002

The initial Decision Basis registry SHALL include at least:

```text
ExactIdentity
ExactRepresentationMatch
ExplicitSourceDeclaration
ExplicitUserDesignation
ApplicationDeclaration
ProfileAuthorizedEquivalence
ProfileAuthorizedComposition
ProfileAuthorizedPrecedence
SharedEvidenceAssociation
SharedProvenanceLineage
DeterministicTieBreak
PreservationRequired
InsufficientAuthority
```

## SRE-009-BASIS-003

A Decision Basis SHALL identify why the disposition was issued.

It SHALL NOT, by itself, establish truth, correctness, authority, or evidence sufficiency.

## SRE-009-BASIS-004

`SharedEvidenceAssociation` SHALL NOT automatically establish semantic equivalence.

## SRE-009-BASIS-005

`SharedProvenanceLineage` SHALL NOT automatically establish semantic equivalence or authority continuity.

## SRE-009-BASIS-006

`InsufficientAuthority` SHOULD be used where Contract 009 lawfully preserves an unresolved disposition because no authorized rule permits a more specific result.

---

# 25. Reconciliation Profile

**Specification level:** Required runtime profile

## SRE-009-PROFILE-001

Every Semantic Reconciliation Operation SHALL execute under exactly one applicable Reconciliation Profile.

## SRE-009-PROFILE-002

A Reconciliation Profile MAY govern:

- eligible upstream artifact types;
- eligible semantic domains;
- group-formation rules;
- comparison relationships;
- equivalence rules;
- duplicate rules;
- compatible-composition rules;
- split rules;
- precedence rules;
- tie-break rules;
- permitted dispositions;
- permitted Downstream Standing values;
- cross-domain interaction classes;
- conflict classification;
- evidence requirements;
- provenance requirements;
- unresolved-state permissions;
- blocked-state permissions;
- empty-set behavior;
- publication content requirements;
- deterministic replay requirements.

## SRE-009-PROFILE-003

A Reconciliation Profile MAY constrain reconciliation behavior but SHALL NOT expand, transfer, reduce, redefine, or bypass the constitutional authority granted by this contract.

## SRE-009-PROFILE-004

A Reconciliation Profile SHALL NOT establish:

- truth;
- correctness;
- evidentiary sufficiency;
- provenance trustworthiness;
- policy approval;
- feasibility;
- capability availability;
- execution authority;
- generation authority;
- release authority.

## SRE-009-PROFILE-005

A profile revision capable of changing reconciliation output SHALL receive a new profile version and SHALL NOT retroactively alter previously committed artifacts.

## SRE-009-PROFILE-006

A committed reconciliation artifact SHALL be replayed and evaluated under the exact contract, profile, schema, registry, and configuration versions that governed its construction.

---

# 26. Reconciliation status

**Specification level:** Required runtime registry

## SRE-009-STATUS-001

Every Semantic Reconciliation Representation, Reconciliation Group, Reconciliation Decision, and Reconciled Semantic Representation SHALL possess an applicable status.

## SRE-009-STATUS-002

The initial status registry SHALL include at least:

```text
Reconciled
PartiallyReconciled
Deferred
Unresolved
Blocked
NotApplicable
```

## SRE-009-STATUS-003

`Reconciled` SHALL indicate that all required decisions for the represented reconciliation scope possess completed dispositions.

## SRE-009-STATUS-004

`PartiallyReconciled` SHALL indicate that some but not all groups possess completed dispositions under a profile that permits partial successful publication.

## SRE-009-STATUS-005

`Deferred` SHALL indicate that one or more decisions await a declared condition or additional constitutional input.

## SRE-009-STATUS-006

`Unresolved` SHALL indicate that one or more decisions remain lawfully unsettled.

## SRE-009-STATUS-007

`Blocked` SHALL indicate that the published reconciliation set is valid but is marked as not presently eligible to proceed under a declared downstream condition.

## SRE-009-STATUS-008

`NotApplicable` SHALL indicate that no reconciliation was required for the represented scope.

## SRE-009-STATUS-009

Status SHALL NOT imply structural eligibility, authorization, feasibility, or execution readiness.

---

# 27. Reconciliation Disposition Completeness

**Specification level:** Required runtime behavior

## SRE-009-COMPLETENESS-001

Contract 009 SHALL evaluate Reconciliation Disposition Completeness only.

## SRE-009-COMPLETENESS-002

A reconciliation publication SHALL possess Reconciliation Disposition Completeness when every required Reconciliation Group has received a valid identified disposition, including a valid `Deferred` or `Unresolved` disposition where permitted.

## SRE-009-COMPLETENESS-003

Reconciliation Disposition Completeness SHALL NOT imply:

- complete understanding;
- semantic resolution of every conflict;
- structural construction eligibility;
- canonical artifact completeness;
- authorization.

## SRE-009-COMPLETENESS-004

Structural construction eligibility belongs exclusively to Contract 012.

---

# 28. SemanticReconciliationSet

**Specification level:** Required runtime artifact

## SRE-009-SET-001

A successful Semantic Reconciliation Operation SHALL produce exactly one committed `SemanticReconciliationSet`.

## SRE-009-SET-002

Every `SemanticReconciliationSet` SHALL possess exactly one immutable `SemanticReconciliationSetId`.

## SRE-009-SET-003

The set SHALL contain or reference at least:

- `SemanticReconciliationOperationId`;
- `SemanticReconciliationId`;
- `SemanticReconciliationRepresentationId`;
- all Reconciliation Groups;
- all Reconciliation Decisions;
- all Downstream Standing assignments;
- all Reconciled Semantic Elements and representations;
- all preserved alternatives;
- all conflict references;
- all unresolved, deferred, or blocked conditions;
- the applicable Reconciliation Profile identity and version;
- applicable schema and registry versions;
- input artifact identities;
- evidence references;
- provenance references;
- commitment metadata;
- publication status.

## SRE-009-SET-004

The set SHALL represent the complete committed result of one Semantic Reconciliation Operation.

## SRE-009-SET-005

The set MAY contain unresolved, deferred, blocked, excluded, or preserved-alternative states where the applicable profile permits them.

## SRE-009-SET-006

The existence of unresolved or blocked state SHALL NOT by itself make the set a failure artifact.

## SRE-009-SET-007

A valid empty `SemanticReconciliationSet` MAY be published when no reconciliation is required and the applicable profile permits an empty successful result.

---

# 29. Failure model

**Specification level:** Required runtime behavior

## SRE-009-FAILURE-001

A Semantic Reconciliation Operation that cannot produce a conforming `SemanticReconciliationSet` SHALL produce exactly one committed `SemanticReconciliationFailureRecord` when commitment remains possible.

## SRE-009-FAILURE-002

Failure conditions MAY include:

- input artifacts cannot be associated with one reconciliation context;
- required profile, schema, or registry versions are unavailable;
- reconciliation rules are internally contradictory;
- deterministic replay cannot be preserved;
- required identities cannot be established or preserved;
- group formation cannot be performed without unauthorized semantic inference;
- a required decision cannot be represented under any permitted disposition;
- atomic commitment cannot be completed;
- publication would require modification of upstream artifacts;
- publication would require semantic invention beyond authorized rules.

## SRE-009-FAILURE-003

The following conditions SHALL NOT automatically constitute Semantic Reconciliation failure:

- unresolved conflict;
- deferred decision;
- preserved alternatives;
- partial reconciliation permitted by profile;
- blocked downstream condition;
- excluded upstream alternative;
- empty reconciliation scope permitted by profile.

## SRE-009-FAILURE-004

Every failure record SHALL identify at least:

- `SemanticReconciliationFailureRecordId`;
- `SemanticReconciliationOperationId`;
- input artifact identities;
- applicable contract, profile, schema, registry, and configuration versions;
- failure category;
- affected groups or subjects;
- observed decision state;
- failure basis;
- replay information;
- commitment metadata.

## SRE-009-FAILURE-005

A failed operation SHALL NOT emit a partially authoritative `SemanticReconciliationSet`.

---

# 30. Atomic commitment

**Specification level:** Required runtime behavior

## SRE-009-COMMIT-001

Every completed Semantic Reconciliation Operation SHALL commit exactly one authoritative outcome:

```text
SemanticReconciliationSet
```

or:

```text
SemanticReconciliationFailureRecord
```

## SRE-009-COMMIT-002

A completed operation SHALL NOT commit both outcomes.

## SRE-009-COMMIT-003

A completed operation SHALL NOT commit neither outcome.

## SRE-009-COMMIT-004

Commitment SHALL be atomic.

## SRE-009-COMMIT-005

Diagnostic material MAY accompany either outcome.

Diagnostics SHALL NOT replace the required constitutional outcome.

---

# 31. Immutability and revision

**Specification level:** Required runtime behavior

## SRE-009-IMMUTABILITY-001

Committed Reconciliation Decisions, Reconciled Semantic Representations, `SemanticReconciliationSet` artifacts, and `SemanticReconciliationFailureRecord` artifacts SHALL be immutable.

## SRE-009-IMMUTABILITY-002

Correction, replacement, changed standing, changed grouping, changed disposition, added evidence, changed provenance, changed profile, changed rule, or changed semantic result SHALL occur through a new Semantic Reconciliation Operation and new immutable representation identities.

## SRE-009-IMMUTABILITY-003

A later artifact MAY supersede the present downstream usability of an earlier reconciliation publication.

It SHALL NOT erase the historical existence of the earlier publication or its decisions.

---

# 32. Determinism and replay

**Specification level:** Required runtime behavior

## SRE-009-DETERMINISM-001

Equivalent declared inputs processed under equivalent contract, profile, schema, registry, and configuration versions SHALL produce equivalent:

- Reconciliation Groups;
- comparison findings;
- Reconciliation Decisions;
- Downstream Standing assignments;
- Reconciled Semantic Elements;
- publication outcome.

## SRE-009-DETERMINISM-002

Deterministic equivalence SHALL NOT require identical storage layout, memory address, database ordering, transport encoding, or implementation-specific internal structure.

## SRE-009-DETERMINISM-003

A conforming implementation SHALL preserve sufficient information to replay every Reconciliation Decision.

## SRE-009-DETERMINISM-004

Replay SHALL use the exact governing versions originally applied unless an explicitly separate re-reconciliation operation is initiated.

---

# 33. Downstream handoff

**Specification level:** Required runtime behavior

## SRE-009-HANDOFF-001

Successful completion of Contract 009 SHALL authorize only consideration under Contract 010.

## SRE-009-HANDOFF-002

The `SemanticReconciliationSet` SHALL be handed downstream as an immutable, non-authorizing input.

## SRE-009-HANDOFF-003

Handoff SHALL preserve:

- all Reconciliation Decision identities;
- all Reconciled Semantic Element identities;
- all Downstream Standing assignments;
- all contributing, excluded, deferred, and unresolved upstream alternatives;
- all evidence and provenance references;
- all profile, schema, registry, and configuration versions;
- all commitment and publication metadata.

## SRE-009-HANDOFF-004

Downstream receipt SHALL NOT imply:

- semantic truth;
- correctness;
- normalization;
- structural eligibility;
- canonical request construction;
- authorization;
- execution eligibility.

---

# 34. Neighbor boundaries

**Specification level:** Constitutional boundary

## 34.1 Contract 008 boundary

### SRE-009-BOUNDARY-001

Contract 008 SHALL provide represented provenance context only.

Contract 009 SHALL NOT treat lineage, custody, publication, or transformation history as automatic proof of semantic equivalence or precedence.

## 34.2 Contract 010 boundary

### SRE-009-BOUNDARY-002

Contract 009 SHALL determine what proceeds.

Contract 010 SHALL determine how the reconciled semantic basis is normalized without changing represented meaning.

## 34.3 Contract 012 boundary

### SRE-009-BOUNDARY-003

Contract 009 MAY publish valid unresolved or blocked semantic state.

Contract 012 SHALL determine whether that state is structurally eligible for canonical request construction under the applicable validation profile.

## 34.4 Contract 013 boundary

### SRE-009-BOUNDARY-004

Contract 013 SHALL mechanically construct the canonical request from an eligible validated representation.

Contract 009 SHALL NOT perform canonical request assembly.

---

# 35. Deferred responsibilities

**Specification level:** Constitutional boundary

## SRE-009-DEFER-001

Contract 009 explicitly defers:

- semantic normalization to Contract 010;
- canonical ordering to Contract 011;
- structural construction eligibility to Contract 012;
- canonical request construction to Contract 013;
- deterministic identity and issuance standing to Contract 014;
- downstream custody transfer to Contract 015;
- policy, safety, feasibility, capability availability, authorization, planning, execution, generation, and release to lawful downstream authorities.

## SRE-009-DEFER-002

No implementation convenience SHALL collapse these deferred responsibilities into Contract 009 authority.

---

# 36. Conformance requirements

**Specification level:** Conformance doctrine

## SRE-009-CONFORM-001

A conforming implementation SHALL preserve the distinctions among:

```text
Reconciliation Subject
Reconciliation Group
Reconciliation Decision
Downstream Standing
Reconciled Semantic Element
SemanticReconciliationSet
```

## SRE-009-CONFORM-002

A conforming implementation SHALL preserve upstream artifact immutability.

## SRE-009-CONFORM-003

A conforming implementation SHALL preserve every material alternative and decision basis.

## SRE-009-CONFORM-004

A conforming implementation SHALL represent every disposition through an identified Reconciliation Decision.

## SRE-009-CONFORM-005

A conforming implementation SHALL distinguish unresolved valid state from operation failure.

## SRE-009-CONFORM-006

A conforming implementation SHALL not use hidden context or nondeterministic iteration behavior to determine reconciliation outcomes.

## SRE-009-CONFORM-007

A conforming implementation SHALL preserve deterministic replay from declared inputs and governing versions.

## SRE-009-CONFORM-008

A conforming implementation SHALL not treat Contract 009 output as authorization, feasibility, structural eligibility, or canonical request issuance.

---

# 37. Fundamental invariants

**Specification level:** Constitutional invariant

## SRE-009-INVARIANT-001 — Upstream plurality preservation

Every completed Semantic Reconciliation Operation SHALL preserve the identity and historical existence of every participating upstream representation.

## SRE-009-INVARIANT-002 — Decision explicitness

Every assigned Downstream Standing value SHALL be attributable to exactly one identified Reconciliation Decision within its decision context.

## SRE-009-INVARIANT-003 — Non-invention

No Reconciled Semantic Element SHALL contain meaning unsupported by admitted upstream representations or an explicit profile-authorized deterministic rule.

## SRE-009-INVARIANT-004 — Unresolved success

A `SemanticReconciliationSet` MAY be constitutionally successful while preserving unresolved, deferred, or blocked semantic state.

## SRE-009-INVARIANT-005 — Failure separation

Operation failure SHALL remain distinct from semantic disagreement, unresolved conflict, deferral, exclusion, or preserved alternatives.

## SRE-009-INVARIANT-006 — Authority non-expansion

Reconciliation precision, standing assignment, publication, or downstream handoff SHALL NOT create authorization, feasibility, execution authority, generation authority, provider authority, tool authority, or release authority.

## SRE-009-INVARIANT-007 — Atomic outcome

Every completed Semantic Reconciliation Operation SHALL produce exactly one committed success or failure artifact.

## SRE-009-INVARIANT-008 — Neighbor separation

Contract 009 SHALL not perform normalization, canonical ordering, structural eligibility evaluation, canonical request construction, identity issuance, or downstream custody transfer.

---

# 38. Constitutional transformation summary

```text
Committed Upstream Semantic Representations
        +
Evidence and Grounding
        +
Provenance
        +
Reconciliation Profile and Registries
        │
        ▼
Reconciliation Subjects
        │
        ▼
Reconciliation Groups
        │
        ▼
Comparison Findings
        │
        ▼
Reconciliation Decisions
        │
        ▼
Downstream Standing
        │
        ▼
Reconciled Semantic Elements
        │
        ▼
SemanticReconciliationSet
        │
        ▼
Contract 010 — Semantic Normalization
```

Failure path:

```text
Semantic Reconciliation Operation
        ↓
Admission, Grouping, Rule, Decision, Replay, or Commitment Failure
        ↓
SemanticReconciliationFailureRecord
```

---

# 39. Completion condition

**Specification level:** Required observable behavior

## SRE-009-COMPLETION-001

A Semantic Reconciliation Operation SHALL be considered complete only after exactly one authoritative outcome has been atomically committed.

## SRE-009-COMPLETION-002

Successful completion SHALL establish:

- the governed reconciliation scope;
- all required Reconciliation Groups;
- all required Reconciliation Decisions;
- all Downstream Standing assignments;
- all Reconciled Semantic Elements;
- preserved alternatives;
- unresolved, deferred, blocked, or excluded states where applicable;
- complete decision traceability;
- immutable publication under exact governing versions.

## SRE-009-COMPLETION-003

Successful completion SHALL establish nothing further.

It SHALL NOT establish semantic truth, request authorization, feasibility, structural eligibility, canonical request construction, issuance standing, or downstream execution rights.

---

# 40. Adoption note

This draft implements the Contract 009 boundary established by `SRE-CONTRACT-PLAN v2.1.0`.

The plan is authoritative where earlier upstream contracts contain stale downstream numbering or superseded contradiction-and-completeness assignments.

Those references shall be harmonized before Candidate Review or ratification of the complete contract family.
