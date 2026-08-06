# Structured Request Engine

## Contract 007 — Evidence Representation

**Document ID:** `SRE-CONTRACT-007`  
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
- `SRE-ARCH-001 v0.1.0`
- `SRE-ARCH-002 v0.1.0`

---

## Normative requirement identifiers

Every normative `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, and `MAY` statement in this contract shall possess a stable requirement identifier.

Requirement identifiers use:

```text
SRE-007-<DOMAIN>-<SEQUENCE>
```

Examples:

```text
SRE-007-EVIDENCE-001
SRE-007-GROUNDING-004
SRE-007-COMMIT-003
```

Requirement identifiers exist solely for traceability, implementation verification, conformance testing, amendment tracking, and cross-contract dependency analysis.

Requirement identifiers SHALL NOT alter the normative meaning of any requirement.

---

# 1. Architectural context

**Specification level:** Constitutional concept

## SRE-007-CONTEXT-001

This contract implements the Canonical Representation Pattern established by `SRE-ARCH-001` and the Constitutional Grounding Architecture established by `SRE-ARCH-002`.

## SRE-007-CONTEXT-002

This contract introduces no new architectural doctrine.

It applies established constitutional architecture to the representation of evidence and explicit support relationships associated with represented semantic artifacts.

## SRE-007-CONTEXT-003

Where this contract adopts an identity, representation, publication, commitment, immutability, or failure doctrine owned by `SRE-ARCH-001` or `SRE-ARCH-002`, this contract SHALL reference and specialize that doctrine only as required for the Evidence Representation domain.

It SHALL NOT redefine the governing architectural doctrine.

---

# 2. Purpose

**Specification level:** Constitutional concept

## SRE-007-PURPOSE-001

This contract establishes the constitutional rules by which supporting material represented within admitted interpretation proposals, referenced artifacts, application declarations, and other authorized constitutional inputs may be constructed as bounded Evidence Representations and explicitly associated with represented semantic artifacts through Grounding Assertions.

## SRE-007-PURPOSE-002

This contract defines:

- Evidence Representation Authority;
- Evidence Representation Operations;
- the formal meaning of Evidence Representation;
- logical evidence identity and immutable representation identity;
- Evidence Representation Instances;
- Evidence Classes;
- Evidence Origin;
- Representation Basis;
- Evidence Status;
- source spans and quoted text;
- structured evidence fields;
- referenced artifacts;
- admitted proposal elements;
- derived observations;
- composite evidence;
- Grounding Assertions;
- supported artifact references;
- grounding relationship types;
- Grounding Representation Profiles;
- evidence integrity;
- structural validation;
- unsupported, incomplete, conflicting, unavailable, and external evidence states;
- `InterpretationEvidenceSet` construction;
- failure outcomes;
- atomic commitment;
- downstream handoff;
- deferred responsibilities.

## SRE-007-PURPOSE-003

The organizing doctrine of this contract is:

> **Evidence Representation records what constitutional support has been represented and how that support is explicitly associated with represented semantic artifacts, without assessing persuasiveness, sufficiency, correctness, truth, relevance, confidence, or authority.**

## SRE-007-PURPOSE-004

This contract SHALL answer only:

> **What evidence has been represented in support of represented semantic artifacts?**

It SHALL NOT answer:

- whether the evidence is persuasive;
- whether the evidence is sufficient;
- whether the evidence is relevant;
- whether the evidence is correct;
- whether the evidence is true;
- whether a supported artifact is correct;
- whether a supported artifact is authorized;
- whether a supported artifact should be selected;
- whether downstream action may occur.

---

# 3. Architectural identity

**Specification level:** Constitutional concept

## SRE-007-IDENTITY-001

Contract 007 establishes the Evidence Representation domain of the Structured Request Engine.

## SRE-007-IDENTITY-002

The constitutional transformation governed by this contract is:

```text
AdmittedInterpretationProposalSet
        +
DeclaredObjectiveSet
        +
DeclaredConstraintSet
        +
CapabilityRequirementSet
        +
SemanticClarificationSet(s)
        +
GroundingRepresentationProfile
        +
Applicable Grounding Registries
        │
        ▼
EvidenceRepresentationAuthority
        │
        ├── success ──► InterpretationEvidenceSet
        │
        └── failure ──► EvidenceRepresentationFailureRecord
```

## SRE-007-IDENTITY-003

Contract 007 SHALL operate over one admitted interpretation proposal set, including a set containing only one admitted proposal.

## SRE-007-IDENTITY-004

The semantic representation sets consumed by Contract 007 SHALL provide constitutional context, supported-artifact references, and traceability targets only.

Contract 007 SHALL NOT modify, reinterpret, replace, reconcile, normalize, canonicalize, rank, or authorize any artifact represented under Contracts 003 through 006.

## SRE-007-IDENTITY-005

The presence of a semantic artifact in an input set SHALL NOT itself cause that artifact to become evidence.

Evidence SHALL be independently represented under this contract.

---

# 4. Evidence constitutional ladder

**Specification level:** Constitutional concept

## SRE-007-LADDER-001

The following constitutional distinctions SHALL remain explicit:

```text
Evidence Exists
        ≠
Evidence Is Represented
        ≠
Evidence Is Structurally Valid
        ≠
Evidence Is Grounded
        ≠
Evidence Supports a Claim
        ≠
Evidence Is Sufficient
        ≠
