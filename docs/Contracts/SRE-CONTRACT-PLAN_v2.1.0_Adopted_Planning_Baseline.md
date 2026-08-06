# Structured Request Engine

## Contract Planning Record

**Document ID:** `SRE-CONTRACT-PLAN`  
**Version:** `v2.1.0`  
**Status:** Adopted Planning Baseline — Harmonized Contract Family  
**Project:** Structured-Request-Engine  
**Rust package family:** `structured-request-*`  
**Supersedes:** `SRE-CONTRACT-PLAN v2.0.0`  

---

# 1. Purpose

This document defines the authoritative planning framework for the Structured Request Engine contract series. It preserves the original constitutional purpose and lifecycle while formally decomposing the processing, construction, issuance, and handoff responsibilities into bounded normative contracts.

It establishes:

* the identity and architectural position of the Structured Request Engine;
* the authority boundaries governing every contract;
* the complete Contract 000–015 inventory;
* the governing question, constitutional subject, constitutional act, authority, inputs, publications, failures, dependencies, and implementation expectations of each contract;
* the canonical artifact flow;
* the contract-family dependency order;
* the review, drafting, adoption, and implementation-planning gates;
* the rationale for expanding the original processing inventory.

This planning record does not replace the normative contracts. It constrains their scope so that normative drafting realizes an approved architecture rather than inventing one.

## 1.1 v2.1.0 Harmonization Amendment

Version 2.1.0 preserves the Contract 000–015 inventory adopted in version 2.0.0 and introduces no additional contract. It harmonizes the planning baseline by:

* establishing the Constitutional Question Doctrine as a family-wide boundary rule;
* confirming exactly one governing constitutional question for every contract;
* assigning semantic conflict recognition and reconciliation disposition to Contract 009;
* assigning structural completeness and construction eligibility to Contract 012;
* limiting Contract 013 to mechanical canonical request construction;
* distinguishing unresolved, deferred, blocked, and incomplete valid outcomes from operation failure;
* replacing ambiguous uses of “contradiction” and “completeness” with authority-specific terminology;
* requiring Contracts 003–008 to harmonize stale downstream references with the adopted 009–015 sequence;
* confirming that provisional `SRE-ARCH-003` is architectural evidence only and is not a normative dependency until formally adopted.

This amendment resolves responsibility redistribution created by removal of the former standalone contradiction-and-completeness stage. No constitutional responsibility is discarded. Each responsibility is assigned to the contract whose governing question and authority properly own it.

---

# 2. Planning Amendment and Rationale

The original planning baseline identified the same major lifecycle responsibilities now assigned to Contracts 009–015: reconciliation, normalization, canonical ordering, structural validation, canonical request construction, deterministic identity, serialization, and downstream handoff. However, several of those responsibilities were grouped within a smaller contract inventory.

Architectural development of Contracts 003–008 established a stronger constitutional pattern:

```text
One governing question
One constitutional subject
One constitutional act
One bounded authority
One principal successful publication
```

Once the representation artifacts became immutable, independently identified, evidence-linked, and provenance-preserving, the downstream processing stages could no longer remain implicit substeps of a broad “canonical request” operation. Each stage performs a materially distinct constitutional act and can succeed, fail, evolve, and be audited independently.

Version 2.0.0 formalized, and version 2.1.0 preserves, the following bounded authorities:

```text
009 Reconcile
010 Normalize
011 Order
012 Validate
013 Construct
014 Issue
015 Transfer
```

The amendment does not add new SRE mission scope. It makes previously planned lifecycle responsibilities explicit and constitutionally bounded.

No additional contract may be inserted into the 000–015 baseline without a formal plan amendment establishing that the proposed responsibility cannot be governed by an existing contract without expanding that contract’s constitutional authority.

---

# 3. Project Identity

The Structured Request Engine is the portable request-structuring boundary for Mineard AI systems.

Its role is:

> Convert source input into a canonical, provenance-preserving, machine-readable representation of what was requested without granting execution authority, selecting providers, planning work, invoking tools, producing an answer, or releasing output.

The engine may identify and represent requested objectives, declared constraints, capability requirements, source references, ambiguities, assumptions, uncertainty, interpretation evidence, relationships, provenance, lineage, reconciliation decisions, canonical structure, identity, issuance, and handoff state.

It may not transform those descriptions into permission to act.

```text
CanonicalStructuredRequest
        != AuthorizedRequest
        != GenerationEnvelope
        != ExecutionPlan
        != GeneratedOutput
```

Canonicalization records a request in a stable form. Canonicalization is not authorization.

---

# 4. Architectural Position

```text
Human / Application / API / Referenced Input
                     |
                     v
          Structured Request Engine
                     |
                     v
        CanonicalStructuredRequest
                     |
                     v
      Authority / Governance Boundary
        IBOS, SACS, or another lawful
             downstream authority
                     |
                     v
       Planning / Generation / Tools
                     |
                     v
              Result / Release
```

The SRE belongs before authority evaluation, governance decisions, planning, generation, provider invocation, tool execution, and output release. Downstream systems may consume its output but may not silently redefine its identity, semantics, evidence, provenance, or issuance history.

---

# 5. Contract-Program Objective

The contract program shall produce a complete normative specification from which the SRE can be implemented without requiring implementers to invent architectural meaning.

The complete family must define:

1. what enters the system;
2. how interpretation remains optional, external, and untrusted;
3. how objectives, constraints, capability needs, ambiguity, assumptions, uncertainty, evidence, and provenance are represented;
4. how multiple representations are reconciled;
5. how meaning-preserving normalization occurs;
6. how canonical ordering is established;
7. how structural construction eligibility is evaluated;
8. how the canonical request is mechanically constructed;
9. how deterministic identity and issuance standing are established;
10. how the request is transferred to a downstream boundary;
11. how every stage represents failure without inventing certainty or authority;
12. how every decision remains replayable, attributable, and version-bound.

The contracts shall distinguish:

* **Constitutional concepts** — distinctions every conforming implementation must preserve.
* **Logical artifacts** — normative information groupings that must be representable.
* **Required runtime artifacts** — inspectable values every conforming implementation must construct, consume, or emit.
* **Implementation conveniences** — internal structures not mandated by the contracts.

A named concept does not automatically require a dedicated Rust type, module, service, file, or serialized artifact.

---

# 6. Family-Wide Constitutional Principles

## 6.1 Canonicalization is not authorization

A CanonicalStructuredRequest represents what was requested. It does not mean the request is approved, permitted, safe, feasible, funded, authenticated, executable, schedulable, or supported by available capabilities.

## 6.2 Interpretation is not authority

