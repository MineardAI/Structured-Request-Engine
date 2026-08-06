# Structured Request Engine

## Contract 012 — Structural Validation

**Document ID:** `SRE-CONTRACT-012`  
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
- `SRE-CONTRACT-009 v0.1.0`
- `SRE-CONTRACT-010 v0.1.0`
- `SRE-CONTRACT-011 v0.1.0`
- `SRE-CONTRACT-PLAN v2.1.0`

---

## Normative requirement identifiers

Every normative `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, and `MAY` statement in this contract shall possess a stable requirement identifier.

Requirement identifiers use:

```text
SRE-012-<DOMAIN>-<SEQUENCE>
```

Examples:

```text
SRE-012-VALIDATE-001
SRE-012-FINDING-004
SRE-012-RESULT-003
```

Requirement identifiers exist solely for traceability, implementation verification, conformance testing, amendment tracking, and cross-contract dependency analysis.

Requirement identifiers SHALL NOT alter the normative meaning of any requirement.

---

# 1. Architectural context

**Specification level:** Constitutional concept

## SRE-012-CONTEXT-001

Contract 011 establishes the deterministic canonical arrangement of normalized request elements and relationships.

Contract 012 evaluates whether that canonically ordered representation satisfies the structural conditions required to enter canonical request construction.

## SRE-012-CONTEXT-002

The neighboring constitutional sequence SHALL remain:

```text
Contract 011
Canonical Ordering
        ↓
Contract 012
Structural Validation
        ↓
Contract 013
Canonical Request Construction
        ↓
Contract 014
Identity and Issuance
        ↓
Contract 015
Downstream Handoff
```

## SRE-012-CONTEXT-003

Contract 012 SHALL validate the integrity and construction eligibility of the publication produced by Contract 011.

It SHALL NOT establish, revise, recompute, repair, or replace canonical ordering.

## SRE-012-CONTEXT-004

Contract 012 is the final structural gate before canonical request construction.

It SHALL NOT perform canonical request construction.

---

# 2. Purpose

**Specification level:** Constitutional concept

## SRE-012-PURPOSE-001

This contract establishes the constitutional rules by which one immutable `CanonicallyOrderedRequestRepresentation` may be evaluated under declared validation profiles, construction requirements, schemas, registries, and rule sets to determine whether that representation is structurally eligible for canonical request construction.

## SRE-012-PURPOSE-002

This contract defines:

- the Structural Validation Authority;
- Structural Validation operations;
- Validation Rule Applications;
- Validation Findings;
- Validation Decisions;
- validation completion status;
- structural eligibility status;
- finding severity;
- construction-requirement binding;
- validation profile binding;
- schema, registry, and rule-set binding;
- required structural presence;
- identifier integrity;
- relationship integrity;
- ordering integrity;
- cardinality integrity;
- construction prerequisite validation;
- explicit non-repair behavior;
- completed negative results;
- deferred eligibility;
- successful result publication;
- failure outcomes;
- atomic commitment;
- immutable publication;
- downstream handoff;
- deferred responsibilities.

## SRE-012-PURPOSE-003

The organizing doctrine of this contract is:

> **Structure determines eligibility. Eligibility does not determine meaning.**

## SRE-012-PURPOSE-004

The operational doctrine of this contract is:

> **Structural ineligibility is a completed validation result. It is not a validation-operation failure.**

## SRE-012-PURPOSE-005

This contract SHALL answer only:

> **Is the canonically ordered representation structurally eligible for canonical request construction?**

## SRE-012-PURPOSE-006

This contract SHALL NOT answer:

- whether represented meaning is true;
- whether represented meaning is correct;
- whether the request is semantically complete;
- whether the request is safe;
- whether the request is lawful;
- whether the request is authorized;
- whether the request is feasible;
- whether the request is executable;
- whether canonical construction will ultimately succeed;
- whether the request may be issued;
- whether the request may be transferred downstream.

---

# 3. Governing constitutional elements

**Specification level:** Constitutional concept

## SRE-012-GOVERNING-001

The governing constitutional question is:

> **Is the canonically ordered representation structurally eligible for canonical request construction?**

## SRE-012-GOVERNING-002

The constitutional subject is:

> **The profile-governed structural eligibility of a canonically ordered request representation.**

## SRE-012-GOVERNING-003

The constitutional act is:

> **Validate**

## SRE-012-GOVERNING-004

The principal successful publication is:

```text
StructuralValidationResult
```

## SRE-012-GOVERNING-005

The principal failed publication is:

```text
StructuralValidationFailureRecord
```

---

# 4. Architectural identity

**Specification level:** Constitutional concept

## SRE-012-IDENTITY-001

Contract 012 establishes the Structural Validation domain of the Structured Request Engine.

## SRE-012-IDENTITY-002

The constitutional transformation governed by this contract is:

```text
CanonicallyOrderedRequestRepresentation
        +
ValidationProfile
        +
ApplicableConstructionRequirements
        +
ApplicableSchemas
        +
ApplicableStructuralRegistries
        +
ApplicableValidationRuleSet
        │
        ▼
StructuralValidationAuthority
        │
        ├── completed ──► StructuralValidationResult
        │
        └── failed ─────► StructuralValidationFailureRecord
