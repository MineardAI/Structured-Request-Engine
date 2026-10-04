//! Approved local profile bindings. Legacy Fixture type names remain for API compatibility.
//! No fixture constructors or fixture authority values are used here.
use super::*;

impl FixtureObjectiveRepresentationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: ObjectiveRepresentationProfileId::derive(&[
                "ulantra-literal-request-representation-v1",
            ]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            scope: "Contract 003 objective representation tests".to_owned(),
            allow_empty: true,
            allow_incomplete: true,
            allow_unsupported: true,
            allow_evidence_limited: true,
            allow_decomposition: true,
            allow_cross_proposal_relationships: true,
            allowed_forms: BTreeSet::from([ObjectiveForm::Atomic, ObjectiveForm::Composite]),
            allowed_origins: BTreeSet::from([
                ObjectiveOrigin::ExplicitSource,
                ObjectiveOrigin::InterpreterInference,
                ObjectiveOrigin::ApplicationSupplied,
                ObjectiveOrigin::ReferencedArtifact,
                ObjectiveOrigin::MixedOrigin,
            ]),
            allowed_bases: BTreeSet::from([
                RepresentationBasis::DirectQuotation,
                RepresentationBasis::StructuredExtraction,
                RepresentationBasis::InterpreterSynthesis,
                RepresentationBasis::ApplicationDeclaration,
                RepresentationBasis::ReferencedArtifactDeclaration,
            ]),
            allowed_designations: BTreeSet::from([
                ObjectiveDesignation::Primary,
                ObjectiveDesignation::Secondary,
                ObjectiveDesignation::Subordinate,
                ObjectiveDesignation::Conditional,
                ObjectiveDesignation::Parallel,
                ObjectiveDesignation::Alternative,
            ]),
            allowed_statuses: BTreeSet::from([
                ObjectiveRepresentationStatus::Represented,
                ObjectiveRepresentationStatus::Incomplete,
                ObjectiveRepresentationStatus::Unsupported,
                ObjectiveRepresentationStatus::Conflicting,
                ObjectiveRepresentationStatus::Unresolved,
                ObjectiveRepresentationStatus::EvidenceLimited,
            ]),
        }
    }
}

impl FixtureObjectiveRegistries {
    pub(crate) fn local_governed_text() -> Self {
        fn registry(name: &str, values: &[&str]) -> FixtureObjectiveRegistry {
            FixtureObjectiveRegistry {
                identity: ObjectiveRegistryId::derive(&[
                    name,
                    "ulantra-literal-request-representation-v1",
                ]),
                version: "1.0.0".to_owned(),
                authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
                values: values.iter().map(|value| (*value).to_owned()).collect(),
            }
        }
        Self {
            class_registry: registry("class", &["RequestedOutcome"]),
            form_registry: registry("form", &["Atomic", "Composite"]),
            relationship_registry: registry(
                "relationship",
                &[
                    "Supports",
                    "DependsOn",
                    "ConditionalUpon",
                    "AlternativeTo",
                    "MutuallyExclusiveWith",
                    "ParallelWith",
                    "SequentialTo",
                    "ComponentOf",
                    "ConflictsWith",
                ],
            ),
            designation_registry: registry(
                "designation",
                &[
                    "Primary",
                    "Secondary",
                    "Subordinate",
                    "Conditional",
                    "Parallel",
                    "Alternative",
                ],
            ),
            status_registry: registry(
                "status",
                &[
                    "Represented",
                    "Incomplete",
                    "Unsupported",
                    "Conflicting",
                    "Unresolved",
                    "EvidenceLimited",
                ],
            ),
        }
    }
}

impl FixtureConstraintRepresentationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: ConstraintRepresentationProfileId::derive(&["ulantra-contract-004-local-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            scope: "Contract 004 declared constraint representation tests".to_owned(),
            allow_empty: true,
            allow_inferred: true,
            allow_unresolved_reference: true,
            allow_scope_unresolved: true,
            allow_decomposition: true,
            allow_cross_proposal_relationships: true,
            allow_explicit_conflict: true,
            allowed_forms: BTreeSet::from([
                ConstraintForm::RequiredInclusion,
                ConstraintForm::RequiredExclusion,
                ConstraintForm::MaximumBound,
                ConstraintForm::MinimumBound,
                ConstraintForm::ExactRequirement,
                ConstraintForm::ConditionalRequirement,
                ConstraintForm::Prohibition,
                ConstraintForm::ScopeRestriction,
                ConstraintForm::TemporalRestriction,
                ConstraintForm::FormatRestriction,
                ConstraintForm::ResourceRestriction,
                ConstraintForm::ConfidentialityRestriction,
                ConstraintForm::Composite,
            ]),
            allowed_origins: BTreeSet::from([
                ConstraintOrigin::ExplicitSource,
                ConstraintOrigin::InterpreterInference,
                ConstraintOrigin::ApplicationSupplied,
                ConstraintOrigin::ReferencedArtifact,
            ]),
            allowed_bases: BTreeSet::from([
                ConstraintBasis::DirectQuotation,
                ConstraintBasis::StructuredExtraction,
                ConstraintBasis::InterpreterSynthesis,
                ConstraintBasis::ApplicationDeclaration,
                ConstraintBasis::ReferencedArtifactDeclaration,
            ]),
            allowed_statuses: BTreeSet::from([
                ConstraintRepresentationStatus::Represented,
                ConstraintRepresentationStatus::Incomplete,
                ConstraintRepresentationStatus::Unsupported,
                ConstraintRepresentationStatus::Conflicting,
                ConstraintRepresentationStatus::EvidenceLimited,
                ConstraintRepresentationStatus::Unresolved,
                ConstraintRepresentationStatus::ScopeUnresolved,
            ]),
            allowed_scope_kinds: BTreeSet::from([
                ConstraintScopeKind::RequestWide,
                ConstraintScopeKind::ObjectiveSpecific,
                ConstraintScopeKind::ObjectiveSetSpecific,
                ConstraintScopeKind::ArtifactSpecific,
                ConstraintScopeKind::OutputSpecific,
                ConstraintScopeKind::SourceSpecific,
                ConstraintScopeKind::Conditional,
                ConstraintScopeKind::Unresolved,
            ]),
        }
    }
}

