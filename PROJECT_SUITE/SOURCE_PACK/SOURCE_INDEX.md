# Structured Request Engine Source Index

This index is the evidence map for the current sparse repository.

## Recommended Reading Order

1. `SRE-CONTRACT 000.txt`
2. `SRE-CONTRACT-PLAN_v2.1.0_Adopted_Planning_Baseline.md`
3. `SRE-ARCH-001_Canonical_Representation_Pattern_v0.1.0_Draft.md`
4. `SRE-CONTRACT-001 Sec 001 To Section 004.txt`
5. `SRE-CONTRACT-001 Sec 005 Canonical Inputs.txt`
6. `SRE-CONTRACT-001 Sec 006 Source Admission Lifestyle.txt`
7. `SRE-CONTRACT-002_Interpretation_Boundary_v0.1.0_Draft.md` through `SRE-CONTRACT-015_Canonical_Request_Handoff_v0.1.0_Draft.md`
8. `SRE-IMP-001 Implementation Plan.txt` through `SRE-IMP-003 Implementation Plan.txt`
9. `PROJECT_SUITE/STATUS_CURRENT.md`
10. `PROJECT_SUITE/STATUS_LOG/`

## Normative and Planning Sources

| Path | Role | Authority / Status | Dependencies | Notes |
| --- | --- | --- | --- | --- |
| `SRE-CONTRACT 000.txt` | constitutional contract | highest authority; Candidate; constitutional review passed; implementation mapping pending | planning baseline | constitutional identity, authority boundary, and prohibitions |
| `SRE-CONTRACT-PLAN_v2.1.0_Adopted_Planning_Baseline.md` | contract plan and dependency map | adopted planning baseline; planning authority only | Contract 000 | harmonized Contract 000-015 inventory; not a contract |
| `SRE-ARCH-001_Canonical_Representation_Pattern_v0.1.0_Draft.md` | architectural pattern | Draft; architectural development | Contract 000 and planning baseline | canonical representation pattern |
| `SRE-ARCH-002-Constitutional-Grounding-Architecture.txt` | architectural grounding | Candidate Draft | Contract 000 | constitutional grounding architecture |
| `SRE-ARCH-003_Constitutional_Lifecycle_Pattern_v0.1.0_Provisional_Draft.md` | lifecycle architecture | Provisional Draft; non-binding | Contract 000 and planning baseline | architectural evidence only until formally adopted |
| `SRE-CONTRACT-001 Sec 001 To Section 004.txt` | source submission and intake | Draft; constitutional development | Contract 000 | Contract 001 sections 001-004 |
| `SRE-CONTRACT-001 Sec 005 Canonical Inputs.txt` | canonical inputs | Draft section | Contract 000 and prior Contract 001 sections | Contract 001 section 005 |
| `SRE-CONTRACT-001 Sec 006 Source Admission Lifestyle.txt` | source admission lifecycle | Draft section | Contract 000 and prior Contract 001 sections | Contract 001 section 006; filename wording retained |
| `SRE-CONTRACT-002_Interpretation_Boundary_v0.1.0_Draft.md` | interpretation boundary | Draft; constitutional development | Contract 000 and Contract 001 | interpretation remains a proposal boundary |
| `SRE-CONTRACT-003_Objectives_and_Objective_Relationships_v0.1.0_Draft.md` | objectives and relationships | Draft; constitutional development | prior contract family | objective representation |
| `SRE-CONTRACT-004_Declared_Constraints_v0.1.0_Draft.md` | declared constraints | Draft; constitutional development | prior contract family | constraint representation |
| `SRE-CONTRACT-005_Capability_Requirements_v0.1.0_Draft.md` | capability requirements | Draft; constitutional development | prior contract family | capability requirement representation |
| `SRE-CONTRACT-006_Meaning_Qualification_Representation_v0.1.0_Draft.md` | meaning qualification | Draft; constitutional development | prior contract family | qualified meaning representation |
| `SRE-CONTRACT-007_Evidence_Representation_v0.1.0_Draft.md` | evidence representation | Draft; constitutional development | prior contract family | evidence representation |
| `SRE-CONTRACT-008_Provenance_Representation_v0.1.0_Draft.md` | provenance representation | Draft; constitutional development | prior contract family | provenance representation |
| `SRE-CONTRACT-009_Semantic_Reconciliation_v0.1.0_Draft.md` | semantic reconciliation | Draft; constitutional development | prior contract family | reconciliation boundary |
| `SRE-CONTRACT-010_Semantic_Normalization_v0.1.0_Draft.md` | semantic normalization | Draft; status not separately stated | Contract 009 and prior family | filename and document version identify draft material |
| `SRE-CONTRACT-011_Canonical_Ordering_v0.1.0_Draft.md` | canonical ordering | Draft; status not separately stated | Contract 010 and prior family | filename and document version identify draft material |
| `SRE-CONTRACT-012_Structural_Validation_v0.1.0_Draft.md` | structural validation | Draft; constitutional development | prior contract family | structural eligibility boundary |
| `SRE-CONTRACT-013_Canonical_Request_Construction_v0.1.0_Draft.md` | canonical request construction | v0.1.0 Draft; architectural status Frozen | Contracts 011-012 | mechanical construction boundary |
| `SRE-CONTRACT-014_Canonical_Request_Identity_and_Issuance_v0.1.0_Draft.md` | request identity and issuance | v0.1.0 Draft; architectural status Frozen | Contract 013 | identity and issuance boundary |
| `SRE-CONTRACT-015_Canonical_Request_Handoff_v0.1.0_Draft.md` | canonical request handoff | v0.1.0 Draft; architectural status Frozen | Contract 014 | handoff boundary |