Interpreter outputs are proposals. No interpreter may independently issue a canonical request or authorize downstream action.

## 6.3 Complete representation is not complete understanding

A request may be structurally complete while retaining unresolved ambiguity, assumptions, uncertainty, or conflict.

## 6.4 Capability requirement is not capability authority

A capability requirement does not establish availability, access, credentials, provider selection, or permission.

## 6.5 Evidence and provenance remain distinct

Evidence represents explicit support relationships. Provenance represents constitutional history. Neither may absorb or reevaluate the other.

## 6.6 Determinism begins after interpretation input

External interpretation may be nondeterministic. Once proposals, profiles, registries, and admitted artifacts enter the deterministic boundary, equivalent inputs must yield reproducible outputs.

## 6.7 Upstream immutability

No downstream contract may silently rewrite an upstream committed artifact. Refinement occurs through new publications linked to prior artifacts.

## 6.8 No hidden semantic invention

No stage may create objectives, constraints, capabilities, evidence, provenance, certainty, or authority merely to satisfy a downstream schema.

## 6.9 Failure is distinct from valid unresolved state

Unresolved, incomplete, blocked, rejected, not-applicable, or downstream-unavailable outcomes may be valid constitutional results. A failure artifact represents inability to perform or commit the governed act.

## 6.10 One principal publication per contract

Each contract shall identify one principal successful publication. Supporting decisions, manifests, acknowledgments, and failure artifacts may exist beneath that publication.

## 6.11 Meaning remains distinct from implementation form

Normative contract separation does not require one crate, service, module, or process per contract. Implementation shall preserve authority boundaries even where multiple authorities share code or runtime infrastructure.

## 6.12 Constitutional Question Doctrine

Every normative SRE contract SHALL answer exactly one governing constitutional question.

The governing question SHALL:

* identify the contract's unique constitutional responsibility;
* be answerable by that contract's principal successful publication;
* identify a boundary that no neighboring contract may silently absorb;
* avoid combining independent constitutional acts;
* remain stable across implementation forms.

If one contract answers multiple independent governing questions, the contract SHALL be reviewed for decomposition. If multiple contracts answer the same governing question, the family SHALL be reviewed for overlap or authority conflict.

The preferred constitutional pattern is:

```text
One Governing Question
        ↓
One Constitutional Subject
        ↓
One Constitutional Act
        ↓
One Bounded Authority
        ↓
One Principal Successful Publication
```

## 6.13 Conflict and Completeness Doctrine

Conflict and completeness SHALL be qualified by constitutional domain. The unqualified phrases `conflict resolution`, `contradiction evaluation`, and `request completeness` SHALL NOT be used where they obscure authority ownership.

The family SHALL distinguish:

```text
Represented Conflict
        upstream fact preserved by Contracts 003–008

Reconciliation Conflict
        semantic alternatives evaluated for downstream standing by Contract 009

Unresolved Reconciliation
        valid Contract 009 disposition preserving unresolved alternatives

Structural Conflict Defect
        malformed, missing, or profile-invalid conflict representation under Contract 012

Construction Failure
        inability to mechanically assemble an eligible artifact under Contract 013
```

The family SHALL also distinguish:

```text
Domain Representation Completeness
        Contracts 003–008

Reconciliation Disposition Completeness
        Contract 009

Structural Construction Eligibility
        Contract 012

Canonical Artifact Construction Completeness
        Contract 013
```

A semantically unresolved request MAY be structurally eligible where the applicable profile permits preserved unresolved state. Structural eligibility SHALL NOT imply semantic resolution.

---

# 7. Contract-Family Organization

## 7.1 Constitutional Foundation

* Contract 000

## 7.2 Representation Family

* Contracts 001–008

These contracts admit and represent constitutional information. They do not reconcile, normalize, validate, construct, issue, or transfer a request.

## 7.3 Deterministic Processing Family

* Contracts 009–013

These contracts transform immutable representations through reconciliation, normalization, ordering, validation, and construction.

## 7.4 Issuance and Handoff Family

* Contracts 014–015

These contracts establish identity-bearing issuance and govern the final transfer to a downstream authority boundary.

---

# 8. Governing Question Map

The contract family is constitutionally bounded by the following governing questions. These questions are authoritative planning constraints for normative drafting.

| Contract | Governing constitutional question | Principal answer publication |
|---|---|---|
| 000 | What is the Structured Request Engine, where does its authority begin and end, and which invariants govern every subordinate contract? | SRE constitutional architecture |
| 001 | What externally supplied or referenced material may enter the SRE, and what authoritative facts of admission must be preserved? | `SourceIntakeRecord` |
| 002 | How may an external interpreter propose meaning without gaining authority to issue or alter a canonical request? | `AdmittedInterpretationProposalSet` |
| 003 | What requested outcomes are represented, and how are their relationships recorded without converting them into permission or execution plans? | `DeclaredObjectiveSet` |
| 004 | What source-level limits, requirements, inclusions, exclusions, and conditions are represented without treating them as downstream policy? | `DeclaredConstraintSet` |
| 005 | What abstract capabilities may be required by the request without selecting, granting, or invoking any concrete capability? | `CapabilityRequirementSet` |
| 006 | What unresolved meaning, declared or inferred assumptions, and uncertainty must remain explicit rather than being silently resolved? | `SemanticClarificationSet` |
| 007 | What evidence has been represented in support of represented semantic artifacts? | `InterpretationEvidenceSet` |
| 008 | What constitutional history is represented as having produced, affected, stewarded, published, replaced, or superseded a represented artifact? | `ProvenanceRecordSet` |
| 009 | What represented semantic alternatives receive what governed downstream standing? | `SemanticReconciliationSet` |
| 010 | How are reconciled semantic elements expressed in stable canonical forms without changing represented meaning? | `NormalizedRequestRepresentation` |
| 011 | In what deterministic sequence must normalized request elements and relationships be arranged? | `CanonicallyOrderedRequestRepresentation` |
| 012 | Is the canonically ordered representation structurally eligible for canonical request construction? | `StructuralValidationResult` |
| 013 | What immutable canonical request artifact is mechanically constructed from the eligible validated representation? | `ConstructedCanonicalRequest` |
| 014 | How does the constructed request acquire deterministic identity and formal issuance standing? | `CanonicalStructuredRequest` |
| 015 | How is the issued request transferred to a declared downstream constitutional authority while preserving identity, integrity, provenance, and non-authorization? | `CanonicalRequestHandoffRecord` |

The deterministic processing sequence may therefore be summarized as:

```text
009 — What proceeds?
010 — How is it expressed?
011 — In what order?
012 — Is it structurally eligible?
013 — What artifact is constructed?
014 — What identity and issuance standing does it receive?
015 — How is custody transferred?
```

