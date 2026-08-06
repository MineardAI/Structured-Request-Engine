# Structured Request Engine

## Contract 002 — Interpretation Boundary

**Document ID:** `SRE-CONTRACT-002`  
**Version:** `v0.1.0`  
**Contract-set version:** `v0.1.0`  
**Status:** Draft — Constitutional Development  
**Project:** Structured-Request-Engine  
**Normative dependencies:**

- `SRE-CONTRACT-000 v0.1.0`
- `SRE-CONTRACT-001 v0.1.0`

---

## Normative requirement identifiers

Every normative `SHALL`, `SHALL NOT`, `SHOULD`, `SHOULD NOT`, and `MAY` statement in this contract shall possess a stable requirement identifier.

Requirement identifiers use:

```text
SRE-002-<DOMAIN>-<SEQUENCE>
```

Examples:

```text
SRE-002-REQUEST-001
SRE-002-PROPOSAL-004
SRE-002-ADMISSION-006
```

Requirement identifiers exist solely for:

- traceability;
- implementation verification;
- conformance testing;
- amendment tracking;
- cross-contract dependency analysis.

Requirement identifiers SHALL NOT alter the normative meaning of any requirement.

---

# 1. Purpose

**Specification level:** Constitutional concept

## SRE-002-PURPOSE-001

This contract establishes the constitutional boundary through which admitted source material may be submitted for interpretation and through which interpretation outputs may become eligible inputs to deterministic request construction.

This contract defines:

- interpretation request issuance;
- interpretation operation identity;
- interpreter constitutional status;
- interpretation proposal construction requirements;
- proposal submission;
- deterministic proposal admission;
- structural admission criteria;
- proposal and admission-decision identity;
- proposal immutability;
- admission-decision immutability;
- evidence-profile compliance;
- admission outcomes;
- failure outcomes;
- downstream eligibility;
- deferred responsibilities.

This contract does not define:

- objective correctness;
- constraint correctness;
- capability correctness;
- semantic preference;
- proposal reconciliation;
- representation normalization;
- canonical ordering;
- structural completeness of the final request;
- canonical request construction;
- authorization;
- planning;
- execution;
- generation;
- release.

## SRE-002-PURPOSE-002

The constitutional purpose of Contract 002 is to ensure that proposed meaning enters the Structured Request Engine only through an explicit, deterministic, replayable, non-authoritative admission boundary.

## SRE-002-PURPOSE-003

The organizing doctrine of this contract is:

> **Interpretation proposals are constitutional inputs to deterministic request construction, never constitutional authority over it.**

---

# 2. Architectural identity

**Specification level:** Constitutional concept

## SRE-002-IDENTITY-001

Contract 002 establishes the Interpretation Boundary of the Structured Request Engine.

The Interpretation Boundary SHALL govern two distinct constitutional authorities:

1. **Interpretation Request Authority** — constructs the bounded request presented for interpretation.
2. **Proposal Admission Authority** — determines whether a returned interpretation proposal is structurally eligible for later deterministic evaluation.

These authorities SHALL remain distinct even when implemented in the same process, crate, module, service, or runtime.

## SRE-002-IDENTITY-002

The constitutional sequence governed by this contract is:

```text
SourceIntakeRecord
        │
        ▼
InterpretationRequest
        │
        ▼
Constitutionally External Interpreter
        │
        ▼
InterpretationProposal
        │
        ▼
Proposal Admission
        │
        ├── admitted ──► eligible for later deterministic evaluation
        │
        └── not admitted ──► explicit admission decision or boundary failure
```

## SRE-002-IDENTITY-003

Contract 002 SHALL govern proposal eligibility only.

It SHALL NOT govern semantic correctness, semantic preference, canonical interpretation, or canonical request state.

---

# 3. Organizing constitutional doctrines

**Specification level:** Constitutional concept

## 3.1 Interpretation is proposal

### SRE-002-DOCTRINE-001

Every interpretation output SHALL remain a proposal unless and until a later contract performs an explicitly authorized deterministic transformation.

No interpreter output SHALL become authoritative merely because:

- only one proposal exists;
- the interpreter is highly trusted;
- the interpreter is deterministic;
- the interpreter is a language model;
- the interpreter is human-authored;
- the interpreter runs inside the same process;
- the proposal is structurally valid;
- the proposal agrees with source text;
- the proposal contains evidence references.

## 3.2 Constitutional state-entry rule

### SRE-002-DOCTRINE-002

No interpreter output may alter Structured Request Engine constitutional state except through deterministic proposal admission.

## 3.3 Work boundary, not meaning

### SRE-002-DOCTRINE-003

An `InterpretationRequest` SHALL define the permitted work boundary.

It SHALL NOT prescribe the meaning that an interpreter must produce.

## 3.4 Admission is eligibility

### SRE-002-DOCTRINE-004

Proposal admission SHALL establish only structural eligibility for later deterministic evaluation.

Admission SHALL NOT establish:

- interpretive correctness;
- semantic acceptance;
- semantic preference;
- source truth;
- canonical meaning;
- canonical request state;
- downstream authorization.

## 3.5 No canonical state

### SRE-002-DOCTRINE-005

Contract 002 SHALL NOT establish canonical interpretation state.

## 3.6 Authority non-expansion

### SRE-002-DOCTRINE-006

Neither proposal generation nor proposal admission SHALL create:

- authorization;
- canonical request identity;
- generation authority;
- provider authority;
- tool authority;
- execution authority;
- release authority.

---

# 4. Constitutional position

**Specification level:** Constitutional concept

## SRE-002-POSITION-001

Contract 002 SHALL operate after successful completion of Contract 001 and before semantic artifact construction and deterministic proposal processing.

```text
Contract 001
Source Admission
        │
        ▼
SourceIntakeRecord
        │
        ▼
Contract 002
Interpretation Request and Proposal Admission
        │
        ▼
Contracts 003–008
Semantic Artifact Domains and Grounding
        │
        ▼
Contract 009
Proposal Reconciliation
        │
        ▼
Contracts 010 onward
Normalization, Ordering, Validation, Construction, Identity, and Handoff
```

## SRE-002-POSITION-002

Successful completion of Contract 002 SHALL authorize only consideration under later Structured Request Engine contracts.

It SHALL NOT authorize canonical request construction or downstream action.

---

# 5. Definition of constitutionally external

**Specification level:** Constitutional concept

## SRE-002-EXTERNAL-001

For this contract, **constitutionally external** means a participant whose outputs require deterministic constitutional admission before they may influence Structured Request Engine state, regardless of deployment topology.

## SRE-002-EXTERNAL-002

An interpreter MAY execute:

- in the same process as the engine;
- in a separate process;
- in a local service;
- in a remote service;
- in a library;
- in a user interface;
- through a human-assisted workflow.

Its deployment location SHALL NOT alter its constitutional status.

## SRE-002-EXTERNAL-003

An interpreter SHALL remain constitutionally external because it possesses no authority to alter engine state directly.

---

# 6. Authority boundary

**Specification level:** Constitutional concept

## SRE-002-AUTH-001

Contract 002 possesses only the authority required to:

1. issue a bounded interpretation request;
2. receive one or more interpretation proposals;
3. validate proposal structure and association;
4. validate compliance with declared evidence requirements;
5. issue an interpretation proposal admission decision;
6. issue an explicit boundary failure when the operation cannot complete;
7. transfer admitted proposals for later deterministic evaluation.

## SRE-002-AUTH-002

Contract 002 SHALL NOT:

- determine whether an interpretation is true;
- choose the best interpretation;
- reconcile competing proposals;
- resolve ambiguity;
- infer missing meaning during admission;
- normalize semantic content;
- create objectives, constraints, or capability requirements on its own authority;
- establish canonical interpretation state;
- construct a `CanonicalStructuredRequest`;
- issue a canonical request identity;
- authorize planning, execution, generation, or release.

## SRE-002-AUTH-003

Every authority granted by this contract SHALL possess an explicit complementary prohibition preventing adjacent authority from being exercised.

---

# 7. Artifact classification doctrine

**Specification level:** Constitutional concept

## SRE-002-MODEL-001

The major artifacts governed by this contract SHALL be classified as follows:

| Item | Classification |
|---|---|
| Interpretation boundary | Constitutional concept |
| Constitutionally external interpreter | Constitutional concept |
| Interpretation operation | Logical artifact and required correlation domain |
| `InterpretationRequest` | Required runtime artifact |
| `InterpretationProposal` | Required runtime artifact |
| `InterpretationProposalAdmissionDecision` | Required runtime constitutional decision artifact |
| `InterpretationBoundaryFailureRecord` | Required runtime failure artifact |
| Proposal production state | Logical artifact semantics |
| Admission disposition | Required decision semantics |
| Administrative relationship | Logical artifact semantics unless elevated by later lifecycle specifications |
| Internal transport envelope | Implementation convenience unless externally observable |
| Internal invocation state | Implementation convenience unless required for replay or conformance |

## SRE-002-MODEL-002

The existence of a named logical artifact SHALL NOT require a dedicated Rust type, module, file, or top-level serialized object unless later implementation specifications make that representation necessary for interoperability, determinism, replay, or conformance.

---

# 8. Canonical inputs

**Specification level:** Required observable behavior

## SRE-002-INPUT-001

The Interpretation Request Authority MAY admit only declared constitutional inputs.

Supported input classes SHALL include at least:

1. one committed `SourceIntakeRecord`;
2. applicable contract versions;
3. applicable interpreter profile references;
4. applicable evidence capability profile references;
5. applicable schema references;
6. declared interpretation scope;
7. declared source scope;
8. declared completion expectations;
9. declared bounds and exclusions;
10. configuration snapshot references when they materially affect the request.

## SRE-002-INPUT-002

No undeclared environmental context SHALL influence interpretation request construction or proposal admission.

## SRE-002-INPUT-003

Any context that materially affects the interpretation boundary SHALL be represented as a traceable, versioned input.

---

# 9. Interpretation operation identity

**Specification level:** Required observable behavior

## SRE-002-OPERATION-001

Every interpretation operation SHALL possess exactly one immutable `InterpretationOperationId`.

## SRE-002-OPERATION-002

The `InterpretationOperationId` SHALL correlate the complete Contract 002 lifecycle without replacing the identities of its constituent artifacts.

The operation MAY correlate:

```text
InterpretationOperationId
├── InterpretationRequestId
├── InterpreterInvocationId, when applicable
├── InterpretationProposalId
└── ProposalAdmissionId
```

## SRE-002-OPERATION-003

One interpretation operation MAY be associated with multiple interpretation proposals.

## SRE-002-OPERATION-004

Interpretation operation identity SHALL NOT imply semantic equivalence among associated proposals.

---

# 10. Authority A — Interpretation Request Authority

**Specification level:** Required observable behavior

## SRE-002-REQUEST-001

The Interpretation Request Authority SHALL be the exclusive constitutional owner of `InterpretationRequest` construction.

## SRE-002-REQUEST-002

Every issued `InterpretationRequest` SHALL possess exactly one immutable `InterpretationRequestId`.

## SRE-002-REQUEST-003

Every `InterpretationRequest` SHALL reference exactly one `InterpretationOperationId`.

## SRE-002-REQUEST-004

Every `InterpretationRequest` SHALL reference the `SourceIntakeRecord` from which its source scope is derived.

## SRE-002-REQUEST-005

An `InterpretationRequest` SHALL define at least:

- request identity;
- interpretation operation identity;
- source intake association;
- included source identities;
- excluded source identities, when applicable;
- requested interpretation scope;
- applicable contract version;
- applicable schema version;
- interpreter profile requirements;
- evidence capability profile requirements;
- required proposal metadata;
- permitted response forms;
- completion expectations;
- declared bounds;
- declared exclusions.

## SRE-002-REQUEST-006

An `InterpretationRequest` MAY request proposed representations of:

- objectives;
- constraints;
- capability requirements;
- ambiguities;
- assumptions;
- uncertainty;
- relationships;
- evidence associations.

## SRE-002-REQUEST-007

An `InterpretationRequest` SHALL NOT prescribe which objectives, constraints, capabilities, ambiguities, assumptions, or conclusions must be returned.

## SRE-002-REQUEST-008

An `InterpretationRequest` SHALL NOT contain hidden instructions that materially alter interpreter obligations without being represented in the request artifact.

## SRE-002-REQUEST-009