impl FixtureConstraintRegistries {
    pub(crate) fn local_governed_text() -> Self {
        fn registry(name: &str, values: &[&str]) -> FixtureConstraintRegistry {
            FixtureConstraintRegistry {
                identity: ConstraintRegistryId::derive(&[name, "ulantra-contract-004-local-v1"]),
                version: "1.0.0".to_owned(),
                authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
                values: values.iter().map(|value| (*value).to_owned()).collect(),
            }
        }
        Self {
            class_registry: registry(
                "class",
                &[
                    "Content",
                    "Format",
                    "Temporal",
                    "Scope",
                    "Resource",
                    "Confidentiality",
                ],
            ),
            form_registry: registry(
                "form",
                &[
                    "RequiredInclusion",
                    "RequiredExclusion",
                    "MaximumBound",
                    "MinimumBound",
                    "ExactRequirement",
                    "ConditionalRequirement",
                    "Prohibition",
                    "ScopeRestriction",
                    "TemporalRestriction",
                    "FormatRestriction",
                    "ResourceRestriction",
                    "ConfidentialityRestriction",
                    "Composite",
                ],
            ),
            origin_registry: registry(
                "origin",
                &[
                    "ExplicitSource",
                    "InterpreterInference",
                    "ApplicationSupplied",
                    "ReferencedArtifact",
                ],
            ),
            basis_registry: registry(
                "basis",
                &[
                    "DirectQuotation",
                    "StructuredExtraction",
                    "InterpreterSynthesis",
                    "ApplicationDeclaration",
                    "ReferencedArtifactDeclaration",
                ],
            ),
            relationship_registry: registry(
                "relationship",
                &[
                    "SupportsAsDeclared",
                    "DependsOnAsDeclared",
                    "ConditionalUponAsDeclared",
                    "OverridesAsDeclared",
                    "SubordinateToAsDeclared",
                    "AlternativeToAsDeclared",
                    "ConflictsWith",
                    "ComponentOf",
                ],
            ),
            status_registry: registry(
                "status",
                &[
                    "Represented",
                    "Incomplete",
                    "Unsupported",
                    "Conflicting",
                    "EvidenceLimited",
                    "Unresolved",
                    "ScopeUnresolved",
                ],
            ),
            scope_status_registry: registry(
                "scope-status",
                &["Resolved", "Partial", "Conflicting", "Unresolved"],
            ),
            conflict_registry: registry(
                "conflict",
                &["ExplicitConflict", "ClosedPredicateConflict"],
            ),
        }
    }
}

impl FixtureCapabilityRepresentationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: CapabilityRepresentationProfileId::derive(&["ulantra-contract-005-local-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            allow_empty: true,
            allow_inferred: true,
            allow_unresolved: true,
            allow_scope_unresolved: true,
            allow_decomposition: true,
            allow_cross_proposal_relationships: true,
            allowed_forms: BTreeSet::from([CapabilityForm::Atomic, CapabilityForm::Composite]),
            allowed_origins: BTreeSet::from([
                CapabilityOrigin::ExplicitSource,
                CapabilityOrigin::InterpreterInference,
                CapabilityOrigin::ApplicationSupplied,
                CapabilityOrigin::ReferencedArtifact,
            ]),
            allowed_bases: BTreeSet::from([
                CapabilityBasis::DirectQuotation,
                CapabilityBasis::StructuredExtraction,
                CapabilityBasis::InterpreterSynthesis,
                CapabilityBasis::ApplicationDeclaration,
                CapabilityBasis::ReferencedArtifactDeclaration,
            ]),
            allowed_necessities: BTreeSet::from([
                CapabilityNecessity::Required,
                CapabilityNecessity::Optional,
                CapabilityNecessity::Conditional,
                CapabilityNecessity::Supporting,
                CapabilityNecessity::Alternative,
                CapabilityNecessity::Unresolved,
                CapabilityNecessity::Unknown,
            ]),
            allowed_statuses: BTreeSet::from([
                CapabilityRepresentationStatus::Represented,
                CapabilityRepresentationStatus::Incomplete,
                CapabilityRepresentationStatus::Unsupported,
                CapabilityRepresentationStatus::Unresolved,
                CapabilityRepresentationStatus::EvidenceLimited,
                CapabilityRepresentationStatus::ScopeUnresolved,
            ]),
            allowed_scope_kinds: BTreeSet::from([
                CapabilityScopeKind::RequestWide,
                CapabilityScopeKind::ObjectiveSpecific,
                CapabilityScopeKind::ObjectiveSetSpecific,
                CapabilityScopeKind::ConstraintSpecific,
                CapabilityScopeKind::Conditional,
                CapabilityScopeKind::Unresolved,
            ]),
            allowed_classes: BTreeSet::from([
                "LanguageGeneration".to_owned(),
                "LanguageUnderstanding".to_owned(),
                "ExternalInformationRetrieval".to_owned(),
                "StructuredDataProcessing".to_owned(),
                "DocumentIngestion".to_owned(),
                "ImageUnderstanding".to_owned(),
                "AudioProcessing".to_owned(),
                "PersistentStorage".to_owned(),
                "StructuredComputation".to_owned(),
                "HumanInteraction".to_owned(),
                "ExternalCommunication".to_owned(),
                "IdentityVerification".to_owned(),
                "CryptographicOperation".to_owned(),
            ]),
            abstraction_rules: BTreeMap::from([
                (
                    "ulantra-rule-retrieve-v1".to_owned(),
                    "retrieve=ExternalInformationRetrieval".to_owned(),
                ),
                (
                    "ulantra-rule-generate-v1".to_owned(),
                    "generate=LanguageGeneration".to_owned(),
                ),
            ]),
            prohibited_classes: BTreeSet::from([
                "OpenAI".to_owned(),
                "NamedModel".to_owned(),
                "NamedTool".to_owned(),
                "ConcreteDatabase".to_owned(),
            ]),
        }
    }
}

impl FixtureCapabilityRegistries {
    pub(crate) fn local_governed_text() -> Self {
        fn registry(name: &str, values: &[&str]) -> FixtureCapabilityRegistry {
            FixtureCapabilityRegistry {
                identity: CapabilityRegistryId::derive(&[name, "ulantra-contract-005-local-v1"]),
                version: "1.0.0".to_owned(),
                authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
                values: values.iter().map(|value| (*value).to_owned()).collect(),
            }
        }
        Self {
            class_registry: registry(
                "class",
                &[
                    "LanguageGeneration",
                    "LanguageUnderstanding",
                    "ExternalInformationRetrieval",
                    "StructuredDataProcessing",
                    "DocumentIngestion",
                    "ImageUnderstanding",
                    "AudioProcessing",
                    "PersistentStorage",
                    "StructuredComputation",
                    "HumanInteraction",
                    "ExternalCommunication",
                    "IdentityVerification",
                    "CryptographicOperation",
                ],
            ),
            form_registry: registry("form", &["Atomic", "Composite"]),
            origin_registry: registry(
                "origin",
                &[
                    "ExplicitSource",
                    "InterpreterInference",
                    "ApplicationSupplied",
                    "ReferencedArtifact",
                ],
            ),
            basis_registry: registry(
                "basis",
                &[
                    "DirectQuotation",
                    "StructuredExtraction",
                    "InterpreterSynthesis",
                    "ApplicationDeclaration",
                    "ReferencedArtifactDeclaration",
                ],
            ),
            necessity_registry: registry(
                "necessity",
                &[
                    "Required",
                    "Optional",
                    "Conditional",
                    "Supporting",
                    "Alternative",
                    "Unresolved",
                    "Unknown",
                ],
            ),
            relationship_registry: registry(
                "relationship",
                &[
                    "SupportsAsDeclared",
                    "DependsOnAsDeclared",
                    "RequiredForAsDeclared",
                    "AlternativeToAsDeclared",
                    "IncompatibleWithAsDeclared",
                    "SubstitutableWithAsDeclared",
                    "ComponentOf",
                ],
            ),
            status_registry: registry(
                "status",
                &[
                    "Represented",
                    "Incomplete",
                    "Unsupported",
                    "Unresolved",
                    "EvidenceLimited",
                    "ScopeUnresolved",
                ],
            ),
            scope_status_registry: registry(
                "scope-status",
                &["Resolved", "Partial", "Conflicting", "Unresolved"],
            ),
        }
    }
}