---

# 9. Canonical Lifecycle

```text
001 Source Admission
        |
        v
002 Interpretation Boundary
        |
        v
003 Objectives
        |
        v
004 Constraints
        |
        v
005 Capability Requirements
        |
        v
006 Ambiguity / Assumptions / Uncertainty
        |
        v
007 Evidence Representation
        |
        v
008 Provenance Representation
        |
        v
009 Proposal Reconciliation
        |
        v
010 Semantic Normalization
        |
        v
011 Canonical Ordering
        |
        v
012 Structural Validation
        |
        v
013 Canonical Request Construction
        |
        v
014 Identity and Issuance
        |
        v
015 Downstream Handoff
```

The diagram is a constitutional dependency sequence, not necessarily a one-process runtime sequence. A conforming implementation may pipeline, batch, cache, or co-locate operations, but it may not collapse their authority distinctions.

---

# 10. Contract Specification Template

Every contract draft shall define at minimum:

1. Purpose.
2. Governing question.
3. Constitutional subject.
4. Constitutional act.
5. Scope.
6. Explicit exclusions.
7. Canonical inputs.
8. Principal successful publication.
9. Supporting publications.
10. Granted authority.
11. Prohibited authority.
12. Success conditions.
13. Valid non-success states.
14. Failure conditions and failure artifact.
15. Identity and immutability rules.
16. Registry and profile dependencies.
17. Neighbor boundaries.
18. Normative dependencies.
19. Downstream consumers.
20. Expected implementation responsibility.

---

# 11. Detailed Contract Inventory

## Contract 000 — Architecture, Identity, and Authority

**Family:** Constitutional Foundation  
**Governing question:** What is the Structured Request Engine, where does its authority begin and end, and which invariants govern every subordinate contract?  
**Constitutional subject:** The Structured Request Engine as a bounded constitutional subsystem.  
**Constitutional act:** **Constitute**

### Purpose

Establish the constitutional identity, architectural position, authority boundary, lifecycle doctrine, identity distinctions, and non-expansion invariants of the Structured Request Engine.

### Must define

* project purpose and architectural position;
* canonicalization doctrine;
* authority and prohibition model;
* fundamental artifact distinctions;
* successful and unsuccessful lifecycle outcomes;
* relationship to interpreters, downstream governance, Generation-Stack, tools, and providers;
* contract-family invariants and dependency doctrine;

### Explicitly outside scope

* runtime algorithms;
* domain-specific representation rules owned by Contracts 001–015;
* provider selection, planning, execution, generation, release, or authorization;

### Canonical inputs

* Project charter and adopted architectural source material..

### Principal successful publication

The normative constitutional architecture of the SRE contract family; no standalone runtime artifact is required solely because Contract 000 exists.

### Supporting publications

* Normative dependency map.
* Controlled terminology and invariant set.

### Granted authority

* define constitutional concepts and boundaries;
* bind all subordinate contracts;
* prohibit authority expansion;
* classify successful and failed lifecycle outcomes;

### Prohibited authority

* admit source material;
* interpret source meaning;
* construct or issue requests;
* authorize downstream action;

### Success condition

The SRE family has a coherent constitutional identity and every later contract can be checked for conformance to it.

### Failure condition

A subordinate contract or implementation contradicts the authority, identity, or non-expansion doctrines established here.

### Normative dependencies

None.

### Downstream consumers

Contracts 001–015 and all conforming implementations.

### Expected implementation responsibility

Cross-cutting architecture, contract schemas, invariants, and conformance tests rather than a single runtime module.

---

## Contract 001 — Source Admission

**Family:** Representation Family  
**Governing question:** What externally supplied or referenced material may enter the SRE, and what authoritative facts of admission must be preserved?  
**Constitutional subject:** Source submissions and their admitted source artifacts.  
**Constitutional act:** **Admit**

### Purpose

Define the boundary at which supplied material becomes an authoritative admitted source without assigning semantic meaning.

### Must define

* submission identity and source identity;
* source enumeration and boundaries;
* mechanically observable metadata;
* encoding and admissibility rules;
* source preservation;
* unsupported, unavailable, and malformed source states;
* successful and failed intake artifacts;

### Explicitly outside scope

* semantic interpretation;
* objective, constraint, capability, ambiguity, evidence, or provenance representation beyond intake facts;
* normalization, validation, construction, issuance, or authorization;

### Canonical inputs

* Externally supplied content.
* Externally referenced content resolvable under declared intake rules.
* Application-supplied submission metadata.

### Principal successful publication

`SourceIntakeRecord`

### Supporting publications

* `SourceIntakeFailureRecord`.
* Source identity and preservation references.

### Granted authority

* validate admissibility;
* establish submission and source identities;
* capture mechanically observable metadata;
* preserve admitted content;

### Prohibited authority

* assign meaning;
* repair semantic defects;
* infer request elements;
* authorize any downstream activity;

### Success condition

Exactly one authoritative SourceIntakeRecord is committed for the admitted submission.

### Failure condition

The submission cannot be admitted, preserved, identified, or represented under the applicable intake rules.

### Normative dependencies

Contract 000.

### Downstream consumers

Contract 002 and all later contracts requiring source identity, source content, or source references.

### Expected implementation responsibility

Source intake boundary, adapters, content preservation, identity derivation, and intake validation.

---

## Contract 002 — Interpretation Boundary

**Family:** Representation Family  
**Governing question:** How may an external interpreter propose meaning without gaining authority to issue or alter a canonical request?  
**Constitutional subject:** Interpretation requests, interpreter responses, and admitted interpretation proposals.  
**Constitutional act:** **Bound**

### Purpose

Define the optional, transport-neutral interface between admitted source material and one or more interpretation implementations while preserving the untrusted-proposal doctrine.

### Must define

* InterpretationRequest;
* InterpretationProposal;
* interpreter identity and version;
* declared interpretation bounds;
* finish status and proposal status;
* transport neutrality;
* malformed, partial, and incomplete responses;
* proposal admission and boundary failure;
* prohibition against canonical issuance by interpreters;

### Explicitly outside scope

* deterministic reconciliation;
* semantic normalization;
* request validation or construction;
* authorization of tools, providers, or execution;

### Canonical inputs

* `SourceIntakeRecord`.
* Admitted source references.
* Declared interpretation profile and interpreter metadata.

### Principal successful publication

`AdmittedInterpretationProposalSet`

### Supporting publications

* `InterpretationBoundaryFailureRecord`.
* Interpreter and proposal metadata.
* Boundary validation report.

### Granted authority