```

## SRE-012-IDENTITY-003

A completed `StructuralValidationResult` MAY determine:

```text
Eligible
EligibleWithWarnings
Ineligible
Deferred
```

## SRE-012-IDENTITY-004

An `Ineligible` result SHALL remain a completed validation result.

It SHALL NOT be converted into or represented as validation-operation failure.

## SRE-012-IDENTITY-005

Contract 012 SHALL operate only over immutable committed upstream artifacts and declared validation inputs.

---

# 5. Organizing constitutional doctrines

**Specification level:** Constitutional concept

## 5.1 Structure is not meaning

### SRE-012-DOCTRINE-001

Structural Validation SHALL determine structural eligibility only.

It SHALL NOT determine semantic truth, correctness, quality, safety, authorization, executability, or fitness for purpose.

## 5.2 Structural presence is not semantic completeness

### SRE-012-DOCTRINE-002

The following distinction SHALL remain explicit:

```text
Required Structural Presence
        ≠
Semantic Completeness
```

A representation MAY satisfy all required structural presence rules while retaining unresolved semantic ambiguity, assumptions, uncertainty, or preserved conflict where the applicable profile permits those states.

## 5.3 No repair

### SRE-012-DOCTRINE-003

Structural Validation SHALL observe and determine.

It SHALL NOT repair, supplement, reinterpret, reconcile, normalize, reorder, reconstruct, replace, default, or otherwise mutate the representation being validated.

## 5.4 Negative result is not operation failure

### SRE-012-DOCTRINE-004

A determination of structural ineligibility SHALL constitute a completed validation result.

It SHALL NOT, by itself, constitute validation-operation failure.

## 5.5 External rule authority

### SRE-012-DOCTRINE-005

Contract 012 SHALL apply authoritative validation rules identified by applicable profiles, schemas, registries, construction requirements, and rule sets.

Contract 012 SHALL NOT create validation authority during evaluation.

## 5.6 Finding-decision separation

### SRE-012-DOCTRINE-006

A `ValidationFinding` SHALL represent an observed structural condition.

A `ValidationDecision` SHALL represent the governed constitutional effect of one or more findings.

## 5.7 Construction separation

### SRE-012-DOCTRINE-007

Structural eligibility permits entry into canonical construction.

It does not construct the request and does not guarantee construction success.

## 5.8 Ordering limitation

### SRE-012-DOCTRINE-008

Contract 012 MAY determine whether the canonical arrangement published by Contract 011 is intact and internally consistent.

It SHALL NOT derive, revise, recompute, replace, or repair that arrangement.

## 5.9 Identity limitation

### SRE-012-DOCTRINE-009

Contract 012 MAY evaluate identifier presence, syntax, uniqueness, type consistency, and reference integrity.

It SHALL NOT issue canonical request identity.

## 5.10 Authority non-expansion

### SRE-012-DOCTRINE-010

No validation finding, decision, result, warning, exception, eligibility state, profile, schema, registry, or rule application created under this contract SHALL expand downstream operational authority.

## 5.11 No hidden validation inputs

### SRE-012-DOCTRINE-011

No hidden prompt, ambient runtime state, provider default, undocumented session context, mutable global, live tool inventory, operator preference, or unrecorded assumption SHALL influence a validation result.

## 5.12 Upstream immutability

### SRE-012-DOCTRINE-012

Contract 012 SHALL preserve the identity, content, meaning, canonical expression, canonical ordering, evidence, grounding, provenance, and downstream standing of every consumed upstream artifact.

---

# 6. Constitutional authority

**Specification level:** Constitutional authority

## 6.1 Structural Validation Authority

### SRE-012-AUTHORITY-001

The Structural Validation Authority SHALL be the exclusive constitutional owner of the transformation governed by this contract.

### SRE-012-AUTHORITY-002

The Structural Validation Authority MAY:

- identify the representation subject to validation;
- resolve applicable validation profiles;
- resolve applicable construction requirements;
- resolve applicable schemas, structural registries, and validation rule sets;
- construct Validation Rule Applications;
- execute authoritative validation rules;
- construct Validation Findings;
- classify finding severity;
- construct Validation Decisions;
- determine structural eligibility;
- publish one `StructuralValidationResult`;
- publish one `StructuralValidationFailureRecord` when the validation act cannot complete.

### SRE-012-AUTHORITY-003

The Structural Validation Authority SHALL NOT:

- reinterpret semantic meaning;
- alter reconciliation decisions;
- alter canonical expression;
- alter canonical ordering;
- create missing semantic content;
- create missing evidence or provenance;
- repair identifiers or relationships;
- inject defaults that materially alter meaning;
- construct a canonical request;
- assign canonical request identity;
- authorize issuance;
- authorize transfer;
- authorize planning, execution, generation, or release.

## 6.2 Exclusive publication authority

### SRE-012-AUTHORITY-004

Only the Structural Validation Authority MAY commit a `StructuralValidationResult` or `StructuralValidationFailureRecord` under this contract.

### SRE-012-AUTHORITY-005

Profiles, schemas, registries, construction requirements, applications, upstream contracts, and downstream consumers MAY supply constitutional inputs.

They SHALL NOT directly publish Contract 012 outcomes unless they are themselves the recognized Structural Validation Authority for the operation.

---

# 7. Major concept and artifact classification

**Specification level:** Constitutional concept

| Concept or artifact | Classification |
|---|---|
| Structural Validation doctrine | Constitutional concept |
| Structural Validation Authority | Constitutional authority |
| Structural Validation | Logical constitutional activity |
| Validation Rule Application | Required runtime artifact |
| Validation Finding | Required runtime artifact |
| Validation Decision | Required runtime artifact |
| Validation Completion | Required result dimension |
| Structural Eligibility | Required result dimension |
| Finding Severity | Required finding dimension |
| Validation Profile | Required runtime artifact or externally supplied normative profile |
| Construction Requirements Reference | Required runtime reference |
| `StructuralValidationResult` | Required runtime artifact |
| `StructuralValidationFailureRecord` | Required runtime artifact |
| Internal validation engine structures | Implementation convenience |

## SRE-012-CLASSIFICATION-001

A named concept SHALL NOT automatically require a dedicated Rust type, module, file, service, database table, or top-level serialized artifact unless this contract classifies it as a required runtime artifact.

## SRE-012-CLASSIFICATION-002

`ValidationFinding` and `ValidationDecision` SHALL remain independently identifiable and auditable even when physically embedded within a `StructuralValidationResult`.

---

# 8. Canonical inputs

**Specification level:** Required runtime behavior

## SRE-012-INPUT-001

A conforming Structural Validation operation SHALL consume:

- one committed `CanonicallyOrderedRequestRepresentation`;
- one applicable `ValidationProfileId` and exact profile version;
- one applicable construction-requirements reference;
- applicable schema identities and versions;
- applicable structural-registry identities and versions;
- one applicable validation rule-set identity and version;
- configuration snapshot references when they materially affect validation;
- applicable contract and implementation version context.

## SRE-012-INPUT-002

The validated representation SHALL possess immutable identity and SHALL be the exact publication produced by Contract 011 or an equivalent publication explicitly recognized by the governing contract family.

## SRE-012-INPUT-003

Every input that materially affects validation SHALL be declared, versioned, traceable, and replayable.

## SRE-012-INPUT-004

Contract 012 SHALL reject or fail any operation whose declared inputs cannot be deterministically associated with one validation context.

## SRE-012-INPUT-005

The construction-requirements reference SHALL identify the exact structural construction conditions against which eligibility is evaluated.

## SRE-012-INPUT-006

Contract 012 SHALL NOT require a separate Construction Eligibility Profile unless a later adopted architecture or Contract 013 establishes that independent profile governance is necessary.

---

# 9. Structural Validation operation

**Specification level:** Required runtime behavior

## SRE-012-OPERATION-001

Every Structural Validation operation SHALL possess exactly one immutable `StructuralValidationId`.

## SRE-012-OPERATION-002

The operation identity SHALL correlate:

```text
StructuralValidationId
├── CanonicallyOrderedRequestRepresentationId
├── ValidationProfileId and Version
├── ConstructionRequirementsReference
├── Schema Identities and Versions
├── Structural Registry Identities and Versions
├── Validation Rule Set Identity and Version
├── ValidationRuleApplicationIds
├── ValidationFindingIds
├── ValidationDecisionIds
├── StructuralValidationResultId, on completion
└── StructuralValidationFailureRecordId, on failure
```

## SRE-012-OPERATION-003

The operation identity SHALL NOT replace the identity of any consumed or produced artifact.

## SRE-012-OPERATION-004

Equivalent declared inputs processed under equivalent contract, profile, schema, registry, rule-set, construction-requirement, and configuration versions SHALL produce equivalent constitutional outcomes.

---

# 10. Runtime object model

**Specification level:** Required runtime architecture

## SRE-012-OBJECT-001

Contract 012 defines the following primary runtime roles:

1. `StructuralValidation` owns one governed validation activity;
2. `ValidationRuleApplication` owns one execution of an authoritative validation rule;
3. `ValidationFinding` owns one observed structural condition;
4. `ValidationDecision` owns one governed determination of the constitutional effect of one or more findings;
5. `StructuralValidationResult` owns one completed structural-eligibility publication;
6. `StructuralValidationFailureRecord` owns one failed validation-operation publication.

No runtime role SHALL assume the constitutional responsibility of another.

## 10.1 Structural Validation

### SRE-012-OBJECT-002

A Structural Validation is one governed activity that evaluates one canonically ordered request representation under one declared validation context.

## 10.2 Validation Rule Application

### SRE-012-OBJECT-003

A `ValidationRuleApplication` is one immutable record of the execution of one authoritative validation rule against one or more identified structural subjects.

### SRE-012-OBJECT-004

Every `ValidationRuleApplication` SHALL possess one immutable `ValidationRuleApplicationId`.

### SRE-012-OBJECT-005

A Validation Rule Application SHALL identify:

- the authoritative rule reference;
- the validation profile reference;
- applicable schema or registry reference;
- applicable construction requirement;
- subject references;
- evaluation method;
- rule version;
- deterministic input context;
- completion status.

## 10.3 Validation Finding

### SRE-012-OBJECT-006

A `ValidationFinding` is one immutable representation of an observed structural condition produced through one or more Validation Rule Applications.

### SRE-012-OBJECT-007

Every Validation Finding SHALL possess one immutable `ValidationFindingId`.

### SRE-012-OBJECT-008

A Validation Finding SHALL identify at least:

- rule-application reference;
- subject reference;
- finding type;
- observed value or condition;
- expected condition;
- finding severity;
- structural location;
- relevant evidence or trace reference when applicable.

## 10.4 Validation Decision

### SRE-012-OBJECT-009

A `ValidationDecision` is one immutable constitutional artifact recording the effect of one or more Validation Findings under the applicable profile, rule, and construction requirement.

### SRE-012-OBJECT-010

Every Validation Decision SHALL possess one immutable `ValidationDecisionId`.

### SRE-012-OBJECT-011

A Validation Decision SHALL identify at least:

- contributing finding references;
- applicable requirement reference;
- decision type;
- eligibility effect;
- profile basis;
- exception reference when applicable;
- deterministic rationale or rule basis.

## 10.5 Structural Validation Result

### SRE-012-OBJECT-012

A `StructuralValidationResult` is the immutable committed artifact produced when the Structural Validation Authority completes the governed validation act.

### SRE-012-OBJECT-013

Every Structural Validation Result SHALL possess one immutable `StructuralValidationResultId`.

### SRE-012-OBJECT-014

A Structural Validation Result MAY publish structural eligibility as `Eligible`, `EligibleWithWarnings`, `Ineligible`, or `Deferred`.

## 10.6 Structural Validation Failure Record

### SRE-012-OBJECT-015

A `StructuralValidationFailureRecord` is the immutable committed artifact produced when the Structural Validation Authority cannot complete the governed validation act.

### SRE-012-OBJECT-016

Every Structural Validation Failure Record SHALL possess one immutable `StructuralValidationFailureRecordId`.

### SRE-012-OBJECT-017

A Structural Validation Failure Record SHALL publish:

```text
ValidationCompletion = Failed
StructuralEligibility = NotDetermined
```

---

# 11. Identity doctrine

**Specification level:** Constitutional concept

## SRE-012-ID-001

The following identities SHALL remain distinct:

```text
StructuralValidationId
        ≠