Claim Is True
```

## SRE-007-LADDER-002

Evidence existence describes the presence of potentially supporting material.

Evidence Representation describes the constitutional recording of that material.

Structural validity describes conformity with applicable schemas, profiles, identifiers, and reference rules.

Grounding describes an explicit represented support relationship.

Support describes the represented relationship asserted between evidence and a supported artifact.

Sufficiency describes an evaluative conclusion regarding whether evidence is adequate for a purpose.

Truth describes a conclusion regarding correspondence, correctness, or factual validity.

## SRE-007-LADDER-003

Contract 007 SHALL own only Evidence Representation, structural validation within its bounded domain, and explicit Grounding Assertion construction.

It SHALL NOT own evidence evaluation, sufficiency determination, truth determination, semantic validation, authorization, or certification.

---

# 5. Organizing constitutional doctrines

**Specification level:** Constitutional concept

## 5.1 Representation does not create evidence

### SRE-007-DOCTRINE-001

Committed semantic representations SHALL NOT constitute constitutional evidence solely by virtue of their existence.

Evidence SHALL be represented independently.

Grounding SHALL explicitly associate represented evidence with represented semantic artifacts.

No semantic artifact SHALL implicitly satisfy evidence requirements.

## 5.2 Evidence is not evaluation

### SRE-007-DOCTRINE-002

Evidence Representation SHALL record supporting material and support relationships only.

It SHALL NOT assess, weigh, rank, score, prefer, validate, reject, or adjudicate that material.

## 5.3 Evidence is not truth

### SRE-007-DOCTRINE-003

The existence, representation, structural validity, publication, or grounding of evidence SHALL NOT establish truth, correctness, accuracy, factuality, or semantic validity.

## 5.4 Grounding does not alter meaning

### SRE-007-DOCTRINE-004

Grounding Assertions SHALL establish constitutional support relationships only.

Grounding Assertions SHALL NOT:

- modify represented evidence;
- modify represented semantic artifacts;
- reinterpret represented meaning;
- strengthen represented meaning;
- weaken represented meaning;
- merge represented meaning;
- establish semantic equivalence;
- establish truth;
- establish correctness;
- establish authority;
- determine evidence sufficiency.

## 5.5 Support must be explicit

### SRE-007-DOCTRINE-005

A support relationship SHALL exist constitutionally only when represented by a valid Grounding Assertion.

A shared identifier, source reference, co-location, field reference, textual proximity, or common origin SHALL NOT by itself constitute a Grounding Assertion.

## 5.6 Evidence independence

### SRE-007-DOCTRINE-006

Evidence Representations SHALL remain constitutionally independent from the artifacts they support.

A represented semantic artifact SHALL NOT own, absorb, or mutate an Evidence Representation.

One Evidence Representation MAY participate in multiple Grounding Assertions.

## 5.7 Origin is not authority

### SRE-007-DOCTRINE-007

Evidence Origin SHALL preserve where supporting material was represented as originating.

Origin SHALL NOT establish authority, correctness, priority, trustworthiness, relevance, or sufficiency.

## 5.8 Confidence is not represented here

### SRE-007-DOCTRINE-008

Contract 007 SHALL NOT assign evidentiary confidence, reliability scores, trust scores, probability values, or persuasive weight.

Any confidence value received as source content MAY be preserved as quoted or structured source material only when clearly identified as source-declared content and SHALL NOT become a Contract 007 evaluation.

## 5.9 Authority non-expansion

### SRE-007-DOCTRINE-009

No Evidence Representation, Evidence Class, Evidence Status, Grounding Assertion, relationship type, profile, or publication created under this contract SHALL expand downstream authority.

## 5.10 Non-invention

### SRE-007-DOCTRINE-010

Contract 007 SHALL NOT invent supporting material, source references, source spans, quoted text, artifact references, support relationships, or evidence origins absent authorized constitutional basis.

## 5.11 No silent repair

### SRE-007-DOCTRINE-011

Contract 007 SHALL NOT silently repair malformed evidence references, fabricate missing spans, replace unavailable artifacts, infer unsupported support relationships, or conceal conflicting evidence states.

## 5.12 Structural precision is not operational permission

### SRE-007-DOCTRINE-012

Greater structural precision in an `InterpretationEvidenceSet` SHALL NOT create greater operational permission.

---

# 6. Constitutional authority

**Specification level:** Constitutional authority

## 6.1 Evidence Representation Authority

### SRE-007-AUTHORITY-001

The Evidence Representation Authority is the exclusive constitutional owner of the transformation governed by this contract.

### SRE-007-AUTHORITY-002

The Evidence Representation Authority MAY:

- identify evidence candidates within authorized constitutional inputs;
- construct Evidence Representations;
- construct Evidence Representation Instances;
- classify evidence using applicable registries;
- record Evidence Origin;
- record Representation Basis;
- preserve source spans, quotations, structured fields, and artifact references;
- construct Grounding Assertions;
- associate evidence with supported artifacts;
- validate structural references;
- apply Grounding Representation Profiles;
- assign Evidence Status and Representation Status;
- construct an `InterpretationEvidenceSet`;
- construct an `EvidenceRepresentationFailureRecord`.

### SRE-007-AUTHORITY-003

The Evidence Representation Authority SHALL NOT:

- interpret new semantic objectives, constraints, capabilities, ambiguities, assumptions, or uncertainty;
- reconcile competing proposals;
- normalize semantic meaning;
- assess evidence quality;
- assess evidence relevance;
- determine evidence sufficiency;
- rank or weigh evidence;
- determine truth or correctness;
- validate a supported semantic artifact;
- establish provenance beyond the bounded origin and reference fields required by this contract;
- authorize tools, models, providers, connectors, resources, or execution;
- release a request for downstream action.

## 6.2 Exclusive construction authority

### SRE-007-AUTHORITY-004

Only the Evidence Representation Authority MAY commit an `InterpretationEvidenceSet` or an `EvidenceRepresentationFailureRecord` under this contract.

### SRE-007-AUTHORITY-005

Applications, interpreters, source artifacts, semantic representation contracts, and downstream consumers MAY supply constitutional inputs.

They SHALL NOT directly construct or commit Contract 007 output artifacts unless they are themselves the recognized Evidence Representation Authority for the operation.

---

# 7. Inputs

**Specification level:** Required runtime input

## SRE-007-INPUT-001

A conforming Evidence Representation Operation SHALL consume:

- one `AdmittedInterpretationProposalSet`;
- all applicable represented semantic artifact sets produced under Contracts 003 through 006;
- one applicable `GroundingRepresentationProfile`;
- all applicable Evidence Class, Origin, Representation Basis, Evidence Status, Representation Status, Supported Artifact Type, and Grounding Relationship registries;
- all required schemas and identifier rules.

## SRE-007-INPUT-002

A semantic representation set MAY be absent only when its governing upstream contract validly produced an empty set, non-applicable outcome, or explicit failure state permitted by the contract set.

## SRE-007-INPUT-003

Contract 007 SHALL distinguish between:

- absence of evidence;
- unavailable evidence;
- incomplete evidence;
- conflicting evidence;
- external evidence;
- malformed evidence references;
- unsupported grounding assertions.

These conditions SHALL NOT be collapsed into a single generic missing-evidence state.

---

# 8. Runtime object model

**Specification level:** Required runtime architecture

## SRE-007-OBJECT-001

Contract 007 defines five primary runtime objects because each object owns a distinct constitutional responsibility:

1. `EvidenceRepresentation` owns logical evidence identity;
2. `EvidenceRepresentationInstance` owns one immutable represented state of that evidence;
3. `GroundingAssertion` owns one explicit support relationship;
4. `InterpretationEvidenceSet` owns one atomic publication result;
5. `EvidenceRepresentationFailureRecord` owns one explicit failed operation outcome.

No object SHALL assume the constitutional responsibility of another.

## 8.1 Evidence Representation

### SRE-007-OBJECT-002

An `EvidenceRepresentation` is the logical constitutional object identifying one represented item or bounded body of supporting material.

### SRE-007-OBJECT-003

Each `EvidenceRepresentation` SHALL possess one stable `EvidenceId`.

### SRE-007-OBJECT-004

An `EvidenceRepresentation` SHALL NOT itself contain mutable publication history.

Its immutable states SHALL be expressed through Evidence Representation Instances.

## 8.2 Evidence Representation Instance

### SRE-007-OBJECT-005

An `EvidenceRepresentationInstance` is one immutable expression, classification, origin assignment, representation-basis assignment, evidence-status assignment, representation-status assignment, and structural-reference state of one logical Evidence Representation.

### SRE-007-OBJECT-006

Each Evidence Representation Instance SHALL possess one unique `EvidenceRepresentationId`.

### SRE-007-OBJECT-007

Two Evidence Representation Instances MAY share one `EvidenceId` only when they represent distinct immutable states of the same logical evidence object.

## 8.3 Grounding Assertion

### SRE-007-OBJECT-008

A `GroundingAssertion` is one immutable constitutional object representing one explicit support relationship between one Evidence Representation Instance and one supported semantic artifact.

### SRE-007-OBJECT-009

Each Grounding Assertion SHALL possess one unique `GroundingAssertionId`.

### SRE-007-OBJECT-010

A Grounding Assertion SHALL own only the support relationship.

It SHALL NOT own the evidence or the supported artifact.

## 8.4 Interpretation Evidence Set

### SRE-007-OBJECT-011

An `InterpretationEvidenceSet` is the immutable committed runtime artifact containing the complete successful result of one Evidence Representation Operation.

### SRE-007-OBJECT-012

Each `InterpretationEvidenceSet` SHALL possess one unique `InterpretationEvidenceSetId`.

## 8.5 Evidence Representation Failure Record

### SRE-007-OBJECT-013

An `EvidenceRepresentationFailureRecord` is the immutable committed artifact produced when a conforming `InterpretationEvidenceSet` cannot be constructed without violating this contract.

### SRE-007-OBJECT-014

Each failure record SHALL possess one unique `EvidenceRepresentationFailureRecordId`.

---

# 9. Identity doctrine

**Specification level:** Constitutional concept

## SRE-007-ID-001

This contract adopts the logical identity, representation identity, set identity, publication, and immutability doctrines established by `SRE-ARCH-001` and `SRE-ARCH-002`.

## SRE-007-ID-002

The following identities SHALL remain distinct:

```text
EvidenceId
        ≠