Equivalent request inputs under equivalent versions and profiles SHALL produce equivalent `InterpretationRequest` artifacts.

---

# 11. Interpretation request outcome

**Specification level:** Required observable behavior

## SRE-002-REQUEST-OUTCOME-001

Every completed interpretation request issuance operation SHALL produce exactly one of:

1. one committed `InterpretationRequest`; or
2. one committed `InterpretationBoundaryFailureRecord`.

## SRE-002-REQUEST-OUTCOME-002

The request issuance operation SHALL NOT produce both authoritative outcomes.

## SRE-002-REQUEST-OUTCOME-003

The request issuance operation SHALL NOT complete without one authoritative outcome.

---

# 12. Interpreter constitutional role

**Specification level:** Constitutional concept

## SRE-002-INTERPRETER-001

An interpreter is a constitutionally external participant that transforms an `InterpretationRequest` into zero or more `InterpretationProposal` artifacts.

## SRE-002-INTERPRETER-002

An interpreter MAY be:

- a deterministic parser;
- a rules engine;
- a symbolic system;
- a language model;
- a human-authored adapter;
- a human-assisted process;
- an application-specific extractor;
- another conforming implementation.

## SRE-002-INTERPRETER-003

The interpreter owns only the transformation:

```text
InterpretationRequest
        ↓
InterpretationProposal
```

## SRE-002-INTERPRETER-004

An interpreter SHALL NOT:

- issue a canonical request;
- issue a canonical request identity;
- alter admitted source artifacts;
- alter an interpretation request;
- alter constitutional engine state directly;
- grant execution authority;
- grant generation authority;
- grant provider authority;
- resolve downstream policy;
- redefine contract semantics.

## SRE-002-INTERPRETER-005

An interpreter MAY be nondeterministic.

Determinism under this contract begins from the fixed set of submitted proposals, fixed source artifacts, fixed profiles, fixed registries, fixed versions, and fixed configurations admitted to the Proposal Admission Authority.

---

# 13. Interpretation proposal artifact

**Specification level:** Required runtime artifact

## SRE-002-PROPOSAL-001

Every interpretation output submitted to Contract 002 SHALL be represented as an `InterpretationProposal`.

## SRE-002-PROPOSAL-002

Every `InterpretationProposal` SHALL possess exactly one immutable `InterpretationProposalId`.

## SRE-002-PROPOSAL-003

Every `InterpretationProposal` SHALL reference:

- one `InterpretationOperationId`;
- one `InterpretationRequestId`;
- one interpreter identity;
- one interpreter version or version-equivalent identity;
- one applicable proposal schema version;
- one completion status;
- one source scope declaration;
- one evidence capability profile;
- zero or more proposed request elements;
- zero or more evidence references;
- zero or more declared assumptions;
- zero or more declared uncertainties;
- zero or more declared bounds.

## SRE-002-PROPOSAL-004

A proposal MAY be empty when the applicable interpreter profile permits an explicit no-result outcome.

## SRE-002-PROPOSAL-005

A proposal SHALL NOT claim authority beyond proposing a structured interpretation of the bounded source scope.

## SRE-002-PROPOSAL-006

A proposal SHALL remain distinguishable from:

- admitted source;
- interpretation request;
- admission decision;
- reconciled proposal;
- normalized representation;
- canonical request.

---

# 14. Proposal production state

**Specification level:** Logical artifact semantics

## SRE-002-PRODUCTION-001

Proposal production state SHALL describe interpreter-side completion only.

The initial production state registry SHALL include:

- `Requested`;
- `Returned`;
- `Incomplete`;
- `Failed`.

## SRE-002-PRODUCTION-002

Proposal production state SHALL NOT establish proposal admission disposition.

## SRE-002-PRODUCTION-003

Proposal production state SHALL NOT establish semantic correctness or canonical status.

---

# 15. Proposal submission

**Specification level:** Required observable behavior

## SRE-002-SUBMIT-001

A proposal submission SHALL present one immutable `InterpretationProposal` to the Proposal Admission Authority.

## SRE-002-SUBMIT-002

The submitted proposal SHALL preserve its original interpreter-produced content.

## SRE-002-SUBMIT-003