* request interpretation;
* receive zero or more proposals;
* validate proposal structure and declared bounds;
* admit structurally eligible proposals;

### Prohibited authority

* treat proposals as authoritative;
* silently repair proposal meaning;
* reconcile competing proposals;
* issue a CanonicalStructuredRequest;

### Success condition

One admitted proposal set is committed, including the valid possibility of zero admitted proposals when the profile permits it.

### Failure condition

The interpretation exchange cannot be represented, attributed, or evaluated under the boundary rules.

### Normative dependencies

Contracts 000–001.

### Downstream consumers

Contracts 003–009.

### Expected implementation responsibility

Interpreter seam, transport adapters, response validation, interpreter identity capture, and proposal admission.

---

## Contract 003 — Objectives and Objective Relationships

**Family:** Representation Family  
**Governing question:** What requested outcomes are represented, and how are their relationships recorded without converting them into permission or execution plans?  
**Constitutional subject:** Declared objectives and objective relationships.  
**Constitutional act:** **Represent**

### Purpose

Define the constitutional representation of requested outcomes, their origins, scopes, conditions, and relationships.

### Must define

* declared, primary, secondary, conditional, compound, and parallel objectives;
* objective origin and evidence references;
* objective scope and relationships;
* objective identity and representation instances;
* unsupported, contradictory, and unresolved objective states;
* immutable set-level publication under ARCH-001;

### Explicitly outside scope

* constraint representation;
* capability determination;
* reconciliation among competing proposals;
* authorization, planning, or execution;

### Canonical inputs

* `SourceIntakeRecord`.
* `AdmittedInterpretationProposalSet`.
* Applicable objective representation profile and registries.

### Principal successful publication

`DeclaredObjectiveSet`

### Supporting publications

* Objective representation decisions.
* `ObjectiveRepresentationFailureRecord`.

### Granted authority

* represent explicit and admitted inferred objectives;
* record relationships and conditions;
* preserve unsupported or unresolved states;

### Prohibited authority

* invent objectives;
* rank objectives without declared authority;
* convert objectives into permission or plans;
* reconcile conflicts owned by Contract 009;

### Success condition

One immutable DeclaredObjectiveSet is committed.

### Failure condition

The objective domain cannot be represented under the applicable profile without violating source, evidence, or identity requirements.

### Normative dependencies

Contracts 000–002 and ARCH-001.

### Downstream consumers

Contracts 004–015.

### Expected implementation responsibility

Objective representation authority, objective registries, identity derivation, and set publication.

---

## Contract 004 — Declared Constraints

**Family:** Representation Family  
**Governing question:** What source-level limits, requirements, inclusions, exclusions, and conditions are represented without treating them as downstream policy?  
**Constitutional subject:** Declared constraint representations and their relationships.  
**Constitutional act:** **Represent**

### Purpose

Define how request constraints are represented as constitutional facts of the request rather than policy decisions or execution restrictions.

### Must define

* explicit, inferred, application-supplied, and referenced-artifact constraints;
* negative constraints, inclusions, exclusions, temporal, format, resource, confidentiality, and scope constraints;
* constraint origin, evidence, priority when explicitly supplied, conditions, conflicts, and relationships;
* immutable set-level publication;

### Explicitly outside scope

* policy creation or policy evaluation;
* capability authorization;
* conflict reconciliation;
* execution enforcement;

### Canonical inputs

* Artifacts from Contracts 001–003.
* Applicable constraint representation profile and registries.

### Principal successful publication

`DeclaredConstraintSet`

### Supporting publications

* Constraint representation decisions.
* `ConstraintRepresentationFailureRecord`.

### Granted authority

* represent admitted constraint claims;
* preserve conflict and explicit priority information;
* record applicability and conditions;

### Prohibited authority

* invent policy;
* convert a constraint into authority;
* silently choose between conflicting constraints;
* enforce execution behavior;

### Success condition

One immutable DeclaredConstraintSet is committed.

### Failure condition

The constraint domain cannot be represented under the applicable profile.

### Normative dependencies

Contracts 000–003 and ARCH-001.

### Downstream consumers

Contracts 005–015.

### Expected implementation responsibility

Constraint representation authority, registries, identity derivation, and publication.

---

## Contract 005 — Capability Requirements

**Family:** Representation Family  
**Governing question:** What abstract capabilities may be required by the request without selecting, granting, or invoking any concrete capability?  
**Constitutional subject:** Capability requirement representations.  
**Constitutional act:** **Represent**

### Purpose

Define abstract capability needs while preserving the distinction between a requirement, capability availability, access, provider selection, and authorization.

### Must define

* capability class and description;
* necessity, optionality, and conditionality;
* requirement origin and evidence basis;
* capability relationships and composition non-expansion;
* unsupported capability descriptions;
* immutable set-level publication;

### Explicitly outside scope

* tool or provider selection;
* credential or connector evaluation;
* capability availability claims;
* authorization or execution;

### Canonical inputs

* Artifacts from Contracts 001–004.
* Applicable capability representation profile and registries.

### Principal successful publication

`CapabilityRequirementSet`

### Supporting publications

* Capability representation decisions.
* `CapabilityRepresentationFailureRecord`.

### Granted authority

* represent abstract capability needs;
* classify necessity and origin;
* record conditional and relational requirements;

### Prohibited authority

* claim capability availability;
* select tools or providers;
* grant access;
* authorize execution;

### Success condition

One immutable CapabilityRequirementSet is committed.

### Failure condition

Capability requirements cannot be represented under applicable rules.

### Normative dependencies

Contracts 000–004 and ARCH-001.

### Downstream consumers

Contracts 006–015 and downstream governance systems.

### Expected implementation responsibility

Capability requirement representation authority and registry-backed publication.

---

## Contract 006 — Ambiguity, Assumptions, and Uncertainty

**Family:** Representation Family  
**Governing question:** What unresolved meaning, declared or inferred assumptions, and uncertainty must remain explicit rather than being silently resolved?  
**Constitutional subject:** Ambiguity, assumption, and uncertainty representations.  
**Constitutional act:** **Represent**

### Purpose

Preserve incomplete understanding as explicit constitutional information and distinguish valid unresolved states from failure.

### Must define

* ambiguity records and alternatives;
* ambiguity scope and severity;
* assumption origin and status;
* explicit and inferred assumptions;
* uncertainty dimensions and confidence representation;
* clarification requirements;
* survivable, blocking, unresolved, and not-applicable states;
* immutable set-level publication;

### Explicitly outside scope

* silent resolution;
* clarification dialogue management;
* reconciliation;
* authorization or refusal;

### Canonical inputs

* Artifacts from Contracts 001–005.
* Applicable clarification representation profile and registries.