EvidenceRepresentationId
        ≠
GroundingAssertionId
        ≠
InterpretationEvidenceSetId
        ≠
EvidenceRepresentationFailureRecordId
```

## SRE-007-ID-003

`EvidenceId` SHALL identify the logical evidence object.

`EvidenceRepresentationId` SHALL identify one immutable representation instance of that logical object.

`GroundingAssertionId` SHALL identify one immutable support relationship.

`InterpretationEvidenceSetId` SHALL identify one immutable successful publication.

`EvidenceRepresentationFailureRecordId` SHALL identify one immutable failed publication outcome.

## SRE-007-ID-004

Identity equality SHALL NOT imply semantic equivalence, evidentiary equivalence, support equivalence, truth, correctness, or sufficiency.

---

# 10. Evidence Class registry

**Specification level:** Required runtime registry

## SRE-007-CLASS-001

Each Evidence Representation Instance SHALL possess exactly one primary Evidence Class from the applicable registry.

## SRE-007-CLASS-002

The initial Evidence Class registry SHALL include:

```text
SourceSpan
QuotedText
StructuredField
ApplicationDeclaration
ReferencedArtifact
AdmittedProposalElement
DerivedObservation
CompositeEvidence
```

## SRE-007-CLASS-003 — SourceSpan

`SourceSpan` SHALL represent a bounded location within an admitted source or referenced artifact.

It SHALL preserve sufficient locator information to support deterministic retrieval when retrieval is constitutionally available.

## SRE-007-CLASS-004 — QuotedText

`QuotedText` SHALL represent text preserved as source material.

Quotation SHALL NOT imply endorsement, truth, or sufficiency.

## SRE-007-CLASS-005 — StructuredField

`StructuredField` SHALL represent supporting material preserved from a bounded structured source field.

## SRE-007-CLASS-006 — ApplicationDeclaration

`ApplicationDeclaration` SHALL represent supporting material explicitly supplied by an application through an authorized input boundary.

## SRE-007-CLASS-007 — ReferencedArtifact

`ReferencedArtifact` SHALL represent an independently identifiable constitutional or external artifact referenced as supporting material.

## SRE-007-CLASS-008 — AdmittedProposalElement

`AdmittedProposalElement` SHALL represent an individually identifiable element contained within an admitted interpretation proposal.

It SHALL NOT include rejected, excluded, inadmissible, or unadmitted proposal material.

An Admitted Proposal Element MAY preserve whether its immediate proposal origin was:

- `SourceProposalElement`;
- `InterpreterProposalElement`;
- `ApplicationProposalElement`.

This subordinate classification SHALL NOT expand the Evidence Class registry unless separately registered.

## SRE-007-CLASS-009 — DerivedObservation

`DerivedObservation` SHALL represent supporting material produced through an expressly profile-authorized, deterministic, non-evaluative derivation from identified constitutional inputs.

It SHALL preserve all source references required to replay the derivation.

## SRE-007-CLASS-010 — CompositeEvidence

`CompositeEvidence` SHALL represent a bounded evidence object composed from two or more identified Evidence Representation Instances.

Composition SHALL NOT erase component identities, origins, statuses, or references.

## SRE-007-CLASS-011

An Evidence Class SHALL describe representational form only.

It SHALL NOT establish evidentiary weight, relevance, sufficiency, confidence, trust, correctness, or authority.

---

# 11. Evidence Origin registry

**Specification level:** Required runtime registry

## SRE-007-ORIGIN-001

Each Evidence Representation Instance SHALL possess one Evidence Origin value from the applicable shared Origin registry.

## SRE-007-ORIGIN-002

The initial Evidence Origin values SHALL include:

```text
ExplicitSource
InterpreterInference
ApplicationSupplied
ReferencedArtifact
MixedOrigin
```

## SRE-007-ORIGIN-003

`ExplicitSource` SHALL indicate that the evidence was represented directly from admitted source material.

## SRE-007-ORIGIN-004

`InterpreterInference` SHALL indicate that the evidence was represented from interpreter-supplied interpretive material admitted under Contract 002.

It SHALL NOT imply that the inference is correct.

## SRE-007-ORIGIN-005

`ApplicationSupplied` SHALL indicate that the evidence was supplied through an authorized application boundary.

## SRE-007-ORIGIN-006

`ReferencedArtifact` SHALL indicate that the evidence is represented through an identified artifact reference.

## SRE-007-ORIGIN-007

`MixedOrigin` SHALL be used only when one Evidence Representation Instance cannot be faithfully represented using a single origin and all contributing origins are explicitly recorded.

## SRE-007-ORIGIN-008

Origin SHALL remain descriptive and non-authorizing.

---

# 12. Representation Basis registry

**Specification level:** Required runtime registry

## SRE-007-BASIS-001

Each Evidence Representation Instance SHALL possess one Representation Basis value from the applicable shared registry.

## SRE-007-BASIS-002

The initial Representation Basis values SHALL include:

```text
DirectQuotation
StructuredExtraction
InterpreterSynthesis
ApplicationDeclaration
ReferencedArtifactDeclaration
ProfileAuthorizedDerivation
```

## SRE-007-BASIS-003

Representation Basis SHALL describe how the Evidence Representation Instance was constructed.

It SHALL NOT describe whether the evidence is persuasive, relevant, sufficient, correct, or true.

## SRE-007-BASIS-004

`ProfileAuthorizedDerivation` SHALL be permitted only when the applicable Grounding Representation Profile defines a deterministic derivation mechanism, required inputs, output structure, and replay requirements.

---

# 13. Evidence Status registry

**Specification level:** Required runtime registry

## SRE-007-STATUS-001

Each Evidence Representation Instance SHALL possess exactly one primary Evidence Status from the applicable registry.

## SRE-007-STATUS-002

The initial Evidence Status registry SHALL include:

```text
Observed
Referenced
Unavailable
Incomplete
Conflicting
External
Composite
```

## SRE-007-STATUS-003 — Observed

`Observed` SHALL indicate that the represented supporting material was available within admitted constitutional inputs at representation time.

## SRE-007-STATUS-004 — Referenced

`Referenced` SHALL indicate that the Evidence Representation points to identified supporting material without embedding the complete material.

## SRE-007-STATUS-005 — Unavailable

`Unavailable` SHALL indicate that supporting material was identified or declared but was not constitutionally available for representation or retrieval.

## SRE-007-STATUS-006 — Incomplete

`Incomplete` SHALL indicate that only part of the identified supporting material could be represented.

## SRE-007-STATUS-007 — Conflicting

`Conflicting` SHALL indicate that the evidence object preserves an explicit conflict within or among represented supporting materials.

Conflict status SHALL NOT resolve the conflict.

## SRE-007-STATUS-008 — External

`External` SHALL indicate that the supporting material exists outside the immediate admitted proposal and semantic representation boundary.

## SRE-007-STATUS-009 — Composite

`Composite` SHALL indicate that the evidence object is structurally composed of multiple identified Evidence Representation Instances.

## SRE-007-STATUS-010

Evidence Status SHALL describe the condition of represented evidence only.

It SHALL remain distinct from Representation Status.

---

# 14. Representation Status

**Specification level:** Required runtime registry

## SRE-007-REPSTATUS-001

Each Evidence Representation Instance and each Grounding Assertion SHALL possess one Representation Status from the shared registry established under the Canonical Representation Pattern.

## SRE-007-REPSTATUS-002

The initial applicable Representation Status values SHALL include:

```text
Represented
Incomplete
Unsupported
EvidenceLimited
Conflicting
Unresolved
```

## SRE-007-REPSTATUS-003

Representation Status SHALL describe the constitutional condition of the representation.

It SHALL NOT substitute for Evidence Status.

## SRE-007-REPSTATUS-004

A representation MAY be structurally `Represented` while the evidence itself is `Unavailable`, `External`, `Incomplete`, or `Conflicting`, provided that the status is faithfully and explicitly represented.

---

# 15. Evidence content and references

**Specification level:** Required runtime data

## 15.1 Source spans

### SRE-007-CONTENT-001

A Source Span SHALL preserve:

- the identity of the source or referenced artifact;
- a deterministic locator or bounded location description;
- the represented span boundaries when available;
- the source-version or artifact-version reference when available;
- the method by which the span was identified.

### SRE-007-CONTENT-002

A Source Span SHALL NOT be fabricated when precise boundaries are unavailable.

The absence of precise boundaries SHALL be represented explicitly.

## 15.2 Quoted text

### SRE-007-CONTENT-003

Quoted text SHALL preserve the represented text without silent semantic rewriting.

Formatting normalization MAY occur only when authorized by profile and when the original source reference remains available.

### SRE-007-CONTENT-004

Ellipsis, truncation, redaction, normalization, or omission within quoted text SHALL be explicitly represented.

## 15.3 Structured fields

### SRE-007-CONTENT-005

A Structured Field representation SHALL preserve:

- source field identity;
- source object identity;
- represented field value;
- source schema or structure reference when available;
- extraction basis.

## 15.4 Referenced artifacts

### SRE-007-CONTENT-006

A Referenced Artifact representation SHALL preserve:

- artifact identity;
- artifact type;
- artifact version or version state when available;
- retrieval or custody reference when available;
- the bounded portion relied upon, when applicable.

### SRE-007-CONTENT-007

Contract 007 SHALL NOT claim custody, authenticity, integrity certification, or provenance completeness for a referenced artifact unless such claims are independently established by an authorized downstream or external authority.

## 15.5 Derived observations

### SRE-007-CONTENT-008

A Derived Observation SHALL preserve:

- all source Evidence Representation identifiers;
- the applicable profile identifier and version;
- the deterministic derivation rule identifier;
- all parameters affecting the derivation;
- the resulting represented observation.

### SRE-007-CONTENT-009

A Derived Observation SHALL NOT conceal its derived status or be represented as direct source material.

---

# 16. Grounding Assertions

**Specification level:** Required runtime artifact

## SRE-007-GROUNDING-001

Each Grounding Assertion SHALL contain at least:

```text
GroundingAssertionId
EvidenceRepresentationId
SupportedArtifactId
SupportedArtifactType
RelationshipType
EvidenceStatus
RepresentationStatus
GroundingRepresentationProfileId
```

## SRE-007-GROUNDING-002

Each Grounding Assertion SHALL reference exactly one Evidence Representation Instance and exactly one supported artifact.

Multiple evidence items supporting one artifact SHALL be represented through multiple Grounding Assertions or through an explicitly identified Composite Evidence object and its grounding assertion.

## SRE-007-GROUNDING-003

The supported artifact SHALL already possess constitutional identity established by its governing upstream contract.

Contract 007 SHALL NOT invent or alter that identity.

## SRE-007-GROUNDING-004

The Supported Artifact Type registry SHALL initially include:

```text
DeclaredObjective
DeclaredConstraint
CapabilityRequirement
Ambiguity
Assumption
Uncertainty
```

## SRE-007-GROUNDING-005

The Grounding Relationship registry SHALL initially include:

```text
Supports
PartiallySupports
References
DerivedFrom
Aggregates
Refines
Supersedes
```

## SRE-007-GROUNDING-006 — Supports

`Supports` SHALL represent that evidence has been explicitly associated as support for the identified artifact.

It SHALL NOT imply sufficiency or truth.

## SRE-007-GROUNDING-007 — PartiallySupports

`PartiallySupports` SHALL represent that evidence has been explicitly associated with only a bounded portion, aspect, or condition of the supported artifact.

The bounded portion or aspect SHALL be identified.

## SRE-007-GROUNDING-008 — References

`References` SHALL represent an explicit representational reference without asserting substantive support.

## SRE-007-GROUNDING-009 — DerivedFrom

`DerivedFrom` SHALL represent that the supported artifact or evidence representation was represented as derived from the identified evidence.

It SHALL NOT establish correctness of the derivation.

## SRE-007-GROUNDING-010 — Aggregates

`Aggregates` SHALL represent that the evidence participates in a bounded aggregate evidence object.

## SRE-007-GROUNDING-011 — Refines

`Refines` SHALL represent that the evidence provides more specific supporting material associated with an already represented support context.

It SHALL NOT mutate the earlier evidence or artifact.

## SRE-007-GROUNDING-012 — Supersedes

`Supersedes` SHALL represent an explicit support relationship indicating replacement in representational standing.

It SHALL NOT delete, rewrite, or invalidate the superseded object.

## SRE-007-GROUNDING-013

Relationship types SHALL remain representational.

They SHALL NOT establish evaluation, relevance, sufficiency, truth, correctness, authority, or precedence unless a future authorized contract expressly governs such determination.

---

# 17. Grounding Representation Profile

**Specification level:** Required runtime profile

## SRE-007-PROFILE-001

Every Evidence Representation Operation SHALL execute under exactly one applicable `GroundingRepresentationProfile`.

## SRE-007-PROFILE-002

A Grounding Representation Profile SHALL define, at minimum:

- permitted Evidence Classes;
- permitted Origin values;
- permitted Representation Basis values;
- permitted Evidence Status values;
- permitted Supported Artifact Types;
- permitted Grounding Relationship types;
- required structural fields;
- source-span and reference requirements;
- permitted deterministic derivations;
- composite evidence rules;
- unsupported, incomplete, conflicting, external, and unavailable evidence behavior;
- validation rules;
- empty-set behavior;
- failure conditions.

## SRE-007-PROFILE-003

A Grounding Representation Profile MAY narrow the registries and mechanisms permitted by this contract.

It SHALL NOT expand Contract 007 authority.

## SRE-007-PROFILE-004

A Grounding Representation Profile SHALL NOT:

- assess evidence;
- rank evidence;
- determine relevance;
- determine sufficiency;
- determine confidence;
- establish truth;
- authorize downstream action.

## SRE-007-PROFILE-005

Each committed output SHALL identify the governing profile and profile version.

---

# 18. Evidence integrity

**Specification level:** Constitutional requirement

## SRE-007-INTEGRITY-001

Evidence integrity under this contract means faithful preservation of represented content, identity, status, references, classifications, and support relationships within the bounded representation operation.

## SRE-007-INTEGRITY-002

Evidence integrity SHALL NOT be interpreted as proof of source authenticity, factual correctness, truth, custody, provenance completeness, or evidentiary sufficiency.

## SRE-007-INTEGRITY-003

A conforming Evidence Representation Operation SHALL preserve:

- evidence identity;
- source or artifact references;
- quoted or extracted content boundaries;
- origin;
- representation basis;
- evidence status;
- representation status;
- grounding relationships;
- profile and registry versions;
- operation identity;
- commitment state.

## SRE-007-INTEGRITY-004

Any transformation affecting represented text, structure, locator form, field encoding, or composite construction SHALL be explicitly authorized by the governing profile and replayable from preserved inputs.

## SRE-007-INTEGRITY-005

Silent alteration of represented evidence SHALL constitute failure.

---

# 19. Structural validation

**Specification level:** Required runtime behavior

## SRE-007-VALIDATE-001

Contract 007 SHALL validate only structural conformity within its constitutional authority.

## SRE-007-VALIDATE-002

Structural validation SHALL include, as applicable:

- identifier validity;
- schema validity;
- registry membership;
- required-field presence;
- reference integrity;
- supported-artifact existence within consumed constitutional inputs;
- relationship-form validity;
- profile compliance;
- source-span syntax;
- composite membership integrity;
- uniqueness constraints;
- atomic-set completeness.

## SRE-007-VALIDATE-003

Structural validation SHALL NOT include:

- factual verification;
- semantic correctness;
- evidentiary relevance;
- evidentiary sufficiency;
- truth determination;
- confidence assessment;
- authorization determination.

## SRE-007-VALIDATE-004

A structurally valid Grounding Assertion MAY relate evidence whose Evidence Status is `Unavailable`, `Incomplete`, `Conflicting`, `External`, or `Composite`, provided that the status and permitted relationship are faithfully represented under the profile.

---

# 20. Unsupported and limited evidence

**Specification level:** Required runtime behavior

## SRE-007-LIMIT-001

Contract 007 SHALL preserve the absence of represented support where no authorized evidence can be constructed for a semantic artifact.

It SHALL NOT fabricate evidence or Grounding Assertions to avoid an unsupported state.

## SRE-007-LIMIT-002

An unsupported semantic artifact MAY remain present in its upstream committed representation set.

Contract 007 SHALL NOT delete or invalidate that artifact solely because no evidence was represented.

## SRE-007-LIMIT-003

When the governing profile requires explicit accounting for unsupported artifacts, the `InterpretationEvidenceSet` SHALL contain a bounded unsupported-artifact reference or equivalent profile-defined representation without manufacturing evidence.

## SRE-007-LIMIT-004

Evidence-limited, incomplete, unavailable, external, conflicting, and unresolved conditions SHALL be represented explicitly and SHALL survive downstream handoff until an authorized downstream contract resolves or otherwise governs them.

## SRE-007-LIMIT-005

Contract 007 SHALL NOT determine that unsupported or limited evidence makes a request invalid, inadmissible, unauthorized, or non-executable.

---

# 21. Composite evidence

**Specification level:** Required runtime behavior

## SRE-007-COMPOSITE-001

Composite Evidence MAY be constructed only when permitted by the governing Grounding Representation Profile.

## SRE-007-COMPOSITE-002

A Composite Evidence object SHALL preserve:

- its own `EvidenceId` and `EvidenceRepresentationId`;
- every component Evidence Representation identifier;
- component ordering when order is meaningful;
- the composition rule;
- profile and registry versions;
- all component origins and statuses, either directly or by immutable reference.

## SRE-007-COMPOSITE-003

Composition SHALL NOT:

- erase component identity;
- conceal conflicting evidence;
- convert unavailable evidence into observed evidence;
- establish evidentiary sufficiency;
- establish semantic equivalence;
- establish authority.

---

# 22. InterpretationEvidenceSet

**Specification level:** Required runtime artifact

## SRE-007-SET-001

A successful Evidence Representation Operation SHALL produce exactly one immutable `InterpretationEvidenceSet`.

## SRE-007-SET-002

The `InterpretationEvidenceSet` SHALL contain, at minimum:

```text
InterpretationEvidenceSetId
EvidenceRepresentationOperationId
AdmittedInterpretationProposalSetId
ConsumedSemanticRepresentationSetIds
GroundingRepresentationProfileId
GroundingRepresentationProfileVersion
ApplicableRegistryVersions
EvidenceRepresentationInstances
GroundingAssertions
UnsupportedArtifactReferences, when required
SetRepresentationStatus
CommitmentMetadata
```

## SRE-007-SET-003

The set SHALL contain the complete successful output of one operation.

It SHALL NOT be an incrementally mutable accumulation.

## SRE-007-SET-004

A new Evidence Representation Operation SHALL produce a new `InterpretationEvidenceSetId` even when it consumes the same constitutional inputs.

## SRE-007-SET-005

An empty `InterpretationEvidenceSet` MAY be valid only when:

- no Evidence Representation is constitutionally required by the applicable profile;
- no evidence candidate is authorized or available for representation;
- all required unsupported-artifact accounting is present;
- no failure condition applies.

## SRE-007-SET-006

An empty set SHALL NOT imply that no evidence exists outside the constitutional input boundary.

It SHALL mean only that no Evidence Representation was committed by the operation.

---

# 23. Failure model

**Specification level:** Required runtime artifact

## SRE-007-FAILURE-001

When a conforming `InterpretationEvidenceSet` cannot be constructed, the Evidence Representation Authority SHALL commit exactly one `EvidenceRepresentationFailureRecord`.

## SRE-007-FAILURE-002

Failure conditions SHALL include, as applicable:

- missing required constitutional inputs;
- invalid or unrecognized profile;
- invalid registry state;
- malformed evidence identity;
- malformed supported-artifact identity;
- unsupported Evidence Class;
- unsupported Origin or Representation Basis;
- unsupported Evidence Status or Representation Status;
- invalid source span;
- invalid artifact reference;
- unresolved structural reference;
- prohibited derivation;
- prohibited relationship type;
- profile violation;
- non-replayable composite construction;
- silent evidence alteration;
- attempted evidence invention;
- attempted evaluation or sufficiency determination;
- attempted authority expansion;
- inability to preserve atomic commitment.

## SRE-007-FAILURE-003

A failure record SHALL contain, at minimum:

```text
EvidenceRepresentationFailureRecordId
EvidenceRepresentationOperationId
AdmittedInterpretationProposalSetId
ConsumedSemanticRepresentationSetIds
GroundingRepresentationProfileId, when available
ApplicableRegistryVersions, when available
FailureCode
FailureDescription
AffectedInputReferences
PartialConstructionDisposition
CommitmentMetadata
```

## SRE-007-FAILURE-004

A failure record SHALL preserve auditability and deterministic replay to the extent constitutionally possible.

## SRE-007-FAILURE-005

A failure record SHALL NOT authorize fallback interpretation, silent repair, evidence fabrication, semantic modification, alternative grounding, or downstream execution.

---

# 24. Atomic commitment and publication

**Specification level:** Required runtime behavior

## SRE-007-COMMIT-001

An Evidence Representation Operation SHALL commit exactly one terminal artifact:

```text
InterpretationEvidenceSet
```

or

```text
EvidenceRepresentationFailureRecord
```

It SHALL NOT commit both.

## SRE-007-COMMIT-002

No Evidence Representation Instance, Grounding Assertion, unsupported-artifact reference, or partial set SHALL acquire committed constitutional standing before terminal artifact commitment.

## SRE-007-COMMIT-003

Commitment SHALL be atomic.

## SRE-007-COMMIT-004

After commitment, the terminal artifact and all contained Evidence Representation Instances and Grounding Assertions SHALL be immutable.

## SRE-007-COMMIT-005

Additional evidence, corrected evidence, changed relationships, new references, changed statuses, or profile changes SHALL require a new Evidence Representation Operation and a new terminal artifact.

## SRE-007-COMMIT-006

A later publication SHALL NOT erase, rewrite, or silently replace an earlier committed publication.

---

# 25. Determinism and replay

**Specification level:** Constitutional requirement

## SRE-007-REPLAY-001

Given identical admitted constitutional inputs, identical profile and registry versions, identical schemas, and identical deterministic derivation rules, a conforming implementation SHALL produce constitutionally equivalent output.

## SRE-007-REPLAY-002

Replayability SHALL include the ability to reconstruct:

- evidence candidate admission;
- Evidence Class assignment;
- Origin assignment;
- Representation Basis assignment;
- Evidence Status assignment;
- Representation Status assignment;
- source and artifact references;
- derived-observation construction;
- composite-evidence construction;
- Grounding Assertion construction;
- terminal commitment outcome.

## SRE-007-REPLAY-003

Replayability SHALL NOT require re-performing external retrieval when the external artifact is unavailable, but the unavailability and preserved reference state SHALL remain explicit.

---

# 26. Downstream handoff

**Specification level:** Constitutional boundary

## SRE-007-HANDOFF-001

A committed `InterpretationEvidenceSet` MAY be handed downstream only as a non-authorizing constitutional artifact.

## SRE-007-HANDOFF-002

Downstream consumers MAY use the set for authorized provenance construction, reconciliation, normalization, validation, canonical request construction, audit, or other expressly governed purposes.

## SRE-007-HANDOFF-003

Receipt of an `InterpretationEvidenceSet` SHALL NOT grant a downstream consumer authority to:

- treat represented evidence as true;
- treat represented evidence as sufficient;
- treat represented evidence as relevant;
- authorize execution;
- bypass unresolved evidence states;
- alter committed evidence or Grounding Assertions.

## SRE-007-HANDOFF-004

Downstream transformations SHALL preserve all identities and references required for provenance and deterministic reconstruction.

---

# 27. Deferred responsibilities

**Specification level:** Constitutional boundary

## SRE-007-DEFER-001

Contract 007 SHALL defer provenance, lineage, custody, transformation history, and provenance completeness to Contract 008.

## SRE-007-DEFER-002

Contract 007 SHALL defer proposal reconciliation, conflict adjudication, duplicate treatment, equivalence handling, and source-precedence decisions to Contract 009.

## SRE-007-DEFER-003

Contract 007 SHALL defer semantic normalization and canonical representation decisions to Contract 010.

## SRE-007-DEFER-004

Contract 007 SHALL defer canonical request construction, request validity, issuance, authorization, planning, execution, generation, and release to their governing downstream contracts.

## SRE-007-DEFER-005

Evidence sufficiency, evidence quality, evidence relevance, evidence weighting, factual verification, truth determination, and certification SHALL remain outside Contract 007 unless a future constitutional authority expressly governs them.

---

# 28. Invariants

**Specification level:** Constitutional invariant

## SRE-007-INVARIANT-001

Representation SHALL NOT create evidence solely from the existence of a semantic artifact.

## SRE-007-INVARIANT-002

Evidence SHALL remain independently represented from the artifact it supports.

## SRE-007-INVARIANT-003

Support SHALL exist only through an explicit Grounding Assertion.

## SRE-007-INVARIANT-004

Grounding SHALL NOT modify evidence or supported meaning.

## SRE-007-INVARIANT-005

Evidence Representation SHALL NOT imply evaluation.

## SRE-007-INVARIANT-006

Evidence Representation SHALL NOT imply truth.

## SRE-007-INVARIANT-007

Evidence Representation SHALL NOT imply sufficiency.

## SRE-007-INVARIANT-008

Evidence Origin SHALL NOT imply authority.

## SRE-007-INVARIANT-009

A Grounding Relationship SHALL NOT expand authority.

## SRE-007-INVARIANT-010

No unsupported evidence, source span, artifact reference, or support relationship SHALL be invented.

## SRE-007-INVARIANT-011

No committed Evidence Representation Instance, Grounding Assertion, InterpretationEvidenceSet, or failure record SHALL be mutated.

## SRE-007-INVARIANT-012

Every operation SHALL terminate in exactly one atomic committed success or failure artifact.

## SRE-007-INVARIANT-013

Structural validation SHALL remain distinct from factual, semantic, and evaluative judgment.

## SRE-007-INVARIANT-014

Greater structural precision SHALL NOT create greater operational authority.

---

# 29. Minimum conformance requirements

**Specification level:** Constitutional requirement

## SRE-007-CONFORM-001

An implementation conforms to Contract 007 only if it:

- recognizes the Evidence Representation Authority;
- consumes only authorized constitutional inputs;
- independently represents evidence;
- preserves logical and representation identity;
- applies an explicit Grounding Representation Profile;
- applies versioned registries;
- distinguishes Evidence Status from Representation Status;
- represents support only through Grounding Assertions;
- preserves evidence and supported-artifact independence;
- performs structural validation only within bounded authority;
- produces exactly one atomic success or failure artifact;
- preserves immutability;
- supports deterministic replay;
- preserves all deferred-responsibility boundaries;
- does not assess sufficiency, truth, relevance, confidence, or authority.

## SRE-007-CONFORM-002

Silent evidence fabrication, silent semantic alteration, implicit grounding, mutable publication, non-atomic commitment, or evaluative authority SHALL constitute non-conformance.

---

# 30. Architectural conformance declaration

**Informative — Non-Normative**

This section records the intended architectural relationship of Contract 007 to its governing architectural specifications. It does not create an independent conformance-assessment or certification authority.

| Governing specification | Architectural requirement | Contract 007 disposition |
|---|---|---|
| `SRE-ARCH-001` | Canonical Representation Pattern | Conforming |
| `SRE-ARCH-001` | Logical identity distinct from representation identity | Conforming |
| `SRE-ARCH-001` | Bounded representation authority | Conforming |
| `SRE-ARCH-001` | Explicit profiles and registries | Conforming |
| `SRE-ARCH-001` | Atomic success or failure publication | Conforming |
| `SRE-ARCH-001` | Immutable committed artifacts | Conforming |
| `SRE-ARCH-002` | Evidence independence | Conforming |
| `SRE-ARCH-002` | Explicit Grounding Assertions | Conforming |
| `SRE-ARCH-002` | Grounding is representational, not evaluative | Conforming |
| `SRE-ARCH-002` | Grounding does not establish truth, relevance, sufficiency, confidence, or authority | Conforming |
| `SRE-ARCH-002` | Grounding publication is immutable and replayable | Conforming |

---

# 31. Constitutional summary

**Specification level:** Informative summary of normative doctrine

Contract 007 establishes a bounded constitutional authority for representing evidence and explicit support relationships.

Its constitutional progression is:

```text
Authorized Constitutional Inputs
        ↓
Evidence Representation Operation
        ↓
Evidence Representation Instance
        ↓
Grounding Assertion
        ↓
InterpretationEvidenceSet
        ↓
Deferred Provenance, Reconciliation, Normalization, and Evaluation
```

The progression reflects the following doctrine:

```text
Evidence Representation
        ≠
Evidence Evaluation
        ≠
Evidence Sufficiency
        ≠
Truth
        ≠
Authority
```

Contract 007 therefore increases constitutional traceability and structural precision without increasing operational authority.
