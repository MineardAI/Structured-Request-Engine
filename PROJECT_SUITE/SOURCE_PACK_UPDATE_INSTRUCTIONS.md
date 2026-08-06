# Source Pack Update Instructions

## Repository correction

The Source Pack may contain up to 40 files. Consolidated summaries are Portfolio surfaces, not repository Source Pack contents. The current SRE Source Pack is the bounded canonical Contract 000-015 source set.

## Operational Command

`Run source pack update.`

## Source Pack Model

The repository correction above supersedes the older 3-5-file summary guidance: the permitted maximum is 40 files, and the current source set is 33 bounded contract, architecture, planning, status, implementation-control, traceability/decision, and selected evidence files.

- The Source Pack may contain up to **40 files**; the current SRE source set contains the 18 Contract 000-015 source files.
- It is a lightweight AI onboarding and orientation layer.
- It must not reproduce the repository’s canonical document hierarchy.
- The selected canonical Contract 000-015 source files may be copied into this derivative pack; architecture files, review evidence, implementation plans, code, archives, logs, and generated outputs remain referenced rather than copied.
- Full canonical documents may be included only when explicitly required by a project-specific exception recorded in `PROJECT_SUITE/PROJECT_RULES.md`.
- Synchronization means copying or refreshing the selected bounded source set from canonical source, not creating consolidated summaries. Consolidated summaries belong to Portfolio.

## Synchronization Direction

```text
Canonical Repository Source
        ↓
Source Pack Summary and Index
```

Source Pack content never becomes canonical merely because it is copied or summarized there.

## Update Process

1. Inspect repository state and current canonical sources.
2. Identify new or changed authoritative sources and their current controlling versions.
3. Identify status, dependency, artifact, review, and implementation-plan changes.
4. Identify superseded, duplicate, temporary, generated, and routine historical material to exclude.
5. Update the bounded Source Pack source set and canonical-source map.
6. Preserve exact normative meaning without reproducing complete canonical documents.
7. Identify unresolved conflicts and status ambiguities.
8. Validate that mapped paths resolve and no canonical source was altered.
9. Report all Source Pack changes.

## Prohibitions

- Do not exceed the 40-file limit.
- Do not canonize drafts.
- Do not rewrite contracts, specifications, architecture, planning, or review records.
- Do not infer acceptance, closure, adoption, canonization, or implementation readiness.
- Do not invent missing artifacts or fill missing sections.
- Do not create implementation.
- Do not create a repository mirror, contract archive, review repository, package manifest, checksum inventory, packaging log, or category subdirectory as part of the default Source Pack model.
