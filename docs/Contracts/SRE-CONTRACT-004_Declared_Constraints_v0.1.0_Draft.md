# Structured Request Engine

## Contract 004 — Declared Constraints

**Document ID:** `SRE-CONTRACT-004`  
**Version:** `v0.1.0`  
**Contract-set version:** `v0.1.0`  
**Status:** Draft — Constitutional Development  
**Project:** Structured-Request-Engine  
**Normative dependencies:**

- `SRE-CONTRACT-000 v0.1.0`
- `SRE-CONTRACT-001 v0.1.0`
- `SRE-CONTRACT-002 v0.1.0`
- `SRE-CONTRACT-003 v0.1.0`

---

## Normative requirement identifiers

Every normative `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, and `MAY` statement in this contract shall possess a stable requirement identifier.

Requirement identifiers use:

```text
SRE-004-<DOMAIN>-<SEQUENCE>
```

Examples:

```text
SRE-004-CONSTRAINT-001
SRE-004-SCOPE-004
SRE-004-COMMIT-003
```

Requirement identifiers exist solely for traceability, implementation verification, conformance testing, amendment tracking, and cross-contract dependency analysis.

Requirement identifiers SHALL NOT alter the normative meaning of any requirement.

---

# 1. Purpose

**Specification level:** Constitutional concept

## SRE-004-PURPOSE-001

This contract establishes the constitutional rules by which constraint-domain content represented in admitted interpretation proposals may be constructed as a bounded `DeclaredConstraintSet`.

This contract defines:

- the Constraint Representation Authority;
- the formal meaning of a Declared Constraint;
- logical and representation identity;
- constraint origin;
- representation basis;
- evidence association;
- constraint class and form registries;
- explicit, inferred, application-supplied, and referenced-artifact constraints;
- required inclusions, exclusions, prohibitions, conditions, and bounds;
- temporal, format, resource, confidentiality, scope, content, and related constraint classes;
- objective attachment and represented scope;
- scope-resolution status;
- atomic and composite constraint forms;
- decomposition policy;
- declared priority and its separation from precedence;
- constraint relationships and represented conflict;
- representation status;
- empty-set behavior;
- failure outcomes;
- atomic commitment;
- downstream handoff;
- deferred responsibilities.

## SRE-004-PURPOSE-002

The constitutional purpose of Contract 004 is to represent limitations, conditions, inclusions, exclusions, boundaries, and restrictions without converting them into policy obligations, enforcement rules, execution permissions, or canonical constraint state.

## SRE-004-PURPOSE-003

The organizing doctrine of this contract is:

> **Constraint representation records represented limitations without establishing policy, feasibility, precedence, enforcement, or canonical constraint state.**

## SRE-004-PURPOSE-004

This contract SHALL answer only:

> **What constraint-domain content is represented as applying within the admitted submission context?**

It SHALL NOT answer whether that content is valid, lawful, safe, feasible, enforceable, preferred, binding, or canonical.

---

# 2. Architectural identity

**Specification level:** Constitutional concept

## SRE-004-IDENTITY-001

Contract 004 establishes the Constraint Representation domain of the Structured Request Engine.

## SRE-004-IDENTITY-002

The constitutional transformation governed by this contract is:

```text
AdmittedInterpretationProposalSet
        +
DeclaredObjectiveSet
        +
ConstraintRepresentationProfile
        +
Applicable Constraint Registries
        │
        ▼
ConstraintRepresentationAuthority
        │
        ├── success ──► DeclaredConstraintSet
        │
        └── failure ──► ConstraintRepresentationFailureRecord