### Principal successful publication

`SemanticClarificationSet` comprising ambiguity, assumption, and uncertainty representations.

### Supporting publications

* Clarification representation decisions.
* `SemanticClarificationFailureRecord`.

### Granted authority

* represent ambiguity alternatives;
* record assumptions and uncertainty;
* mark clarification needs and blocking conditions;

### Prohibited authority

* invent certainty;
* choose an alternative without authority;
* treat incompleteness as automatic failure;
* authorize downstream action;

### Success condition

One immutable SemanticClarificationSet is committed, including unresolved or blocking states when valid.

### Failure condition

The domain cannot be represented or attributed under applicable rules.

### Normative dependencies

Contracts 000–005 and ARCH-001.

### Downstream consumers

Contracts 007–015.

### Expected implementation responsibility

Clarification representation authority and deterministic uncertainty/assumption schemas.

---

## Contract 007 — Evidence Representation

**Family:** Representation Family  
**Governing question:** What explicit support relationships connect represented request claims to admitted sources or other declared origins?  
**Constitutional subject:** Evidence assertions, evidence relationships, and grounding publication.  
**Constitutional act:** **Ground**

### Purpose

Define evidence representation and explicit support relationships without evaluating truth, policy, feasibility, or downstream authorization.

### Must define

* evidence subjects and evidence items;
* source spans, referenced artifacts, and declared origins;
* support relationship types;
* evidence sufficiency status as representation;
* unsupported and partially supported claims;
* integrity and identity;
* immutable grounding publication;

### Explicitly outside scope

* provenance history;
* truth determination;
* policy or safety assessment;
* reconciliation or canonical construction;

### Canonical inputs

* Artifacts from Contracts 001–006.
* Applicable evidence profile and registries.

### Principal successful publication

`InterpretationEvidenceSet`

### Supporting publications

* Evidence representation decisions.
* `EvidenceRepresentationFailureRecord`.

### Granted authority

* link claims to admitted sources and origins;
* represent support, partial support, absence, and non-applicability;
* publish grounding references;

### Prohibited authority

* declare a claim true merely because evidence exists;
* alter represented claims;
* reinterpret provenance;
* authorize downstream effects;

### Success condition

One immutable InterpretationEvidenceSet is committed.

### Failure condition

Required support relationships cannot be represented, resolved, or attributed.

### Normative dependencies

Contracts 001–006, ARCH-001, and ARCH-002.

### Downstream consumers

Contracts 008–015 and downstream assurance or governance systems.

### Expected implementation responsibility

Evidence/grounding authority, source-span references, relationship registries, and set publication.

---

## Contract 008 — Provenance Representation

**Family:** Representation Family  
**Governing question:** What represented constitutional history produced, transformed, held, published, replaced, or superseded the request artifacts?  
**Constitutional subject:** Provenance subjects, lineage assertions, and provenance events.  
**Constitutional act:** **Trace**

### Purpose

Define bounded, non-evaluative representation of the constitutional history of SRE artifacts and operations.

### Must define

* provenance subjects and representation instances;
* lineage assertions;
* transformation, custody, and publication events;
* application, interpreter, registry, configuration, and artifact lineage;
* replacement and supersession history;
* deterministic reconstruction references;
* immutable set-level publication;

### Explicitly outside scope

* evidence evaluation;
* truth or quality assessment;
* reconciliation;
* authorization or custody transfer outside the represented history;

### Canonical inputs

* Committed artifacts and operation records from Contracts 001–007.
* Applicable provenance profile and registries.

### Principal successful publication

`ProvenanceRecordSet`

### Supporting publications

* Provenance representation decisions.
* `ProvenanceRepresentationFailureRecord`.

### Granted authority

* represent ancestry and events;
* associate subjects with lineage and publication history;
* preserve replacement and supersession relationships;

### Prohibited authority

* reevaluate evidence;
* alter historical subjects;
* infer authority from lineage;
* erase prior history;

### Success condition

One immutable ProvenanceRecordSet is committed.

### Failure condition

Required lineage or event representation cannot be established under applicable rules.

### Normative dependencies

Contracts 001–007, ARCH-001, and ARCH-002. `SRE-ARCH-003` remains provisional architectural evidence and is not a normative dependency.

### Downstream consumers

Contracts 009–015 and downstream audit, governance, and replay systems.

### Expected implementation responsibility

Provenance authority, event and lineage registries, reconstruction references, and set publication.

---

## Contract 009 — Proposal Reconciliation

**Family:** Deterministic Processing Family  
**Governing question:** Given multiple admitted semantic representations, what governed semantic basis may proceed downstream while preserving every material alternative and decision basis?  
**Constitutional subject:** The governed downstream semantic basis produced from admitted semantic alternatives.  
**Constitutional act:** **Reconcile**

### Purpose

Deterministically reconcile compatible, duplicate, complementary, alternative, competing, or conflicting admitted representations by assigning governed downstream standing while preserving every material alternative, decision basis, and upstream artifact identity without inventing meaning.

The organizing doctrine is:

> **Reconciliation determines downstream semantic standing without erasing upstream constitutional plurality.**

### Must define

* Reconciliation Subjects and Reconciliation Groups;
* eligibility rules for within-domain comparison;
* bounded cross-domain conflict classification;
* duplicate, equivalence, compatibility, complementarity, alternative, competition, and conflict treatment;
* relationship-specific source and declaration precedence where explicitly authorized;
* `Merged`, `Selected`, `Preserved`, `Split`, `Deferred`, `Rejected`, and `Unresolved` dispositions;
* Reconciliation Decisions and Reconciled Semantic Elements;
* decision basis, preserved and rejected alternatives, and conflict records;
* profile-bounded deterministic tie-breaking;
* reconciliation disposition completeness;
* distinction between valid unresolved reconciliation and operation failure;

### Explicitly outside scope

* semantic normalization;
* canonical ordering;
* structural validation;
* request construction;
* truth, policy, feasibility, or authorization;

### Canonical inputs

* Committed representation artifacts from Contracts 003–008.
* Admitted interpretation proposals from Contract 002.
* Reconciliation profile and applicable registries.

### Principal successful publication

`SemanticReconciliationSet`

### Supporting publications

* `ReconciliationDecisionSet`.
* Conflict and alternative records.
* `SemanticReconciliationFailureRecord`.

### Granted authority

* form profile-authorized Reconciliation Groups;
* compare eligible admitted representations;
* merge profile-equivalent or compatible representations;
* select only when an explicit relationship-specific precedence rule permits;
* preserve unresolved, deferred, rejected, and non-selected alternatives;
* classify bounded cross-domain incompatibility without performing policy adjudication;
* publish deterministic reconciliation decisions and downstream standing;

