# Structured Request Engine Project Rules

## General Rules

- Inspect before editing.
- Preserve repository-local instructions.
- Do not modify unrelated files.
- Do not invent missing contracts.
- Do not invent implementation.
- Do not upgrade drafts to canonical status.
- Do not use planning documents as completed specifications.
- Keep constitutional, normative, planning, review, and implementation layers separate.

## Contract Rules

- Contract 000 or its equivalent is constitutional.
- Subordinate contracts must not contradict constitutional authority.
- Each contract should own one bounded transformation.
- Each contract must define input, output, authority, failures, and invariants.
- Contract numbering does not imply completion.
- Canonization must be explicit.

## Interpretation Rules

- Preserve caller-stated meaning.
- Do not invent objectives.
- Do not weaken declared constraints.
- Do not add capability requirements without evidence.
- Keep assumptions explicit.
- Preserve unresolved ambiguity.
- Do not turn interpretation into execution authority.

## Artifact Rules

- Every canonical artifact must have one owning transformation.
- Artifacts must preserve provenance.
- Derived fields must remain distinguishable from submitted fields.
- Commitment must be explicit.
- Failure artifacts must not be confused with successful outputs.
- Serialization must not redefine semantics.
- Identifiers must not grant authority.

## Sparse-Repository Rules

- Missing implementation must be reported as missing.
- Proposed modules must not be treated as existing modules.
- Planned tests must not be counted as tests.
- Example schemas must not be called canonical schemas.
- Architectural discussions must not be called adopted contracts.
- Unknowns must remain unknown.

## Integration Rules

- Downstream systems consume the engine output.
- Downstream systems do not redefine the request.
- The engine does not inherit downstream authority.
- Provider-specific behavior stays outside the core contracts unless explicitly adopted.
- Mentioned downstream systems do not become mandatory integrations without evidence.

## Status Rules

- Drafted is not reviewed.
- Reviewed is not accepted.
- Accepted is not canonized unless the repository defines them as equivalent.
- Canonized is not implemented.
- Implemented is not validated.
- Validated is not integrated.
- Integrated is not production-ready.
- Deferred work must remain deferred.

## Project Source Scope

For Structured Request Engine, the repository Source Pack may contain up to 40 derivative onboarding files. The current bounded source set contains 33 files: the 18 canonical Contract 000-015 source files plus selected architecture, planning, status, implementation-control, traceability/decision, and implementation-evidence sources.

Consolidated `Structured Request Engine__*.md` summaries are Portfolio surfaces and do not belong in the repository Source Pack. Do not copy implementation code, architecture specifications, planning records, review records, status logs, or generated outputs into the Source Pack unless separately authorized.
