# Structured Request Engine Project Profile

## Project Name

Structured Request Engine

## Repository Identity

The repository is a sparse, contract-first specification workspace for a bounded request-interpretation and canonicalization system.
The visible evidence spans constitutional authority, the Contract 000-015 draft family, architectural drafts, implementation plans, and the Project Suite guidance layer.

## Architectural Purpose

Represent submitted source material as a provenance-preserving canonical request without granting authority to authorize, execute, plan, or generate.

## System Position

Structured Request Engine sits before downstream governance and runtime systems.
It produces a canonical request artifact for later evaluation by authorized consumers.

## Constitutional Authority

- Contract 000 is the constitutional contract.
- It defines identity, authority boundary, prohibitions, and the one-authority-per-transformation principle.

## Current Contract Range

- Contract 000: Architecture, Identity, and Authority
- Contract 001: Source Submission and Intake
- Contract 002: Interpretation Boundary
- Contract 003: Objectives and Objective Relationships
- Contract 004: Declared Constraints
- Contract 005: Capability Requirements
- Contract 006: Meaning Qualification Representation
- Contract 007: Evidence Representation
- Contract 008: Provenance Representation
- Contract 009: Semantic Reconciliation
- Contract 010: Semantic Normalization
- Contract 011: Canonical Ordering
- Contract 012: Structural Validation
- Contract 013: Canonical Request Construction
- Contract 014: Canonical Request Identity and Issuance
- Contract 015: Canonical Request Handoff

## Current Contract Statuses

- Contract 000: Candidate, constitutional review passed, implementation mapping pending
- Contract 001: Draft, constitutional development
- Contracts 002-012: Draft subordinate contracts; no canonization identified
- Contracts 013-015: v0.1.0 drafts with architectural status marked Frozen; no canonization identified

## Architecture and Planning Status

- SRE-ARCH-001: Draft; architectural development
- SRE-ARCH-002: Candidate Draft
- SRE-ARCH-003: Provisional Draft; non-binding architectural evidence
- SRE-CONTRACT-PLAN v2.1.0: Adopted planning baseline; planning authority only
- SRE-IMP-001 through SRE-IMP-003: Draft implementation plans; not implementation evidence

## Intended Canonical Input

Source submission material, referenced artifacts, declared metadata, and any explicitly admitted interpretation proposals permitted by later contracts.

## Intended Canonical Output

`CanonicalStructuredRequest` or, when construction fails, a `StructuredRequestFailureRecord`.

## Interpretation Boundary

Interpretation is a proposal boundary, not authority.
The engine may admit and preserve source material, but it may not turn interpretation into authorization.

## Prohibited Authority

- authorization
- policy decision-making
- provider selection
- execution planning
- tool invocation
- answer generation
- release authority

## Downstream Consumers

Downstream governance or runtime systems may consume the canonical request.
The repository evidence mentions IBOS, SACS, and Generation-Stack as downstream architecture references, but no concrete integration is implemented here.

## Current Maturity

Early-stage and pre-implementation.

## Implementation Status

No runtime implementation identified.

## Validation Status

No conformance suite or runtime validation framework identified.

## Active Work

- finish the contract set
- review constitutional boundaries
- define subordinate contract ownership
- keep canonicalization separate from authorization
- perform specification review across Contracts 001-015
- resolve adoption and canonization decisions explicitly

## Deferred Work

- implementation
- tests
- conformance evidence
- downstream integrations

## Known Terminology Risks

The repository uses overlapping terms such as canonical, accepted, candidate, draft, reviewed, and constitutional.
Those terms must not be treated as interchangeable.
