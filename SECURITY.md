# Security Policy

## Project boundary

The Structured Request Engine (SRE) represents, validates, constructs, issues, and hands off canonical request artifacts under bounded authority. It does not authorize requests, select providers or tools, manage credentials, execute tasks, generate responses, or release outputs.

SRE is currently a version `0.1.0` fixture-bounded, in-memory reference implementation. It does not establish production deployment, persistence, registry, provider, or operational guarantees.

## Supported versions

No stable production release or formal version-support matrix has been established. Security review currently applies to the latest maintained state of the repository, including the current `main` branch and the published `0.1.0` package metadata. A future release policy may define narrower support periods or branches.

## Reporting a vulnerability

Please report suspected vulnerabilities privately by email to **tyrone@ulantra.com**. Do not disclose suspected vulnerabilities through public GitHub issues, discussions, pull requests, social media, or other public channels.

Where available, include:

- a clear description of the issue;
- affected files, functions, artifacts, contracts, or lifecycle stages;
- reproduction steps or a minimal proof of concept;
- observed and expected behavior;
- security impact and likely affected boundaries;
- relevant logs or test output;
- relevant identities, manifests, profiles, registries, schemas, rule sets, configuration versions, or replay context;
- suggested remediation, if known; and
- whether the issue has been disclosed elsewhere.

Please minimize sensitive information in the report and do not send credentials or unrelated personal data unless necessary to explain the issue.

## SRE security concerns

Reports are especially relevant when they indicate:

- source-admission bypass or silent source removal;
- undeclared or ambient input influencing a governed outcome;
- collapse of an authority boundary or an interpretation proposal acquiring unauthorized standing;
- identity collision, substitution, cross-artifact identity collapse, or integrity-binding failure;
- replay nondeterminism, mutation of committed artifacts, partial-success publication, or success/failure ambiguity;
- corruption of evidence, grounding, provenance, or lineage;
- substitution of a profile, registry, schema, rule set, or configuration;
- canonical-ordering manipulation or structural-validation bypass;
- construction, issuance, or handoff binding errors;
- forged or misattributed receipt acknowledgments;
- duplicate, retry, expiration, custody, or responsibility-accounting errors; or
- accidental creation of authorization, provider, tool, credential, execution, generation, or release authority.

Reports about downstream authorization, execution systems, providers, tools, credentials, or infrastructure outside this repository may be outside SRE's security scope unless the defect originates in an SRE boundary or artifact.

## Coordinated disclosure

Reports will be handled through a reasonable coordinated process:

1. receipt and initial triage;
2. reproduction and scope assessment;
3. remediation development;
4. regression and conformance testing within the affected declared scope;
5. release or patch preparation; and
6. coordinated disclosure after remediation where appropriate.

No response-time or remediation-time SLA is promised. The process and disclosure timing may depend on reproducibility, impact, affected artifacts, required authority decisions, and the availability of a suitable remediation.

## Safe harbor

Good-faith security research is welcomed when it is conducted responsibly. Researchers must:

- avoid privacy violations, data destruction, service degradation, credential misuse, and unauthorized access;
- use the minimum testing necessary to establish the issue;
- stop testing and report privately if sensitive information is encountered; and
- allow reasonable time for remediation before public disclosure.

This safe harbor does not authorize access to systems, data, providers, tools, credentials, or infrastructure outside the researcher's lawful authority.