The Proposal Admission Authority SHALL NOT silently repair, rewrite, complete, normalize, or reinterpret the submitted proposal.

## SRE-002-SUBMIT-004

A materially corrected proposal SHALL be submitted as a new `InterpretationProposal` with a new identity.

---

# 16. Proposal immutability

**Specification level:** Constitutional concept and required observable behavior

## SRE-002-IMMUTABILITY-001

An `InterpretationProposal` SHALL become immutable upon submission to the Proposal Admission Authority.

## SRE-002-IMMUTABILITY-002

Correction, expansion, replacement, or revision SHALL occur through issuance of a new proposal artifact.

## SRE-002-IMMUTABILITY-003

A new proposal MAY reference an earlier proposal as:

- replaced;
- corrected;
- extended;
- superseded;
- related.

## SRE-002-IMMUTABILITY-004

A later proposal SHALL NOT erase or mutate the historical existence of an earlier proposal.

---

# 17. Authority B — Proposal Admission Authority

**Specification level:** Required observable behavior

## SRE-002-ADMISSION-001

The Proposal Admission Authority SHALL be the exclusive constitutional owner of interpretation proposal admission decisions.

## SRE-002-ADMISSION-002

The Proposal Admission Authority SHALL evaluate only structural and constitutional admissibility.

## SRE-002-ADMISSION-003

Proposal admission SHALL NOT evaluate interpretive correctness.

## SRE-002-ADMISSION-004

Proposal admission SHALL NOT evaluate whether one proposal is preferable to another.

## SRE-002-ADMISSION-005

Proposal admission SHALL NOT reconcile, merge, rank, normalize, or canonize proposals.

## SRE-002-ADMISSION-006

Proposal admission MAY evaluate:

- schema validity;
- required field presence;
- proposal identity validity;
- interpretation operation association;
- interpretation request association;
- interpreter identity presence;
- interpreter version presence;
- contract-version compatibility;
- profile compatibility;
- declared source-scope validity;
- evidence-reference existence;
- evidence-profile compliance;
- completion status;
- encoding validity;
- representation validity;
- internal structural coherence;
- duplicate artifact identity;
- submission integrity.

## SRE-002-ADMISSION-007

Proposal admission SHALL NOT evaluate:

- whether an inferred objective is correct;
- whether an inferred constraint is correct;
- whether a capability requirement is needed;
- whether an ambiguity should be resolved;
- whether an assumption is reasonable;
- whether evidence semantically proves a proposal field;
- whether the proposal is safe;
- whether the proposal is permitted;
- whether the proposal is feasible;
- whether the request should proceed.

---

# 18. Structural admission criteria

**Specification level:** Required observable behavior

## SRE-002-CRITERIA-001

A proposal SHALL be eligible for admission only when all mandatory structural criteria for its applicable contract, schema, interpreter, and evidence profiles are satisfied.

## SRE-002-CRITERIA-002

Structural admission criteria SHALL be:

- explicit;
- versioned;
- deterministic;
- reproducible;
- traceable;
- independent of interpreter reputation;
- independent of semantic preference.

## SRE-002-CRITERIA-003

Equivalent proposal inputs evaluated under equivalent versions, profiles, registries, and configurations SHALL produce equivalent admission dispositions.

---

# 19. Evidence capability profiles

**Specification level:** Required observable behavior

## SRE-002-EVIDENCE-001

Every `InterpretationProposal` SHALL declare one applicable evidence capability profile.

## SRE-002-EVIDENCE-002

An evidence capability profile SHALL define which evidence forms the interpreter can and must provide.

Evidence forms MAY include:

- source-span evidence;
- source-level evidence;
- field-to-source associations;
- referenced-artifact evidence;
- application-supplied evidence;
- declared lack of evidence;
- unsupported evidence capability.

## SRE-002-EVIDENCE-003

The Proposal Admission Authority SHALL evaluate whether the proposal complies with its declared evidence capability profile.

## SRE-002-EVIDENCE-004

The Proposal Admission Authority SHALL NOT determine whether the evidence is semantically sufficient to prove the proposed meaning.

## SRE-002-EVIDENCE-005

Evidence status semantics SHALL include at least:

- `EvidenceProvided`;
- `EvidenceNotRequiredByProfile`;
- `EvidenceUnavailable`;
- `EvidenceUnsupportedByInterpreter`;
- `EvidenceReferenceInvalid`.

## SRE-002-EVIDENCE-006

A later contract MAY require stronger evidence for a proposed request element to survive reconciliation, validation, or canonical construction.

---

# 20. Interpretation proposal admission decision

**Specification level:** Required runtime constitutional decision artifact

## SRE-002-DECISION-001

Every completed proposal admission evaluation SHALL produce exactly one committed `InterpretationProposalAdmissionDecision`.

## SRE-002-DECISION-002

Every `InterpretationProposalAdmissionDecision` SHALL possess exactly one immutable `ProposalAdmissionId`.

## SRE-002-DECISION-003

Every admission decision SHALL reference:

- one `InterpretationOperationId`;
- one `InterpretationRequestId`;
- one `InterpretationProposalId`;
- one admission disposition;
- applicable contract version;
- applicable schema version;
- applicable interpreter profile version;
- applicable evidence capability profile version;
- structural findings;
- evidence-reference findings;
- failure reasons, when applicable;
- decision identity material;
- commitment metadata.

## SRE-002-DECISION-004

The admission decision SHALL constitute the authoritative constitutional result of Authority B within the limited domain of structural proposal admission.

## SRE-002-DECISION-005

An admission decision SHALL NOT constitute semantic acceptance or canonical interpretation.

---

# 21. Admission dispositions

**Specification level:** Required decision semantics

## SRE-002-DISPOSITION-001

The initial admission disposition registry SHALL include at least:

- `Admitted`;
- `RejectedMalformed`;
- `RejectedIncomplete`;
- `RejectedIncompatible`;
- `RejectedUnverifiable`;
- `RejectedOutOfScope`;
- `RejectedInvalidAssociation`;
- `RejectedDuplicateIdentity`.

## SRE-002-DISPOSITION-002

An `Admitted` disposition SHALL mean only that the proposal is structurally eligible for later deterministic evaluation.

## SRE-002-DISPOSITION-003

A rejection disposition SHALL describe why structural admission did not occur.

## SRE-002-DISPOSITION-004

A rejection disposition SHALL NOT imply that the underlying source request is invalid, prohibited, false, or semantically incoherent.

---

# 22. Admission-decision immutability

**Specification level:** Constitutional concept and required observable behavior

## SRE-002-DECISION-IMMUTABILITY-001

A committed `InterpretationProposalAdmissionDecision` SHALL be immutable.

## SRE-002-DECISION-IMMUTABILITY-002

An admission decision SHALL NOT be edited, revoked, or erased in place.

## SRE-002-DECISION-IMMUTABILITY-003

Later reevaluation SHALL produce a new admission decision with a new identity.

## SRE-002-DECISION-IMMUTABILITY-004

A later status change MAY alter whether a previously admitted proposal remains eligible for future use, but SHALL NOT alter the historical admission decision.

## SRE-002-DECISION-IMMUTABILITY-005

The distinction SHALL remain:

```text
Historical admission decision
        ≠
Current proposal eligibility
```

---

# 23. Multiple-proposal coexistence

**Specification level:** Constitutional concept and required observable behavior

## SRE-002-MULTI-001

Multiple admitted proposals MAY coexist for one interpretation operation or one interpretation request.

## SRE-002-MULTI-002

Admission of a later proposal SHALL NOT automatically supersede an earlier proposal.

## SRE-002-MULTI-003

Contract 002 SHALL NOT select a winning proposal.

## SRE-002-MULTI-004

Contract 002 SHALL NOT infer precedence from:

- arrival order;
- proposal sequence;
- interpreter identity;
- interpreter reputation;
- proposal length;
- proposal confidence;
- proposal recency.

## SRE-002-MULTI-005

Duplicate treatment, conflict treatment, source precedence, proposal comparison, and reconciliation SHALL be governed by Contract 009.

---

# 24. Administrative relationships

**Specification level:** Logical artifact semantics

## SRE-002-ADMIN-001

Administrative relationship semantics MAY include:

- `Active`;
- `Withdrawn`;
- `Superseded`;
- `Archived`;
- `Expired`.