ValidationRuleApplicationId
        ≠
ValidationFindingId
        ≠
ValidationDecisionId
        ≠
StructuralValidationResultId
        ≠
StructuralValidationFailureRecordId
        ≠
CanonicallyOrderedRequestRepresentationId
```

## SRE-012-ID-002

`StructuralValidationId` SHALL identify one governed validation activity.

## SRE-012-ID-003

`ValidationRuleApplicationId` SHALL identify one immutable application of an authoritative rule.

## SRE-012-ID-004

`ValidationFindingId` SHALL identify one immutable observed structural condition.

## SRE-012-ID-005

`ValidationDecisionId` SHALL identify one immutable governed determination.

## SRE-012-ID-006

`StructuralValidationResultId` SHALL identify one immutable completed validation publication.

## SRE-012-ID-007

`StructuralValidationFailureRecordId` SHALL identify one immutable failed validation publication.

## SRE-012-ID-008

Identity equality SHALL NOT imply semantic truth, correctness, completeness, authorization, executability, or request identity.

---

# 12. Three-dimensional result model

**Specification level:** Required runtime behavior

## 12.1 Validation Completion

### SRE-012-COMPLETION-001

Every Contract 012 operation SHALL publish exactly one Validation Completion state:

```text
Completed
Failed
```

### SRE-012-COMPLETION-002

`Completed` SHALL mean the Structural Validation Authority completed the governed act and published one `StructuralValidationResult`.

### SRE-012-COMPLETION-003

`Failed` SHALL mean the Structural Validation Authority could not complete the governed act and published one `StructuralValidationFailureRecord`.

## 12.2 Structural Eligibility

### SRE-012-ELIGIBILITY-001

Every completed Structural Validation Result SHALL publish exactly one Structural Eligibility state:

```text
Eligible
EligibleWithWarnings
Ineligible
Deferred
```

### SRE-012-ELIGIBILITY-002

A failed validation operation SHALL publish:

```text
StructuralEligibility = NotDetermined
```

### SRE-012-ELIGIBILITY-003

`NotDetermined` SHALL NOT be a completed eligibility outcome.

## 12.3 Finding Severity

### SRE-012-SEVERITY-001

Every Validation Finding SHALL possess exactly one severity:

```text
Informational
Warning
Blocking
```

### SRE-012-SEVERITY-002

Severity SHALL be determined by the applicable authoritative profile, rule, schema, registry, or construction requirement.

It SHALL NOT be assigned through implementation discretion when that discretion could alter eligibility.

### SRE-012-SEVERITY-003

Finding severity alone SHALL NOT determine top-level eligibility without an explicit Validation Decision.

---

# 13. Eligibility states

**Specification level:** Required runtime behavior

## 13.1 Eligible

### SRE-012-ELIGIBLE-001

`Eligible` SHALL mean all mandatory structural construction prerequisites are satisfied and no non-blocking condition requires warning publication.

## 13.2 EligibleWithWarnings

### SRE-012-ELIGIBLE-002

`EligibleWithWarnings` SHALL mean:

```text
All mandatory construction prerequisites satisfied
        +