```

## SRE-004-IDENTITY-003

Contract 004 SHALL operate over an admitted interpretation proposal set, including a set containing only one admitted proposal.

## SRE-004-IDENTITY-004

The `DeclaredObjectiveSet` SHALL provide semantic context and attachment targets only.

Contract 004 SHALL NOT modify, reinterpret, replace, reconcile, or canonicalize any Declared Objective.

---

# 3. Organizing constitutional doctrines

**Specification level:** Constitutional concept

## 3.1 Representation is not policy

### SRE-004-DOCTRINE-001

A Declared Constraint SHALL represent constraint-domain content only.

It SHALL NOT establish:

- downstream policy;
- legal obligation;
- authorization;
- safety approval;
- provider rule;
- execution permission;
- runtime enforcement;
- release authority;
- access-control authority;
- redaction authority.

## 3.2 Representation is not enforcement

### SRE-004-DOCTRINE-002

Contract 004 SHALL NOT transform a represented limitation into an enforcement rule or operational control.

## 3.3 Representation is not feasibility

### SRE-004-DOCTRINE-003

Contract 004 SHALL NOT determine whether a Declared Constraint is achievable individually or jointly with any objective, capability requirement, or other constraint.

## 3.4 Priority is not precedence

### SRE-004-DOCTRINE-004

Declared priority MAY preserve represented emphasis or ordering.

Declared priority SHALL NOT establish constitutional precedence or determine which constraint governs when multiple constraints cannot all be satisfied.

## 3.5 No canonical constraint state

### SRE-004-DOCTRINE-005

Contract 004 SHALL NOT establish canonical constraint state.

## 3.6 Authority non-expansion

### SRE-004-DOCTRINE-006

No constraint representation, classification, priority, relationship, scope, or status created under this contract SHALL expand downstream authority.

## 3.7 Non-invention

### SRE-004-DOCTRINE-007

Contract 004 SHALL NOT invent a Declared Constraint solely because an implementation, interpreter, application, provider, connector, policy engine, or downstream system would prefer it.

---

# 4. Constitutional position

**Specification level:** Constitutional concept

## SRE-004-POSITION-001

Contract 004 SHALL operate after successful objective representation under Contract 003 and before capability representation, ambiguity and assumption representation, evidence consolidation, provenance consolidation, reconciliation, semantic normalization, canonical ordering, validation, canonical request construction, identity issuance, or handoff.

```text
Contract 001
Source Admission
        │
        ▼
Contract 002
Interpretation Boundary
        │
        ▼
Contract 003
Objective Representation
        │
        ▼
Contract 004
Constraint Representation
        │
        ▼
Contracts 005–008
Other Semantic Domains and Grounding
        │
        ▼
Contract 009
Proposal Reconciliation
        │
        ▼