### Prohibited authority

* reinterpret source material;
* invent objectives, constraints, capabilities, evidence, or provenance;
* erase rejected alternatives;
* normalize expression;
* use lineage, shared evidence, source class, model confidence, or implementation preference as automatic proof of semantic equivalence or precedence;
* perform policy-based conflict resolution;
* authorize execution;

### Success condition

One immutable SemanticReconciliationSet is committed, including valid unresolved, deferred, or split outcomes where the profile permits.

### Failure condition

The authority cannot produce a lawful, deterministic, identity-preserving reconciliation publication under the declared profile. Conflict, deferral, rejection, or unresolved standing alone SHALL NOT constitute operation failure.

### Normative dependencies

Contracts 002–008.

### Downstream consumers

Contracts 010–015.

### Expected implementation responsibility

Deterministic semantic reconciliation engine, Reconciliation Group formation, equivalence/conflict/precedence registries, decision publication, lineage preservation, and replay tests.

---

## Contract 010 — Semantic Normalization

**Family:** Deterministic Processing Family  
**Governing question:** How are reconciled semantic elements expressed in stable canonical forms without changing their represented meaning?  
**Constitutional subject:** Reconciled semantic elements and their normalized expressions.  
**Constitutional act:** **Normalize**

### Purpose

Transform reconciled representations into stable, equivalent, profile-defined semantic forms while preserving identity, meaning, evidence, and provenance links.

### Must define

* lexical and identifier normalization;
* enum and controlled-vocabulary normalization;
* whitespace, encoding, numeric, temporal, unit, and locale-sensitive forms;
* collection normalization and duplicate treatment after reconciliation;
* normalization idempotence;
* meaning-preservation and reversibility references;
* profile and registry version capture;

### Explicitly outside scope

* conflict resolution;
* canonical ordering;
* structural validity determination;
* construction, identity issuance, or handoff;

### Canonical inputs

* `SemanticReconciliationSet`.
* Referenced upstream representations.
* Normalization profile and registries.

### Principal successful publication

`NormalizedRequestRepresentation`

### Supporting publications

* Normalization decision and mapping records.
* `NormalizationFailureRecord`.

### Granted authority

* map equivalent expressions to declared canonical forms;
* apply deterministic formatting and controlled vocabulary rules;
* preserve source and prior-form references;

### Prohibited authority

* change semantic scope or intent;
* resolve remaining conflicts;
* discard uncertainty or alternatives;
* invent missing data;

### Success condition

One immutable NormalizedRequestRepresentation is committed and repeated application under the same profile is idempotent.

### Failure condition

Normalization cannot preserve meaning or produce a profile-conformant representation.

### Normative dependencies

Contracts 003–009.

### Downstream consumers

Contracts 011–015.

### Expected implementation responsibility

Registry-backed semantic normalizer, canonical form mappings, and idempotence tests.

---

## Contract 011 — Canonical Ordering

**Family:** Deterministic Processing Family  
**Governing question:** In what deterministic sequence must normalized request elements and their relationships be arranged before validation and construction?  
**Constitutional subject:** The canonical sequence and structural placement of normalized request elements.  
**Constitutional act:** **Order**

### Purpose

Define deterministic ordering independent of serialization format, validation outcome, construction, and identity issuance.

### Must define

* ordering of objectives, constraints, capability requirements, ambiguities, assumptions, uncertainty, evidence, provenance, and reconciliation decisions;
* ordering of relationships and nested collections;
* stable tie-breaking and registry versioning;
* ordering independence from incidental source or insertion order;
* canonical sequence representation;

### Explicitly outside scope

* semantic normalization;
* validation of required structure;
* serialization bytes;
* request construction;
* identity derivation or issuance;

### Canonical inputs

* `NormalizedRequestRepresentation`.
* Ordering profile and registries.

### Principal successful publication

`CanonicallyOrderedRequestRepresentation`

### Supporting publications

* Ordering decision record.
* `CanonicalOrderingFailureRecord`.

### Granted authority

* apply deterministic ordering keys;
* order nested and related elements;
* publish explicit tie-break decisions;

### Prohibited authority

* change normalized values;
* drop or merge elements;
* declare structural validity;
* serialize or issue the request;

### Success condition

One immutable CanonicallyOrderedRequestRepresentation is committed and equivalent inputs produce equivalent order under the same profile.

### Failure condition

A deterministic lawful order cannot be established.

### Normative dependencies

Contract 010 and referenced outputs of Contracts 003–009.

### Downstream consumers

Contracts 012–015.

### Expected implementation responsibility

Canonical ordering engine, registry-defined sort keys, stable collection handling, and determinism tests.

---

## Contract 012 — Structural Validation

**Family:** Deterministic Processing Family  
**Governing question:** Does the canonically ordered representation satisfy every structural condition required to become a CanonicalStructuredRequest?  
**Constitutional subject:** The construction eligibility and structural conformance of the ordered representation.  
**Constitutional act:** **Validate**

### Purpose

Evaluate structural completeness, schema conformance, relationship integrity, reconciliation-disposition completeness, and construction prerequisites without modifying the candidate representation or resolving semantic conflict.

### Must define

* required and optional element rules;
* cardinality and relationship integrity;
* identifier and reference integrity;
* profile/version compatibility;
* cross-domain structural consistency;
* required presence and structural validity of reconciliation decisions and conflict records;
* profile-governed treatment of unresolved, deferred, rejected, blocked, incomplete, and not-applicable states;
* structural construction eligibility;
* valid unresolved and not-applicable states;
* validation rule registry and deterministic reports;

### Explicitly outside scope

* semantic reinterpretation;
* repair or mutation of the candidate;
* policy, safety, feasibility, or execution validation;
* request construction or issuance;

### Canonical inputs

* `CanonicallyOrderedRequestRepresentation`.
* Upstream evidence, provenance, reconciliation, and normalization references.
* Structural validation profile and rule registries.

### Principal successful publication

`StructuralValidationResult`

### Supporting publications

* Rule-level findings.
* Construction eligibility disposition.
* `StructuralValidationFailureRecord`.

### Granted authority

* evaluate declared structural rules;
* determine whether every eligible semantic element possesses a structurally valid reconciliation disposition;
* classify eligible, ineligible, blocked, incomplete, or not-applicable structural states;
* determine whether an unresolved semantic state is structurally permitted by profile;
* publish rule-level deterministic findings;

### Prohibited authority

* repair the representation;
* invent missing elements;
* reinterpret semantics;
* resolve semantic conflicts or choose among alternatives;
* treat semantic conflict as structural failure solely because conflict exists;
* construct or issue the request;