impl FixtureMeaningQualificationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: MeaningQualificationProfileId::derive(&["ulantra-local"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            allowed_domains: BTreeSet::from([
                "Ambiguity".to_owned(),
                "Assumption".to_owned(),
                "Uncertainty".to_owned(),
                "ClarificationRequirement".to_owned(),
            ]),
            allow_empty: true,
        }
    }
}

impl FixtureMeaningQualificationRegistries {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            domain_values: BTreeSet::from([
                "Ambiguity".to_owned(),
                "Assumption".to_owned(),
                "Uncertainty".to_owned(),
                "ClarificationRequirement".to_owned(),
            ]),
            relationship_values: BTreeSet::from(["AsDeclared".to_owned()]),
        }
    }
}

impl FixtureEvidenceProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: GroundingRepresentationProfileId::derive(&["ulantra-local"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            allowed_classes: [
                "SourceSpan",
                "QuotedText",
                "StructuredField",
                "ApplicationDeclaration",
                "ReferencedArtifact",
                "AdmittedProposalElement",
                "DerivedObservation",
                "CompositeEvidence",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_origins: [
                "Source",
                "Application",
                "ReferencedArtifact",
                "AdmittedProposal",
                "Derived",
                "Composite",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_material_kinds: [
                "AdmittedSourceReference",
                "QuotedTextValue",
                "StructuredFieldReference",
                "ApplicationDeclarationReference",
                "ReferencedArtifactReference",
                "AdmittedProposalElementReference",
                "DerivedObservationInputs",
                "CompositeEvidenceMembers",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_evidence_statuses: [
                "Observed",
                "Referenced",
                "Unavailable",
                "Incomplete",
                "Conflicting",
                "External",
                "Composite",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_representation_statuses: [
                "Represented",
                "Incomplete",
                "Unsupported",
                "EvidenceLimited",
                "Conflicting",
                "Unresolved",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_target_types: [
                "DeclaredObjective",
                "DeclaredConstraint",
                "CapabilityRequirement",
                "Ambiguity",
                "Assumption",
                "Uncertainty",
                "ClarificationRequirement",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_relationships: [
                "Supports",
                "PartiallySupports",
                "References",
                "DerivedFrom",
                "Aggregates",
                "Refines",
                "Supersedes",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allow_empty: true,
            allow_ungrounded_accounting: true,
        }
    }
}

impl FixtureEvidenceRegistries {
    pub(crate) fn local_governed_text() -> Self {
        let profile = FixtureEvidenceProfile::local_governed_text();
        Self {
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            evidence_classes: profile.allowed_classes.clone(),
            origins: profile.allowed_origins.clone(),
            bases: [
                "Declared",
                "Quoted",
                "Structured",
                "Referenced",
                "Derived",
                "Composed",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            evidence_statuses: profile.allowed_evidence_statuses.clone(),
            representation_statuses: profile.allowed_representation_statuses.clone(),
            target_types: profile.allowed_target_types.clone(),
            grounding_relationships: profile.allowed_relationships.clone(),
            locators: [
                "PreciseOffset",
                "PreciseLineRange",
                "BoundedSection",
                "StructuredField",
                "AmbiguousBounded",
                "NoLocatorSupplied",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        }
    }
}

impl FixtureProvenanceProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: ProvenanceProfileId::derive(&["ulantra-local"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            allowed_origins: [
                "ApplicationSupplied",
                "AdmittedProposalSupplied",
                "ImportedDeclaration",
                "PriorGovernedDeclaration",
                "ExternalIdentityMappingDeclaration",
                "TestFixture",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_subject_classes: [
                "SourceIntakeRecord",
                "AdmittedInterpretationProposalSet",
                "DeclaredObjectiveSet",
                "DeclaredConstraintSet",
                "CapabilityRequirementSet",
                "MeaningQualificationSet",
                "InterpretationEvidenceSet",
                "ExternalSubject",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_event_classes: ["Transformation", "Custody", "Publication", "Lifecycle"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            allowed_lineage_relationships: ["DerivedFrom", "ComponentDerivedFrom", "RelatedTo"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            allowed_lifecycle_relationships: ["Supersedes", "Corrects", "Replaces", "RelatedTo"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            allowed_mapping_namespaces: ["ulantra-external"]
                .into_iter()
                .map(str::to_owned)
                .collect(),
            allowed_conflict_classes: [
                "ConflictingHistory",
                "ConflictingTime",
                "ConflictingLineage",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allowed_time_variants: [
                "Exact",
                "Range",
                "Approximate",
                "Relative",
                "Unknown",
                "Unavailable",
                "Conflicting",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            allow_empty: true,
        }
    }
}

impl FixtureProvenanceRegistries {
    pub(crate) fn local_governed_text() -> Self {
        let profile = FixtureProvenanceProfile::local_governed_text();
        Self {
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            subject_classes: profile.allowed_subject_classes.clone(),
            event_classes: profile.allowed_event_classes.clone(),
            lineage_relationships: profile.allowed_lineage_relationships.clone(),
            lifecycle_relationships: profile.allowed_lifecycle_relationships.clone(),
            mapping_namespaces: profile.allowed_mapping_namespaces.clone(),
            conflict_classes: profile.allowed_conflict_classes.clone(),
            time_variants: profile.allowed_time_variants.clone(),
        }
    }
}

impl FixtureSemanticReconciliationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: SemanticReconciliationProfileId::derive(&["ulantra-contract-009-local-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            registry_version: "1.0.0".to_owned(),
            schema_version: "1.0.0".to_owned(),
            configuration_version: "1.0.0".to_owned(),
            allowed_domains: BTreeSet::from([
                "Objective".to_owned(),
                "Constraint".to_owned(),
                "CapabilityRequirement".to_owned(),
                "Ambiguity".to_owned(),
                "Assumption".to_owned(),
                "Uncertainty".to_owned(),
            ]),
            allowed_interactions: BTreeSet::from([
                "Objective<->Objective".to_owned(),
                "Constraint<->Constraint".to_owned(),
                "CapabilityRequirement<->CapabilityRequirement".to_owned(),
                "Ambiguity<->Ambiguity".to_owned(),
                "Assumption<->Assumption".to_owned(),
                "Uncertainty<->Uncertainty".to_owned(),
                "Objective<->Constraint".to_owned(),
                "Objective<->CapabilityRequirement".to_owned(),
                "Constraint<->CapabilityRequirement".to_owned(),
            ]),
            allowed_grouping_rules: BTreeSet::from([
                "ExplicitGroupDeclarationRule".to_owned(),
                "ExactUpstreamLogicalIdentityGroupingRule".to_owned(),
                "ClosedFixtureInteractionGroupingRule".to_owned(),
            ]),
            allowed_comparison_rules: BTreeSet::from([
                "ExactIdentityRule".to_owned(),
                "ExactRepresentationMatchRule".to_owned(),
                "ExactDuplicateRule".to_owned(),
                "ProfileEquivalentFixtureRule".to_owned(),
                "CompatibleFixtureRule".to_owned(),
                "ComplementaryFixtureRule".to_owned(),
                "AlternativeFixtureRule".to_owned(),
                "CompetingFixtureRule".to_owned(),
                "ConflictFixtureRule".to_owned(),
                "UnrelatedFixtureRule".to_owned(),
                "IndeterminateRule".to_owned(),
            ]),
            allowed_decision_rules: BTreeSet::from([
                "DuplicateMergeRule".to_owned(),
                "CompatibleCompositionRule".to_owned(),
                "ExplicitSelectionRule".to_owned(),
                "AlternativePreservationRule".to_owned(),
                "DeclaredStructuralSplitRule".to_owned(),
                "ConflictPreservationRule".to_owned(),
                "DeferralRule".to_owned(),
                "ExclusionRule".to_owned(),
                "TieBreakRule".to_owned(),
                "InsufficientAuthorityRule".to_owned(),
                "CrossDomainConflictPreservationRule".to_owned(),
            ]),
            allowed_standings: [
                Standing::Included,
                Standing::Contributing,
                Standing::PreservedAsAlternative,
                Standing::Deferred,
                Standing::Excluded,
                Standing::Unresolved,
            ]
            .into_iter()
            .collect(),
            allow_empty: true,
        }
    }
}

impl FixtureSemanticReconciliationRegistries {
    pub(crate) fn local_governed_text() -> Self {
        let p = FixtureSemanticReconciliationProfile::local_governed_text();
        Self {
            version: p.registry_version.clone(),
            grouping_rules: p.allowed_grouping_rules.clone(),
            comparison_rules: p.allowed_comparison_rules.clone(),
            decision_rules: p.allowed_decision_rules.clone(),
            standing_values: p.allowed_standings.clone(),
        }
    }
}

impl FixtureNormalizationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: NormalizationProfileId::derive(&["ulantra-contract-010-local-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            supported_semantic_domains: BTreeSet::from([
                "Objective".to_owned(),
                "Constraint".to_owned(),
                "CapabilityRequirement".to_owned(),
            ]),
            supported_semantic_classes: BTreeSet::from([
                "RequestedOutcome".to_owned(),
                "Content".to_owned(),
                "LanguageUnderstanding".to_owned(),
            ]),
            supported_relationship_classes: BTreeSet::from(["DependsOn".to_owned()]),
            eligible_standings: BTreeSet::from([Standing::Included, Standing::Contributing]),
            deferred_conditions: BTreeSet::from(["profile-deferred".to_owned()]),
            permitted_dispositions: BTreeSet::from([
                NormalizationDisposition::Mapped,
                NormalizationDisposition::Identity,
                NormalizationDisposition::Preserved,
                NormalizationDisposition::Deferred,
            ]),
            canonical_registry_id: CanonicalRegistryId::derive(&["ulantra-contract-010-local-v1"]),
            canonical_registry_version: "1.0.0".to_owned(),
            mapping_rule_registry_id: MappingRuleRegistryId::derive(&[
                "ulantra-contract-010-local-v1",
            ]),
            mapping_rule_registry_version: "1.0.0".to_owned(),
            schema_version: "1.0.0".to_owned(),
            configuration_version: "1.0.0".to_owned(),
            implementation_version: "ulantra-sre-ibos-local-integration-v1".to_owned(),
            preservation_permissions: BTreeSet::new(),
            identity_conditions: BTreeSet::new(),
            relationship_preservation_permissions: BTreeSet::new(),
            relationship_identity_conditions: BTreeSet::new(),
            forbidden_transformations: BTreeSet::from([
                "representational decomposition".to_owned(),
                "collection deduplication".to_owned(),
                "unit conversion".to_owned(),
                "temporal conversion".to_owned(),
                "locale-sensitive conversion".to_owned(),
                "free-text synonym mapping".to_owned(),
                "open identifier repair".to_owned(),
                "semantic abbreviation expansion".to_owned(),
            ]),
            decomposition_enabled: false,
            collection_deduplication_enabled: false,
        }
    }
}

impl FixtureCanonicalRegistry {
    pub(crate) fn local_governed_text() -> Self {
        let p = FixtureNormalizationProfile::local_governed_text();
        Self {
            identity: p.canonical_registry_id,
            version: p.canonical_registry_version,
            entries: vec![].into(),
        }
    }
}
impl FixtureMappingRuleRegistry {
    pub(crate) fn local_governed_text() -> Self {
        let p = FixtureNormalizationProfile::local_governed_text();
        Self {
            identity: p.mapping_rule_registry_id,
            version: p.mapping_rule_registry_version,
            rules: vec![].into(),
        }
    }
}

impl SemanticNormalizationInput {
    pub(crate) fn local_governed_text(set: &SemanticReconciliationSet) -> Self {
        let p = FixtureNormalizationProfile::local_governed_text();
        Self {
            input_id: NormalizationInputId::derive(&[set.set_id.as_str(), "ulantra-input"]),
            operation_id: NormalizationOperationId::derive(&[
                set.set_id.as_str(),
                "ulantra-operation",
            ]),
            reconciliation_set_id: set.set_id.clone(),
            upstream_content_binding: reconciliation_content_binding(set),
            profile_id: p.identity,
            profile_version: p.version,
            canonical_registry_id: p.canonical_registry_id,
            canonical_registry_version: p.canonical_registry_version,
            mapping_rule_registry_id: p.mapping_rule_registry_id,
            mapping_rule_registry_version: p.mapping_rule_registry_version,
            schema_version: p.schema_version,
            configuration_version: p.configuration_version,
            implementation_version: p.implementation_version,
            replay_context: set.replay_context.clone(),
        }
    }
}

impl FixtureCanonicalOrderingProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: CanonicalOrderingProfileId::derive(&["ulantra-contract-011-local-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            ordering_registry_id: OrderingRegistryId::derive(&["ulantra-contract-011-local-v1"]),
            ordering_registry_version: "1.0.0".to_owned(),
            ordering_rule_registry_id: OrderingRuleRegistryId::derive(&[
                "ulantra-contract-011-local-v1",
            ]),
            ordering_rule_registry_version: "1.0.0".to_owned(),
            scope_registry_version: "1.0.0".to_owned(),
            comparison_version: "1.0.0".to_owned(),
            traversal_rule_id: "DeterministicKahnTraversal".to_owned(),
            traversal_rule_version: "1.0.0".to_owned(),
            tie_break_rule_id: "SubjectIdentityAscendingTieBreak".to_owned(),
            tie_break_rule_version: "1.0.0".to_owned(),
            schema_version: "1.0.0".to_owned(),
            configuration_version: "1.0.0".to_owned(),
            implementation_version: "ulantra-sre-ibos-local-integration-v1".to_owned(),
            relationship_bridges: BTreeMap::from([(
                "DependsOn".to_owned(),
                "target-before-source".to_owned(),
            )]),
            require_tie_break: true,
            identity_order_authorized: false,
            preserved_order_authorized: false,
        }
    }
}

impl CanonicalOrderingInput {
    pub(crate) fn local_governed_text(publication: &NormalizedRequestRepresentation) -> Self {
        let profile = FixtureCanonicalOrderingProfile::local_governed_text();
        let subjects = orderable_subjects(publication);
        let scope_id =
            OrderingScopeId::derive(&[publication.publication_id.as_str(), "request-wide"]);
        let subject_ids: Arc<[OrderableSubjectId]> = subjects
            .iter()
            .map(|subject| subject.subject_id.clone())
            .collect::<Vec<_>>()
            .into();
        let scope = OrderingScope {
            scope_id,
            scope_kind: "RequestWide".to_owned(),
            participating_subject_ids: subject_ids,
            governing_constraint_ids: Vec::new().into(),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            registry_binding: profile.registry_binding(),
        };
        Self {
            input_id: CanonicalOrderingInputId::derive(&[
                publication.publication_id.as_str(),
                "ulantra-input",
            ]),
            operation_id: CanonicalOrderingOperationId::derive(&[
                publication.publication_id.as_str(),
                "ulantra-operation",
            ]),
            normalized_publication_id: publication.publication_id.clone(),
            normalized_content_binding: normalized_publication_content_binding(publication),
            profile_id: profile.identity,
            profile_version: profile.version,
            ordering_registry_id: profile.ordering_registry_id,
            ordering_registry_version: profile.ordering_registry_version,
            ordering_rule_registry_id: profile.ordering_rule_registry_id,
            ordering_rule_registry_version: profile.ordering_rule_registry_version,
            scope_registry_version: profile.scope_registry_version,
            comparison_version: profile.comparison_version,
            traversal_rule_id: profile.traversal_rule_id,
            traversal_rule_version: profile.traversal_rule_version,
            tie_break_rule_id: profile.tie_break_rule_id,
            tie_break_rule_version: profile.tie_break_rule_version,
            schema_version: profile.schema_version,
            configuration_version: profile.configuration_version,
            implementation_version: profile.implementation_version,
            scopes: vec![scope].into(),
            constraints: Vec::new().into(),
            replay_context: publication.replay_context.clone(),
        }
    }
}

impl FixtureValidationProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: ValidationProfileId::derive(&["ulantra-contract-012-local-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            requirements_id: ConstructionRequirementsId::derive(&["ulantra-contract-012-local-v1"]),
            requirements_version: "1.0.0".to_owned(),
            schema_version: "1.0.0".to_owned(),
            registry_id: StructuralRegistryId::derive(&["ulantra-contract-012-local-v1"]),
            registry_version: "1.0.0".to_owned(),
            rule_set_id: ValidationRuleSetId::derive(&["ulantra-contract-012-local-v1"]),
            rule_set_version: "1.0.0".to_owned(),
            configuration_version: "1.0.0".to_owned(),
            implementation_version: "ulantra-sre-ibos-local-integration-v1".to_owned(),
            aggregation_rule_id: "MandatoryThenDeferredThenWarning".to_owned(),
            aggregation_rule_version: "1.0.0".to_owned(),
            warning_condition: None,
            deferred_condition: None,
            omit_decision_for: None,
        }
    }
}

impl FixtureConstructionRequirements {
    pub(crate) fn local_governed_text(profile: &FixtureValidationProfile) -> Self {
        let binding = (profile.identity.clone(), profile.version.clone());
        let requirement = |class: &str, rule: &str, mandatory: bool| ValidationRequirement {
            requirement_id: ValidationRequirementId::derive(&[class]),
            requirement_class: class.to_owned(),
            rule_id: rule.to_owned(),
            rule_version: profile.rule_set_version.clone(),
            mandatory,
            structural_only: true,
            profile_binding: binding.clone(),
        };
        Self {
            identity: profile.requirements_id.clone(),
            version: profile.requirements_version.clone(),
            requirements: vec![
                requirement(
                    "RequiredStructuralPresence",
                    "RequiredStructuralPresenceRule",
                    true,
                ),
                requirement("IdentifierIntegrity", "IdentifierIntegrityRule", true),
                requirement("RelationshipIntegrity", "RelationshipIntegrityRule", true),
                requirement("OrderingIntegrity", "OrderingIntegrityRule", true),
                requirement("CardinalityIntegrity", "CardinalityIntegrityRule", true),
                requirement(
                    "ConstructionPrerequisiteIntegrity",
                    "ConstructionPrerequisiteRule",
                    true,
                ),
            ]
            .into(),
        }
    }
}

impl FixtureStructuralRegistries {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: StructuralRegistryId::derive(&["ulantra-contract-012-local-v1"]),
            version: "1.0.0".to_owned(),
        }
    }
}

impl FixtureValidationRuleSet {
    pub(crate) fn local_governed_text(profile: &FixtureValidationProfile) -> Self {
        Self {
            identity: profile.rule_set_id.clone(),
            version: profile.rule_set_version.clone(),
            known_rule_ids: vec![
                "RequiredStructuralPresenceRule".to_owned(),
                "IdentifierIntegrityRule".to_owned(),
                "RelationshipIntegrityRule".to_owned(),
                "OrderingIntegrityRule".to_owned(),
                "CardinalityIntegrityRule".to_owned(),
                "ConstructionPrerequisiteRule".to_owned(),
            ]
            .into(),
        }
    }
}

impl StructuralValidationInput {
    pub(crate) fn local_governed_text(
        publication: &CanonicallyOrderedRequestRepresentation,
    ) -> Self {
        let profile = FixtureValidationProfile::local_governed_text();
        Self {
            validation_id: StructuralValidationId::derive(&[
                publication.publication_id.as_str(),
                "ulantra-validation",
            ]),
            operation_id: StructuralValidationOperationId::derive(&[
                publication.publication_id.as_str(),
                "ulantra-operation",
            ]),
            ordered_publication_id: publication.publication_id.clone(),
            ordered_content_binding: ordered_publication_content_binding(publication),
            profile_id: profile.identity,
            profile_version: profile.version,
            requirements_id: profile.requirements_id,
            requirements_version: profile.requirements_version,
            schema_version: profile.schema_version,
            registry_id: profile.registry_id,
            registry_version: profile.registry_version,
            rule_set_id: profile.rule_set_id,
            rule_set_version: profile.rule_set_version,
            configuration_version: profile.configuration_version,
            implementation_version: profile.implementation_version,
            aggregation_rule_id: profile.aggregation_rule_id,
            aggregation_rule_version: profile.aggregation_rule_version,
            replay_context: publication.replay_context.clone(),
        }
    }
}

impl FixtureConstructionProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: ConstructionProfileId::derive(&["ulantra-governed-text-construction-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            schema_id: ConstructionSchemaId::derive(&["ulantra-contract-013-local-v1"]),
            schema_version: "1.0.0".to_owned(),
            rule_set_id: ConstructionRuleSetId::derive(&["ulantra-exact-construction-rules-v1"]),
            rule_set_version: "1.0.0".to_owned(),
            registry_id: ConstructionRegistryId::derive(&["ulantra-contract-013-local-v1"]),
            registry_version: "1.0.0".to_owned(),
            configuration_id: ConstructionConfigurationId::derive(&[
                "ulantra-contract-013-local-v1",
            ]),
            configuration_version: "1.0.0".to_owned(),
            implementation_version: "ulantra-sre-ibos-local-integration-v1".to_owned(),
            optional_structural_class: "IndexHolder".to_owned(),
            optional_omission_rule_id: "OmitEmptyOptionalIndexRule".to_owned(),
            emit_optional_structural_component: false,
            omission_rule_authorized: true,
        }
    }
}

impl FixtureConstructionSchema {
    pub(crate) fn local_governed_text(profile: &FixtureConstructionProfile) -> Self {
        Self {
            identity: profile.schema_id.clone(),
            version: profile.schema_version.clone(),
            structural_classes: vec![
                "Container".to_owned(),
                "ReferenceHolder".to_owned(),
                "StructuralGrouping".to_owned(),
                "ValidationLineageHolder".to_owned(),
                "ConstructionMetadataHolder".to_owned(),
                "SectionHeader".to_owned(),
                "IndexHolder".to_owned(),
            ]
            .into(),
            optional_classes: vec![profile.optional_structural_class.clone()].into(),
        }
    }
}

impl FixtureConstructionRuleSet {
    pub(crate) fn local_governed_text(profile: &FixtureConstructionProfile) -> Self {
        Self {
            identity: profile.rule_set_id.clone(),
            version: profile.rule_set_version.clone(),
            known_rule_ids: vec![
                "IncludeNormalizedElementRule".to_owned(),
                "IncludeNormalizedRelationshipRule".to_owned(),
                profile.optional_omission_rule_id.clone(),
            ]
            .into(),
        }
    }
}

impl FixtureConstructionRegistries {
    pub(crate) fn local_governed_text(profile: &FixtureConstructionProfile) -> Self {
        Self {
            identity: profile.registry_id.clone(),
            version: profile.registry_version.clone(),
        }
    }
}

impl FixtureConstructionConfiguration {
    pub(crate) fn local_governed_text(profile: &FixtureConstructionProfile) -> Self {
        Self {
            identity: profile.configuration_id.clone(),
            version: profile.configuration_version.clone(),
        }
    }
}

impl FixtureConstructionAuthorityContext {
    pub(crate) fn local_governed_text(profile: &FixtureConstructionProfile) -> Self {
        Self {
            profile: profile.clone(),
            schema: FixtureConstructionSchema::local_governed_text(profile),
            rules: FixtureConstructionRuleSet::local_governed_text(profile),
            registries: FixtureConstructionRegistries::local_governed_text(profile),
            configuration: FixtureConstructionConfiguration::local_governed_text(profile),
        }
    }
}

impl CanonicalRequestConstructionInput {
    pub(crate) fn local_governed_text(
        publication: &CanonicallyOrderedRequestRepresentation,
        validation: &StructuralValidationResult,
    ) -> Self {
        let profile = FixtureConstructionProfile::local_governed_text();
        Self {
            construction_id: CanonicalRequestConstructionId::derive(&[
                publication.publication_id.as_str(),
                validation.result_id.as_str(),
                "ulantra-construction",
            ]),
            operation_id: CanonicalRequestConstructionId::derive(&[
                publication.publication_id.as_str(),
                validation.result_id.as_str(),
                "ulantra-operation",
            ]),
            ordered_representation_id: publication.publication_id.clone(),
            ordered_representation_content_binding: ordered_publication_content_binding(
                publication,
            ),
            validation_result_id: validation.result_id.clone(),
            validation_subject_binding: structural_validation_subject_binding(validation),
            validation_completion: validation.completion,
            validation_eligibility: validation.eligibility,
            construction_profile_id: profile.identity,
            construction_profile_version: profile.version,
            construction_schema_id: profile.schema_id,
            construction_schema_version: profile.schema_version,
            construction_rule_set_id: profile.rule_set_id,
            construction_rule_set_version: profile.rule_set_version,
            construction_registry_id: profile.registry_id,
            construction_registry_version: profile.registry_version,
            construction_configuration_id: profile.configuration_id,
            construction_configuration_version: profile.configuration_version,
            implementation_version: profile.implementation_version,
            replay_context: publication.replay_context.clone(),
        }
    }
}

impl IssuanceContext {
    pub(crate) fn local_governed_text(name: &str) -> Self {
        Self {
            identity: IssuanceContextId::derive(&["ulantra-local-request-issuance-v1", name]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            context_name: name.to_owned(),
            initial_standing_authorized: true,
        }
    }
}

impl IdentityPolicy {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: IdentityPolicyId::derive(&["ulantra-governed-text-identity-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            identity_input_labels: vec![
                "constructed_request_id".to_owned(),
                "construction_manifest_id".to_owned(),
                "construction_id".to_owned(),
                "issuance_context_id".to_owned(),
                "issuance_context_version".to_owned(),
                "identity_policy_id".to_owned(),
                "identity_policy_version".to_owned(),
                "issuance_profile_id".to_owned(),
                "issuance_profile_version".to_owned(),
                "issuance_schema_id".to_owned(),
                "issuance_schema_version".to_owned(),
                "issuance_rule_set_id".to_owned(),
                "issuance_rule_set_version".to_owned(),
                "issuance_registry_id".to_owned(),
                "issuance_registry_version".to_owned(),
                "issuance_configuration_id".to_owned(),
                "issuance_configuration_version".to_owned(),
            ]
            .into(),
            standing_rule_id: "AssignInitialIssuedStanding".to_owned(),
            standing_rule_version: "1.0.0".to_owned(),
        }
    }
}

impl FixtureIssuanceProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: IssuanceProfileId::derive(&["ulantra-governed-text-issuance-v1"]),
            version: "1.0.0".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            schema_id: IssuanceSchemaId::derive(&["ulantra-contract-014-local-v1"]),
            schema_version: "1.0.0".to_owned(),
            rule_set_id: IssuanceRuleSetId::derive(&["ulantra-contract-014-local-v1"]),
            rule_set_version: "1.0.0".to_owned(),
            registry_id: IssuanceRegistryId::derive(&["ulantra-contract-014-local-v1"]),
            registry_version: "1.0.0".to_owned(),
            configuration_id: IssuanceConfigurationId::derive(&["ulantra-contract-014-local-v1"]),
            configuration_version: "1.0.0".to_owned(),
            implementation_version: "ulantra-sre-ibos-local-integration-v1".to_owned(),
        }
    }
}

impl FixtureIssuanceSchema {
    pub(crate) fn local_governed_text(profile: &FixtureIssuanceProfile) -> Self {
        Self {
            identity: profile.schema_id.clone(),
            version: profile.schema_version.clone(),
        }
    }
}

impl FixtureIssuanceRuleSet {
    pub(crate) fn local_governed_text(
        profile: &FixtureIssuanceProfile,
        policy: &IdentityPolicy,
    ) -> Self {
        Self {
            identity: profile.rule_set_id.clone(),
            version: profile.rule_set_version.clone(),
            issuance_rule_id: policy.standing_rule_id.clone(),
            issuance_rule_version: policy.standing_rule_version.clone(),
        }
    }
}

impl FixtureIssuanceRegistries {
    pub(crate) fn local_governed_text(profile: &FixtureIssuanceProfile) -> Self {
        Self {
            identity: profile.registry_id.clone(),
            version: profile.registry_version.clone(),
        }
    }
}

impl FixtureIssuanceConfiguration {
    pub(crate) fn local_governed_text(profile: &FixtureIssuanceProfile) -> Self {
        Self {
            identity: profile.configuration_id.clone(),
            version: profile.configuration_version.clone(),
        }
    }
}

impl FixtureIssuanceAuthorityContext {
    pub(crate) fn local_governed_text(context: &IssuanceContext) -> Self {
        let profile = FixtureIssuanceProfile::local_governed_text();
        let identity_policy = IdentityPolicy::local_governed_text();
        Self {
            schema: FixtureIssuanceSchema::local_governed_text(&profile),
            rules: FixtureIssuanceRuleSet::local_governed_text(&profile, &identity_policy),
            registries: FixtureIssuanceRegistries::local_governed_text(&profile),
            configuration: FixtureIssuanceConfiguration::local_governed_text(&profile),
            profile,
            identity_policy,
            context: context.clone(),
        }
    }
}

impl CanonicalRequestIssuanceInput {
    pub(crate) fn local_governed_text(
        request: &ConstructedCanonicalRequest,
        manifest: &ConstructionManifest,
        context: &IssuanceContext,
    ) -> Self {
        let authorities = FixtureIssuanceAuthorityContext::local_governed_text(context);
        Self {
            issuance_id: CanonicalRequestIssuanceId::derive(&[
                request.constructed_canonical_request_id.as_str(),
                manifest.construction_manifest_id.as_str(),
                context.identity.as_str(),
                "ulantra-issuance",
            ]),
            operation_id: CanonicalRequestIssuanceId::derive(&[
                request.constructed_canonical_request_id.as_str(),
                manifest.construction_manifest_id.as_str(),
                context.identity.as_str(),
                "ulantra-operation",
            ]),
            constructed_request_id: request.constructed_canonical_request_id.clone(),
            construction_manifest_id: manifest.construction_manifest_id.clone(),
            construction_publication_binding: manifest.publication_binding.clone(),
            issuance_context_id: context.identity.clone(),
            issuance_context_version: context.version.clone(),
            identity_policy_id: authorities.identity_policy.identity,
            identity_policy_version: authorities.identity_policy.version,
            issuance_profile_id: authorities.profile.identity,
            issuance_profile_version: authorities.profile.version,
            issuance_schema_id: authorities.schema.identity,
            issuance_schema_version: authorities.schema.version,
            issuance_rule_set_id: authorities.rules.identity,
            issuance_rule_set_version: authorities.rules.version,
            issuance_registry_id: authorities.registries.identity,
            issuance_registry_version: authorities.registries.version,
            issuance_configuration_id: authorities.configuration.identity,
            issuance_configuration_version: authorities.configuration.version,
            implementation_version: authorities.profile.implementation_version,
            replay_context: request.construction_context.clone(),
        }
    }
}

impl DownstreamBoundaryDeclaration {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            declaration_id: DownstreamBoundaryDeclarationId::derive(&[
                "ulantra-ibos-governed-text-recipient-v1",
            ]),
            declaration_version: "1.0.0".to_owned(),
            downstream_boundary_id: "ibos-local-governed-text-intake-v1".to_owned(),
            boundary_class: "DeclaredGovernedTextRecipient".to_owned(),
            authority_reference: "mineard-local-ulantra-sre-assignments-v1".to_owned(),
            permitted_handoff_role: "ReceiveForIndependentGovernedEvaluation".to_owned(),
            receipt_authority: "ibos-local-governed-text-recipient".to_owned(),
            supported_acknowledgment_method: "RecipientGeneratedExactPublicationReceipt".to_owned(),
            supported_custody_model: "QualifiedReceiptOfExactPublication".to_owned(),
            supported_responsibility_model: "AttemptIndependentGovernedEvaluationOnly".to_owned(),
            supported_transport_binding: "ulantra-ibos-local-process-v1".to_owned(),
            effective_status: BoundaryEffectiveStatus::Active,
        }
    }
}