Contracts 010 onward
Normalization, Ordering, Validation, Construction, Identity, and Handoff
```

## SRE-004-POSITION-002

Successful completion of Contract 004 SHALL authorize only downstream consideration of the committed `DeclaredConstraintSet`.

---

# 5. Formal definitions

**Specification level:** Constitutional concept

## SRE-004-DEFINITION-001 — Declared Constraint

A **Declared Constraint** is a bounded semantic representation of constraint-domain content supported by admitted interpretation material and associated evidence within the submission context.

Constraint-domain content may express a limitation, inclusion, exclusion, condition, boundary, restriction, prohibition, minimum, maximum, exact requirement, or other represented applicability condition.

## SRE-004-DEFINITION-002

A Declared Constraint is not, by its existence alone:

- a policy requirement;
- an enforcement rule;
- an authorization decision;
- an access-control decision;
- a capability requirement;
- an objective;
- an assumption;
- a feasibility judgment;
- a legal conclusion;
- a canonical request element.

## SRE-004-DEFINITION-003 — Constraint representation

A **Constraint Representation** is one immutable instance by which a logical Declared Constraint is expressed, classified, scoped, evidenced, and related under this contract.

## SRE-004-DEFINITION-004 — Represented scope

**Represented Scope** identifies the objective, objective set, artifact, output, source, request domain, or conditional context to which a Declared Constraint is represented as applying.

## SRE-004-DEFINITION-005 — Declared priority

**Declared Priority** is a preserved representation of explicitly supplied or traceably proposed emphasis or ordering among constraints.

It is not adjudicated precedence.

---

# 6. Major concept and artifact classification

| Concept or artifact | Classification |
|---|---|
| Constraint representation doctrine | Constitutional concept |
| Constraint Representation Authority | Constitutional authority |
| Declared Constraint | Logical artifact |
| Constraint Representation | Logical artifact |
| Constraint relationship | Logical artifact |
| Represented Scope | Logical artifact |
| Constraint class and form entries | Registry-defined logical values |
| Constraint Representation Profile | Required runtime artifact or externally supplied normative profile |
| `DeclaredConstraintSet` | Required runtime artifact |
| `ConstraintRepresentationFailureRecord` | Required runtime artifact |
| Internal parser structures | Implementation convenience |

## SRE-004-CLASSIFICATION-001

A named constraint concept SHALL NOT automatically require a dedicated Rust type, module, file, database table, or serialized artifact unless this contract classifies it as a required runtime artifact.

---

# 7. Canonical inputs

**Specification level:** Required runtime behavior

## SRE-004-INPUT-001

A conforming Contract 004 operation SHALL consume:

- one committed `DeclaredObjectiveSet`;
- one admitted interpretation proposal set;
- the corresponding proposal admission decisions;
- one applicable `ConstraintRepresentationProfileId`;
- applicable constraint class, form, origin, basis, relationship, status, and scope-status registry versions;
- the source and operation identities needed for traceability.

## SRE-004-INPUT-002

Every consumed proposal SHALL have a structurally valid admission decision under Contract 002.

## SRE-004-INPUT-003

Contract 004 SHALL reject or fail any operation whose declared inputs cannot be deterministically associated with the same constitutional interpretation lineage.

## SRE-004-INPUT-004

Contract 004 SHALL preserve all consumed proposal, admission, source, objective-set, profile, and registry identifiers in the resulting committed artifact or failure record.

---

# 8. Constraint Representation Authority

**Specification level:** Constitutional authority

## SRE-004-AUTHORITY-001

The Constraint Representation Authority SHALL be the sole Contract 004 authority permitted to construct a `DeclaredConstraintSet`.

## SRE-004-AUTHORITY-002

The Constraint Representation Authority MAY:

- identify constraint-domain content supplied by admitted proposals;
- construct logical Declared Constraints;
- preserve explicit, inferred, application-supplied, and referenced-artifact origins;
- associate evidence and proposal references;
- classify constraints through applicable registries;
- preserve represented scope and scope uncertainty;
- preserve explicitly supplied or traceably proposed priority;
- preserve represented relationships and conflicts;
- construct atomic or composite forms as permitted by profile;
- produce a committed set or failure record.

## SRE-004-AUTHORITY-003

The Constraint Representation Authority SHALL NOT:

- create downstream policy;
- enforce constraints;
- determine feasibility;
- assign undeclared priority;
- adjudicate precedence;
- suppress conflicting constraints;
- choose a provider, tool, connector, model, or method;
- convert a capability need into a constraint;
- perform redaction or access control;
- authorize execution;
- establish canonical constraint state.

---

# 9. Declared Constraint identity

**Specification level:** Required runtime behavior

## SRE-004-ID-001

Every logical Declared Constraint SHALL possess a `DeclaredConstraintId`.

## SRE-004-ID-002

Every immutable representation instance SHALL possess a `ConstraintRepresentationId` distinct from its `DeclaredConstraintId`.

## SRE-004-ID-003

`DeclaredConstraintId` identifies logical continuity only.

`ConstraintRepresentationId` identifies the specific committed representation instance.

## SRE-004-ID-004

A later operation that changes any representation field, including expression, evidence, scope, class, form, status, priority, or relationship, SHALL issue a new `ConstraintRepresentationId`.

## SRE-004-ID-005

A later representation MAY retain the same `DeclaredConstraintId` only where logical continuity is explicitly supported by upstream association, an applicable representation profile, or a later constitutional reconciliation authority.

## SRE-004-ID-006

Contract 004 SHALL NOT infer semantic sameness across unrelated representations solely to reuse a `DeclaredConstraintId`.

---

# 10. Constraint origin

**Specification level:** Required observable attribute

## SRE-004-ORIGIN-001

Every Declared Constraint SHALL possess exactly one origin classification from the applicable origin registry.

## SRE-004-ORIGIN-002

The initial origin registry SHALL support at least:

```text
ExplicitSource
InterpreterInference
ApplicationSupplied
ReferencedArtifact
```

## SRE-004-ORIGIN-003

Origin SHALL answer who or what introduced the representation.

Origin SHALL NOT substitute for evidence.

## SRE-004-ORIGIN-004

An inferred constraint SHALL remain observably distinguishable from an explicit-source constraint throughout the SRE lifecycle.

## SRE-004-ORIGIN-005

Inference SHALL NOT increase constraint authority.

## SRE-004-ORIGIN-006

Application-supplied constraint content SHALL remain distinguishable from user-supplied source content and SHALL NOT be treated as downstream policy merely because the application supplied it.

---

# 11. Representation basis

**Specification level:** Required observable attribute

## SRE-004-BASIS-001

Every Constraint Representation SHALL possess at least one representation-basis classification.

## SRE-004-BASIS-002

The initial representation-basis registry SHALL support at least:

```text
DirectQuotation
StructuredExtraction
InterpreterSynthesis
ApplicationDeclaration
ReferencedArtifactDeclaration
```

## SRE-004-BASIS-003

Representation basis SHALL answer how the constraint was represented.

It SHALL remain distinct from origin and evidence.

---

# 12. Evidence association

**Specification level:** Required runtime behavior

## SRE-004-EVIDENCE-001

Every Declared Constraint SHALL preserve the evidence references required by the applicable `ConstraintRepresentationProfile`.

## SRE-004-EVIDENCE-002

Evidence references MAY identify:

- source spans;
- entire sources;
- admitted proposals;
- application declarations;
- referenced artifacts;
- structured fields;
- declared evidence-unavailability conditions permitted by profile.

## SRE-004-EVIDENCE-003

Contract 004 SHALL validate evidence-reference structure and association only.

It SHALL NOT adjudicate semantic sufficiency, truth, or policy adequacy of evidence.

## SRE-004-EVIDENCE-004

No Declared Constraint SHALL exist without an observable basis explaining why the representation exists.

---

# 13. Constraint Representation Profile

**Specification level:** Required runtime artifact or externally supplied normative profile

## SRE-004-PROFILE-001

Every `DeclaredConstraintSet` SHALL identify one `ConstraintRepresentationProfileId`.

## SRE-004-PROFILE-002

The profile MAY govern:

- supported class and form registries;
- permitted origin classes;
- permitted representation bases;
- evidence requirements;
- scope-attachment rules;
- decomposition behavior;
- composite-form behavior;
- inferred-constraint permissions;
- priority representation rules;
- empty-set behavior;
- permitted representation statuses.

## SRE-004-PROFILE-003

A Constraint Representation Profile MAY constrain representation behavior but SHALL NOT establish validity, precedence, enforcement, authorization, or policy authority.

## SRE-004-PROFILE-004

The profile identifier and version SHALL be preserved at set level and SHALL be available to downstream contracts without requiring reinterpretation of each constraint.

---

# 14. Constraint classes and forms

**Specification level:** Registry-defined logical values

## SRE-004-REGISTRY-001

Constraint classes and forms SHALL be registry-defined and extensible.

They SHALL NOT be treated as constitutionally exhaustive unless a later constitutional authority expressly restricts them.

## SRE-004-REGISTRY-002

The initial class registry SHOULD support at least:

```text
Content
Format
Temporal
Scope
Resource
Confidentiality
Quantity
Quality
Audience
Language
Ordering
Method
Location
Dependency
```

## SRE-004-REGISTRY-003

The initial form registry SHOULD support at least:

```text
RequiredInclusion
RequiredExclusion
MaximumBound
MinimumBound
ExactRequirement
ConditionalRequirement
Prohibition
ScopeRestriction
TemporalRestriction
FormatRestriction
ResourceRestriction
ConfidentialityRestriction
```

## SRE-004-REGISTRY-004

Class and form SHALL describe represented semantic structure only.

They SHALL NOT imply policy status, severity, precedence, enforcement mechanism, or runtime owner.

---

# 15. Constraint forms and semantic boundaries

## 15.1 Required inclusion

### SRE-004-FORM-001

A required inclusion SHALL represent content, structure, data, or other elements represented as required to be present.

## 15.2 Required exclusion

### SRE-004-FORM-002

A required exclusion SHALL represent content, structure, data, or other elements represented as required to be absent.

## 15.3 Prohibition

### SRE-004-FORM-003

A prohibition SHALL represent a negative limitation on an outcome characteristic, method, or included element.

## 15.4 Bounds

### SRE-004-FORM-004

Minimum, maximum, and exact bounds SHALL preserve their represented operator and value without converting them into feasibility judgments.

## 15.5 Conditional requirement

### SRE-004-FORM-005

A conditional constraint SHALL preserve its condition and SHALL NOT be treated as universally applicable when its condition is unresolved or false.

---

# 16. Represented scope

**Specification level:** Logical artifact with required observable attributes

## SRE-004-SCOPE-001

Every Declared Constraint SHALL possess Represented Scope.

## SRE-004-SCOPE-002

The applicable scope registry SHOULD support at least:

```text
RequestWide
ObjectiveSpecific
ObjectiveSetSpecific
ArtifactSpecific
OutputSpecific
SourceSpecific
Conditional
Unresolved
```

## SRE-004-SCOPE-003

Objective-specific scope SHALL reference one or more valid `DeclaredObjectiveId` values from the consumed `DeclaredObjectiveSet`.

## SRE-004-SCOPE-004

Absence of explicit scope SHALL NOT automatically create request-wide applicability.

## SRE-004-SCOPE-005

Unresolved, partial, or conflicting scope SHALL remain observable and SHALL NOT be silently widened.

## SRE-004-SCOPE-006

Contract 004 SHALL NOT use source proximity, field order, proposal order, or implementation convenience as sufficient authority to attach a constraint to an objective unless the applicable profile explicitly permits that representation rule.

---

# 17. Constraint scope resolution status

**Specification level:** Required observable attribute

## SRE-004-SCOPESTATUS-001

Every Declared Constraint SHALL possess one `ConstraintScopeResolutionStatus`.

## SRE-004-SCOPESTATUS-002

The initial scope-status registry SHALL support at least:

```text
Resolved
PartiallyResolved
Unresolved
Conflicting
NotApplicable
```

## SRE-004-SCOPESTATUS-003

Represented Scope and `ConstraintScopeResolutionStatus` SHALL remain separate attributes.

Represented Scope states what attachment is represented.

Scope-resolution status states whether that attachment is complete and unambiguous under the applicable profile.

---

# 18. Objective and constraint domain separation

**Specification level:** Constitutional boundary

## SRE-004-DOMAIN-001

Contract 004 SHALL NOT reclassify an outcome as a constraint solely because it appears with constraint-domain content.

## SRE-004-DOMAIN-002

Contract 004 SHALL NOT classify a capability need, ambiguity, assumption, preference, policy requirement, or execution method as a Declared Constraint solely because it limits an implementation choice.

## SRE-004-DOMAIN-003

Mixed statements MAY yield partial representations across multiple contracts.

## SRE-004-DOMAIN-004

For mixed statements, Contract 004 SHALL represent only the constraint-domain portion supported by admitted material and SHALL preserve traceability to the full source context.

---

# 19. Constraint decomposition and composition

**Specification level:** Constitutional concept and profile-governed behavior

## SRE-004-DECOMP-001

A Constraint Representation MAY be atomic or composite.

## SRE-004-DECOMP-002

Decomposition MAY occur only when:

- supplied by an admitted proposal;
- explicitly supplied by an application declaration;
- required or permitted by the applicable Constraint Representation Profile;
- fully traceable to evidence and representation basis.

## SRE-004-DECOMP-003

Constraint decomposition is representational and SHALL NOT establish semantic superiority, canonical granularity, or constitutional preference.

## SRE-004-DECOMP-004

No decomposition strategy is constitutionally preferred unless established by the applicable profile.

## SRE-004-DECOMP-005

Competing atomic and composite representations MAY coexist until later reconciliation.

---

# 20. Declared priority

**Specification level:** Logical artifact attribute

## SRE-004-PRIORITY-001

Declared priority MAY be represented only when explicitly supplied or traceably proposed under the applicable profile.

## SRE-004-PRIORITY-002

Every represented priority SHALL preserve its origin, evidence, representation basis, and target constraints.

## SRE-004-PRIORITY-003

Contract 004 SHALL NOT derive priority from:

- constraint class;
- source order;
- wording intensity;
- application location;
- data-field order;
- implementation preference;
- provider preference;
- presumed safety or business importance.

## SRE-004-PRIORITY-004

Declared priority SHALL NOT be converted into adjudicated precedence.

---

# 21. Constraint relationships

**Specification level:** Logical artifact

## SRE-004-RELATION-001

Constraint relationships SHALL be registry-defined and traceable to admitted material.

## SRE-004-RELATION-002

The initial relationship registry SHOULD support at least:

```text
AppliesTo
ConditionalUpon
OverridesAsDeclared
SubordinateToAsDeclared
ConflictsWith
Supports
Refines
ComponentOf
AlternativeTo
Requires
Excludes
```

## SRE-004-RELATION-003

A relationship containing `AsDeclared` SHALL preserve a represented relationship without constituting independent constitutional adjudication.

## SRE-004-RELATION-004

Contract 004 SHALL NOT infer a controlling relationship solely to make the constraint set easier to process.

---

# 22. Conflicting constraints

**Specification level:** Constitutional concept and logical artifact

## SRE-004-CONFLICT-001

Contract 004 MAY represent conflict among Declared Constraints when supported by admitted material or permitted analysis under the applicable profile.

## SRE-004-CONFLICT-002

The conflict registry MAY include:

```text
DirectContradiction
MutualExclusion
BoundaryConflict
ScopeConflict
PriorityConflict
ConditionalConflict
ResourceConflict
UnresolvedConflict
```

## SRE-004-CONFLICT-003

Conflicting constraints SHALL remain present in the committed set unless excluded for structural invalidity under this contract.

## SRE-004-CONFLICT-004

Contract 004 SHALL NOT resolve conflict, discard a competing constraint, merge constraints, invent a compromise, or select a winner.

## SRE-004-CONFLICT-005

A represented conflict SHALL NOT by itself make the Contract 004 operation fail.

---

# 23. Confidentiality constraints

**Specification level:** Constitutional boundary

## SRE-004-CONFIDENTIALITY-001

A confidentiality-class constraint MAY represent limitations concerning disclosure, exposure, audience, handling, or presentation.

## SRE-004-CONFIDENTIALITY-002

A confidentiality-class constraint SHALL NOT establish:

- legal confidentiality;
- data classification;
- access-control policy;
- redaction authority;
- disclosure authorization;
- source suppression;
- evidence deletion.

## SRE-004-CONFIDENTIALITY-003

Contract 004 SHALL NOT remove, redact, conceal, or restrict constitutional source material merely because a confidentiality-class constraint is represented.

---

# 24. Representation status

**Specification level:** Required observable attribute

## SRE-004-STATUS-001

Every Constraint Representation SHALL possess one representation status from the applicable registry.

## SRE-004-STATUS-002

The initial status registry SHALL support at least:

```text
Represented
Incomplete
Unsupported
Conflicting
EvidenceLimited
Unresolved
```

## SRE-004-STATUS-003

Representation status SHALL describe the quality or condition of the representation only.

It SHALL NOT describe validity, enforceability, authorization, feasibility, or policy acceptance.

## SRE-004-STATUS-004

Representation status and scope-resolution status SHALL remain distinct.

---

# 25. DeclaredConstraintSet

**Specification level:** Required runtime artifact

## SRE-004-SET-001

Every successful Contract 004 operation SHALL produce exactly one committed `DeclaredConstraintSet`.

## SRE-004-SET-002

A `DeclaredConstraintSet` SHALL contain at least:

```text
DeclaredConstraintSetId
ConstraintRepresentationOperationId
InterpretationOperationId
SourceIntakeRecordId
DeclaredObjectiveSetId
ConstraintRepresentationProfileId
ApplicableRegistryVersions
InputProposalIds
InputAdmissionDecisionIds
DeclaredConstraints
ConstraintRelationships
SetLevelFindings
EvidenceReferences
ProvenanceReferences
CommitmentMetadata
```

## SRE-004-SET-003

Each Declared Constraint representation SHALL contain at least:

```text
DeclaredConstraintId
ConstraintRepresentationId
ConstraintExpression
ConstraintClass
ConstraintForm
Origin
RepresentationBasis
EvidenceReferences
ProposalReferences
RepresentedScope
ConstraintScopeResolutionStatus
DeclaredPriority
ConstraintRelationships
RepresentationStatus
UncertaintyReferences
```

## SRE-004-SET-004

The set SHALL preserve all identifiers necessary to reconstruct its constitutional lineage.

## SRE-004-SET-005

A set containing zero Declared Constraints MAY be valid when permitted by the applicable profile.

---

# 26. Empty-set behavior

**Specification level:** Required runtime behavior

## SRE-004-EMPTY-001

A successful empty `DeclaredConstraintSet` SHALL mean that no constraint-domain content was represented under the applicable profile and admitted inputs.

## SRE-004-EMPTY-002

An empty set SHALL remain distinguishable from a failed representation operation.

## SRE-004-EMPTY-003

The absence of represented constraints SHALL NOT imply the absence of downstream policy, capability limitations, execution limits, or governance obligations.

---

# 27. Construction lifecycle

**Specification level:** Required runtime behavior

## SRE-004-LIFECYCLE-001

A Contract 004 operation SHALL proceed through deterministic phases equivalent to:

```text
Input Association
        ↓