## SRE-002-ADMIN-002

Administrative relationships SHALL remain distinct from proposal production state and admission disposition.

## SRE-002-ADMIN-003

An administrative relationship SHALL NOT rewrite a proposal or its historical admission decision.

## SRE-002-ADMIN-004

The authority and lifecycle governing administrative relationship changes MAY be defined by later lifecycle, registry, or operations specifications.

---

# 25. Boundary failures

**Specification level:** Required runtime failure artifact

## SRE-002-FAILURE-001

When Contract 002 cannot complete an interpretation-boundary operation, it SHALL produce one `InterpretationBoundaryFailureRecord`.

## SRE-002-FAILURE-002

A boundary failure record SHALL identify at least:

- interpretation operation identity;
- request identity, when available;
- proposal identity, when available;
- failure category;
- observed constitutional facts;
- affected profiles or versions;
- recoverability status;
- commitment metadata.

## SRE-002-FAILURE-003

A boundary failure SHALL indicate only that the Contract 002 operation could not complete under the applicable requirements.

## SRE-002-FAILURE-004

A boundary failure SHALL NOT imply:

- that the source request is invalid;
- that the user is at fault;
- that the proposed meaning is false;
- that downstream authorization would have been denied.

---

# 26. Atomic commitment

**Specification level:** Required observable behavior

## SRE-002-COMMIT-001

Every interpretation request issuance operation SHALL terminate with exactly one committed request or one committed boundary failure.

## SRE-002-COMMIT-002

Every proposal admission evaluation SHALL terminate with exactly one committed `InterpretationProposalAdmissionDecision` or one committed boundary failure when no valid decision can be constructed.

## SRE-002-COMMIT-003

Commitment SHALL be atomic.

## SRE-002-COMMIT-004

Partial authoritative proposal admission SHALL NOT exist.

## SRE-002-COMMIT-005

Diagnostics MAY accompany committed artifacts.

Diagnostics SHALL NOT replace required constitutional artifacts.

---

# 27. Downstream eligibility and handoff

**Specification level:** Required observable behavior

## SRE-002-HANDOFF-001

A proposal receiving an `Admitted` disposition SHALL become eligible for evaluation under later Structured Request Engine contracts.

## SRE-002-HANDOFF-002

Admission SHALL authorize only later deterministic evaluation.

It SHALL NOT authorize:

- semantic acceptance;
- reconciliation outcome;
- canonicalization;
- canonical request construction;
- downstream governance;
- provider invocation;
- tool invocation;
- generation;
- release.

## SRE-002-HANDOFF-003

The downstream handoff SHALL preserve:

- the original immutable proposal;
- the immutable admission decision;
- interpretation operation identity;
- request identity;
- interpreter identity and version;
- applicable profile and contract versions;
- source scope association;
- evidence capability declarations.

---

# 28. Deferred responsibilities

**Specification level:** Constitutional concept

## SRE-002-DEFER-001

Contract 002 SHALL intentionally defer semantic artifact definitions to Contracts 003 through 008.

## SRE-002-DEFER-002

Contract 002 SHALL intentionally defer duplicate treatment, conflicting proposal treatment, proposal comparison, source precedence, and reconciliation to Contract 009.

## SRE-002-DEFER-003

Contract 002 SHALL intentionally defer normalization, canonical ordering, structural validation of the complete request, deterministic identity, serialization, canonical construction, and downstream handoff packaging to later contracts.

## SRE-002-DEFER-004

No deferred responsibility SHALL be partially implemented under Contract 002 authority merely for convenience.

---

# 29. Prohibited shortcuts

**Specification level:** Constitutional concept

## SRE-002-PROHIB-001

The following shortcut SHALL be prohibited:

```text
InterpretationProposal
        ↓
Canonical interpretation state
```

## SRE-002-PROHIB-002

The following shortcut SHALL be prohibited:

```text
InterpretationProposal
        ↓
CanonicalStructuredRequest
```

## SRE-002-PROHIB-003

The following shortcut SHALL be prohibited:

```text
Interpreter confidence
        ↓
Admission authority
```

## SRE-002-PROHIB-004

The following shortcut SHALL be prohibited:

```text
Evidence reference exists
        ↓
Interpretation is correct
```