impl HandoffContext {
    pub(crate) fn local_governed_text(
        request: &CanonicalStructuredRequest,
        manifest: &IssuanceManifest,
    ) -> Self {
        Self {
            handoff_context_id: HandoffContextId::derive(&[
                request.canonical_structured_request_id.as_str(),
                manifest.issuance_manifest_id.as_str(),
                "ulantra-contract-015-local-v1",
            ]),
            context_version: "1.0.0".to_owned(),
            canonical_structured_request_id: request.canonical_structured_request_id.clone(),
            issuance_manifest_id: manifest.issuance_manifest_id.clone(),
            issuance_operation_id: manifest.issuance_id.clone(),
            sender_boundary_id: "structured-request-engine".to_owned(),
            downstream_boundary_declaration_id: DownstreamBoundaryDeclaration::local_governed_text(
            )
            .declaration_id,
            downstream_boundary_declaration_version: "1.0.0".to_owned(),
            recipient_boundary_id: DownstreamBoundaryDeclaration::local_governed_text()
                .downstream_boundary_id,
            handoff_profile_id: HandoffProfileId::derive(&[
                "ulantra-to-ibos-governed-text-handoff-v1",
            ]),
            handoff_profile_version: "1.0.0".to_owned(),
            transfer_rule_set_id: HandoffRuleSetId::derive(&[
                "ulantra-exact-publication-transfer-v1",
            ]),
            transfer_rule_set_version: "1.0.0".to_owned(),
            custody_rule_set_id: HandoffRuleSetId::derive(&[
                "ulantra-qualified-request-custody-v1",
            ]),
            custody_rule_set_version: "1.0.0".to_owned(),
            responsibility_rule_set_id: HandoffRuleSetId::derive(&[
                "ulantra-evaluation-responsibility-v1",
            ]),
            responsibility_rule_set_version: "1.0.0".to_owned(),
            acknowledgment_rule_set_id: HandoffRuleSetId::derive(&[
                "ulantra-exact-receipt-acknowledgment-v1",
            ]),
            acknowledgment_rule_set_version: "1.0.0".to_owned(),
            duplicate_rule_set_id: HandoffRuleSetId::derive(&[
                "ulantra-no-duplicate-recognition-v1",
            ]),
            duplicate_rule_set_version: "1.0.0".to_owned(),
            retry_rule_set_id: HandoffRuleSetId::derive(&["ulantra-no-automatic-retry-v1"]),
            retry_rule_set_version: "1.0.0".to_owned(),
            expiration_rule_set_id: HandoffRuleSetId::derive(&[
                "ulantra-local-request-expiration-v1",
            ]),
            expiration_rule_set_version: "1.0.0".to_owned(),
            transfer_purpose: "ReceiveExactIssuedRequestForIndependentGovernedEvaluation"
                .to_owned(),
            reference_mode: "ReferenceOnly".to_owned(),
            custody_model: "QualifiedReceiptOfExactPublication".to_owned(),
            responsibility_model: "AttemptIndependentGovernedEvaluationOnly".to_owned(),
            acknowledgment_model: "RecipientAttributable".to_owned(),
            retry_policy: "NoAutomaticRetry".to_owned(),
            duplicate_policy: "NoDuplicateRecognition".to_owned(),
            expiration_policy: "ContextOnly".to_owned(),
            schema_version: "ulantra-sre-handoff-schema-v1".to_owned(),
            registry_version: "1.0.0".to_owned(),
            configuration_version: "1.0.0".to_owned(),
            implementation_version: "ulantra-sre-ibos-local-integration-v1".to_owned(),
            replay_context: BTreeMap::from([(
                "ulantra-local".to_owned(),
                "contract-015".to_owned(),
            )]),
        }
    }
}