One or more non-blocking findings remain
```

### SRE-012-ELIGIBLE-003

`EligibleWithWarnings` SHALL NOT be published where any mandatory structural requirement remains unsatisfied.

## 13.3 Ineligible

### SRE-012-INELIGIBLE-001

`Ineligible` SHALL mean at least one mandatory structural requirement is unsatisfied.

### SRE-012-INELIGIBLE-002

`Ineligible` SHALL constitute a completed validation result.

## 13.4 Deferred

### SRE-012-DEFERRED-001

`Deferred` SHALL be published only when the applicable Validation Profile or construction requirements explicitly authorize completion of the validation act without a final eligibility determination.

### SRE-012-DEFERRED-002

`Deferred` MAY represent:

- a profile-authorized postponement of final eligibility determination;
- a profile-authorized non-blocking structural condition intentionally deferred to a later governed stage.

### SRE-012-DEFERRED-003

`Deferred` SHALL NOT be used where required validation could not be completed because of:

- missing validation profiles;
- unresolved schemas;
- unresolved registries;
- missing rule sets;
- unavailable required dependencies;
- rule execution failure;
- nondeterministic rule behavior;
- validation engine failure.

Those conditions SHALL produce a Structural Validation Failure Record.

---

# 14. Validation Finding types

**Specification level:** Required runtime registry

## SRE-012-FINDING-001

Validation Finding types SHALL be registry-defined and extensible.

## SRE-012-FINDING-002

The initial Finding Type registry SHALL include at least:

```text
MissingRequiredStructure
SchemaViolation
RegistryConsistencyViolation
IdentifierIntegrityViolation
RelationshipIntegrityViolation
OrderingIntegrityViolation
CardinalityViolation
UnresolvedRequiredReference
ForbiddenStructuralElement
ConstructionPrerequisiteViolation
DuplicateStructuralPosition
UnexpectedStructuralElement
ProfilePermittedException
```

## SRE-012-FINDING-003

A finding type SHALL describe an observed structural condition only.

It SHALL NOT by itself determine semantic correctness or authorization.

---

# 15. Validation Decision model

**Specification level:** Required runtime behavior

## SRE-012-DECISION-001

Every condition affecting structural eligibility SHALL be resolved through one or more explicit Validation Decisions.

## SRE-012-DECISION-002

A Validation Decision SHALL determine at least one of:

```text
RequirementSatisfied
RequirementUnsatisfied
RequirementSatisfiedWithWarning
ProfileExceptionApplied
EligibilityDeferred
NoEligibilityEffect
```

## SRE-012-DECISION-003

A Validation Decision SHALL preserve the rule, profile, schema, registry, construction requirement, and finding basis from which it was derived.

## SRE-012-DECISION-004

One Validation Decision MAY aggregate multiple related findings only when the applicable profile or rule model expressly permits that aggregation.

## SRE-012-DECISION-005

A Validation Decision SHALL NOT mutate any upstream artifact.

---

# 16. Required structural presence

**Specification level:** Required runtime behavior

## SRE-012-STRUCTURE-001

Contract 012 MAY validate the presence of required:

- containers;
- sections;
- collections;
- fields;
- relationship records;
- ordering assignments;
- references;
- profile identifiers;
- version identifiers;
- structural markers;
- construction prerequisites.

## SRE-012-STRUCTURE-002

Required structural presence SHALL NOT be interpreted as proof that all intended semantic content has been represented.

## SRE-012-STRUCTURE-003

Contract 012 SHALL NOT invent missing content merely to satisfy required structural presence.

---

# 17. Identifier integrity

**Specification level:** Required runtime behavior

## SRE-012-IDENTIFIER-001

Contract 012 MAY validate:

- required identifier presence;
- identifier syntax;
- identifier type consistency;
- uniqueness within required scopes;
- reference resolution;
- object-type compatibility;
- cross-reference integrity;
- publication-set membership.

## SRE-012-IDENTIFIER-002

Contract 012 SHALL NOT assign, repair, replace, or issue final canonical request identity.

## SRE-012-IDENTIFIER-003

A malformed or unresolved identifier SHALL produce one or more Validation Findings when the governing validation act can otherwise complete.

---

# 18. Relationship integrity

**Specification level:** Required runtime behavior

## SRE-012-RELATION-001

Contract 012 MAY validate:

- required relationship presence;
- valid relationship type;
- valid source and target references;
- relationship cardinality;
- scope consistency;
- forbidden relationship combinations;
- profile-authorized relationship constraints.

## SRE-012-RELATION-002

Contract 012 SHALL NOT infer, create, redirect, merge, split, or repair relationships.

## SRE-012-RELATION-003

A relationship defect SHALL produce one or more Validation Findings when the governed validation act can otherwise complete.

---

# 19. Ordering integrity

**Specification level:** Required runtime behavior

## SRE-012-ORDER-001

Contract 012 MAY validate that:

- required ordering assignments exist;
- ordering scopes are complete;
- positions are unique where required;
- assignments reference valid subjects;
- the publication matches Contract 011's declared linearization;
- required positions have not been omitted, duplicated, or mutated.

## SRE-012-ORDER-002

Contract 012 SHALL NOT:

- recompute canonical order;
- choose alternative tie-breakers;
- resolve ordering cycles;
- derive a new linearization;
- repair or replace canonical ordering assignments.

## SRE-012-ORDER-003

Any ordering correction SHALL require a new governed publication under Contract 011.

---

# 20. Cardinality integrity

**Specification level:** Required runtime behavior

## SRE-012-CARDINALITY-001

Contract 012 MAY validate minimum, maximum, exact, conditional, and profile-dependent cardinality requirements.

## SRE-012-CARDINALITY-002

Cardinality validation SHALL apply only to structural quantity requirements.

It SHALL NOT determine semantic sufficiency.

## SRE-012-CARDINALITY-003

Cardinality defects SHALL remain explicit through Validation Findings and Decisions.

---

# 21. Construction prerequisites

**Specification level:** Required runtime behavior

## SRE-012-CONSTRUCTION-001

Contract 012 SHALL validate only representation-independent construction prerequisites and profile-declared structural requirements necessary for Contract 013 to attempt construction.

## SRE-012-CONSTRUCTION-002

Construction prerequisites MAY include:

- required sections;
- required field presence;
- complete canonical ordering assignments;
- valid references;
- valid cardinalities;
- valid structural identifiers;
- required profile bindings;
- required construction inputs.

## SRE-012-CONSTRUCTION-003

Contract 012 SHALL NOT own serializer-specific behavior, including:

- JSON punctuation;
- XML namespace emission;
- byte encoding;
- whitespace;
- wire framing;
- serializer implementation behavior;
- transport-specific formatting.

## SRE-012-CONSTRUCTION-004

Serializer-specific representation belongs outside this contract unless an adopted construction requirement expressly elevates a representation-independent structural prerequisite.

---

# 22. Validation Profile

**Specification level:** Required runtime artifact or externally supplied normative profile

## SRE-012-PROFILE-001

Every Structural Validation operation SHALL identify one applicable `ValidationProfileId` and exact profile version.

## SRE-012-PROFILE-002

A Validation Profile MAY govern:

- applicable construction requirements;
- required schemas;
- required structural registries;
- applicable validation rule sets;
- finding severity;
- decision derivation;
- warning behavior;
- profile-permitted exceptions;
- `Deferred` behavior;
- eligibility aggregation;
- required snapshots and references.

## SRE-012-PROFILE-003

A Validation Profile SHALL NOT expand, transfer, reduce, redefine, or bypass constitutional authority.

## SRE-012-PROFILE-004

A Validation Profile SHALL NOT:

- reinterpret meaning;
- create semantic content;
- establish canonical ordering;
- perform canonical construction;
- issue request identity;
- authorize downstream action.

## SRE-012-PROFILE-005

A profile revision capable of altering validation output SHALL receive a new version and SHALL NOT retroactively alter previously committed results.

---

# 23. Rule authority and Validation Rule Applications

**Specification level:** Required runtime behavior

## SRE-012-RULE-001

Every Validation Rule Application SHALL reference one authoritative rule owned by an applicable profile, schema, registry, construction requirement, or rule set.

## SRE-012-RULE-002

Contract 012 SHALL NOT silently synthesize new validation requirements during evaluation.

## SRE-012-RULE-003

A rule application SHALL preserve sufficient information to replay:

- the exact rule;
- the exact subject;
- the exact expected condition;
- the exact evaluation method;
- the exact governing versions;
- the exact observed result.

## SRE-012-RULE-004

Where multiple rules apply, rule order and aggregation SHALL be deterministic and traceable.

---

# 24. Structural Validation Result

**Specification level:** Required runtime artifact

## SRE-012-RESULT-001

Every completed Structural Validation operation SHALL commit exactly one `StructuralValidationResult`.

## SRE-012-RESULT-002

The result SHALL include at least:

- `StructuralValidationResultId`;
- `StructuralValidationId`;
- `ValidationCompletion = Completed`;
- exactly one Structural Eligibility state;
- validated `CanonicallyOrderedRequestRepresentationId`;
- validation profile identity and version;
- construction requirements reference;
- schema identities and versions;
- registry identities and versions;
- validation rule-set identity and version;
- Validation Rule Application references;
- Validation Finding references;
- Validation Decision references;
- deterministic snapshot context;
- publication metadata;
- provenance and traceability references.

## SRE-012-RESULT-003

The result SHALL preserve sufficient information to determine why the published eligibility state was reached.

## SRE-012-RESULT-004

A Structural Validation Result SHALL NOT be interpreted as:

- a canonical request;
- an issued request;
- an authorized request;
- an execution plan;
- a generation envelope;
- downstream permission.

---

# 25. Structural Validation Failure Record

**Specification level:** Required runtime artifact

## SRE-012-FAILURE-001

A Structural Validation operation that cannot complete the governed validation act SHALL commit exactly one `StructuralValidationFailureRecord`.

## SRE-012-FAILURE-002

The initial Validation Operation Failure registry SHALL include at least:

```text
ValidationProfileUnavailable
ProfileResolutionFailure
SchemaResolutionFailure
RegistryResolutionFailure
RuleSetResolutionFailure
RuleExecutionFailure
NonDeterministicValidationRule
UnsupportedRuleType
ValidationDependencyUnavailable
ValidationEngineFailure
InvalidValidationContext
CommitmentFailure
```

## SRE-012-FAILURE-003

The failure record SHALL include at least:

- `StructuralValidationFailureRecordId`;
- `StructuralValidationId`;
- `ValidationCompletion = Failed`;
- `StructuralEligibility = NotDetermined`;
- validated representation reference;
- resolved and unresolved profile references;
- resolved and unresolved schema references;
- resolved and unresolved registry references;
- resolved and unresolved rule-set references;
- failure category;
- failure stage;
- available rule-application references;
- available findings and decisions, if any;
- replay and traceability context;
- publication metadata.

## SRE-012-FAILURE-004

A failure record SHALL NOT publish `Ineligible`.

## SRE-012-FAILURE-005

A failed operation SHALL NOT emit a partially authoritative Structural Validation Result.

---

# 26. Completed findings versus operation failures

**Specification level:** Constitutional concept and required runtime behavior

## SRE-012-BOUNDARY-001

The following conditions SHALL normally be represented as findings or reasons for ineligibility within a completed Structural Validation Result:

```text
MissingRequiredStructure
SchemaViolation
RegistryConsistencyViolation
IdentifierIntegrityViolation
RelationshipIntegrityViolation
OrderingIntegrityViolation
CardinalityViolation
UnresolvedRequiredReference
ForbiddenStructuralElement
ConstructionPrerequisiteViolation
DuplicateStructuralPosition
```

## SRE-012-BOUNDARY-002

The following conditions SHALL normally produce a Structural Validation Failure Record:

```text
ValidationProfileUnavailable
ProfileResolutionFailure
SchemaResolutionFailure
RegistryResolutionFailure
RuleSetResolutionFailure
RuleExecutionFailure
NonDeterministicValidationRule
UnsupportedRuleType
ValidationDependencyUnavailable
ValidationEngineFailure
```

## SRE-012-BOUNDARY-003

A condition MAY move between these categories only where the applicable adopted profile expressly establishes that the validation act can deterministically complete and publish a governed eligibility determination.

---

# 27. Atomic commitment

**Specification level:** Required runtime behavior

## SRE-012-COMMIT-001

Every completed Contract 012 operation SHALL commit exactly one authoritative outcome:

```text
StructuralValidationResult
```

or:

```text
StructuralValidationFailureRecord
```

## SRE-012-COMMIT-002

A completed operation SHALL NOT commit both outcomes.

## SRE-012-COMMIT-003

A completed operation SHALL NOT commit neither outcome.

## SRE-012-COMMIT-004

Commitment SHALL be atomic.

## SRE-012-COMMIT-005

Diagnostic information MAY accompany a committed artifact.

Diagnostics SHALL NOT replace the authoritative artifact.

---

# 28. Immutability and revision

**Specification level:** Required runtime behavior

## SRE-012-IMMUTABILITY-001

Committed Validation Rule Applications, Validation Findings, Validation Decisions, Structural Validation Results, and Structural Validation Failure Records SHALL be immutable.

## SRE-012-IMMUTABILITY-002

A later change to:

- validation profile;
- construction requirements;
- schema version;
- registry version;
- rule-set version;
- finding state;
- decision state;
- eligibility state;
- validated representation;
- snapshot context;

SHALL require a new Structural Validation operation and new publication identity.

## SRE-012-IMMUTABILITY-003

A later validation operation SHALL NOT overwrite or retroactively revise a prior publication.

---

# 29. Determinism and replay

**Specification level:** Required runtime behavior

## SRE-012-DETERMINISM-001

Equivalent declared inputs processed under identical contract, profile, schema, registry, rule-set, construction-requirement, and configuration versions SHALL produce equivalent Validation Rule Applications, Findings, Decisions, and top-level outcomes.

## SRE-012-DETERMINISM-002

Rule evaluation order, aggregation order, exception application, and eligibility derivation SHALL be deterministic.

## SRE-012-DETERMINISM-003

A conforming implementation SHALL preserve sufficient information to replay the validation operation.

## SRE-012-DETERMINISM-004

Nondeterministic validation behavior SHALL produce a Structural Validation Failure Record unless the applicable adopted profile expressly defines a deterministic constitutional resolution.

---

# 30. Downstream handoff

**Specification level:** Required runtime behavior

## SRE-012-HANDOFF-001

A completed `StructuralValidationResult` with:

```text
StructuralEligibility = Eligible
```

or:

```text
StructuralEligibility = EligibleWithWarnings
```

MAY become an eligible input to Contract 013.

## SRE-012-HANDOFF-002

A `Deferred` result MAY become an input to Contract 013 only where the applicable construction requirements expressly permit deferred eligibility.

## SRE-012-HANDOFF-003

An `Ineligible` result SHALL NOT authorize Contract 013 construction.

## SRE-012-HANDOFF-004

A Structural Validation Failure Record SHALL NOT authorize Contract 013 construction.

## SRE-012-HANDOFF-005

Contract 013 MAY verify that it received an authentic, applicable, non-stale Structural Validation Result.

Contract 013 SHALL NOT silently repeat, replace, or reinterpret the Contract 012 eligibility determination.

---

# 31. Neighbor boundaries

**Specification level:** Constitutional concept

## SRE-012-BOUNDARY-004

The following responsibility boundaries SHALL remain explicit:

```text
Contract 011
Determines and publishes canonical arrangement.
        ↓