Profile and Registry Validation
        ↓
Constraint-Domain Collection
        ↓
Identity Construction
        ↓
Origin, Basis, Evidence, Class, Form, and Scope Association
        ↓
Relationship and Status Construction
        ↓
Set Validation
        ↓
Atomic Commitment or Failure Commitment
```

## SRE-004-LIFECYCLE-002

Internal phase names and implementation structures MAY differ, provided the constitutional effects are equivalent and verifiable.

## SRE-004-LIFECYCLE-003

No intermediate construction state SHALL be treated as a committed Declared Constraint or committed `DeclaredConstraintSet`.

---

# 28. Failure model

**Specification level:** Required runtime artifact and behavior

## SRE-004-FAILURE-001

A Contract 004 operation SHALL fail when it cannot deterministically produce a structurally valid `DeclaredConstraintSet` under the applicable contract, profile, and registries.

## SRE-004-FAILURE-002

The following conditions SHALL NOT automatically constitute operation failure:

- no represented constraints;
- incomplete constraint content;
- unsupported inferred constraints;
- conflicting constraints;
- unresolved priority;
- unresolved or partial scope;
- impossible combinations;
- evidence limitations permitted by profile.

## SRE-004-FAILURE-003

A failed operation SHALL produce exactly one `ConstraintRepresentationFailureRecord`.

## SRE-004-FAILURE-004

A failure record SHALL contain at least:

```text
ConstraintRepresentationFailureRecordId
ConstraintRepresentationOperationId
InterpretationOperationId
DeclaredObjectiveSetId
ConstraintRepresentationProfileId
InputProposalIds
InputAdmissionDecisionIds
ApplicableContractVersion
ApplicableRegistryVersions
FailureCode
FailureFindings
ObservedAt
CommitmentMetadata
```

## SRE-004-FAILURE-005

A failed operation SHALL NOT emit a partially authoritative `DeclaredConstraintSet`.

---

# 29. Atomic commitment and immutability

**Specification level:** Constitutional commitment rule

## SRE-004-COMMIT-001

Every completed Contract 004 operation SHALL commit exactly one authoritative outcome:

```text
DeclaredConstraintSet
```

or:

```text
ConstraintRepresentationFailureRecord
```

It SHALL NOT commit both for the same operation.

## SRE-004-COMMIT-002

A committed `DeclaredConstraintSet`, Declared Constraint representation, and failure record SHALL be immutable.

## SRE-004-COMMIT-003

Corrections, added evidence, changed scope, changed classification, or changed relationships SHALL occur through a new operation and new representation identity rather than in-place mutation.

## SRE-004-COMMIT-004

A later artifact MAY supersede or administratively relate to an earlier artifact, but it SHALL NOT erase the historical fact of the earlier commitment.

---

# 30. Prohibited transformations

**Specification level:** Constitutional prohibition

Contract 004 SHALL NOT:

## SRE-004-PROHIBIT-001

invent downstream policy;

## SRE-004-PROHIBIT-002

convert application configuration into policy authority;

## SRE-004-PROHIBIT-003

determine feasibility, legality, safety, acceptability, or enforceability;

## SRE-004-PROHIBIT-004

enforce represented constraints;

## SRE-004-PROHIBIT-005

assign undeclared priority or adjudicated precedence;

## SRE-004-PROHIBIT-006

resolve, suppress, or silently merge conflicting constraints;

## SRE-004-PROHIBIT-007

choose tools, providers, connectors, models, execution methods, or resources;

## SRE-004-PROHIBIT-008

transform capability needs into Declared Constraints;

## SRE-004-PROHIBIT-009

authorize planning, generation, execution, disclosure, or release;

## SRE-004-PROHIBIT-010

perform redaction, access-control, or evidence-suppression decisions;

## SRE-004-PROHIBIT-011

establish canonical constraint state;

## SRE-004-PROHIBIT-012

silently widen unresolved scope;

## SRE-004-PROHIBIT-013

prefer one decomposition solely because an implementation finds it easier to process.

---

# 31. Downstream handoff

**Specification level:** Constitutional boundary

## SRE-004-HANDOFF-001

A committed `DeclaredConstraintSet` MAY be consumed by later SRE contracts as an immutable representation artifact.

## SRE-004-HANDOFF-002

Downstream consumption SHALL preserve the set identity, representation identities, logical identities, evidence references, origin, basis, profile, registry versions, scope, status, and commitment lineage.

## SRE-004-HANDOFF-003

Downstream contracts MAY reconcile, normalize, validate, adjudicate contradiction, or construct canonical request elements only under their own expressly assigned authority.

## SRE-004-HANDOFF-004

Handoff of a `DeclaredConstraintSet` SHALL NOT imply that any constraint is valid, binding, feasible, enforceable, authorized, or preferred.

---

# 32. Deferred responsibilities

**Specification level:** Constitutional boundary

## SRE-004-DEFER-001

Contract 004 SHALL defer capability requirements to Contract 005.

## SRE-004-DEFER-002

Contract 004 SHALL defer ambiguities, assumptions, uncertainty, and clarification requirements to Contract 006.

## SRE-004-DEFER-003

Contract 004 SHALL defer consolidated evidence semantics to Contract 007.

## SRE-004-DEFER-004

Contract 004 SHALL defer provenance and lineage consolidation to Contract 008.

## SRE-004-DEFER-005

Contract 004 SHALL defer competing-proposal reconciliation and semantic equivalence to Contract 009.

## SRE-004-DEFER-006

Contract 004 SHALL defer semantic normalization to Contract 010.

## SRE-004-DEFER-007

Contract 004 SHALL defer canonical ordering to Contract 011.

## SRE-004-DEFER-008

Contract 004 SHALL defer structural validation to Contract 012.

## SRE-004-DEFER-009

Contract 004 SHALL defer contradiction and completeness adjudication to Contract 013.

## SRE-004-DEFER-010

Contract 004 SHALL defer canonical request construction to Contract 014.

## SRE-004-DEFER-011

Contract 004 SHALL defer downstream authorization, policy enforcement, execution, generation, provider selection, tool invocation, and release outside the Structured Request Engine.

---

# 33. Fundamental invariants

## SRE-004-INVARIANT-001 — Representation invariant

Constraint representation records constraint-domain content without establishing policy, feasibility, precedence, enforcement, or canonical constraint state.

## SRE-004-INVARIANT-002 — Origin invariant

Every Declared Constraint SHALL preserve whether it originated from explicit source, interpreter inference, application-supplied information, or a referenced artifact.

## SRE-004-INVARIANT-003 — Evidence invariant

Every Declared Constraint SHALL preserve an observable basis explaining why the representation exists.

## SRE-004-INVARIANT-004 — Scope invariant

Constraint applicability SHALL remain explicit, evidence-associated, and bounded; unresolved scope SHALL not be silently widened.

## SRE-004-INVARIANT-005 — Priority invariant

Declared priority may be preserved but SHALL NOT be manufactured or converted into adjudicated precedence.

## SRE-004-INVARIANT-006 — Registry invariant

Constraint classes, forms, relationships, and statuses SHALL remain registry-defined and extensible unless constitutionally restricted elsewhere.

## SRE-004-INVARIANT-007 — Identity invariant

Logical constraint identity SHALL remain distinct from immutable representation identity.

## SRE-004-INVARIANT-008 — Authority invariant

No Declared Constraint may create authorization, execution authority, policy authority, tool authority, provider authority, disclosure authority, or release authority.

## SRE-004-INVARIANT-009 — Immutability invariant

Committed constraint representations and committed Contract 004 outcomes SHALL be immutable.

---

# 34. Conformance requirements

**Specification level:** Conformance obligation

## SRE-004-CONFORMANCE-001

A conforming implementation SHALL construct `DeclaredConstraintSet` artifacts and failure records according to this contract.

## SRE-004-CONFORMANCE-002

A conforming implementation SHALL preserve the distinction among:

```text
Declared Constraint
Policy Requirement
Enforcement Rule
```

## SRE-004-CONFORMANCE-003

A conforming implementation SHALL preserve the distinction among:

```text
Origin
Representation Basis
Evidence
```

## SRE-004-CONFORMANCE-004

A conforming implementation SHALL preserve the distinction among:

```text
Declared Priority
Adjudicated Precedence
```

## SRE-004-CONFORMANCE-005

A conforming implementation SHALL preserve the distinction among:

```text
Represented Scope
Constraint Scope Resolution Status
```

## SRE-004-CONFORMANCE-006

A conforming implementation SHALL preserve the distinction among:

```text
DeclaredConstraintId
ConstraintRepresentationId
DeclaredConstraintSetId
ConstraintRepresentationOperationId
```

## SRE-004-CONFORMANCE-007

A conforming implementation SHALL demonstrate through tests that conflicting, incomplete, unsupported, evidence-limited, and scope-unresolved constraints can remain represented without automatic policy or enforcement effects.

## SRE-004-CONFORMANCE-008

A conforming implementation SHALL demonstrate that committed outcomes are immutable and that revisions create new operation and representation identities.

## SRE-004-CONFORMANCE-009

A conforming implementation SHALL demonstrate that no Contract 004 operation can authorize execution, choose a provider, impose policy, or release output.

---

# 35. Constitutional summary

Contract 004 establishes the constitutional home of represented constraints.

Its authoritative progression is:

```text
Admitted Interpretation Proposals
        +
Declared Objectives
        +
Constraint Representation Profile
        +
Applicable Registries
        │
        ▼
Constraint Representation Authority
        │
        ▼
DeclaredConstraintSet
        │
        ▼
Later Reconciliation, Normalization,
Validation, Contradiction Analysis,
Canonical Construction, and Governance
```

Contract 004 does not create binding rules.

It creates an immutable, evidence-associated, scope-aware representation of what limitations, inclusions, exclusions, conditions, and boundaries are represented within the submission context.

Its controlling constitutional principle is:

> **Represent faithfully. Preserve uncertainty. Defer judgment. Never allow representation to become governance.**