impl FixtureHandoffProfile {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffProfileId::derive(&["ulantra-to-ibos-governed-text-handoff-v1"]),
            version: "1.0.0".to_owned(),
            reference_mode: "ReferenceOnly".to_owned(),
            allow_delivered_unacknowledged: false,
            recipient_unavailable_attempt_threshold: 2,
            custody_on_received: true,
            responsibility_on_received: true,
        }
    }
}

impl FixtureHandoffAuthorityContext {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            profile: FixtureHandoffProfile::local_governed_text(),
            transfer_rules: FixtureTransferRuleSet::local_governed_text(),
            custody_rules: FixtureCustodyRuleSet::local_governed_text(),
            responsibility_rules: FixtureResponsibilityRuleSet::local_governed_text(),
            acknowledgment_rules: FixtureAcknowledgmentRuleSet::local_governed_text(),
            retry_rules: FixtureRetryRuleSet::local_governed_text(),
            duplicate_rules: FixtureDuplicateRuleSet::local_governed_text(),
            expiration_rules: FixtureExpirationRuleSet::local_governed_text(),
            registries: FixtureHandoffRegistries {
                identity: HandoffRegistryId::derive(&["ulantra-contract-015-local-v1"]),
                version: "1.0.0".to_owned(),
            },
            transport: FixtureTransportBinding {
                identity: "ulantra-ibos-local-process-v1".to_owned(),
                version: "1.0.0".to_owned(),
            },
            configuration: FixtureHandoffConfiguration {
                identity: HandoffConfigurationId::derive(&["ulantra-contract-015-local-v1"]),
                version: "1.0.0".to_owned(),
            },
        }
    }
}