Contract 012
Evaluates the integrity and construction eligibility
of that arranged representation.
        ↓
Contract 013
Mechanically constructs the immutable canonical request artifact.
        ↓
Contract 014
Assigns deterministic identity and formal issuance standing.
        ↓
Contract 015
Transfers the issued request to a declared downstream authority.
```

## SRE-012-BOUNDARY-005

Contract 012 SHALL stop before canonical request construction.

## SRE-012-BOUNDARY-006

Contract 013 SHALL not treat receipt of a structurally eligible result as authority to reinterpret, normalize, reorder, repair, or revalidate semantic content.

---

# 32. Security and integrity considerations

**Specification level:** Constitutional concept and required runtime behavior

## SRE-012-SECURITY-001

Validation profiles, schemas, registries, rule sets, and construction requirements SHALL be version-bound and integrity-protected.

## SRE-012-SECURITY-002

A conforming implementation SHALL prevent unauthorized alteration of validation inputs, findings, decisions, results, and failure records.

## SRE-012-SECURITY-003

A profile, schema, registry, or rule-set substitution capable of changing eligibility SHALL be observable and SHALL require a new validation operation.

## SRE-012-SECURITY-004

Structural Validation SHALL not be used as a covert channel for policy, authorization, provider selection, execution preference, or semantic invention.

---

# 33. Conformance requirements

**Specification level:** Normative conformance

## SRE-012-CONFORM-001

An implementation conforms to this contract only if it:

1. consumes one immutable canonically ordered representation;
2. identifies all governing profiles, construction requirements, schemas, registries, rule sets, and versions;
3. preserves rule authority outside the validator;
4. constructs identifiable Validation Rule Applications;
5. constructs identifiable Validation Findings;
6. constructs identifiable Validation Decisions;
7. preserves completion, eligibility, and finding severity as distinct dimensions;
8. treats `Ineligible` as a completed result;
9. restricts `Deferred` to profile-authorized completed states;
10. distinguishes findings from validation-operation failures;
11. performs no repair or mutation;
12. preserves upstream meaning, identity, expression, order, evidence, provenance, and standing;
13. commits exactly one immutable result or failure record;
14. supports deterministic replay;
15. transfers only eligible results to Contract 013 under applicable construction requirements.

## SRE-012-CONFORM-002

An implementation SHALL be non-conforming if it:

- converts `Ineligible` into generic operation error;
- repairs the validated representation;
- recomputes canonical order;
- issues canonical request identity;
- performs construction;
- creates hidden validation rules;
- collapses findings and decisions;
- allows warnings to bypass mandatory requirements;
- uses `Deferred` as a substitute for unresolved operation failure;
- permits undeclared inputs to influence eligibility;
- mutates or overwrites committed validation publications.

---

# 34. Fundamental structural validation invariant

**Specification level:** Constitutional invariant

## SRE-012-INVARIANT-001

Every completed Structural Validation operation SHALL:

- preserve the exact canonically ordered representation being evaluated;
- apply only declared authoritative validation rules;
- represent every eligibility-affecting observation through identified Validation Findings;
- represent every constitutional effect through identified Validation Decisions;
- publish exactly one explicit Structural Eligibility state;
- preserve all governing versions and construction requirements;
- commit one immutable `StructuralValidationResult`;

while creating no new semantic meaning, no canonical ordering, no repaired structure, no canonical request artifact, no request identity, no issuance standing, and no downstream authorization.

## SRE-012-INVARIANT-002

Every failed Structural Validation operation SHALL:

- preserve the attempted validation context;
- publish `ValidationCompletion = Failed`;
- publish `StructuralEligibility = NotDetermined`;
- identify why the governed validation act could not complete;
- commit one immutable `StructuralValidationFailureRecord`;

while publishing no partial authoritative result and no ineligibility determination.

---

# Appendix A — Canonical runtime model

```text
StructuralValidation
        ↓