### Success condition

One immutable StructuralValidationResult is committed; success of the validation operation may lawfully report that construction is ineligible.

### Failure condition

The validation operation itself cannot be completed or its result cannot be committed.

### Normative dependencies

Contracts 001–011.

### Downstream consumers

Contract 013 and downstream conformance/audit systems.

### Expected implementation responsibility

Structural validator, rule registry, cross-reference checker, reconciliation-disposition completeness checker, construction-eligibility evaluator, and deterministic report publication.

---

## Contract 013 — Canonical Request Construction

**Family:** Deterministic Processing Family  
**Governing question:** What immutable CanonicalStructuredRequest may be mechanically constructed from a structurally eligible ordered representation?  
**Constitutional subject:** The pre-issuance canonical request artifact.  
**Constitutional act:** **Construct**

### Purpose

Mechanically assemble the structurally eligible validated semantic structure into the canonical request artifact without making new semantic, conflict, completeness, authority, identity, or transport decisions.

### Must define

* construction prerequisites;
* field and relationship assembly;
* required embedded references to objectives, constraints, capabilities, clarifications, evidence, provenance, and profiles;
* canonical logical schema;
* construction determinism;
* construction failure and non-construction states;
* distinction between constructed and issued artifacts;

### Explicitly outside scope

* semantic reconciliation or normalization;
* structural rule evaluation or construction-eligibility adjudication;
* semantic conflict disposition or completeness adjudication;
* final constitutional identity issuance;
* downstream transfer;
* authorization or execution;

### Canonical inputs

* Construction-eligible `StructuralValidationResult`.
* `CanonicallyOrderedRequestRepresentation`.
* Applicable construction profile and schema version.

### Principal successful publication

`ConstructedCanonicalRequest`

### Supporting publications

* Construction manifest.
* `CanonicalRequestConstructionFailureRecord`.

### Granted authority

* assemble validated fields and relationships;
* bind required version and reference information;
* produce a deterministic pre-issuance request artifact;

### Prohibited authority

* change meaning;
* waive or reinterpret validation findings;
* decide whether unresolved, deferred, blocked, or incomplete states are permissible;
* assign final issued standing;
* authorize action;
* select downstream recipients;

### Success condition

One immutable ConstructedCanonicalRequest is committed from an eligible validated representation.

### Failure condition

The eligible representation cannot be assembled under the applicable canonical schema.

### Normative dependencies

Contracts 001–012.

### Downstream consumers

Contract 014.

### Expected implementation responsibility

Canonical request builder, schema bindings, construction manifest generation, and deterministic assembly tests.

---

## Contract 014 — Identity and Issuance

**Family:** Issuance and Handoff Family  
**Governing question:** How does a constructed canonical request receive deterministic identity and formal constitutional standing as an issued request?  
**Constitutional subject:** The identity-bearing issued CanonicalStructuredRequest and its issuance act.  
**Constitutional act:** **Issue**

### Purpose

Derive and bind deterministic semantic identity, distinguish submission and event identity, commit the final issued artifact, and record its issuance without implying authorization.

### Must define

* semantic identity, submission identity, representation identity, and issuance event identity;
* identity derivation inputs and algorithms;
* version and registry binding;
* canonical serialization required for identity calculation where applicable;
* issuance eligibility and commitment;
* replacement, reissuance, and supersession rules;
* issuance record and failure;

### Explicitly outside scope

* downstream routing or transfer;
* governance or authorization decisions;
* provider selection, execution, generation, or release;

### Canonical inputs

* `ConstructedCanonicalRequest`.
* Construction manifest.
* Identity and issuance profile.
* Applicable schema, registry, and serialization versions.

### Principal successful publication

`CanonicalStructuredRequest`

### Supporting publications

* `CanonicalRequestIssuanceRecord`.
* Identity derivation manifest.
* `CanonicalRequestIssuanceFailureRecord`.

### Granted authority

* derive deterministic identity;
* bind required versions and digests;
* commit and publish the issued request;
* record reissuance or supersession;

### Prohibited authority

* alter request semantics;
* treat issuance as approval;
* select an execution path;
* transfer custody without Contract 015;

### Success condition

One immutable, identity-bearing CanonicalStructuredRequest and associated issuance record are committed.

### Failure condition

Identity cannot be deterministically derived, the artifact cannot be committed, or issuance prerequisites are not met.

### Normative dependencies

Contracts 000–013.

### Downstream consumers

Contract 015 and all lawful downstream governance boundaries.

### Expected implementation responsibility

Identity derivation, canonical serialization/digest binding, issuance registry, immutable publication, and supersession handling.

---

## Contract 015 — Downstream Handoff

**Family:** Issuance and Handoff Family  
**Governing question:** How may an issued CanonicalStructuredRequest be transferred to a declared downstream boundary while preserving identity, integrity, provenance, and non-authorization doctrine?  
**Constitutional subject:** The constitutional transfer and receipt status of an issued canonical request.  
**Constitutional act:** **Transfer**

### Purpose

Govern the final SRE boundary transition from issued request publication to downstream receipt without converting handoff into authorization, acceptance, scheduling, or execution.

### Must define

* handoff package and required references;
* recipient declaration and boundary identity;
* transfer event, custody, integrity, and acknowledgment;
* delivery, rejection, expiration, retry, duplicate, and unavailable-recipient states;
* handoff provenance continuation;
* completion of SRE responsibility;
* transport neutrality;

### Explicitly outside scope

* downstream authorization or acceptance semantics;
* planning, generation, provider invocation, tool use, execution, or output release;
* mutation of the issued request;

### Canonical inputs

* Issued `CanonicalStructuredRequest`.
* `CanonicalRequestIssuanceRecord`.
* Declared downstream boundary and handoff profile.

### Principal successful publication

`CanonicalRequestHandoffRecord`

### Supporting publications

* Transfer package manifest.
* Receipt or rejection acknowledgment.
* `CanonicalRequestHandoffFailureRecord`.

### Granted authority

* package and transfer the immutable issued request;
* verify integrity and recipient boundary identity;
* record delivery and acknowledgment states;
* preserve provenance continuation;

### Prohibited authority

* authorize the request;
* claim downstream acceptance beyond recorded acknowledgment;
* alter the request;
* select providers or execute work;

### Success condition

One immutable CanonicalRequestHandoffRecord is committed, recording a lawful transfer outcome such as delivered, acknowledged, rejected, expired, or unavailable as defined by profile.

### Failure condition

The transfer operation or its constitutional record cannot be completed under the applicable handoff rules.

### Normative dependencies

Contracts 000–014.

### Downstream consumers