impl CanonicalRequestHandoffInput {
    pub(crate) fn local_governed_text(
        request: &CanonicalStructuredRequest,
        manifest: &IssuanceManifest,
        context: &HandoffContext,
    ) -> Self {
        let authorities = FixtureHandoffAuthorityContext::local_governed_text();
        Self {
            handoff_id: CanonicalRequestHandoffId::derive(&[
                request.canonical_structured_request_id.as_str(),
                manifest.issuance_manifest_id.as_str(),
                context.handoff_context_id.as_str(),
            ]),
            operation_id: CanonicalRequestHandoffId::derive(&[
                request.canonical_structured_request_id.as_str(),
                manifest.issuance_manifest_id.as_str(),
                context.handoff_context_id.as_str(),
                "operation",
            ]),
            canonical_structured_request_id: request.canonical_structured_request_id.clone(),
            issuance_manifest_id: manifest.issuance_manifest_id.clone(),
            issuance_operation_id: manifest.issuance_id.clone(),
            issuance_publication_binding: manifest.publication_binding.clone(),
            boundary_declaration_id: context.downstream_boundary_declaration_id.clone(),
            boundary_declaration_version: context.downstream_boundary_declaration_version.clone(),
            handoff_context_id: context.handoff_context_id.clone(),
            handoff_context_version: context.context_version.clone(),
            handoff_profile_id: authorities.profile.identity,
            handoff_profile_version: authorities.profile.version,
            transfer_rule_set_id: authorities.transfer_rules.identity,
            transfer_rule_set_version: authorities.transfer_rules.version,
            custody_rule_set_id: authorities.custody_rules.identity,
            custody_rule_set_version: authorities.custody_rules.version,
            responsibility_rule_set_id: authorities.responsibility_rules.identity,
            responsibility_rule_set_version: authorities.responsibility_rules.version,
            transport_binding: authorities.transport.identity,
            schema_version: context.schema_version.clone(),
            registry_version: context.registry_version.clone(),
            configuration_version: context.configuration_version.clone(),
            implementation_version: context.implementation_version.clone(),
            replay_context: context.replay_context.clone(),
        }
    }
}

impl FixtureTransferRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-exact-publication-transfer-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-exact-publication-transfer-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}

impl FixtureCustodyRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-qualified-request-custody-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-qualified-request-custody-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}

impl FixtureResponsibilityRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-evaluation-responsibility-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-evaluation-responsibility-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}

impl FixtureAcknowledgmentRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-exact-receipt-acknowledgment-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-exact-receipt-acknowledgment-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}

impl FixtureRetryRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-no-automatic-retry-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-no-automatic-retry-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}

impl FixtureDuplicateRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-no-duplicate-recognition-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-no-duplicate-recognition-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}

impl FixtureExpirationRuleSet {
    pub(crate) fn local_governed_text() -> Self {
        Self {
            identity: HandoffRuleSetId::derive(&["ulantra-local-request-expiration-v1"]),
            version: "1.0.0".into(),
            rule_id: "ulantra-local-request-expiration-v1".into(),
            rule_version: "1.0.0".into(),
        }
    }
}