ValidationRuleApplication
        ↓
ValidationFinding
        ↓
ValidationDecision
        ↓
StructuralValidationResult

StructuralValidationFailureRecord
```

---

# Appendix B — Three-dimensional result model

```text
Validation Completion
├── Completed
└── Failed
```

```text
Structural Eligibility
├── Eligible
├── EligibleWithWarnings
├── Ineligible
└── Deferred
```

```text
Finding Severity
├── Informational
├── Warning
└── Blocking
```

---

# Appendix C — Example completed negative result

```text
Rule:
Every required relationship must reference an existing subject.

Finding:
Relationship R-17 references missing subject E-92.

Decision:
The relationship-integrity requirement is unsatisfied.

Validation Completion:
Completed

Structural Eligibility:
Ineligible
```

This is a completed validation operation.

Nothing failed operationally.

Nothing was repaired.

Nothing semantic was reconsidered.

---

# Appendix D — Example failed operation

```text
Validation Profile:
Resolved

Schema:
Unavailable

Rule Application:
Not completed

Validation Completion:
Failed

Structural Eligibility:
NotDetermined

Failure:
SchemaResolutionFailure
```

No governed eligibility determination exists.

---

# Appendix E — Deferred-state rule

```text
Deferred
```

is valid only where:

```text
Validation Profile or Construction Requirements
        explicitly authorize
Completed validation without final eligibility determination
```

Otherwise inability to determine eligibility SHALL produce:

```text
StructuralValidationFailureRecord
```