## SRE-002-PROHIB-005

The following shortcut SHALL be prohibited:

```text
Later proposal
        ↓
Automatic supersession
```

---

# 30. Determinism requirements

**Specification level:** Required observable behavior

## SRE-002-DETERMINISM-001

Interpretation request construction SHALL be deterministic for equivalent admitted inputs, versions, profiles, registries, and configurations.

## SRE-002-DETERMINISM-002

Proposal admission SHALL be deterministic for equivalent proposals, admitted source references, versions, profiles, registries, and configurations.

## SRE-002-DETERMINISM-003

Interpreter generation itself MAY remain nondeterministic.

## SRE-002-DETERMINISM-004

Nondeterminism in proposal generation SHALL NOT weaken deterministic admission obligations.

---

# 31. Conformance requirements

**Specification level:** Conformance requirement

## SRE-002-CONFORM-001

A conforming implementation SHALL preserve the distinction among:

```text
InterpretationRequest
InterpretationProposal
InterpretationProposalAdmissionDecision
CanonicalStructuredRequest
```

## SRE-002-CONFORM-002

A conforming implementation SHALL expose sufficient information to verify:

- request identity;
- proposal identity;
- admission identity;
- interpretation operation identity;
- interpreter identity and version;
- source scope;
- profile versions;
- admission disposition;
- proposal immutability;
- decision immutability;
- deterministic admission behavior.

## SRE-002-CONFORM-003

A conforming implementation SHALL demonstrate that semantic correctness is not evaluated during proposal admission.

## SRE-002-CONFORM-004

A conforming implementation SHALL demonstrate that no interpreter output can directly alter constitutional engine state.

## SRE-002-CONFORM-005

A conforming implementation SHALL demonstrate that Contract 002 cannot create canonical interpretation state or a `CanonicalStructuredRequest`.

---

# 32. Fundamental invariants

## SRE-002-INVARIANT-001 — Interpretation authority invariant

Interpretation proposals are constitutional inputs to deterministic request construction, never constitutional authority over it.

## SRE-002-INVARIANT-002 — State-entry invariant

No interpreter output may alter Structured Request Engine constitutional state except through deterministic proposal admission.

## SRE-002-INVARIANT-003 — Request-boundary invariant

An `InterpretationRequest` defines the permitted work boundary and SHALL NOT prescribe interpretive meaning.

## SRE-002-INVARIANT-004 — Admission invariant

Proposal admission establishes structural eligibility for later evaluation and SHALL NOT establish interpretive correctness, preference, acceptance, or canonical state.

## SRE-002-INVARIANT-005 — Immutability invariant

Submitted interpretation proposals and committed interpretation proposal admission decisions SHALL be immutable.

## SRE-002-INVARIANT-006 — Coexistence invariant

Multiple admitted proposals MAY coexist without automatic precedence, replacement, or supersession.

## SRE-002-INVARIANT-007 — Authority non-expansion invariant

Neither proposal generation nor proposal admission may create authorization, canonical request identity, execution authority, generation authority, provider authority, tool authority, or release authority.

## SRE-002-INVARIANT-008 — Boundary outcome invariant

Every completed Contract 002 operation SHALL terminate in an explicit, committed constitutional artifact.

---

# 33. Constitutional completion condition

**Specification level:** Required observable behavior

## SRE-002-COMPLETE-001

Contract 002 processing for a proposal SHALL be constitutionally complete only when:

- the interpretation operation identity is established;
- the interpretation request is committed;
- the proposal is immutable;
- the proposal admission evaluation is complete;
- exactly one admission decision is committed;
- all applicable versions and profiles are recorded;
- the proposal and decision remain traceably associated with the admitted source scope;
- no semantic acceptance or canonical state has been created.

## SRE-002-COMPLETE-002

Completion SHALL establish only that the interpretation boundary has performed its bounded constitutional responsibilities.

It SHALL establish nothing further.

---

# 34. Closing constitutional statement

Contract 002 governs the boundary between proposed meaning and governed meaning.

It ensures that interpretation may inform deterministic request construction without becoming constitutional authority over that construction.

The Structured Request Engine may receive meaning from many interpreters.

It SHALL trust none of them merely because they spoke.