## Implementation Planning Sources

| Path | Role | Status | Notes |
| --- | --- | --- | --- |
| `SRE-IMP-001 Implementation Plan.txt` | implementation plan | Draft; implementation planning | implementation scope, dependency model, and evidence expectations |
| `SRE-IMP-002 Implementation Plan.txt` | implementation objectives | Draft; implementation planning | constitutional fidelity and deterministic realization objectives |
| `SRE-IMP-003 Implementation Plan.txt` | repository architecture plan | Draft; implementation planning | proposed implementation organization; not implementation evidence |

## Project Suite Operational Documents

| Path | Role | Status | Notes |
| --- | --- | --- | --- |
| `PROJECT_SUITE/README.md` | suite boundary and commands | current | derivative operational guidance |
| `PROJECT_SUITE/PROJECT_PROFILE.md` | project identity and layer map | current | requires future alignment with the 000-015 family |
| `PROJECT_SUITE/PROJECT_RULES.md` | local maintenance rules | current | preserves sparse-repository discipline |
| `PROJECT_SUITE/SOURCE_INDEX.md` | bounded evidence map | current | navigation entrypoint |
| `PROJECT_SUITE/STATUS_CURRENT.md` | live status snapshot | current | snapshot only, not history |
| `PROJECT_SUITE/STATUS_LOG/` | append-only history | current | historical review and status records |

## Source Pack Contents

The repository Source Pack is a bounded, derivative source set of 33 canonical and directly supporting onboarding files under `PROJECT_SUITE/SOURCE_PACK/`: Contract 000, the three Contract 001 section files, Contracts 002-015, the three architecture specifications, the adopted plan, current status, implementation control records, traceability/decision records, and selected implementation evidence. Consolidated summaries are reserved for Portfolio and are not Source Pack contents. The pack is one-way from canonical repository source and remains non-authoritative.

## Repository Shape Notes

- A bounded Rust runtime is present in `src/lib.rs` with fixture-profile verification evidence for Contracts 000-015.
- Contract 015 evidence records 35 focused tests and 150 passing workspace unit tests; this does not establish canonization or production conformance.
- Bounded Contracts 000-015 implementation evidence is present; no canonization, acceptance, or production implementation authorization is implied.
- No Git repository identified at the project root.
- Public-release metadata and review files are present, but the initial Git history, remote accessibility, and security-reporting configuration still require human review.
- Source Pack contents are derivative and non-authoritative.

## Unresolved Classification Notes

- Draft, Frozen, Provisional, Candidate, and Adopted Planning statuses are not interchangeable.
- Contract 013-015 architectural Frozen status does not establish canonization or adoption; implementation evidence is recorded separately.
- The exact adoption/canonization status of Contracts 001-015 remains unresolved.