IBOS, SACS, or another authorized downstream governance/runtime boundary; audit and provenance systems.

### Expected implementation responsibility

Transport-neutral handoff adapter, integrity verification, acknowledgment handling, transfer event recording, and retry/idempotency logic.

---

# 11.1 Upstream Reference Harmonization Requirement

Before Contract 009 advances beyond architectural drafting, Contracts 003–008 SHALL undergo a controlled downstream-reference harmonization pass. The pass SHALL update stale references inherited from earlier contract-plan versions without changing the substantive authority of those contracts.

The harmonized downstream allocation SHALL be:

```text
Contract 009 — semantic comparison, conflict classification, reconciliation disposition, and downstream standing
Contract 010 — meaning-preserving semantic normalization
Contract 011 — deterministic canonical ordering
Contract 012 — structural completeness and construction eligibility
Contract 013 — mechanical canonical request construction
Contract 014 — identity and issuance
Contract 015 — downstream handoff
```

Contracts 003–008 SHALL continue to preserve represented conflicts, ambiguity, incompleteness, evidence limitations, and provenance conflicts that they lack authority to resolve. They SHALL NOT retroactively absorb Contract 009 reconciliation authority or Contract 012 structural eligibility authority.

---

# 12. Artifact Progression

The principal artifact progression is:

```text
SourceIntakeRecord
        |
        v
AdmittedInterpretationProposalSet
        |
        +--> DeclaredObjectiveSet
        +--> DeclaredConstraintSet
        +--> CapabilityRequirementSet
        +--> SemanticClarificationSet
        +--> InterpretationEvidenceSet
        +--> ProvenanceRecordSet
                    |
                    v
        SemanticReconciliationSet
                    |
                    v
        NormalizedRequestRepresentation
                    |
                    v
        CanonicallyOrderedRequestRepresentation
                    |
                    v
        StructuralValidationResult
                    |
                    v
        ConstructedCanonicalRequest
                    |
                    v
        CanonicalStructuredRequest
                    |
                    v
        CanonicalRequestHandoffRecord
```

Supporting decisions and failure artifacts remain linked to the principal publications and shall not be treated as replacements for them.

---

# 13. Dependency Model

## 13.1 Normative dependency

A contract may depend only upon Contract 000 and contracts with lower numbers, except where a separately adopted architecture document is explicitly named.

## 13.2 Artifact dependency

Downstream artifacts may reference upstream artifacts by identity and immutable digest. They may not silently embed altered copies as if they were the upstream publication.

## 13.3 Registry dependency

Every registry-dependent result shall record the applicable registry identity and version. Registry updates shall not retroactively alter prior publications.

## 13.4 Profile dependency

Profiles may narrow or configure authority granted by a contract. A profile may not expand constitutional authority or contradict Contract 000.

---

# 14. Registry Strategy

The contract family is expected to require bounded registries for at least:

* source and content classes;
* interpreter and proposal statuses;
* objective, constraint, capability, clarification, evidence, and provenance relationship types;
* reconciliation decisions, conflict classes, and equivalence bases;
* normalized identifiers, enums, units, temporal forms, and locale rules;
* canonical ordering keys and tie-break rules;
* structural validation rules and construction eligibility dispositions;
* identity algorithms, canonical serialization versions, and issuance statuses;
* handoff statuses, acknowledgment classes, and transfer failure reasons.

A registry defines permitted values or deterministic rules. It does not grant authority beyond the governing contract.

---

# 15. Prototype Feedback Gate

Before the entire contract family is canonized, a bounded prototype may be used to test whether the planned artifacts and transformations are implementable without collapsing authority distinctions.

The prototype may test:

* artifact shape and serialization practicality;
* determinism and idempotence;
* identity derivation;
* registry usability;
* replay and provenance continuity;
* error and valid non-success distinctions;
* implementation mapping across the proposed Rust package family.

Prototype findings may refine logical artifact boundaries, field shapes, and implementation conveniences. They may not silently change constitutional authority. Any authority-level change requires a formal plan or contract amendment.

---

# 16. Drafting and Review Order

The normative drafting order is fixed as follows:

1. Maintain Contract 000 as the controlling constitutional dependency.
2. Complete and harmonize Contracts 001–008 against ARCH-001 and ARCH-002; use provisional ARCH-003 only as non-binding architectural evidence until formally adopted.
3. Draft Contract 009 and review the transition from representation to processing.
4. Draft Contracts 010–011 and verify deterministic semantic processing.
5. Draft Contracts 012–013 and verify that validation does not repair and construction does not decide.
6. Draft Contracts 014–015 and verify that issuance does not authorize and handoff does not execute.
7. Produce a complete normative dependency map and cross-contract artifact matrix.
8. Run family-level Candidate Review.
9. Canonize the contract set only after all open authority-boundary findings are closed.
10. Produce the implementation plan and Codex handoff from the canonized set.

---

# 17. Completion Criteria

The contract program is ready for implementation planning only when:

* every contract has an adopted governing question, subject, act, authority, and principal publication;
* every governing question is unique, singular, and answerable by its contract's principal publication;
* all cross-contract terms and identities are harmonized;
* every principal artifact has a defined lifecycle and failure model;
* valid unresolved states are distinguished from failure;
* every authority prohibition is testable;
* the normative dependency map is complete and acyclic;
* registry and profile ownership are assigned;
* deterministic replay requirements are explicit;
* identity, issuance, and handoff do not imply authorization;
* the complete Contract 000–015 family passes Candidate Review;
* no implementation-critical architectural meaning remains undeclared.

---

# 18. Implementation Planning Guidance

The normative contract family does not mandate one-to-one implementation decomposition. A conforming Rust implementation may retain the planned package family:

```text
structured-request-contracts
structured-request-engine
structured-request-seam
```

Within that package family, implementation modules may group related authorities, provided that:

* inputs and outputs remain inspectable;
* authority boundaries remain explicit;
* upstream artifacts remain immutable;
* each principal publication can be independently tested;
* failure and valid non-success states remain distinguishable;
* registry and profile versions are recorded;
* no module silently performs an act governed by another contract.

The architecture shall drive the implementation. Existing code organization shall not redefine constitutional boundaries.

---

# 19. Adoption Statement

`SRE-CONTRACT-PLAN v2.1.0` supersedes `SRE-CONTRACT-PLAN v2.0.0` and adopts Contracts 000–015 as the complete planned normative family for the Structured Request Engine.

The expanded inventory is warranted because it constitutionalizes lifecycle responsibilities already present in the original plan and assigns each responsibility one governing question, one subject, one act, one bounded authority, and one principal publication.

Future additions require a formal amendment. Normative drafting shall proceed within this adopted inventory.
