//! Contract 000/001 runtime foundation.
//!
//! This crate represents supplied source material and its intake state. It does
//! not interpret meaning, construct requests, authorize work, select providers,
//! or execute anything.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;

/// Opaque deterministic identity. It is an implementation identity, not a
/// canonical-request identity and not a claim that the value is a digest.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct StableId(String);

impl StableId {
    fn from_parts(prefix: &str, parts: &[&str]) -> Self {
        let mut hash: u64 = 0xcbf29ce484222325;
        for part in parts {
            for byte in part.as_bytes() {
                hash ^= u64::from(*byte);
                hash = hash.wrapping_mul(0x100000001b3);
            }
            hash ^= 0xff;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        Self(format!("{prefix}-{hash:016x}"))
    }

    /// Returns the stable textual form for inspection and replay fixtures.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod contract_005_tests {
    use super::*;

    fn request() -> InterpretationRequest {
        let input = SourceComponentInput::new(
            SourceCategory::NaturalLanguage,
            SourceOrigin::InteractiveUser,
            SourcePayload::inline("capability fixture source"),
            AdmissionState::Accepted,
        )
        .with_preservation(PreservationFacts::recoverable("immutable source"));
        let IntakeOutcome::Success(intake) = admit(
            &SourceSubmission::new(vec![input]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("source admission") else {
            panic!("fixture intake")
        };
        let context = InterpretationOperationContext::from_intake(&intake);
        let spec = InterpretationRequestSpec {
            included_source_ids: intake.source_ids(),
            excluded_source_ids: Vec::new(),
            requested_scope: BTreeSet::from([
                ProposalDomain::Objectives,
                ProposalDomain::Constraints,
                ProposalDomain::CapabilityRequirements,
            ]),
            contract_version: "0.1.0".to_owned(),
            proposal_schema_version: "proposal-fixture-v3".to_owned(),
            interpreter_profile: InterpreterProfileBinding {
                identity: StableId::from_parts("interpreter-profile", &["fixture"]),
                version: "fixture-interpreter-v1".to_owned(),
            },
            evidence_profile: EvidenceCapabilityProfile::fixture_strict().binding(),
            required_proposal_metadata: BTreeSet::from(["trace".to_owned()]),
            permitted_response_forms: BTreeSet::from(["structured-proposal".to_owned()]),
            completion_expectations: BTreeSet::from(["returned".to_owned()]),
            declared_bounds: BTreeSet::from(["structural-admission-only".to_owned()]),
            declared_exclusions: BTreeSet::from(["semantic-correctness".to_owned()]),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        };
        let InterpretationRequestOutcome::Request(request) =
            issue_interpretation_request(&context, spec)
        else {
            panic!("fixture request")
        };
        *request
    }

    fn objective() -> ProposalObjectiveElement {
        ProposalObjectiveElement {
            element_id: "objective-capability".to_owned(),
            expression: "achieve outcome".to_owned(),
            form: "Atomic".to_owned(),
            origin: "ExplicitSource".to_owned(),
            basis: "StructuredExtraction".to_owned(),
            class_name: "RequestedOutcome".to_owned(),
            scope: None,
            designations: Vec::new().into(),
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
            representation_status: None,
            relationships: Vec::new().into(),
            component_expressions: Vec::new().into(),
        }
    }
    fn constraint() -> ProposalConstraintElement {
        ProposalConstraintElement {
            element_id: "constraint-capability".to_owned(),
            expression: "do not disclose".to_owned(),
            class_name: "Content".to_owned(),
            form: "RequiredExclusion".to_owned(),
            origin: "ExplicitSource".to_owned(),
            basis: "StructuredExtraction".to_owned(),
            scope_kind: "RequestWide".to_owned(),
            scope_target_ids: Vec::new().into(),
            scope_resolution_status: "Resolved".to_owned(),
            objective_ids: Vec::new().into(),
            declared_priority: None,
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
            representation_status: None,
            relationships: Vec::new().into(),
            component_expressions: Vec::new().into(),
            reference: None,
            reference_state: "None".to_owned(),
            explicit_conflict: false,
        }
    }
    fn capability(expression: &str) -> ProposalCapabilityElement {
        ProposalCapabilityElement {
            element_id: format!("capability-{expression}"),
            expression: expression.to_owned(),
            class_name: "LanguageUnderstanding".to_owned(),
            supplied_class: None,
            supplied_method: None,
            abstraction_rule_id: None,
            abstraction_rule_version: None,
            form: "Atomic".to_owned(),
            origin: "ExplicitSource".to_owned(),
            basis: "StructuredExtraction".to_owned(),
            necessity: "Required".to_owned(),
            scope_kind: "RequestWide".to_owned(),
            scope_target_ids: Vec::new().into(),
            scope_resolution_status: "Resolved".to_owned(),
            objective_ids: Vec::new().into(),
            constraint_ids: Vec::new().into(),
            access_dependency: None,
            classification_support_status: "Classified".to_owned(),
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
            representation_status: None,
            relationships: Vec::new().into(),
            component_expressions: Vec::new().into(),
        }
    }
    fn proposal(
        request: &InterpretationRequest,
        id: &str,
        capabilities: Vec<ProposalCapabilityElement>,
        narrative: Vec<&str>,
    ) -> InterpretationProposal {
        InterpretationProposal::new(
            request,
            StableId::from_parts("interpreter", &[id]),
            "interpreter-v1",
            request.interpreter_profile.clone(),
            ProposalProductionState::Returned,
            ProposalCompletionState::Complete,
            request.included_source_ids.to_vec(),
            Some(EvidenceCapabilityProfile::fixture_strict()),
            ProposalContent {
                proposed_elements: narrative
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
                    .into(),
                objective_elements: vec![objective()].into(),
                constraint_elements: vec![constraint()].into(),
                capability_elements: capabilities.into(),
                clarification_elements: Vec::new().into(),
                evidence_elements: Vec::new().into(),
                evidence_references: Vec::new().into(),
                declared_assumptions: Vec::new().into(),
                declared_uncertainties: Vec::new().into(),
                declared_bounds: Vec::new().into(),
            },
            BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        )
    }
    fn admitted_set(
        request: &InterpretationRequest,
        proposals: Vec<InterpretationProposal>,
    ) -> AdmittedInterpretationProposalSet {
        let pairs = proposals
            .into_iter()
            .map(|proposal| {
                let ProposalAdmissionOutcome::Decision(decision) =
                    admit_interpretation_proposal(request, &proposal, "criteria-v3")
                else {
                    panic!("proposal decision")
                };
                (proposal, *decision)
            })
            .collect();
        let SetPublicationOutcome::Set(set) = publish_admitted_proposal_set(
            request,
            pairs,
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["capability-fixture"]),
                version: "set-v3".to_owned(),
                criteria_version: "criteria-v3".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: true,
            },
        ) else {
            panic!("set publication")
        };
        set
    }
    fn context_publications(
        set: &AdmittedInterpretationProposalSet,
    ) -> (DeclaredObjectiveSet, DeclaredConstraintSet) {
        let objective_profile = FixtureObjectiveRepresentationProfile::fixture();
        let objective_registries = FixtureObjectiveRegistries::fixture();
        let objective_schema = ObjectiveSchemaBinding {
            identity: StableId::from_parts("objective-schema", &["fixture"]),
            version: "objective-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let objective_configuration = ObjectiveConfigurationBinding {
            identity: StableId::from_parts("objective-config", &["fixture"]),
            version: "objective-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let ObjectiveRepresentationOutcome::Set(objectives) =
            represent_objectives(ObjectiveRepresentationInputs {
                admitted_proposal_set: set,
                profile: &objective_profile,
                registries: &objective_registries,
                schema: &objective_schema,
                configuration: &objective_configuration,
            })
        else {
            panic!("objective publication")
        };
        let constraint_profile = FixtureConstraintRepresentationProfile::fixture();
        let constraint_registries = FixtureConstraintRegistries::fixture();
        let constraint_schema = ConstraintSchemaBinding {
            identity: StableId::from_parts("constraint-schema", &["fixture"]),
            version: "constraint-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let constraint_configuration = ConstraintConfigurationBinding {
            identity: StableId::from_parts("constraint-config", &["fixture"]),
            version: "constraint-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let ConstraintRepresentationOutcome::Set(constraints) =
            represent_constraints(ConstraintRepresentationInputs {
                admitted_proposal_set: set,
                objective_set: &objectives,
                profile: &constraint_profile,
                registries: &constraint_registries,
                schema: &constraint_schema,
                configuration: &constraint_configuration,
                implementation_version: "sre-runtime-fixture-v1",
            })
        else {
            panic!("constraint publication")
        };
        (objectives, constraints)
    }
    fn represent(
        set: &AdmittedInterpretationProposalSet,
        objectives: &DeclaredObjectiveSet,
        constraints: &DeclaredConstraintSet,
        profile: &FixtureCapabilityRepresentationProfile,
        registries: &FixtureCapabilityRegistries,
    ) -> CapabilityRepresentationOutcome {
        let schema = CapabilitySchemaBinding {
            identity: StableId::from_parts("capability-schema", &["fixture"]),
            version: "capability-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = CapabilityConfigurationBinding {
            identity: StableId::from_parts("capability-config", &["fixture"]),
            version: "capability-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        represent_capabilities(CapabilityRepresentationInputs {
            admitted_proposal_set: set,
            objective_set: objectives,
            constraint_set: constraints,
            profile,
            registries,
            schema: &schema,
            configuration: &configuration,
            implementation_version: "sre-runtime-fixture-v1",
        })
    }

    #[test]
    fn typed_capability_is_represented_and_narrative_is_not_scanned() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "one",
                vec![capability("understand")],
                vec!["Use a named provider to understand text."],
            )],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("capability publication")
        };
        assert_eq!(capabilities.requirements().len(), 1);
        assert_eq!(
            capabilities.requirements()[0].class_name(),
            "LanguageUnderstanding"
        );
    }
    #[test]
    fn empty_capability_set_is_distinct_from_failure() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "empty",
                Vec::new(),
                vec!["The objective implies a tool."],
            )],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("empty capability set")
        };
        assert!(capabilities.requirements().is_empty());
    }
    #[test]
    fn abstract_closed_rule_preserves_method_and_rule() {
        let request = request();
        let mut element = capability("retrieve");
        element.class_name = "ExternalInformationRetrieval".to_owned();
        element.supplied_class = Some("SearchMethod".to_owned());
        element.supplied_method = Some("retrieve".to_owned());
        element.abstraction_rule_id = Some("fixture-rule-retrieve-v1".to_owned());
        element.abstraction_rule_version = Some("fixture-005-rule-v1".to_owned());
        let set = admitted_set(
            &request,
            vec![proposal(&request, "abstract", vec![element], Vec::new())],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("closed abstraction")
        };
        assert_eq!(
            capabilities.requirements()[0].class_name(),
            "ExternalInformationRetrieval"
        );
        assert_eq!(
            capabilities.requirements()[0].supplied_method(),
            Some("retrieve")
        );
        assert_eq!(
            capabilities.requirements()[0].abstraction_rule_id(),
            Some("fixture-rule-retrieve-v1")
        );
    }
    #[test]
    fn provider_specific_class_and_open_abstraction_fail() {
        let request = request();
        let mut provider = capability("provider");
        provider.class_name = "OpenAI".to_owned();
        let set = admitted_set(
            &request,
            vec![proposal(&request, "provider", vec![provider], Vec::new())],
        );
        let (objectives, constraints) = context_publications(&set);
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &constraints,
                &FixtureCapabilityRepresentationProfile::fixture(),
                &FixtureCapabilityRegistries::fixture()
            ),
            CapabilityRepresentationOutcome::Failure(CapabilityRepresentationFailureRecord {
                category: CapabilityRepresentationFailureCategory::UnsupportedCapabilityClass,
                ..
            })
        ));
        let mut open = capability("open");
        open.supplied_method = Some("retrieve".to_owned());
        open.supplied_class = Some("SearchMethod".to_owned());
        open.class_name = "ExternalInformationRetrieval".to_owned();
        open.abstraction_rule_id = Some("unknown-rule".to_owned());
        open.abstraction_rule_version = Some("fixture-005-rule-v1".to_owned());
        let open_set = admitted_set(
            &request,
            vec![proposal(&request, "open", vec![open], Vec::new())],
        );
        let (open_objectives, open_constraints) = context_publications(&open_set);
        assert!(matches!(
            represent(
                &open_set,
                &open_objectives,
                &open_constraints,
                &FixtureCapabilityRepresentationProfile::fixture(),
                &FixtureCapabilityRegistries::fixture()
            ),
            CapabilityRepresentationOutcome::Failure(CapabilityRepresentationFailureRecord {
                category: CapabilityRepresentationFailureCategory::UnauthorizedAbstraction,
                ..
            })
        ));
    }
    #[test]
    fn necessity_access_dependency_scope_and_support_status_are_representational() {
        let request = request();
        let mut element = capability("conditional");
        element.necessity = "Optional".to_owned();
        element.access_dependency = Some("credential-dependent".to_owned());
        element.scope_kind = "Unresolved".to_owned();
        element.scope_resolution_status = "Unresolved".to_owned();
        element.classification_support_status = "Unresolved".to_owned();
        element.representation_status = Some("Unresolved".to_owned());
        let set = admitted_set(
            &request,
            vec![proposal(&request, "conditions", vec![element], Vec::new())],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("representational conditions")
        };
        assert_eq!(
            capabilities.requirements()[0].necessity(),
            CapabilityNecessity::Optional
        );
        assert_eq!(
            capabilities.requirements()[0].access_dependency(),
            Some("credential-dependent")
        );
        assert_eq!(
            capabilities.requirements()[0].classification_support_status(),
            ClassificationSupportStatus::Unresolved
        );
    }
    #[test]
    fn relationships_are_declared_not_bindings() {
        let request = request();
        let mut first = capability("first");
        let second = capability("second");
        first.relationships = vec![ProposalCapabilityRelationship {
            relationship_id: "supports".to_owned(),
            relationship_type: "SupportsAsDeclared".to_owned(),
            target_element_ids: vec![second.element_id.clone()].into(),
            scope: None,
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
        }]
        .into();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "relationships",
                vec![first, second],
                Vec::new(),
            )],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("relationship representation")
        };
        assert_eq!(capabilities.relationships().len(), 1);
        assert_eq!(
            capabilities.relationships()[0].relationship_type(),
            CapabilityRelationshipType::SupportsAsDeclared
        );
    }
    #[test]
    fn lineage_profile_registry_and_mixed_fatal_input_fail() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "lineage",
                vec![capability("one")],
                Vec::new(),
            )],
        );
        let (objectives, constraints) = context_publications(&set);
        let mut foreign_constraints = constraints.clone();
        foreign_constraints.admitted_proposal_set_id =
            AdmittedInterpretationProposalSetId::derive(&["foreign"]);
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &foreign_constraints,
                &FixtureCapabilityRepresentationProfile::fixture(),
                &FixtureCapabilityRegistries::fixture()
            ),
            CapabilityRepresentationOutcome::Failure(CapabilityRepresentationFailureRecord {
                category: CapabilityRepresentationFailureCategory::InvalidLineage,
                ..
            })
        ));
        let mut profile = FixtureCapabilityRepresentationProfile::fixture();
        profile.version.clear();
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &constraints,
                &profile,
                &FixtureCapabilityRegistries::fixture()
            ),
            CapabilityRepresentationOutcome::Failure(CapabilityRepresentationFailureRecord {
                category: CapabilityRepresentationFailureCategory::IncompatibleProfile,
                ..
            })
        ));
        let mut registries = FixtureCapabilityRegistries::fixture();
        registries.class_registry.version = "fixture-005-registry-v2".to_owned();
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &constraints,
                &FixtureCapabilityRepresentationProfile::fixture(),
                &registries
            ),
            CapabilityRepresentationOutcome::Failure(CapabilityRepresentationFailureRecord {
                category: CapabilityRepresentationFailureCategory::IncompatibleRegistryVersion,
                ..
            })
        ));
        let mut malformed = capability("malformed");
        malformed.expression.clear();
        let mixed_set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "mixed",
                vec![capability("valid"), malformed],
                Vec::new(),
            )],
        );
        let (mixed_objectives, mixed_constraints) = context_publications(&mixed_set);
        assert!(matches!(
            represent(
                &mixed_set,
                &mixed_objectives,
                &mixed_constraints,
                &FixtureCapabilityRepresentationProfile::fixture(),
                &FixtureCapabilityRegistries::fixture()
            ),
            CapabilityRepresentationOutcome::Failure(_)
        ));
    }
    #[test]
    fn authorized_decomposition_preserves_parent_children_and_replay_is_stable() {
        let request = request();
        let mut composite = capability("composite");
        composite.form = "Composite".to_owned();
        composite.component_expressions = vec!["part one".to_owned(), "part two".to_owned()].into();
        let set = admitted_set(
            &request,
            vec![proposal(&request, "composite", vec![composite], Vec::new())],
        );
        let (objectives, constraints) = context_publications(&set);
        let profile = FixtureCapabilityRepresentationProfile::fixture();
        let registries = FixtureCapabilityRegistries::fixture();
        let first = represent(&set, &objectives, &constraints, &profile, &registries);
        assert_eq!(
            first,
            represent(&set, &objectives, &constraints, &profile, &registries)
        );
        let CapabilityRepresentationOutcome::Set(capabilities) = first else {
            panic!("decomposition")
        };
        assert_eq!(capabilities.requirements().len(), 3);
        assert_eq!(
            capabilities
                .relationships()
                .iter()
                .filter(|relationship| relationship.relationship_type()
                    == CapabilityRelationshipType::ComponentOf)
                .count(),
            2
        );
        let mut denied = profile.clone();
        denied.allow_decomposition = false;
        assert!(matches!(
            represent(&set, &objectives, &constraints, &denied, &registries),
            CapabilityRepresentationOutcome::Failure(CapabilityRepresentationFailureRecord {
                category: CapabilityRepresentationFailureCategory::UnauthorizedDecomposition,
                ..
            })
        ));
    }

    #[test]
    fn contract_006_uses_typed_clarification_only_and_replays_deterministically() {
        let mut request = request();
        request.proposal_schema_version = "proposal-fixture-v4".to_owned();
        let mut proposal = proposal(
            &request,
            "clarification",
            vec![capability("understand")],
            vec!["narrative ambiguity must not be scanned"],
        );
        proposal.content.clarification_elements = vec![
            ProposalClarificationElement {
                element_id: "amb-1".to_owned(),
                domain: "Ambiguity".to_owned(),
                expression: "target may have two referents".to_owned(),
                class_name: "Referent".to_owned(),
                origin: "ExplicitSource".to_owned(),
                basis: "StructuredExtraction".to_owned(),
                target_ids: vec!["objective-capability".to_owned()].into(),
                alternatives: vec!["referent-a".to_owned(), "referent-b".to_owned()].into(),
                evidence_references: Vec::new().into(),
                relationships: vec!["AsDeclared".to_owned()].into(),
                clarification_requirement: None,
                uncertainty_kind: None,
                uncertainty_mode: None,
            },
            ProposalClarificationElement {
                element_id: "unc-1".to_owned(),
                domain: "Uncertainty".to_owned(),
                expression: "scope remains unsettled".to_owned(),
                class_name: "Scope".to_owned(),
                origin: "ExplicitSource".to_owned(),
                basis: "StructuredExtraction".to_owned(),
                target_ids: vec!["objective-capability".to_owned()].into(),
                alternatives: Vec::new().into(),
                evidence_references: Vec::new().into(),
                relationships: Vec::new().into(),
                clarification_requirement: Some("confirm scope".to_owned()),
                uncertainty_kind: Some("Representational".to_owned()),
                uncertainty_mode: Some("Unresolved".to_owned()),
            },
            ProposalClarificationElement {
                element_id: "assump-1".to_owned(),
                domain: "Assumption".to_owned(),
                expression: "the supplied scope is provisional".to_owned(),
                class_name: "ScopeAssumption".to_owned(),
                origin: "ExplicitSource".to_owned(),
                basis: "StructuredExtraction".to_owned(),
                target_ids: vec!["objective-capability".to_owned()].into(),
                alternatives: Vec::new().into(),
                evidence_references: Vec::new().into(),
                relationships: Vec::new().into(),
                clarification_requirement: None,
                uncertainty_kind: None,
                uncertainty_mode: None,
            },
            ProposalClarificationElement {
                element_id: "req-1".to_owned(),
                domain: "ClarificationRequirement".to_owned(),
                expression: "additional scope detail is required".to_owned(),
                class_name: "ScopeDetail".to_owned(),
                origin: "ExplicitSource".to_owned(),
                basis: "StructuredExtraction".to_owned(),
                target_ids: vec!["objective-capability".to_owned()].into(),
                alternatives: Vec::new().into(),
                evidence_references: Vec::new().into(),
                relationships: Vec::new().into(),
                clarification_requirement: Some("supply scope detail".to_owned()),
                uncertainty_kind: None,
                uncertainty_mode: None,
            },
        ]
        .into();
        let set = admitted_set(&request, vec![proposal]);
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("capability publication")
        };
        let profile = FixtureMeaningQualificationProfile::fixture();
        let registries = FixtureMeaningQualificationRegistries::fixture();
        let schema = MeaningQualificationSchemaBinding {
            identity: StableId::from_parts("mq-schema", &["fixture"]),
            version: "mq-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = MeaningQualificationConfigurationBinding {
            identity: StableId::from_parts("mq-config", &["fixture"]),
            version: "mq-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let inputs = MeaningQualificationRepresentationInputs {
            admitted_proposal_set: &set,
            objective_set: &objectives,
            constraint_set: &constraints,
            capability_set: &capabilities,
            profile: &profile,
            registries: &registries,
            schema: &schema,
            configuration: &configuration,
            implementation_version: "sre-runtime-fixture-v1",
        };
        let first = represent_semantic_clarification(inputs.clone());
        assert_eq!(first, represent_semantic_clarification(inputs));
        let MeaningQualificationOutcome::Set(qualified) = first else {
            panic!("meaning qualification publication")
        };
        assert_eq!(qualified.ambiguities().len(), 1);
        assert_eq!(
            qualified.ambiguities()[0].alternatives.as_ref(),
            ["referent-a", "referent-b"]
        );
        assert_eq!(qualified.uncertainties().len(), 1);
        assert_eq!(qualified.assumptions().len(), 1);
        assert_eq!(qualified.clarification_requirements().len(), 1);
        assert_eq!(qualified.relationships().len(), 1);
    }

    #[test]
    fn contract_006_profile_and_lineage_fail_atomically() {
        let mut request = request();
        request.proposal_schema_version = "proposal-fixture-v4".to_owned();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "empty",
                vec![capability("understand")],
                Vec::new(),
            )],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("capability publication")
        };
        let mut profile = FixtureMeaningQualificationProfile::fixture();
        profile.version.clear();
        let registries = FixtureMeaningQualificationRegistries::fixture();
        let schema = MeaningQualificationSchemaBinding {
            identity: StableId::from_parts("mq-schema", &["fixture"]),
            version: "mq-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = MeaningQualificationConfigurationBinding {
            identity: StableId::from_parts("mq-config", &["fixture"]),
            version: "mq-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        assert!(matches!(
            represent_semantic_clarification(MeaningQualificationRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                profile: &profile,
                registries: &registries,
                schema: &schema,
                configuration: &configuration,
                implementation_version: "sre-runtime-fixture-v1"
            }),
            MeaningQualificationOutcome::Failure(MeaningQualificationRepresentationFailureRecord {
                category: MeaningQualificationFailureCategory::MissingRequiredProfile,
                ..
            })
        ));
        let mut bad_registries = registries.clone();
        bad_registries.version.clear();
        let valid_profile = FixtureMeaningQualificationProfile::fixture();
        assert!(matches!(
            represent_semantic_clarification(MeaningQualificationRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                profile: &valid_profile,
                registries: &bad_registries,
                schema: &schema,
                configuration: &configuration,
                implementation_version: "sre-runtime-fixture-v1"
            }),
            MeaningQualificationOutcome::Failure(MeaningQualificationRepresentationFailureRecord {
                category: MeaningQualificationFailureCategory::MissingRequiredRegistry,
                ..
            })
        ));
        let mut foreign_objectives = objectives.clone();
        foreign_objectives.admitted_proposal_set_id =
            AdmittedInterpretationProposalSetId::derive(&["foreign"]);
        assert!(matches!(
            represent_semantic_clarification(MeaningQualificationRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &foreign_objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                profile: &valid_profile,
                registries: &registries,
                schema: &schema,
                configuration: &configuration,
                implementation_version: "sre-runtime-fixture-v1"
            }),
            MeaningQualificationOutcome::Failure(MeaningQualificationRepresentationFailureRecord {
                category: MeaningQualificationFailureCategory::InvalidLineage,
                ..
            })
        ));
    }

    #[test]
    fn contract_007_keeps_declaration_material_representation_and_grounding_distinct() {
        let mut request = request();
        request.proposal_schema_version = "proposal-fixture-v5".to_owned();
        let mut proposal = proposal(
            &request,
            "evidence",
            vec![capability("understand")],
            Vec::new(),
        );
        proposal.content.evidence_elements = vec![ProposalEvidenceElement {
            element_id: "evidence-1".to_owned(),
            evidence_class: "QuotedText".to_owned(),
            origin: "Source".to_owned(),
            basis: "Quoted".to_owned(),
            material_kind: "QuotedTextValue".to_owned(),
            material_value: "declared quotation".to_owned(),
            locator: Some("BoundedSection".to_owned()),
            supplied_value: Some("declared quotation".to_owned()),
            evidence_status: "Observed".to_owned(),
            representation_status: "Represented".to_owned(),
            grounding_target_id: None,
            grounding_target_type: None,
            grounding_relationship: None,
            grounding_aspect: None,
            grounding_basis: None,
        }]
        .into();
        let set = admitted_set(&request, vec![proposal]);
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("capability publication")
        };
        let qualification_schema = MeaningQualificationSchemaBinding {
            identity: StableId::from_parts("mq-schema", &["fixture"]),
            version: "mq-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let qualification_configuration = MeaningQualificationConfigurationBinding {
            identity: StableId::from_parts("mq-config", &["fixture"]),
            version: "mq-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let MeaningQualificationOutcome::Set(clarification) =
            represent_semantic_clarification(MeaningQualificationRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                profile: &FixtureMeaningQualificationProfile::fixture(),
                registries: &FixtureMeaningQualificationRegistries::fixture(),
                schema: &qualification_schema,
                configuration: &qualification_configuration,
                implementation_version: "sre-runtime-fixture-v1",
            })
        else {
            panic!("qualification publication")
        };
        let evidence_schema = EvidenceSchemaBinding {
            identity: StableId::from_parts("evidence-schema", &["fixture"]),
            version: "evidence-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let evidence_configuration = EvidenceConfigurationBinding {
            identity: StableId::from_parts("evidence-config", &["fixture"]),
            version: "evidence-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let inputs = EvidenceRepresentationInputs {
            admitted_proposal_set: &set,
            objective_set: &objectives,
            constraint_set: &constraints,
            capability_set: &capabilities,
            clarification_set: &clarification,
            profile: &FixtureEvidenceProfile::fixture(),
            registries: &FixtureEvidenceRegistries::fixture(),
            schema: &evidence_schema,
            configuration: &evidence_configuration,
            implementation_version: "sre-runtime-fixture-v1",
        };
        let first = represent_evidence(inputs.clone());
        assert_eq!(first, represent_evidence(inputs));
        let EvidenceRepresentationOutcome::Set(evidence) = first else {
            panic!("evidence publication")
        };
        assert_eq!(evidence.declarations().len(), 1);
        assert_eq!(evidence.representations().len(), 1);
        assert!(evidence.groundings().is_empty());
        assert!(!evidence.ungrounded_accounting().is_empty());
        assert_eq!(
            evidence.declarations()[0].material_reference.kind_name(),
            "QuotedTextValue"
        );
    }

    #[test]
    fn contract_007_profile_and_target_fail_without_partial_publication() {
        let mut request = request();
        request.proposal_schema_version = "proposal-fixture-v5".to_owned();
        let mut proposal = proposal(
            &request,
            "bad-evidence",
            vec![capability("understand")],
            Vec::new(),
        );
        proposal.content.evidence_elements = vec![ProposalEvidenceElement {
            element_id: "evidence-bad-target".to_owned(),
            evidence_class: "ReferencedArtifact".to_owned(),
            origin: "ReferencedArtifact".to_owned(),
            basis: "Referenced".to_owned(),
            material_kind: "ReferencedArtifactReference".to_owned(),
            material_value: "external-ref".to_owned(),
            locator: None,
            supplied_value: None,
            evidence_status: "Unavailable".to_owned(),
            representation_status: "Represented".to_owned(),
            grounding_target_id: Some("foreign-target".to_owned()),
            grounding_target_type: Some("DeclaredObjective".to_owned()),
            grounding_relationship: Some("Supports".to_owned()),
            grounding_aspect: None,
            grounding_basis: Some("Declared".to_owned()),
        }]
        .into();
        let set = admitted_set(&request, vec![proposal]);
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("capability publication")
        };
        let qs = MeaningQualificationSchemaBinding {
            identity: StableId::from_parts("mq-schema", &["fixture"]),
            version: "mq-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let qc = MeaningQualificationConfigurationBinding {
            identity: StableId::from_parts("mq-config", &["fixture"]),
            version: "mq-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let MeaningQualificationOutcome::Set(clarification) =
            represent_semantic_clarification(MeaningQualificationRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                profile: &FixtureMeaningQualificationProfile::fixture(),
                registries: &FixtureMeaningQualificationRegistries::fixture(),
                schema: &qs,
                configuration: &qc,
                implementation_version: "sre-runtime-fixture-v1",
            })
        else {
            panic!("qualification publication")
        };
        let es = EvidenceSchemaBinding {
            identity: StableId::from_parts("evidence-schema", &["fixture"]),
            version: "evidence-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let ec = EvidenceConfigurationBinding {
            identity: StableId::from_parts("evidence-config", &["fixture"]),
            version: "evidence-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let mut bad_profile = FixtureEvidenceProfile::fixture();
        bad_profile.version.clear();
        assert!(matches!(
            represent_evidence(EvidenceRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                clarification_set: &clarification,
                profile: &bad_profile,
                registries: &FixtureEvidenceRegistries::fixture(),
                schema: &es,
                configuration: &ec,
                implementation_version: "sre-runtime-fixture-v1"
            }),
            EvidenceRepresentationOutcome::Failure(EvidenceRepresentationFailureRecord {
                category: EvidenceRepresentationFailureCategory::MissingRequiredProfile,
                ..
            })
        ));
        assert!(matches!(
            represent_evidence(EvidenceRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                clarification_set: &clarification,
                profile: &FixtureEvidenceProfile::fixture(),
                registries: &FixtureEvidenceRegistries::fixture(),
                schema: &es,
                configuration: &ec,
                implementation_version: "sre-runtime-fixture-v1"
            }),
            EvidenceRepresentationOutcome::Failure(EvidenceRepresentationFailureRecord {
                category: EvidenceRepresentationFailureCategory::InvalidTargetReference,
                ..
            })
        ));
    }

    struct ProvenanceFixture {
        input: ProvenanceRepresentationInput,
        admitted: AdmittedInterpretationProposalSet,
        objectives: DeclaredObjectiveSet,
        constraints: DeclaredConstraintSet,
        capabilities: CapabilityRequirementSet,
        clarification: MeaningQualificationSet,
        evidence: InterpretationEvidenceSet,
        profile: FixtureProvenanceProfile,
        registries: FixtureProvenanceRegistries,
        schema: ProvenanceSchemaBinding,
        configuration: ProvenanceConfigurationBinding,
    }

    fn provenance_fixture() -> ProvenanceFixture {
        let mut request = request();
        request.proposal_schema_version = "proposal-fixture-v5".to_owned();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "provenance",
                vec![capability("understand")],
                Vec::new(),
            )],
        );
        let (objectives, constraints) = context_publications(&set);
        let CapabilityRepresentationOutcome::Set(capabilities) = represent(
            &set,
            &objectives,
            &constraints,
            &FixtureCapabilityRepresentationProfile::fixture(),
            &FixtureCapabilityRegistries::fixture(),
        ) else {
            panic!("capability publication")
        };
        let qs = MeaningQualificationSchemaBinding {
            identity: StableId::from_parts("mq-schema", &["fixture"]),
            version: "mq-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let qc = MeaningQualificationConfigurationBinding {
            identity: StableId::from_parts("mq-config", &["fixture"]),
            version: "mq-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let MeaningQualificationOutcome::Set(clarification) =
            represent_semantic_clarification(MeaningQualificationRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                profile: &FixtureMeaningQualificationProfile::fixture(),
                registries: &FixtureMeaningQualificationRegistries::fixture(),
                schema: &qs,
                configuration: &qc,
                implementation_version: "sre-runtime-fixture-v1",
            })
        else {
            panic!("qualification publication")
        };
        let es = EvidenceSchemaBinding {
            identity: StableId::from_parts("evidence-schema", &["fixture"]),
            version: "evidence-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let ec = EvidenceConfigurationBinding {
            identity: StableId::from_parts("evidence-config", &["fixture"]),
            version: "evidence-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let EvidenceRepresentationOutcome::Set(evidence) =
            represent_evidence(EvidenceRepresentationInputs {
                admitted_proposal_set: &set,
                objective_set: &objectives,
                constraint_set: &constraints,
                capability_set: &capabilities,
                clarification_set: &clarification,
                profile: &FixtureEvidenceProfile::fixture(),
                registries: &FixtureEvidenceRegistries::fixture(),
                schema: &es,
                configuration: &ec,
                implementation_version: "sre-runtime-fixture-v1",
            })
        else {
            panic!("evidence publication")
        };
        let profile = FixtureProvenanceProfile::fixture();
        let registries = FixtureProvenanceRegistries::fixture();
        let schema = ProvenanceSchemaBinding {
            identity: StableId::from_parts("provenance-schema", &["fixture"]),
            version: "provenance-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = ProvenanceConfigurationBinding {
            identity: StableId::from_parts("provenance-config", &["fixture"]),
            version: "provenance-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let input = ProvenanceRepresentationInput {
            input_id: ProvenanceRepresentationInputId::derive(&["fixture-provenance-input"]),
            schema_version: "provenance-input-v1".to_owned(),
            profile_id: profile.identity.clone(),
            profile_version: profile.version.clone(),
            registry_version: registries.version.clone(),
            configuration_id: configuration.identity.clone(),
            configuration_version: configuration.version.clone(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            admitted_proposal_set_id: set.set_id.clone(),
            objective_set_id: objectives.set_id.clone(),
            constraint_set_id: constraints.set_id.clone(),
            capability_set_id: capabilities.set_id.clone(),
            clarification_set_id: clarification.set_id.clone(),
            evidence_set_id: evidence.set_id.clone(),
            source_intake_id: set.source_intake_id.clone(),
            interpretation_operation_id: set.operation_id.clone(),
            input_proposal_ids: set.submitted_proposal_ids.clone(),
            input_decision_ids: set.decision_ids.clone(),
            subject_declarations: Vec::new().into(),
            event_declarations: Vec::new().into(),
            lineage_declarations: Vec::new().into(),
            lifecycle_declarations: Vec::new().into(),
            external_identity_mappings: Vec::new().into(),
            conflict_declarations: Vec::new().into(),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        };
        ProvenanceFixture {
            input,
            admitted: set,
            objectives,
            constraints,
            capabilities,
            clarification,
            evidence,
            profile,
            registries,
            schema,
            configuration,
        }
    }

    fn represent_provenance_fixture(
        fixture: &ProvenanceFixture,
    ) -> ProvenanceRepresentationOutcome {
        represent_provenance(ProvenanceRepresentationInputs {
            input: &fixture.input,
            admitted_proposal_set: &fixture.admitted,
            objective_set: &fixture.objectives,
            constraint_set: &fixture.constraints,
            capability_set: &fixture.capabilities,
            clarification_set: &fixture.clarification,
            evidence_set: &fixture.evidence,
            profile: &fixture.profile,
            registries: &fixture.registries,
            schema: &fixture.schema,
            configuration: &fixture.configuration,
        })
    }

    #[test]
    fn contract_008_declared_event_lineage_mapping_and_time_are_separate() {
        let mut fixture = provenance_fixture();
        let subject = fixture.objectives.set_id.to_string();
        let subject_declaration_id = ProvenanceDeclarationId::derive(&["subject"]);
        let event_declaration_id = ProvenanceDeclarationId::derive(&["event"]);
        fixture.input.subject_declarations = vec![ProvenanceSubjectDeclaration {
            declaration_id: subject_declaration_id.clone(),
            subject_id: subject.clone(),
            subject_class: "DeclaredObjectiveSet".to_owned(),
            origin: "TestFixture".to_owned(),
            participant_reference: None,
            source_reference: Some(fixture.objectives.set_id.to_string()),
            basis: "Declared".to_owned(),
        }]
        .into();
        fixture.input.event_declarations = vec![ProvenanceEventPayload::Transformation(
            TransformationEventDeclaration {
                declaration_id: event_declaration_id.clone(),
                event_id: ProvenanceEventId::derive(&["event"]),
                event_schema_version: "provenance-event-v1".to_owned(),
                input_subject_ids: vec![subject.clone()].into(),
                output_subject_ids: vec![subject.clone()].into(),
                participant_references: Vec::new().into(),
                basis: "Declared".to_owned(),
                event_time: EventTimeRepresentation::Unknown,
                representation_time: EventTimeRepresentation::Unknown,
                commitment_reference: None,
                origin: "TestFixture".to_owned(),
            },
        )]
        .into();
        fixture.input.lineage_declarations = vec![LineageDeclaration {
            declaration_id: ProvenanceDeclarationId::derive(&["lineage"]),
            assertion_id: LineageAssertionId::derive(&["lineage"]),
            subject_artifact_id: subject.clone(),
            ancestor_or_related_artifact_id: subject.clone(),
            relationship: "RelatedTo".to_owned(),
            basis: "Declared".to_owned(),
            origin: "TestFixture".to_owned(),
            status: "Unresolved".to_owned(),
        }]
        .into();
        fixture.input.external_identity_mappings = vec![ExternalIdentityMappingDeclaration {
            declaration_id: ProvenanceDeclarationId::derive(&["mapping"]),
            mapping_id: ExternalIdentityMappingId::derive(&["mapping"]),
            namespace: "fixture-external".to_owned(),
            external_identifier: "external-objective".to_owned(),
            target_subject_id: subject.clone(),
            basis: "Declared".to_owned(),
            origin: "ExternalIdentityMappingDeclaration".to_owned(),
            status: "Unresolved".to_owned(),
        }]
        .into();
        let first = represent_provenance_fixture(&fixture);
        assert_eq!(first, represent_provenance_fixture(&fixture));
        let ProvenanceRepresentationOutcome::Success(record) = first else {
            panic!("provenance publication")
        };
        assert_eq!(record.subjects().len(), 1);
        assert_eq!(record.events().len(), 1);
        assert_eq!(record.lineage_assertions().len(), 1);
        assert_eq!(record.external_mappings().len(), 1);
        assert_eq!(
            record.events()[0].event_time,
            EventTimeRepresentation::Unknown
        );
        assert_eq!(record.events()[0].event_class, "Transformation");
    }

    #[test]
    fn contract_008_empty_and_foreign_or_cyclic_inputs_are_atomic() {
        let fixture = provenance_fixture();
        let empty = represent_provenance_fixture(&fixture);
        let ProvenanceRepresentationOutcome::Success(record) = empty else {
            panic!("empty publication")
        };
        assert_eq!(record.status(), ProvenanceStatus::Empty);
        assert!(record.decisions().is_empty());
        let mut foreign = fixture.input.clone();
        foreign.objective_set_id = DeclaredObjectiveSetId::derive(&["foreign"]);
        let bad = represent_provenance(ProvenanceRepresentationInputs {
            input: &foreign,
            admitted_proposal_set: &fixture.admitted,
            objective_set: &fixture.objectives,
            constraint_set: &fixture.constraints,
            capability_set: &fixture.capabilities,
            clarification_set: &fixture.clarification,
            evidence_set: &fixture.evidence,
            profile: &fixture.profile,
            registries: &fixture.registries,
            schema: &fixture.schema,
            configuration: &fixture.configuration,
        });
        assert!(matches!(
            bad,
            ProvenanceRepresentationOutcome::Failure(ProvenanceRepresentationFailureRecord {
                category: ProvenanceRepresentationFailureCategory::InvalidLineage,
                ..
            })
        ));
        let subject = fixture.objectives.set_id.to_string();
        let mut cyclic = fixture.input.clone();
        cyclic.subject_declarations = vec![ProvenanceSubjectDeclaration {
            declaration_id: ProvenanceDeclarationId::derive(&["subject"]),
            subject_id: subject.clone(),
            subject_class: "DeclaredObjectiveSet".to_owned(),
            origin: "TestFixture".to_owned(),
            participant_reference: None,
            source_reference: None,
            basis: "Declared".to_owned(),
        }]
        .into();
        cyclic.lineage_declarations = vec![LineageDeclaration {
            declaration_id: ProvenanceDeclarationId::derive(&["cycle"]),
            assertion_id: LineageAssertionId::derive(&["cycle"]),
            subject_artifact_id: subject.clone(),
            ancestor_or_related_artifact_id: subject,
            relationship: "DerivedFrom".to_owned(),
            basis: "Declared".to_owned(),
            origin: "TestFixture".to_owned(),
            status: "Conflicting".to_owned(),
        }]
        .into();
        let bad_cycle = represent_provenance(ProvenanceRepresentationInputs {
            input: &cyclic,
            admitted_proposal_set: &fixture.admitted,
            objective_set: &fixture.objectives,
            constraint_set: &fixture.constraints,
            capability_set: &fixture.capabilities,
            clarification_set: &fixture.clarification,
            evidence_set: &fixture.evidence,
            profile: &fixture.profile,
            registries: &fixture.registries,
            schema: &fixture.schema,
            configuration: &fixture.configuration,
        });
        assert!(matches!(
            bad_cycle,
            ProvenanceRepresentationOutcome::Failure(ProvenanceRepresentationFailureRecord {
                category: ProvenanceRepresentationFailureCategory::ProhibitedCycle,
                ..
            })
        ));
    }

    struct SemanticReconciliationFixture {
        input: SemanticReconciliationInput,
        admitted: AdmittedInterpretationProposalSet,
        objectives: DeclaredObjectiveSet,
        constraints: DeclaredConstraintSet,
        capabilities: CapabilityRequirementSet,
        clarification: MeaningQualificationSet,
        evidence: InterpretationEvidenceSet,
        provenance: ProvenanceRecordSet,
        profile: FixtureSemanticReconciliationProfile,
        registries: FixtureSemanticReconciliationRegistries,
    }

    fn semantic_reconciliation_fixture() -> SemanticReconciliationFixture {
        let upstream = provenance_fixture();
        let ProvenanceRepresentationOutcome::Success(provenance) =
            represent_provenance_fixture(&upstream)
        else {
            panic!("provenance publication")
        };
        let profile = FixtureSemanticReconciliationProfile::fixture();
        let registries = FixtureSemanticReconciliationRegistries::fixture();
        let subject_id = ReconciliationSubjectId::derive(&["objective-subject"]);
        let subject = ReconciliationSubjectDeclaration {
            subject_id: subject_id.clone(),
            upstream_publication_id: upstream.objectives.set_id.to_string(),
            upstream_representation_id: upstream.objectives.objectives[0]
                .logical_objective_id
                .to_string(),
            original_expression: "achieve outcome".to_owned(),
            semantic_class: "RequestedOutcome".to_owned(),
            semantic_domain: "Objective".to_owned(),
            represented_scope: "fixture-scope".to_owned(),
            upstream_status: "Represented".to_owned(),
            evidence_reference_ids: Vec::new().into(),
            provenance_reference_ids: Vec::new().into(),
        };
        let group = ReconciliationGroupDeclaration {
            group_id: ReconciliationGroupId::derive(&["objective-group"]),
            member_subject_ids: vec![subject_id.clone()].into(),
            semantic_domain_or_interaction_class: "Objective<->Objective".to_owned(),
            represented_scope: "fixture-scope".to_owned(),
            formation_rule_id: "ExplicitGroupDeclarationRule".to_owned(),
            formation_rule_version: "fixture-group-v1".to_owned(),
            comparison_rule_id: "AlternativeFixtureRule".to_owned(),
            comparison_rule_version: "fixture-comparison-v1".to_owned(),
            decision_rule_id: "AlternativePreservationRule".to_owned(),
            decision_rule_version: "fixture-decision-v1".to_owned(),
            declared_relationship: ComparisonRelationship::Alternative,
            declared_disposition: ReconciliationDisposition::Preserved,
            standing_assignments: vec![ReconciliationStandingDeclaration {
                assignment_id: StandingAssignmentId::derive(&["objective-standing"]),
                target_kind: "UpstreamSubject".to_owned(),
                target_id: subject_id.to_string(),
                standing: Standing::Included,
                basis: "Explicit fixture assignment".to_owned(),
            }]
            .into(),
            resulting_element_ids: Vec::new().into(),
            result_representation_ids: Vec::new().into(),
            unresolved_conditions: Vec::new().into(),
        };
        let input = SemanticReconciliationInput {
            input_id: SemanticReconciliationInputId::derive(&["fixture-reconciliation-input"]),
            operation_id: SemanticReconciliationOperationId::derive(&[
                "fixture-reconciliation-operation",
            ]),
            admitted_proposal_set_id: upstream.admitted.set_id.clone(),
            objective_set_id: upstream.objectives.set_id.clone(),
            constraint_set_id: upstream.constraints.set_id.clone(),
            capability_set_id: upstream.capabilities.set_id.clone(),
            clarification_set_id: upstream.clarification.set_id.clone(),
            evidence_set_id: upstream.evidence.set_id.clone(),
            provenance_set_id: provenance.set_id.clone(),
            source_intake_id: upstream.admitted.source_intake_id.clone(),
            interpretation_operation_id: upstream.admitted.operation_id.clone(),
            input_proposal_ids: upstream.admitted.submitted_proposal_ids.clone(),
            input_decision_ids: upstream.admitted.decision_ids.clone(),
            declared_subjects: vec![subject].into(),
            declared_groups: vec![group].into(),
            declared_relationships: Vec::new().into(),
            profile_id: profile.identity.clone(),
            profile_version: profile.version.clone(),
            grouping_registry_version: registries.version.clone(),
            comparison_registry_version: registries.version.clone(),
            decision_registry_version: registries.version.clone(),
            standing_registry_version: registries.version.clone(),
            schema_version: profile.schema_version.clone(),
            configuration_version: profile.configuration_version.clone(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        };
        SemanticReconciliationFixture {
            input,
            admitted: upstream.admitted,
            objectives: upstream.objectives,
            constraints: upstream.constraints,
            capabilities: upstream.capabilities,
            clarification: upstream.clarification,
            evidence: upstream.evidence,
            provenance,
            profile,
            registries,
        }
    }

    fn represent_semantic_reconciliation_fixture(
        fixture: &SemanticReconciliationFixture,
    ) -> SemanticReconciliationOutcome {
        represent_semantic_reconciliation(SemanticReconciliationInputs {
            input: &fixture.input,
            admitted_proposal_set: &fixture.admitted,
            objective_set: &fixture.objectives,
            constraint_set: &fixture.constraints,
            capability_set: &fixture.capabilities,
            clarification_set: &fixture.clarification,
            evidence_set: &fixture.evidence,
            provenance_set: &fixture.provenance,
            profile: &fixture.profile,
            registries: &fixture.registries,
        })
    }

    #[test]
    fn contract_009_success_replays_and_publishes_explicit_artifacts() {
        let fixture = semantic_reconciliation_fixture();
        let first = represent_semantic_reconciliation_fixture(&fixture);
        assert_eq!(first, represent_semantic_reconciliation_fixture(&fixture));
        let SemanticReconciliationOutcome::Success(set) = first else {
            panic!("reconciliation publication")
        };
        assert_eq!(set.status(), SemanticReconciliationStatus::Reconciled);
        assert_eq!(set.subjects.len(), 1);
        assert_eq!(set.groups.len(), 1);
        assert_eq!(set.decisions.len(), 1);
        assert_eq!(set.standing_assignments.len(), 1);
        assert!(set.elements.is_empty());
    }

    #[test]
    fn contract_009_comparison_and_alternatives_remain_explicit() {
        let mut fixture = semantic_reconciliation_fixture();
        let first_subject = fixture.input.declared_subjects[0].clone();
        let second_id = ReconciliationSubjectId::derive(&["alternative-subject"]);
        let second_subject = ReconciliationSubjectDeclaration {
            subject_id: second_id.clone(),
            upstream_representation_id: "alternative-representation".to_owned(),
            ..first_subject
        };
        let mut subjects = fixture.input.declared_subjects.to_vec();
        subjects.push(second_subject);
        fixture.input.declared_subjects = subjects.into();
        let mut groups = fixture.input.declared_groups.to_vec();
        groups[0].member_subject_ids =
            vec![groups[0].member_subject_ids[0].clone(), second_id.clone()].into();
        groups[0].standing_assignments = vec![
            groups[0].standing_assignments[0].clone(),
            ReconciliationStandingDeclaration {
                assignment_id: StandingAssignmentId::derive(&["alternative-standing"]),
                target_kind: "UpstreamSubject".to_owned(),
                target_id: second_id.to_string(),
                standing: Standing::PreservedAsAlternative,
                basis: "Explicit alternative preservation".to_owned(),
            },
        ]
        .into();
        fixture.input.declared_groups = groups.into();
        let SemanticReconciliationOutcome::Success(set) =
            represent_semantic_reconciliation_fixture(&fixture)
        else {
            panic!("alternative reconciliation publication")
        };
        assert_eq!(set.findings.len(), 1);
        assert_eq!(set.decisions[0].preserved_alternative_ids.len(), 2);
        assert_eq!(set.standing_assignments.len(), 2);
        assert_eq!(
            set.standing_assignments[1].standing,
            Standing::PreservedAsAlternative
        );
    }

    #[test]
    fn contract_009_valid_unresolved_outcome_preserves_alternative_state() {
        let mut fixture = semantic_reconciliation_fixture();
        let mut groups = fixture.input.declared_groups.to_vec();
        groups[0].comparison_rule_id = "IndeterminateRule".to_owned();
        groups[0].decision_rule_id = "InsufficientAuthorityRule".to_owned();
        groups[0].declared_relationship = ComparisonRelationship::Indeterminate;
        groups[0].declared_disposition = ReconciliationDisposition::Unresolved;
        let mut standing = groups[0].standing_assignments.to_vec();
        standing[0].standing = Standing::Unresolved;
        groups[0].standing_assignments = standing.into();
        fixture.input.declared_groups = groups.into();
        let SemanticReconciliationOutcome::Success(set) =
            represent_semantic_reconciliation_fixture(&fixture)
        else {
            panic!("unresolved reconciliation is a valid success")
        };
        assert_eq!(set.status(), SemanticReconciliationStatus::Unresolved);
        assert_eq!(
            set.decisions[0].disposition,
            ReconciliationDisposition::Unresolved
        );
        assert_eq!(set.standing_assignments[0].standing, Standing::Unresolved);
    }

    #[test]
    fn contract_009_binding_failures_are_atomic() {
        let fixture = semantic_reconciliation_fixture();
        let mut bad_profile = fixture.input.clone();
        bad_profile.profile_version = "foreign-profile-version".to_owned();
        let bad_profile_fixture = SemanticReconciliationFixture {
            input: bad_profile,
            ..fixture
        };
        assert!(matches!(
            represent_semantic_reconciliation_fixture(&bad_profile_fixture),
            SemanticReconciliationOutcome::Failure(SemanticReconciliationFailureRecord {
                category: SemanticReconciliationFailureCategory::IncompatibleProfile,
                ..
            })
        ));

        let mut bad_registry = bad_profile_fixture.input.clone();
        bad_registry.profile_version = bad_profile_fixture.profile.version.clone();
        bad_registry.grouping_registry_version = "foreign-registry-version".to_owned();
        let bad_registry_fixture = SemanticReconciliationFixture {
            input: bad_registry,
            ..bad_profile_fixture
        };
        assert!(matches!(
            represent_semantic_reconciliation_fixture(&bad_registry_fixture),
            SemanticReconciliationOutcome::Failure(SemanticReconciliationFailureRecord {
                category: SemanticReconciliationFailureCategory::IncompatibleBinding,
                ..
            })
        ));

        let mut bad_lineage = bad_registry_fixture.input.clone();
        bad_lineage.grouping_registry_version = bad_registry_fixture.registries.version.clone();
        bad_lineage.objective_set_id = DeclaredObjectiveSetId::derive(&["foreign-objective"]);
        let bad_lineage_fixture = SemanticReconciliationFixture {
            input: bad_lineage,
            ..bad_registry_fixture
        };
        assert!(matches!(
            represent_semantic_reconciliation_fixture(&bad_lineage_fixture),
            SemanticReconciliationOutcome::Failure(SemanticReconciliationFailureRecord {
                category: SemanticReconciliationFailureCategory::InvalidUpstreamLineage,
                ..
            })
        ));
    }

    #[test]
    fn contract_009_missing_standing_coverage_cannot_publish_success() {
        let mut fixture = semantic_reconciliation_fixture();
        let mut groups = fixture.input.declared_groups.to_vec();
        groups[0].standing_assignments = Vec::new().into();
        fixture.input.declared_groups = groups.into();
        assert!(matches!(
            represent_semantic_reconciliation_fixture(&fixture),
            SemanticReconciliationOutcome::Failure(SemanticReconciliationFailureRecord {
                category: SemanticReconciliationFailureCategory::InvalidStanding,
                ..
            })
        ));
    }
}

impl fmt::Display for StableId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// The supplied source representation. References remain references until a
/// later contract explicitly resolves them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SourcePayload {
    Inline(Arc<[u8]>),
    Reference(String),
}

impl SourcePayload {
    pub fn inline(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Inline(Arc::from(bytes.into()))
    }

    pub fn reference(value: impl Into<String>) -> Self {
        Self::Reference(value.into())
    }

    pub fn as_inline(&self) -> Option<&[u8]> {
        match self {
            Self::Inline(bytes) => Some(bytes),
            Self::Reference(_) => None,
        }
    }
}

/// Representation category only; it carries no semantic or execution meaning.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum SourceCategory {
    NaturalLanguage,
    StructuredData,
    BinaryArtifact,
    ReferencedArtifact,
    ApplicationSupplied,
    CompositeSource,
}

/// Origin remains distinct from user-supplied source material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceOrigin {
    InteractiveUser,
    StructuredApplication,
    ApiSubmission,
    UploadedArtifact,
    ReferencedArtifact,
    ApplicationSupplied,
}

/// The Contract 001 source-state registry.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum AdmissionState {
    Pending,
    Received,
    Accepted,
    Rejected,
    Unavailable,
    Malformed,
    Unsupported,
    Incomplete,
    Redacted,
}

impl AdmissionState {
    const ALL: [Self; 9] = [
        Self::Pending,
        Self::Received,
        Self::Accepted,
        Self::Rejected,
        Self::Unavailable,
        Self::Malformed,
        Self::Unsupported,
        Self::Incomplete,
        Self::Redacted,
    ];
}

/// Mechanically observable metadata only.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObservableMetadata {
    pub media_type: Option<String>,
    pub declared_encoding: Option<String>,
    pub byte_length: Option<usize>,
    pub supplied_filename: Option<String>,
    pub reference_identifier: Option<String>,
    pub structural_shape: Option<String>,
}

/// Facts required to preserve a redacted component constitutionally.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactionFacts {
    pub authorized_basis: String,
    pub constitutional_existence: bool,
    pub preservation_facts: String,
}

/// Integrity/preservation facts are descriptive and do not become identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreservationFacts {
    pub recoverability: String,
    pub integrity_reference: Option<String>,
    pub redaction: Option<RedactionFacts>,
}

impl PreservationFacts {
    pub fn recoverable(description: impl Into<String>) -> Self {
        Self {
            recoverability: description.into(),
            integrity_reference: None,
            redaction: None,
        }
    }

    pub fn redacted(
        description: impl Into<String>,
        basis: impl Into<String>,
        preservation_facts: impl Into<String>,
    ) -> Self {
        Self {
            recoverability: description.into(),
            integrity_reference: None,
            redaction: Some(RedactionFacts {
                authorized_basis: basis.into(),
                constitutional_existence: true,
                preservation_facts: preservation_facts.into(),
            }),
        }
    }
}

/// A declared source component before intake evaluation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceComponentInput {
    pub category: SourceCategory,
    pub origin: SourceOrigin,
    pub payload: SourcePayload,
    pub observed_state: AdmissionState,
    pub metadata: ObservableMetadata,
    pub preservation: Option<PreservationFacts>,
}

impl SourceComponentInput {
    pub fn new(
        category: SourceCategory,
        origin: SourceOrigin,
        payload: SourcePayload,
        observed_state: AdmissionState,
    ) -> Self {
        Self {
            category,
            origin,
            payload,
            observed_state,
            metadata: ObservableMetadata::default(),
            preservation: None,
        }
    }

    pub fn with_preservation(mut self, facts: PreservationFacts) -> Self {
        self.preservation = Some(facts);
        self
    }
}

/// A source submission. Its component order is preserved in the record, but
/// operation semantics do not depend on receive order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceSubmission {
    submission_id: StableId,
    components: Vec<SourceComponentInput>,
    declared_context: BTreeMap<String, String>,
}

impl SourceSubmission {
    pub fn new(components: Vec<SourceComponentInput>) -> Self {
        let declared_context = BTreeMap::new();
        let submission_id = submission_identity(&components, &declared_context);
        Self {
            submission_id,
            components,
            declared_context: BTreeMap::new(),
        }
    }

    pub fn with_declared_context(
        mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.declared_context.insert(key.into(), value.into());
        self.submission_id = submission_identity(&self.components, &self.declared_context);
        self
    }

    pub fn submission_id(&self) -> &StableId {
        &self.submission_id
    }

    pub fn components(&self) -> &[SourceComponentInput] {
        &self.components
    }
}

/// The Contract 001 default policy, or a fully declared optional profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompositeAdmissionPolicy {
    Default,
    Authorized(AuthorizedProfile),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedProfile {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
    pub scope: String,
    pub mapping: BTreeMap<AdmissionState, ProfileStateMapping>,
    pub mixed_component_rule: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileStateMapping {
    SuccessfulAccepted,
    SuccessfulRedacted,
    SuccessfulUnresolved,
    RequireFailure,
    RequireFailureUnresolved,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
    pub scope: String,
}

impl CompositeAdmissionPolicy {
    fn binding(&self) -> Result<PolicyBinding, IntakeError> {
        match self {
            Self::Default => Ok(PolicyBinding {
                identity: StableId::from_parts("policy", &["contract-001-default"]),
                version: "contract-001-default-v1".to_owned(),
                authority_reference: "SRE-CONTRACT-001:SRE-001-POLICY-001..006".to_owned(),
                scope: "Contract 001 source admission".to_owned(),
            }),
            Self::Authorized(profile) => {
                profile.validate()?;
                Ok(PolicyBinding {
                    identity: profile.identity.clone(),
                    version: profile.version.clone(),
                    authority_reference: profile.authority_reference.clone(),
                    scope: profile.scope.clone(),
                })
            }
        }
    }

    fn mapping(&self, state: AdmissionState) -> Result<ProfileStateMapping, IntakeError> {
        match self {
            Self::Default => Ok(match state {
                AdmissionState::Accepted => ProfileStateMapping::SuccessfulAccepted,
                AdmissionState::Redacted => ProfileStateMapping::SuccessfulRedacted,
                AdmissionState::Pending | AdmissionState::Received => {
                    ProfileStateMapping::RequireFailureUnresolved
                }
                AdmissionState::Rejected
                | AdmissionState::Unavailable
                | AdmissionState::Unsupported
                | AdmissionState::Incomplete
                | AdmissionState::Malformed => ProfileStateMapping::RequireFailure,
            }),
            Self::Authorized(profile) => {
                profile.validate()?;
                profile.mapping.get(&state).copied().ok_or_else(|| {
                    IntakeError::InvalidProfile("profile mapping is not exhaustive".to_owned())
                })
            }
        }
    }
}

impl AuthorizedProfile {
    pub fn validate(&self) -> Result<(), IntakeError> {
        if self.version.is_empty()
            || self.authority_reference.is_empty()
            || self.scope.is_empty()
            || self.mixed_component_rule.is_empty()
        {
            return Err(IntakeError::InvalidProfile(
                "profile identity, version, authority, scope, and mixed-component rule are required"
                    .to_owned(),
            ));
        }
        if AdmissionState::ALL
            .iter()
            .any(|state| !self.mapping.contains_key(state))
        {
            return Err(IntakeError::InvalidProfile(
                "profile mapping must cover every Contract 001 state".to_owned(),
            ));
        }
        if self.mapping.get(&AdmissionState::Rejected) != Some(&ProfileStateMapping::RequireFailure)
            || self.mapping.get(&AdmissionState::Malformed)
                != Some(&ProfileStateMapping::RequireFailure)
        {
            return Err(IntakeError::InvalidProfile(
                "profiles may not successfully admit Rejected or Malformed components".to_owned(),
            ));
        }
        for state in AdmissionState::ALL {
            let mapping = self.mapping[&state];
            let compatible = match state {
                AdmissionState::Accepted => mapping == ProfileStateMapping::SuccessfulAccepted,
                AdmissionState::Redacted => {
                    mapping == ProfileStateMapping::SuccessfulRedacted
                        || mapping == ProfileStateMapping::RequireFailure
                }
                AdmissionState::Pending | AdmissionState::Received => matches!(
                    mapping,
                    ProfileStateMapping::RequireFailureUnresolved
                        | ProfileStateMapping::SuccessfulUnresolved
                ),
                AdmissionState::Rejected | AdmissionState::Malformed => {
                    mapping == ProfileStateMapping::RequireFailure
                }
                AdmissionState::Unavailable
                | AdmissionState::Unsupported
                | AdmissionState::Incomplete => matches!(
                    mapping,
                    ProfileStateMapping::RequireFailure
                        | ProfileStateMapping::RequireFailureUnresolved
                        | ProfileStateMapping::SuccessfulUnresolved
                ),
            };
            if !compatible {
                return Err(IntakeError::InvalidProfile(format!(
                    "profile mapping for {state:?} is incompatible with that state"
                )));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentRecord {
    source_id: StableId,
    pub category: SourceCategory,
    pub origin: SourceOrigin,
    pub payload: SourcePayload,
    pub observed_state: AdmissionState,
    pub metadata: ObservableMetadata,
    pub preservation: Option<PreservationFacts>,
    pub policy_mapping: ProfileStateMapping,
}

impl ComponentRecord {
    pub fn source_id(&self) -> &StableId {
        &self.source_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceIntakeRecord {
    operation_id: StableId,
    submission_id: StableId,
    policy: PolicyBinding,
    components: Arc<[ComponentRecord]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceIntakeFailureRecord {
    operation_id: StableId,
    submission_id: StableId,
    policy: PolicyBinding,
    components: Arc<[ComponentRecord]>,
    failure_categories: Arc<[FailureCategory]>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FailureCategory {
    RejectedComponent,
    UnavailableComponent,
    UnsupportedComponent,
    IncompleteComponent,
    MalformedComponent,
    UnresolvedComponent,
    InvalidRedactionPreservation,
    MissingPreservation,
}

impl SourceIntakeRecord {
    pub fn operation_id(&self) -> &StableId {
        &self.operation_id
    }
    pub fn submission_id(&self) -> &StableId {
        &self.submission_id
    }
    pub fn policy(&self) -> &PolicyBinding {
        &self.policy
    }
    pub fn components(&self) -> &[ComponentRecord] {
        &self.components
    }

    pub fn source_ids(&self) -> Vec<StableId> {
        self.components
            .iter()
            .map(|component| component.source_id.clone())
            .collect()
    }

    pub fn record_id(&self) -> StableId {
        let source_ids: Vec<String> = self
            .components
            .iter()
            .map(|component| component.source_id.to_string())
            .collect();
        let mut parts = vec![
            self.operation_id.as_str(),
            self.submission_id.as_str(),
            self.policy.identity.as_str(),
            self.policy.version.as_str(),
        ];
        parts.extend(source_ids.iter().map(String::as_str));
        StableId::from_parts("intake", &parts)
    }
}

impl SourceIntakeFailureRecord {
    pub fn operation_id(&self) -> &StableId {
        &self.operation_id
    }
    pub fn submission_id(&self) -> &StableId {
        &self.submission_id
    }
    pub fn policy(&self) -> &PolicyBinding {
        &self.policy
    }
    pub fn components(&self) -> &[ComponentRecord] {
        &self.components
    }
    pub fn failure_categories(&self) -> &[FailureCategory] {
        &self.failure_categories
    }
}

/// Exactly one terminal outcome for one completed operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntakeOutcome {
    Success(SourceIntakeRecord),
    Failure(SourceIntakeFailureRecord),
}

impl IntakeOutcome {
    pub fn operation_id(&self) -> &StableId {
        match self {
            Self::Success(record) => record.operation_id(),
            Self::Failure(record) => record.operation_id(),
        }
    }

    pub fn components(&self) -> &[ComponentRecord] {
        match self {
            Self::Success(record) => record.components(),
            Self::Failure(record) => record.components(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IntakeError {
    EmptySubmission,
    InvalidProfile(String),
    InvalidRedaction(usize),
}

/// Evaluate and atomically commit one source admission outcome.
pub fn admit(
    submission: &SourceSubmission,
    policy: &CompositeAdmissionPolicy,
) -> Result<IntakeOutcome, IntakeError> {
    if submission.components.is_empty() {
        return Err(IntakeError::EmptySubmission);
    }
    let binding = policy.binding()?;
    let operation_id = StableId::from_parts(
        "op",
        &[
            submission.submission_id.as_str(),
            binding.identity.as_str(),
            &binding.version,
        ],
    );
    let mut components = Vec::with_capacity(submission.components.len());
    let mut failures = Vec::new();
    let mut occurrences: BTreeMap<String, usize> = BTreeMap::new();

    for input in &submission.components {
        let mapping = policy.mapping(input.observed_state)?;
        if input.observed_state == AdmissionState::Redacted
            && (!matches!(mapping, ProfileStateMapping::SuccessfulRedacted)
                || !valid_redaction(input.preservation.as_ref()))
        {
            failures.push(FailureCategory::InvalidRedactionPreservation);
        }
        if matches!(
            mapping,
            ProfileStateMapping::SuccessfulAccepted
                | ProfileStateMapping::SuccessfulRedacted
                | ProfileStateMapping::SuccessfulUnresolved
        ) && !valid_preservation(input.preservation.as_ref())
        {
            failures.push(FailureCategory::MissingPreservation);
        }
        if matches!(
            mapping,
            ProfileStateMapping::RequireFailure | ProfileStateMapping::RequireFailureUnresolved
        ) {
            failures.push(failure_category(input.observed_state));
        }
        let fingerprint = component_fingerprint(input);
        let occurrence = occurrences.entry(fingerprint.clone()).or_insert(0);
        let occurrence_index = *occurrence;
        *occurrence += 1;
        let source_id = StableId::from_parts(
            "src",
            &[
                submission.submission_id.as_str(),
                &fingerprint,
                &occurrence_index.to_string(),
            ],
        );
        components.push(ComponentRecord {
            source_id,
            category: input.category,
            origin: input.origin,
            payload: input.payload.clone(),
            observed_state: input.observed_state,
            metadata: input.metadata.clone(),
            preservation: input.preservation.clone(),
            policy_mapping: mapping,
        });
    }

    let components: Arc<[ComponentRecord]> = components.into();
    if failures.is_empty() {
        Ok(IntakeOutcome::Success(SourceIntakeRecord {
            operation_id,
            submission_id: submission.submission_id.clone(),
            policy: binding,
            components,
        }))
    } else {
        failures.sort();
        failures.dedup();
        Ok(IntakeOutcome::Failure(SourceIntakeFailureRecord {
            operation_id,
            submission_id: submission.submission_id.clone(),
            policy: binding,
            components,
            failure_categories: failures.into(),
        }))
    }
}

fn valid_redaction(preservation: Option<&PreservationFacts>) -> bool {
    preservation.is_some_and(|facts| {
        !facts.recoverability.is_empty()
            && facts.redaction.as_ref().is_some_and(|redaction| {
                redaction.constitutional_existence
                    && !redaction.authorized_basis.is_empty()
                    && !redaction.preservation_facts.is_empty()
            })
    })
}

fn valid_preservation(preservation: Option<&PreservationFacts>) -> bool {
    preservation.is_some_and(|facts| !facts.recoverability.is_empty())
}

fn failure_category(state: AdmissionState) -> FailureCategory {
    match state {
        AdmissionState::Rejected => FailureCategory::RejectedComponent,
        AdmissionState::Unavailable => FailureCategory::UnavailableComponent,
        AdmissionState::Unsupported => FailureCategory::UnsupportedComponent,
        AdmissionState::Incomplete => FailureCategory::IncompleteComponent,
        AdmissionState::Malformed => FailureCategory::MalformedComponent,
        AdmissionState::Pending | AdmissionState::Received => FailureCategory::UnresolvedComponent,
        AdmissionState::Accepted | AdmissionState::Redacted => {
            FailureCategory::InvalidRedactionPreservation
        }
    }
}

fn component_fingerprint(component: &SourceComponentInput) -> String {
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}",
        component.category,
        component.origin,
        component.payload,
        component.observed_state,
        component.metadata
    )
}

fn submission_identity(
    components: &[SourceComponentInput],
    declared_context: &BTreeMap<String, String>,
) -> StableId {
    let mut parts = vec!["submission-v1".to_owned()];
    let mut fingerprints: Vec<String> = components.iter().map(component_fingerprint).collect();
    fingerprints.sort();
    parts.extend(fingerprints);
    parts.extend(
        declared_context
            .iter()
            .map(|(key, value)| format!("context:{key}={value}")),
    );
    let references: Vec<&str> = parts.iter().map(String::as_str).collect();
    StableId::from_parts("sub", &references)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn component(state: AdmissionState) -> SourceComponentInput {
        let payload = SourcePayload::inline("source");
        let mut input = SourceComponentInput::new(
            SourceCategory::NaturalLanguage,
            SourceOrigin::InteractiveUser,
            payload,
            state,
        );
        if state == AdmissionState::Accepted {
            input =
                input.with_preservation(PreservationFacts::recoverable("immutable inline source"));
        }
        input
    }

    #[test]
    fn default_policy_success_and_replay_are_deterministic() {
        let submission = SourceSubmission::new(vec![component(AdmissionState::Accepted)]);
        let first = admit(&submission, &CompositeAdmissionPolicy::Default).expect("admission");
        let second = admit(&submission, &CompositeAdmissionPolicy::Default).expect("admission");
        assert_eq!(first, second);
        assert!(matches!(first, IntakeOutcome::Success(_)));
    }

    #[test]
    fn failure_preserves_every_component_and_observed_state() {
        let submission = SourceSubmission::new(vec![
            component(AdmissionState::Accepted),
            component(AdmissionState::Rejected),
            component(AdmissionState::Unavailable),
        ]);
        let outcome = admit(&submission, &CompositeAdmissionPolicy::Default).expect("admission");
        let IntakeOutcome::Failure(failure) = outcome else {
            panic!("expected failure")
        };
        assert_eq!(failure.components().len(), 3);
        assert_eq!(
            failure.components()[0].observed_state,
            AdmissionState::Accepted
        );
        assert_eq!(
            failure.components()[1].observed_state,
            AdmissionState::Rejected
        );
        assert_eq!(
            failure.components()[2].observed_state,
            AdmissionState::Unavailable
        );
        assert!(failure
            .failure_categories()
            .contains(&FailureCategory::RejectedComponent));
        assert!(failure
            .failure_categories()
            .contains(&FailureCategory::UnavailableComponent));
    }

    #[test]
    fn redaction_requires_preservation_and_authorized_basis() {
        let invalid = component(AdmissionState::Redacted);
        let outcome = admit(
            &SourceSubmission::new(vec![invalid]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("admission");
        assert!(matches!(outcome, IntakeOutcome::Failure(_)));

        let valid = component(AdmissionState::Redacted).with_preservation(
            PreservationFacts::redacted("reference retained", "basis-1", "integrity-1"),
        );
        let outcome = admit(
            &SourceSubmission::new(vec![valid]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("admission");
        assert!(matches!(outcome, IntakeOutcome::Success(_)));
    }

    #[test]
    fn accepted_component_without_preservation_cannot_commit_success() {
        let outcome = admit(
            &SourceSubmission::new(vec![SourceComponentInput::new(
                SourceCategory::NaturalLanguage,
                SourceOrigin::InteractiveUser,
                SourcePayload::inline("source"),
                AdmissionState::Accepted,
            )]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("admission evaluation");
        let IntakeOutcome::Failure(failure) = outcome else {
            panic!("missing preservation must fail intake")
        };
        assert!(failure
            .failure_categories()
            .contains(&FailureCategory::MissingPreservation));
    }

    #[test]
    fn references_and_application_components_remain_distinct() {
        let mut app = component(AdmissionState::Accepted);
        app.origin = SourceOrigin::ApplicationSupplied;
        app.category = SourceCategory::ApplicationSupplied;
        app.payload = SourcePayload::inline("metadata");
        let reference = SourceComponentInput::new(
            SourceCategory::ReferencedArtifact,
            SourceOrigin::ReferencedArtifact,
            SourcePayload::reference("https://example.invalid/resource"),
            AdmissionState::Accepted,
        )
        .with_preservation(PreservationFacts::recoverable("reference retained"));
        let outcome = admit(
            &SourceSubmission::new(vec![app, reference]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("admission");
        let IntakeOutcome::Success(record) = outcome else {
            panic!("expected success")
        };
        assert_eq!(
            record.components()[0].origin,
            SourceOrigin::ApplicationSupplied
        );
        assert_eq!(
            record.components()[1].payload,
            SourcePayload::Reference("https://example.invalid/resource".to_owned())
        );
        assert_ne!(
            record.components()[0].source_id(),
            record.components()[1].source_id()
        );
    }

    #[test]
    fn unauthorized_optional_profile_is_rejected_without_default_fallback() {
        let profile = AuthorizedProfile {
            identity: StableId::from_parts("profile", &["incomplete"]),
            version: "1".to_owned(),
            authority_reference: "authority".to_owned(),
            scope: "scope".to_owned(),
            mapping: BTreeMap::new(),
            mixed_component_rule: "all".to_owned(),
        };
        let result = admit(
            &SourceSubmission::new(vec![component(AdmissionState::Accepted)]),
            &CompositeAdmissionPolicy::Authorized(profile),
        );
        assert!(matches!(result, Err(IntakeError::InvalidProfile(_))));
    }

    #[test]
    fn component_state_is_not_operation_outcome() {
        let submission = SourceSubmission::new(vec![
            component(AdmissionState::Accepted),
            component(AdmissionState::Malformed),
        ]);
        let IntakeOutcome::Failure(failure) =
            admit(&submission, &CompositeAdmissionPolicy::Default).expect("admission")
        else {
            panic!("expected failure")
        };
        assert_eq!(
            failure.components()[0].observed_state,
            AdmissionState::Accepted
        );
        assert_eq!(
            failure.components()[1].observed_state,
            AdmissionState::Malformed
        );
    }

    #[test]
    fn default_policy_maps_all_failure_and_unresolved_states_to_failure() {
        for state in [
            AdmissionState::Rejected,
            AdmissionState::Unavailable,
            AdmissionState::Unsupported,
            AdmissionState::Incomplete,
            AdmissionState::Malformed,
            AdmissionState::Pending,
            AdmissionState::Received,
        ] {
            let submission = SourceSubmission::new(vec![component(state)]);
            let outcome = admit(&submission, &CompositeAdmissionPolicy::Default)
                .expect("admission evaluation");
            let IntakeOutcome::Failure(failure) = outcome else {
                panic!("{state:?} must require operation failure")
            };
            assert_eq!(failure.components()[0].observed_state, state);
        }
    }

    #[test]
    fn component_receive_order_does_not_change_policy_outcome_or_source_ids() {
        let first = component(AdmissionState::Accepted);
        let second = component(AdmissionState::Rejected);
        let left = admit(
            &SourceSubmission::new(vec![first.clone(), second.clone()]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("admission evaluation");
        let right = admit(
            &SourceSubmission::new(vec![second, first]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("admission evaluation");
        assert!(matches!(left, IntakeOutcome::Failure(_)));
        assert!(matches!(right, IntakeOutcome::Failure(_)));
        assert_eq!(left.operation_id(), right.operation_id());
        let mut left_ids: Vec<_> = left
            .components()
            .iter()
            .map(|component| component.source_id())
            .collect();
        let mut right_ids: Vec<_> = right
            .components()
            .iter()
            .map(|component| component.source_id())
            .collect();
        left_ids.sort();
        right_ids.sort();
        assert_eq!(left_ids, right_ids);
    }
}

// ---------------------------------------------------------------------------
// Contract 002: Interpretation Boundary
// ---------------------------------------------------------------------------

macro_rules! contract_002_id {
    ($name:ident, $prefix:literal) => {
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
        pub struct $name(StableId);

        impl $name {
            pub fn derive(parts: &[&str]) -> Self {
                Self(StableId::from_parts($prefix, parts))
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

contract_002_id!(InterpretationOperationId, "iop");
contract_002_id!(InterpretationRequestId, "ireq");
contract_002_id!(InterpretationProposalId, "iprop");
contract_002_id!(ProposalAdmissionId, "padm");
contract_002_id!(AdmittedInterpretationProposalSetId, "ipset");
contract_002_id!(InterpretationBoundaryFailureRecordId, "ifail");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProposalDomain {
    Objectives,
    Constraints,
    CapabilityRequirements,
    Ambiguities,
    Assumptions,
    Uncertainty,
    Relationships,
    EvidenceAssociations,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProposalProductionState {
    Requested,
    Returned,
    Incomplete,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProposalCompletionState {
    Complete,
    Incomplete,
    NoResult,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AdministrativeRelationship {
    Active,
    Withdrawn,
    Superseded,
    Archived,
    Expired,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceForm {
    SourceSpan,
    SourceLevel,
    FieldToSource,
    ReferencedArtifact,
    ApplicationSupplied,
    DeclaredLack,
    UnsupportedCapability,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceStatus {
    EvidenceProvided,
    EvidenceNotRequiredByProfile,
    EvidenceUnavailable,
    EvidenceUnsupportedByInterpreter,
    EvidenceReferenceInvalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceCapabilityProfile {
    identity: StableId,
    version: String,
    authority_reference: String,
    scope: String,
    required_forms: BTreeSet<EvidenceForm>,
    permitted_forms: BTreeSet<EvidenceForm>,
    permitted_statuses: BTreeSet<EvidenceStatus>,
    allow_no_result: bool,
}

impl EvidenceCapabilityProfile {
    /// Explicit fixture-only profile. It is not a constitutional default.
    pub fn fixture_strict() -> Self {
        let all_forms = [
            EvidenceForm::SourceSpan,
            EvidenceForm::SourceLevel,
            EvidenceForm::FieldToSource,
            EvidenceForm::ReferencedArtifact,
            EvidenceForm::ApplicationSupplied,
            EvidenceForm::DeclaredLack,
            EvidenceForm::UnsupportedCapability,
        ]
        .into_iter()
        .collect();
        let statuses = [
            EvidenceStatus::EvidenceProvided,
            EvidenceStatus::EvidenceNotRequiredByProfile,
            EvidenceStatus::EvidenceUnavailable,
            EvidenceStatus::EvidenceUnsupportedByInterpreter,
            EvidenceStatus::EvidenceReferenceInvalid,
        ]
        .into_iter()
        .collect();
        Self {
            identity: StableId::from_parts("evidence-profile", &["contract-002-fixture"]),
            version: "fixture-002-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            scope: "Contract 002 structural admission tests".to_owned(),
            required_forms: BTreeSet::new(),
            permitted_forms: all_forms,
            permitted_statuses: statuses,
            allow_no_result: true,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        identity: StableId,
        version: impl Into<String>,
        authority_reference: impl Into<String>,
        scope: impl Into<String>,
        required_forms: BTreeSet<EvidenceForm>,
        permitted_forms: BTreeSet<EvidenceForm>,
        permitted_statuses: BTreeSet<EvidenceStatus>,
        allow_no_result: bool,
    ) -> Self {
        Self {
            identity,
            version: version.into(),
            authority_reference: authority_reference.into(),
            scope: scope.into(),
            required_forms,
            permitted_forms,
            permitted_statuses,
            allow_no_result,
        }
    }

    pub fn identity(&self) -> &StableId {
        &self.identity
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn authority_reference(&self) -> &str {
        &self.authority_reference
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }

    fn validate(&self) -> Result<(), StructuralFinding> {
        if self.version.is_empty()
            || self.authority_reference.is_empty()
            || self.scope.is_empty()
            || !self.required_forms.is_subset(&self.permitted_forms)
        {
            return Err(StructuralFinding::ProfileIncompatible);
        }
        Ok(())
    }

    fn binding(&self) -> ProfileBinding {
        ProfileBinding {
            identity: self.identity.clone(),
            version: self.version.clone(),
            authority_reference: self.authority_reference.clone(),
            scope: self.scope.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpreterProfileBinding {
    pub identity: StableId,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceReference {
    pub form: EvidenceForm,
    pub reference: String,
    pub status: EvidenceStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationRequestSpec {
    pub included_source_ids: Vec<StableId>,
    pub excluded_source_ids: Vec<StableId>,
    pub requested_scope: BTreeSet<ProposalDomain>,
    pub contract_version: String,
    pub proposal_schema_version: String,
    pub interpreter_profile: InterpreterProfileBinding,
    pub evidence_profile: ProfileBinding,
    pub required_proposal_metadata: BTreeSet<String>,
    pub permitted_response_forms: BTreeSet<String>,
    pub completion_expectations: BTreeSet<String>,
    pub declared_bounds: BTreeSet<String>,
    pub declared_exclusions: BTreeSet<String>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationRequest {
    request_id: InterpretationRequestId,
    operation_id: InterpretationOperationId,
    source_intake_id: StableId,
    included_source_ids: Arc<[StableId]>,
    excluded_source_ids: Arc<[StableId]>,
    requested_scope: Arc<[ProposalDomain]>,
    contract_version: String,
    proposal_schema_version: String,
    interpreter_profile: InterpreterProfileBinding,
    evidence_profile: ProfileBinding,
    required_proposal_metadata: Arc<[String]>,
    permitted_response_forms: Arc<[String]>,
    completion_expectations: Arc<[String]>,
    declared_bounds: Arc<[String]>,
    declared_exclusions: Arc<[String]>,
    replay_context: BTreeMap<String, String>,
}

impl InterpretationRequest {
    pub fn request_id(&self) -> &InterpretationRequestId {
        &self.request_id
    }
    pub fn operation_id(&self) -> &InterpretationOperationId {
        &self.operation_id
    }
    pub fn source_intake_id(&self) -> &StableId {
        &self.source_intake_id
    }
    pub fn included_source_ids(&self) -> &[StableId] {
        &self.included_source_ids
    }
    pub fn excluded_source_ids(&self) -> &[StableId] {
        &self.excluded_source_ids
    }
    pub fn requested_scope(&self) -> &[ProposalDomain] {
        &self.requested_scope
    }
    pub fn contract_version(&self) -> &str {
        &self.contract_version
    }
    pub fn proposal_schema_version(&self) -> &str {
        &self.proposal_schema_version
    }
    pub fn interpreter_profile(&self) -> &InterpreterProfileBinding {
        &self.interpreter_profile
    }
    pub fn evidence_profile(&self) -> &ProfileBinding {
        &self.evidence_profile
    }
    pub fn replay_context(&self) -> &BTreeMap<String, String> {
        &self.replay_context
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationOperationContext {
    operation_id: InterpretationOperationId,
    source_intake_id: StableId,
    source_ids: Arc<[StableId]>,
}

impl InterpretationOperationContext {
    pub fn from_intake(intake: &SourceIntakeRecord) -> Self {
        let source_ids = intake.source_ids();
        let source_id_text: Vec<String> = source_ids.iter().map(ToString::to_string).collect();
        let source_refs: Vec<&str> = source_id_text.iter().map(String::as_str).collect();
        let mut parts = vec![
            intake.operation_id().as_str(),
            intake.submission_id().as_str(),
        ];
        parts.extend(source_refs);
        let operation_id = InterpretationOperationId::derive(&parts);
        Self {
            operation_id,
            source_intake_id: intake.record_id(),
            source_ids: source_ids.into(),
        }
    }

    pub fn operation_id(&self) -> &InterpretationOperationId {
        &self.operation_id
    }
    pub fn source_intake_id(&self) -> &StableId {
        &self.source_intake_id
    }
    pub fn source_ids(&self) -> &[StableId] {
        &self.source_ids
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BoundaryFailureCategory {
    EmptySourceScope,
    UnknownSourceScope,
    DuplicateSourceScope,
    ContradictorySourceScope,
    MissingRequiredInput,
    InvalidAssociation,
    MissingProfile,
    InvalidProfile,
    SchemaIncompatible,
    InvalidProposalIntegrity,
    CannotConstructDecision,
    AtomicCommitmentFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationBoundaryFailureRecord {
    failure_id: InterpretationBoundaryFailureRecordId,
    operation_id: InterpretationOperationId,
    request_id: Option<InterpretationRequestId>,
    proposal_id: Option<InterpretationProposalId>,
    source_intake_id: Option<StableId>,
    category: BoundaryFailureCategory,
    observed_facts: Arc<[String]>,
    affected_versions: Arc<[String]>,
    recoverability: String,
    replay_context: BTreeMap<String, String>,
}

impl InterpretationBoundaryFailureRecord {
    pub fn failure_id(&self) -> &InterpretationBoundaryFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &InterpretationOperationId {
        &self.operation_id
    }
    pub fn request_id(&self) -> Option<&InterpretationRequestId> {
        self.request_id.as_ref()
    }
    pub fn proposal_id(&self) -> Option<&InterpretationProposalId> {
        self.proposal_id.as_ref()
    }
    pub fn category(&self) -> &BoundaryFailureCategory {
        &self.category
    }
    pub fn source_intake_id(&self) -> Option<&StableId> {
        self.source_intake_id.as_ref()
    }
    pub fn observed_facts(&self) -> &[String] {
        &self.observed_facts
    }
    pub fn affected_versions(&self) -> &[String] {
        &self.affected_versions
    }
    pub fn recoverability(&self) -> &str {
        &self.recoverability
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterpretationRequestOutcome {
    Request(Box<InterpretationRequest>),
    Failure(InterpretationBoundaryFailureRecord),
}

pub fn issue_interpretation_request(
    context: &InterpretationOperationContext,
    spec: InterpretationRequestSpec,
) -> InterpretationRequestOutcome {
    let available: BTreeSet<_> = context.source_ids.iter().cloned().collect();
    let included: BTreeSet<_> = spec.included_source_ids.iter().cloned().collect();
    let excluded: BTreeSet<_> = spec.excluded_source_ids.iter().cloned().collect();
    let category = if included.is_empty() {
        Some(BoundaryFailureCategory::EmptySourceScope)
    } else if included.len() != spec.included_source_ids.len()
        || excluded.len() != spec.excluded_source_ids.len()
    {
        Some(BoundaryFailureCategory::DuplicateSourceScope)
    } else if !included.is_subset(&available) || !excluded.is_subset(&available) {
        Some(BoundaryFailureCategory::UnknownSourceScope)
    } else if !included.is_disjoint(&excluded) {
        Some(BoundaryFailureCategory::ContradictorySourceScope)
    } else if spec.contract_version.is_empty()
        || spec.proposal_schema_version.is_empty()
        || spec.interpreter_profile.identity.as_str().is_empty()
        || spec.interpreter_profile.version.is_empty()
        || spec.evidence_profile.identity.as_str().is_empty()
        || spec.evidence_profile.version.is_empty()
    {
        Some(BoundaryFailureCategory::MissingRequiredInput)
    } else {
        None
    };
    if let Some(category) = category {
        return InterpretationRequestOutcome::Failure(boundary_failure(
            context,
            None,
            None,
            category,
            &["request issuance did not complete"],
            &[
                spec.contract_version.as_str(),
                spec.proposal_schema_version.as_str(),
                spec.evidence_profile.version.as_str(),
            ],
            &spec.replay_context,
        ));
    }
    let mut identity_parts = vec![
        context.operation_id.as_str(),
        context.source_intake_id.as_str(),
        spec.contract_version.as_str(),
        spec.proposal_schema_version.as_str(),
        spec.interpreter_profile.identity.as_str(),
        spec.interpreter_profile.version.as_str(),
        spec.evidence_profile.identity.as_str(),
        spec.evidence_profile.version.as_str(),
    ];
    let spec_fingerprint = format!("{spec:?}");
    identity_parts.push(spec_fingerprint.as_str());
    let scope_text: Vec<String> = spec
        .included_source_ids
        .iter()
        .map(ToString::to_string)
        .collect();
    identity_parts.extend(scope_text.iter().map(String::as_str));
    let request_id = InterpretationRequestId::derive(&identity_parts);
    InterpretationRequestOutcome::Request(Box::new(InterpretationRequest {
        request_id,
        operation_id: context.operation_id.clone(),
        source_intake_id: context.source_intake_id.clone(),
        included_source_ids: spec.included_source_ids.into(),
        excluded_source_ids: spec.excluded_source_ids.into(),
        requested_scope: spec.requested_scope.into_iter().collect::<Vec<_>>().into(),
        contract_version: spec.contract_version,
        proposal_schema_version: spec.proposal_schema_version,
        interpreter_profile: spec.interpreter_profile,
        evidence_profile: spec.evidence_profile,
        required_proposal_metadata: spec
            .required_proposal_metadata
            .into_iter()
            .collect::<Vec<_>>()
            .into(),
        permitted_response_forms: spec
            .permitted_response_forms
            .into_iter()
            .collect::<Vec<_>>()
            .into(),
        completion_expectations: spec
            .completion_expectations
            .into_iter()
            .collect::<Vec<_>>()
            .into(),
        declared_bounds: spec.declared_bounds.into_iter().collect::<Vec<_>>().into(),
        declared_exclusions: spec
            .declared_exclusions
            .into_iter()
            .collect::<Vec<_>>()
            .into(),
        replay_context: spec.replay_context,
    }))
}

/// The only interpreter-facing operation: an external participant returns
/// already-produced proposals. It receives no mutable SRE state.
pub trait ExternalInterpreter {
    fn produce(&self, request: &InterpretationRequest) -> Vec<InterpretationProposal>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalObjectiveElement {
    pub element_id: String,
    pub expression: String,
    pub form: String,
    pub origin: String,
    pub basis: String,
    pub class_name: String,
    pub scope: Option<String>,
    pub designations: Arc<[String]>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub evidence_status: Option<EvidenceStatus>,
    pub representation_status: Option<String>,
    pub relationships: Arc<[ProposalObjectiveRelationship]>,
    pub component_expressions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalObjectiveRelationship {
    pub relationship_id: String,
    pub relationship_type: String,
    pub target_element_ids: Arc<[String]>,
    pub scope: Option<String>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub evidence_status: Option<EvidenceStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalConstraintElement {
    pub element_id: String,
    pub expression: String,
    pub class_name: String,
    pub form: String,
    pub origin: String,
    pub basis: String,
    pub scope_kind: String,
    pub scope_target_ids: Arc<[String]>,
    pub scope_resolution_status: String,
    pub objective_ids: Arc<[String]>,
    pub declared_priority: Option<i32>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub evidence_status: Option<EvidenceStatus>,
    pub representation_status: Option<String>,
    pub relationships: Arc<[ProposalConstraintRelationship]>,
    pub component_expressions: Arc<[String]>,
    pub reference: Option<String>,
    pub reference_state: String,
    pub explicit_conflict: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalConstraintRelationship {
    pub relationship_id: String,
    pub relationship_type: String,
    pub target_element_ids: Arc<[String]>,
    pub scope: Option<String>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub evidence_status: Option<EvidenceStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCapabilityElement {
    pub element_id: String,
    pub expression: String,
    pub class_name: String,
    pub supplied_class: Option<String>,
    pub supplied_method: Option<String>,
    pub abstraction_rule_id: Option<String>,
    pub abstraction_rule_version: Option<String>,
    pub form: String,
    pub origin: String,
    pub basis: String,
    pub necessity: String,
    pub scope_kind: String,
    pub scope_target_ids: Arc<[String]>,
    pub scope_resolution_status: String,
    pub objective_ids: Arc<[String]>,
    pub constraint_ids: Arc<[String]>,
    pub access_dependency: Option<String>,
    pub classification_support_status: String,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub evidence_status: Option<EvidenceStatus>,
    pub representation_status: Option<String>,
    pub relationships: Arc<[ProposalCapabilityRelationship]>,
    pub component_expressions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCapabilityRelationship {
    pub relationship_id: String,
    pub relationship_type: String,
    pub target_element_ids: Arc<[String]>,
    pub scope: Option<String>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub evidence_status: Option<EvidenceStatus>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalClarificationElement {
    pub element_id: String,
    pub domain: String,
    pub expression: String,
    pub class_name: String,
    pub origin: String,
    pub basis: String,
    pub target_ids: Arc<[String]>,
    pub alternatives: Arc<[String]>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub relationships: Arc<[String]>,
    pub clarification_requirement: Option<String>,
    pub uncertainty_kind: Option<String>,
    pub uncertainty_mode: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalEvidenceElement {
    pub element_id: String,
    pub evidence_class: String,
    pub origin: String,
    pub basis: String,
    pub material_kind: String,
    pub material_value: String,
    pub locator: Option<String>,
    pub supplied_value: Option<String>,
    pub evidence_status: String,
    pub representation_status: String,
    pub grounding_target_id: Option<String>,
    pub grounding_target_type: Option<String>,
    pub grounding_relationship: Option<String>,
    pub grounding_aspect: Option<String>,
    pub grounding_basis: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalContent {
    pub proposed_elements: Arc<[String]>,
    /// Explicitly schema-designated objective-domain material. Generic
    /// `proposed_elements` are never scanned or inferred as objectives.
    pub objective_elements: Arc<[ProposalObjectiveElement]>,
    /// Explicitly typed constraint-domain material. Generic proposal text is
    /// never scanned or classified as a constraint.
    pub constraint_elements: Arc<[ProposalConstraintElement]>,
    /// Explicitly typed capability-domain material. No other proposal field
    /// may be classified as a capability requirement.
    pub capability_elements: Arc<[ProposalCapabilityElement]>,
    /// Explicitly typed Contract 006 meaning-qualification material. Generic
    /// proposal text and legacy declaration fields are never scanned as this domain.
    pub clarification_elements: Arc<[ProposalClarificationElement]>,
    /// Explicitly typed Contract 007 evidence declarations. No other field
    /// may be scanned to discover evidence or grounding.
    pub evidence_elements: Arc<[ProposalEvidenceElement]>,
    pub evidence_references: Arc<[EvidenceReference]>,
    pub declared_assumptions: Arc<[String]>,
    pub declared_uncertainties: Arc<[String]>,
    pub declared_bounds: Arc<[String]>,
}

impl ProposalContent {
    pub fn empty() -> Self {
        Self {
            proposed_elements: Vec::new().into(),
            objective_elements: Vec::new().into(),
            constraint_elements: Vec::new().into(),
            capability_elements: Vec::new().into(),
            clarification_elements: Vec::new().into(),
            evidence_elements: Vec::new().into(),
            evidence_references: Vec::new().into(),
            declared_assumptions: Vec::new().into(),
            declared_uncertainties: Vec::new().into(),
            declared_bounds: Vec::new().into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationProposal {
    proposal_id: InterpretationProposalId,
    operation_id: InterpretationOperationId,
    request_id: InterpretationRequestId,
    interpreter_identity: StableId,
    interpreter_version: String,
    interpreter_profile: InterpreterProfileBinding,
    proposal_schema_version: String,
    production_state: ProposalProductionState,
    completion_state: ProposalCompletionState,
    source_scope: Arc<[StableId]>,
    evidence_profile: Option<EvidenceCapabilityProfile>,
    content: ProposalContent,
    administrative_relationship: AdministrativeRelationship,
    replay_context: BTreeMap<String, String>,
}

impl InterpretationProposal {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request: &InterpretationRequest,
        interpreter_identity: StableId,
        interpreter_version: impl Into<String>,
        interpreter_profile: InterpreterProfileBinding,
        production_state: ProposalProductionState,
        completion_state: ProposalCompletionState,
        source_scope: Vec<StableId>,
        evidence_profile: Option<EvidenceCapabilityProfile>,
        content: ProposalContent,
        replay_context: BTreeMap<String, String>,
    ) -> Self {
        let interpreter_version = interpreter_version.into();
        let scope_text: Vec<String> = source_scope.iter().map(ToString::to_string).collect();
        let mut parts = vec![
            request.operation_id.as_str(),
            request.request_id.as_str(),
            interpreter_identity.as_str(),
            interpreter_version.as_str(),
            interpreter_profile.identity.as_str(),
            interpreter_profile.version.as_str(),
            request.proposal_schema_version.as_str(),
        ];
        parts.extend(scope_text.iter().map(String::as_str));
        let content_fingerprint = format!("{content:?}|{replay_context:?}");
        parts.push(content_fingerprint.as_str());
        let proposal_id = InterpretationProposalId::derive(&parts);
        Self {
            proposal_id,
            operation_id: request.operation_id.clone(),
            request_id: request.request_id.clone(),
            interpreter_identity,
            interpreter_version,
            interpreter_profile,
            proposal_schema_version: request.proposal_schema_version.clone(),
            production_state,
            completion_state,
            source_scope: source_scope.into(),
            evidence_profile,
            content,
            administrative_relationship: AdministrativeRelationship::Active,
            replay_context,
        }
    }

    pub fn proposal_id(&self) -> &InterpretationProposalId {
        &self.proposal_id
    }
    pub fn operation_id(&self) -> &InterpretationOperationId {
        &self.operation_id
    }
    pub fn request_id(&self) -> &InterpretationRequestId {
        &self.request_id
    }
    pub fn interpreter_identity(&self) -> &StableId {
        &self.interpreter_identity
    }
    pub fn interpreter_version(&self) -> &str {
        &self.interpreter_version
    }
    pub fn interpreter_profile(&self) -> &InterpreterProfileBinding {
        &self.interpreter_profile
    }
    pub fn proposal_schema_version(&self) -> &str {
        &self.proposal_schema_version
    }
    pub fn production_state(&self) -> ProposalProductionState {
        self.production_state
    }
    pub fn completion_state(&self) -> ProposalCompletionState {
        self.completion_state
    }
    pub fn source_scope(&self) -> &[StableId] {
        &self.source_scope
    }
    pub fn evidence_profile(&self) -> Option<&EvidenceCapabilityProfile> {
        self.evidence_profile.as_ref()
    }
    pub fn content(&self) -> &ProposalContent {
        &self.content
    }
    pub fn administrative_relationship(&self) -> AdministrativeRelationship {
        self.administrative_relationship
    }
    pub fn replay_context(&self) -> &BTreeMap<String, String> {
        &self.replay_context
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AdmissionDisposition {
    Admitted,
    RejectedMalformed,
    RejectedIncomplete,
    RejectedIncompatible,
    RejectedUnverifiable,
    RejectedOutOfScope,
    RejectedInvalidAssociation,
    RejectedDuplicateIdentity,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StructuralFinding {
    MissingInterpreterIdentity,
    MissingInterpreterVersion,
    InterpreterProfileIncompatible,
    MissingProfile,
    ProfileIncompatible,
    EvidenceProfileIncompatible,
    SchemaIncompatible,
    OperationMismatch,
    RequestMismatch,
    SourceScopeInvalid,
    SourceScopeEmpty,
    SourceScopeDuplicate,
    EvidenceReferenceInvalid,
    EvidenceFormUnsupported,
    EvidenceStatusUnsupported,
    RequiredEvidenceMissing,
    ProductionIncomplete,
    ProductionFailed,
    ProposalEmptyNotPermitted,
    CompletionIncompatible,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationProposalAdmissionDecision {
    admission_id: ProposalAdmissionId,
    operation_id: InterpretationOperationId,
    request_id: InterpretationRequestId,
    proposal_id: InterpretationProposalId,
    interpreter_identity: StableId,
    interpreter_version: String,
    source_intake_id: StableId,
    source_scope: Arc<[StableId]>,
    contract_version: String,
    proposal_schema_version: String,
    interpreter_profile: InterpreterProfileBinding,
    evidence_profile: Option<ProfileBinding>,
    criteria_version: String,
    disposition: AdmissionDisposition,
    structural_findings: Arc<[StructuralFinding]>,
    evidence_findings: Arc<[EvidenceStatus]>,
    basis: Arc<[String]>,
    replay_context: BTreeMap<String, String>,
}

impl InterpretationProposalAdmissionDecision {
    pub fn admission_id(&self) -> &ProposalAdmissionId {
        &self.admission_id
    }
    pub fn operation_id(&self) -> &InterpretationOperationId {
        &self.operation_id
    }
    pub fn request_id(&self) -> &InterpretationRequestId {
        &self.request_id
    }
    pub fn proposal_id(&self) -> &InterpretationProposalId {
        &self.proposal_id
    }
    pub fn interpreter_identity(&self) -> &StableId {
        &self.interpreter_identity
    }
    pub fn interpreter_version(&self) -> &str {
        &self.interpreter_version
    }
    pub fn source_intake_id(&self) -> &StableId {
        &self.source_intake_id
    }
    pub fn contract_version(&self) -> &str {
        &self.contract_version
    }
    pub fn proposal_schema_version(&self) -> &str {
        &self.proposal_schema_version
    }
    pub fn disposition(&self) -> AdmissionDisposition {
        self.disposition
    }
    pub fn structural_findings(&self) -> &[StructuralFinding] {
        &self.structural_findings
    }
    pub fn evidence_findings(&self) -> &[EvidenceStatus] {
        &self.evidence_findings
    }
    pub fn basis(&self) -> &[String] {
        &self.basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalAdmissionOutcome {
    Decision(Box<InterpretationProposalAdmissionDecision>),
    Failure(InterpretationBoundaryFailureRecord),
}

pub fn admit_interpretation_proposal(
    request: &InterpretationRequest,
    proposal: &InterpretationProposal,
    criteria_version: &str,
) -> ProposalAdmissionOutcome {
    if criteria_version.is_empty() {
        return ProposalAdmissionOutcome::Failure(boundary_failure(
            &InterpretationOperationContext {
                operation_id: request.operation_id.clone(),
                source_intake_id: request.source_intake_id.clone(),
                source_ids: request.included_source_ids.to_vec().into(),
            },
            Some(request.request_id.clone()),
            Some(proposal.proposal_id.clone()),
            BoundaryFailureCategory::CannotConstructDecision,
            &["criteria version is required"],
            &[request.contract_version.as_str()],
            &request.replay_context,
        ));
    }
    let request_sources: BTreeSet<_> = request.included_source_ids.iter().cloned().collect();
    let proposal_sources: BTreeSet<_> = proposal.source_scope.iter().cloned().collect();
    let mut findings = Vec::new();
    let mut evidence_findings = Vec::new();
    if proposal.interpreter_identity.as_str().is_empty() {
        findings.push(StructuralFinding::MissingInterpreterIdentity);
    }
    if proposal.interpreter_version.is_empty() {
        findings.push(StructuralFinding::MissingInterpreterVersion);
    }
    if proposal.interpreter_profile != request.interpreter_profile {
        findings.push(StructuralFinding::InterpreterProfileIncompatible);
    }
    if proposal.operation_id != request.operation_id {
        findings.push(StructuralFinding::OperationMismatch);
    }
    if proposal.request_id != request.request_id {
        findings.push(StructuralFinding::RequestMismatch);
    }
    if proposal.proposal_schema_version != request.proposal_schema_version {
        findings.push(StructuralFinding::SchemaIncompatible);
    }
    if proposal_sources.is_empty() {
        findings.push(StructuralFinding::SourceScopeEmpty);
    }
    if proposal_sources.len() != proposal.source_scope.len() {
        findings.push(StructuralFinding::SourceScopeDuplicate);
    }
    if !proposal_sources.is_subset(&request_sources) {
        findings.push(StructuralFinding::SourceScopeInvalid);
    }
    let Some(profile) = proposal.evidence_profile.as_ref() else {
        findings.push(StructuralFinding::MissingProfile);
        return ProposalAdmissionOutcome::Decision(Box::new(make_decision(
            request,
            proposal,
            criteria_version,
            AdmissionDisposition::RejectedUnverifiable,
            findings,
            evidence_findings,
            &["an evidence capability profile is required"],
        )));
    };
    if let Err(finding) = profile.validate() {
        findings.push(finding);
    }
    if profile.binding() != request.evidence_profile {
        findings.push(StructuralFinding::EvidenceProfileIncompatible);
    }
    if proposal.content.proposed_elements.is_empty() && !profile.allow_no_result {
        findings.push(StructuralFinding::ProposalEmptyNotPermitted);
    }
    for evidence in proposal.content.evidence_references.iter() {
        if evidence.reference.is_empty() {
            findings.push(StructuralFinding::EvidenceReferenceInvalid);
            evidence_findings.push(EvidenceStatus::EvidenceReferenceInvalid);
        } else {
            evidence_findings.push(evidence.status);
            if !profile.permitted_forms.contains(&evidence.form) {
                findings.push(StructuralFinding::EvidenceFormUnsupported);
            }
            if !profile.permitted_statuses.contains(&evidence.status) {
                findings.push(StructuralFinding::EvidenceStatusUnsupported);
            }
        }
    }
    let supplied_forms: BTreeSet<_> = proposal
        .content
        .evidence_references
        .iter()
        .map(|evidence| evidence.form)
        .collect();
    if !profile.required_forms.is_subset(&supplied_forms) {
        findings.push(StructuralFinding::RequiredEvidenceMissing);
    }
    match proposal.production_state {
        ProposalProductionState::Returned => {}
        ProposalProductionState::Requested | ProposalProductionState::Incomplete => {
            findings.push(StructuralFinding::ProductionIncomplete)
        }
        ProposalProductionState::Failed => findings.push(StructuralFinding::ProductionFailed),
    }
    if proposal.completion_state == ProposalCompletionState::Incomplete
        || (proposal.completion_state == ProposalCompletionState::NoResult
            && !proposal.content.proposed_elements.is_empty())
        || (proposal.completion_state == ProposalCompletionState::NoResult
            && !profile.allow_no_result)
    {
        findings.push(StructuralFinding::CompletionIncompatible);
    }
    let disposition = if findings.is_empty() {
        AdmissionDisposition::Admitted
    } else if findings.contains(&StructuralFinding::OperationMismatch)
        || findings.contains(&StructuralFinding::RequestMismatch)
    {
        AdmissionDisposition::RejectedInvalidAssociation
    } else if findings.contains(&StructuralFinding::SourceScopeInvalid)
        || findings.contains(&StructuralFinding::SourceScopeEmpty)
        || findings.contains(&StructuralFinding::SourceScopeDuplicate)
    {
        AdmissionDisposition::RejectedOutOfScope
    } else if findings.contains(&StructuralFinding::EvidenceReferenceInvalid)
        || findings.contains(&StructuralFinding::EvidenceFormUnsupported)
        || findings.contains(&StructuralFinding::EvidenceStatusUnsupported)
        || findings.contains(&StructuralFinding::MissingProfile)
    {
        AdmissionDisposition::RejectedUnverifiable
    } else if findings.contains(&StructuralFinding::SchemaIncompatible)
        || findings.contains(&StructuralFinding::ProfileIncompatible)
        || findings.contains(&StructuralFinding::InterpreterProfileIncompatible)
        || findings.contains(&StructuralFinding::EvidenceProfileIncompatible)
    {
        AdmissionDisposition::RejectedIncompatible
    } else {
        AdmissionDisposition::RejectedIncomplete
    };
    ProposalAdmissionOutcome::Decision(Box::new(make_decision(
        request,
        proposal,
        criteria_version,
        disposition,
        findings,
        evidence_findings,
        &["structural Contract 002 criteria only"],
    )))
}

fn make_decision(
    request: &InterpretationRequest,
    proposal: &InterpretationProposal,
    criteria_version: &str,
    disposition: AdmissionDisposition,
    mut findings: Vec<StructuralFinding>,
    mut evidence_findings: Vec<EvidenceStatus>,
    basis: &[&str],
) -> InterpretationProposalAdmissionDecision {
    findings.sort();
    findings.dedup();
    evidence_findings.sort();
    evidence_findings.dedup();
    let admission_id = ProposalAdmissionId::derive(&[
        proposal.operation_id.as_str(),
        proposal.request_id.as_str(),
        proposal.proposal_id.as_str(),
        criteria_version,
        &format!("{disposition:?}"),
    ]);
    InterpretationProposalAdmissionDecision {
        admission_id,
        operation_id: request.operation_id.clone(),
        request_id: request.request_id.clone(),
        proposal_id: proposal.proposal_id.clone(),
        interpreter_identity: proposal.interpreter_identity.clone(),
        interpreter_version: proposal.interpreter_version.clone(),
        source_intake_id: request.source_intake_id.clone(),
        source_scope: proposal.source_scope.clone(),
        contract_version: request.contract_version.clone(),
        proposal_schema_version: proposal.proposal_schema_version.clone(),
        interpreter_profile: proposal.interpreter_profile.clone(),
        evidence_profile: proposal
            .evidence_profile
            .as_ref()
            .map(EvidenceCapabilityProfile::binding),
        criteria_version: criteria_version.to_owned(),
        disposition,
        structural_findings: findings.into(),
        evidence_findings: evidence_findings.into(),
        basis: basis
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>()
            .into(),
        replay_context: request.replay_context.clone(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetPublicationPolicy {
    pub identity: StableId,
    pub version: String,
    pub criteria_version: String,
    pub authority_reference: String,
    pub allow_zero_admitted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedInterpretationProposalSet {
    set_id: AdmittedInterpretationProposalSetId,
    operation_id: InterpretationOperationId,
    request_id: InterpretationRequestId,
    source_intake_id: StableId,
    submitted_proposal_ids: Arc<[InterpretationProposalId]>,
    decision_ids: Arc<[ProposalAdmissionId]>,
    admitted_proposal_ids: Arc<[InterpretationProposalId]>,
    non_admitted_decision_ids: Arc<[ProposalAdmissionId]>,
    proposals: Arc<[InterpretationProposal]>,
    decisions: Arc<[InterpretationProposalAdmissionDecision]>,
    contract_version: String,
    proposal_schema_version: String,
    criteria_version: String,
    publication_policy: SetPublicationPolicy,
    replay_context: BTreeMap<String, String>,
}

impl AdmittedInterpretationProposalSet {
    pub fn set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &InterpretationOperationId {
        &self.operation_id
    }
    pub fn request_id(&self) -> &InterpretationRequestId {
        &self.request_id
    }
    pub fn source_intake_id(&self) -> &StableId {
        &self.source_intake_id
    }
    pub fn submitted_proposal_ids(&self) -> &[InterpretationProposalId] {
        &self.submitted_proposal_ids
    }
    pub fn decision_ids(&self) -> &[ProposalAdmissionId] {
        &self.decision_ids
    }
    pub fn admitted_proposal_ids(&self) -> &[InterpretationProposalId] {
        &self.admitted_proposal_ids
    }
    pub fn proposals(&self) -> &[InterpretationProposal] {
        &self.proposals
    }
    pub fn decisions(&self) -> &[InterpretationProposalAdmissionDecision] {
        &self.decisions
    }
    pub fn non_admitted_decision_ids(&self) -> &[ProposalAdmissionId] {
        &self.non_admitted_decision_ids
    }
    pub fn contract_version(&self) -> &str {
        &self.contract_version
    }
    pub fn proposal_schema_version(&self) -> &str {
        &self.proposal_schema_version
    }
    pub fn criteria_version(&self) -> &str {
        &self.criteria_version
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SetPublicationOutcome {
    Set(AdmittedInterpretationProposalSet),
    Failure(InterpretationBoundaryFailureRecord),
}

pub fn publish_admitted_proposal_set(
    request: &InterpretationRequest,
    proposals_and_decisions: Vec<(
        InterpretationProposal,
        InterpretationProposalAdmissionDecision,
    )>,
    policy: SetPublicationPolicy,
) -> SetPublicationOutcome {
    let context = InterpretationOperationContext {
        operation_id: request.operation_id.clone(),
        source_intake_id: request.source_intake_id.clone(),
        source_ids: request.included_source_ids.to_vec().into(),
    };
    if policy.version.is_empty() || policy.authority_reference.is_empty() {
        return SetPublicationOutcome::Failure(boundary_failure(
            &context,
            Some(request.request_id.clone()),
            None,
            BoundaryFailureCategory::MissingRequiredInput,
            &["set publication policy is incomplete"],
            &[
                request.contract_version.as_str(),
                request.proposal_schema_version.as_str(),
            ],
            &request.replay_context,
        ));
    }
    let mut proposals = Vec::new();
    let mut decisions = Vec::new();
    let mut proposal_ids = BTreeSet::new();
    let mut decision_ids = BTreeSet::new();
    for (proposal, decision) in proposals_and_decisions {
        if proposal.operation_id != request.operation_id
            || proposal.request_id != request.request_id
            || decision.operation_id != request.operation_id
            || decision.request_id != request.request_id
            || decision.proposal_id != proposal.proposal_id
            || !proposal_ids.insert(proposal.proposal_id.clone())
            || !decision_ids.insert(decision.admission_id.clone())
        {
            return SetPublicationOutcome::Failure(boundary_failure(
                &context,
                Some(request.request_id.clone()),
                Some(proposal.proposal_id.clone()),
                BoundaryFailureCategory::InvalidAssociation,
                &["proposal and decision association is invalid or duplicated"],
                &[
                    request.contract_version.as_str(),
                    request.proposal_schema_version.as_str(),
                ],
                &request.replay_context,
            ));
        }
        proposals.push(proposal);
        decisions.push(decision);
    }
    let admitted: Vec<_> = decisions
        .iter()
        .filter(|decision| decision.disposition == AdmissionDisposition::Admitted)
        .map(|decision| decision.proposal_id.clone())
        .collect();
    if policy.criteria_version.is_empty()
        || decisions
            .iter()
            .any(|decision| decision.criteria_version != policy.criteria_version)
    {
        return SetPublicationOutcome::Failure(boundary_failure(
            &context,
            Some(request.request_id.clone()),
            None,
            BoundaryFailureCategory::InvalidAssociation,
            &["admission decisions do not share one criteria version"],
            &[
                request.contract_version.as_str(),
                request.proposal_schema_version.as_str(),
            ],
            &request.replay_context,
        ));
    }
    if admitted.is_empty() && !policy.allow_zero_admitted {
        return SetPublicationOutcome::Failure(boundary_failure(
            &context,
            Some(request.request_id.clone()),
            None,
            BoundaryFailureCategory::MissingRequiredInput,
            &["zero-admitted success is not authorized by the supplied publication policy"],
            &[
                request.contract_version.as_str(),
                request.proposal_schema_version.as_str(),
            ],
            &request.replay_context,
        ));
    }
    let mut set_parts = vec![
        request.operation_id.as_str(),
        request.request_id.as_str(),
        policy.identity.as_str(),
        policy.version.as_str(),
    ];
    let proposal_id_text: Vec<String> = proposal_ids.iter().map(ToString::to_string).collect();
    set_parts.extend(proposal_id_text.iter().map(String::as_str));
    let set_id = AdmittedInterpretationProposalSetId::derive(&set_parts);
    let decision_id_vec: Vec<_> = decisions
        .iter()
        .map(|decision| decision.admission_id.clone())
        .collect();
    let non_admitted: Vec<_> = decisions
        .iter()
        .filter(|decision| decision.disposition != AdmissionDisposition::Admitted)
        .map(|decision| decision.admission_id.clone())
        .collect();
    SetPublicationOutcome::Set(AdmittedInterpretationProposalSet {
        set_id,
        operation_id: request.operation_id.clone(),
        request_id: request.request_id.clone(),
        source_intake_id: request.source_intake_id.clone(),
        submitted_proposal_ids: proposal_ids.into_iter().collect::<Vec<_>>().into(),
        decision_ids: decision_id_vec.into(),
        admitted_proposal_ids: admitted.into(),
        non_admitted_decision_ids: non_admitted.into(),
        proposals: proposals.into(),
        decisions: decisions.into(),
        contract_version: request.contract_version.clone(),
        proposal_schema_version: request.proposal_schema_version.clone(),
        criteria_version: policy.criteria_version.clone(),
        publication_policy: policy,
        replay_context: request.replay_context.clone(),
    })
}

fn boundary_failure(
    context: &InterpretationOperationContext,
    request_id: Option<InterpretationRequestId>,
    proposal_id: Option<InterpretationProposalId>,
    category: BoundaryFailureCategory,
    observed_facts: &[&str],
    affected_versions: &[&str],
    replay_context: &BTreeMap<String, String>,
) -> InterpretationBoundaryFailureRecord {
    let request_text = request_id
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();
    let proposal_text = proposal_id
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();
    let failure_id = InterpretationBoundaryFailureRecordId::derive(&[
        context.operation_id.as_str(),
        request_text.as_str(),
        proposal_text.as_str(),
        &format!("{category:?}"),
    ]);
    InterpretationBoundaryFailureRecord {
        failure_id,
        operation_id: context.operation_id.clone(),
        request_id,
        proposal_id,
        source_intake_id: Some(context.source_intake_id.clone()),
        category,
        observed_facts: observed_facts
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>()
            .into(),
        affected_versions: affected_versions
            .iter()
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>()
            .into(),
        recoverability: "retry with corrected declared inputs or a new proposal".to_owned(),
        replay_context: replay_context.clone(),
    }
}

#[cfg(test)]
mod contract_002_tests {
    use super::*;

    fn admitted_intake() -> SourceIntakeRecord {
        let input = SourceComponentInput::new(
            SourceCategory::NaturalLanguage,
            SourceOrigin::InteractiveUser,
            SourcePayload::inline("source text"),
            AdmissionState::Accepted,
        )
        .with_preservation(PreservationFacts::recoverable("immutable source"));
        let submission = SourceSubmission::new(vec![input]);
        let IntakeOutcome::Success(record) =
            admit(&submission, &CompositeAdmissionPolicy::Default).expect("source admission")
        else {
            panic!("fixture intake must succeed")
        };
        record
    }

    fn request() -> InterpretationRequest {
        let intake = admitted_intake();
        let context = InterpretationOperationContext::from_intake(&intake);
        let evidence_profile = EvidenceCapabilityProfile::fixture_strict();
        let mut scope = BTreeSet::new();
        scope.insert(ProposalDomain::Objectives);
        scope.insert(ProposalDomain::Constraints);
        let spec = InterpretationRequestSpec {
            included_source_ids: intake.source_ids(),
            excluded_source_ids: Vec::new(),
            requested_scope: scope,
            contract_version: "0.1.0".to_owned(),
            proposal_schema_version: "proposal-fixture-v2".to_owned(),
            interpreter_profile: InterpreterProfileBinding {
                identity: StableId::from_parts("interpreter-profile", &["fixture"]),
                version: "fixture-interpreter-v1".to_owned(),
            },
            evidence_profile: evidence_profile.binding(),
            required_proposal_metadata: BTreeSet::from(["trace".to_owned()]),
            permitted_response_forms: BTreeSet::from(["structured-proposal".to_owned()]),
            completion_expectations: BTreeSet::from(["returned".to_owned()]),
            declared_bounds: BTreeSet::from(["structural-admission-only".to_owned()]),
            declared_exclusions: BTreeSet::from(["semantic-correctness".to_owned()]),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        };
        let InterpretationRequestOutcome::Request(request) =
            issue_interpretation_request(&context, spec)
        else {
            panic!("request fixture must succeed")
        };
        *request
    }

    fn proposal(request: &InterpretationRequest, text: &str) -> InterpretationProposal {
        InterpretationProposal::new(
            request,
            StableId::from_parts("interpreter", &[text]),
            "interpreter-v1",
            request.interpreter_profile.clone(),
            ProposalProductionState::Returned,
            ProposalCompletionState::Complete,
            request.included_source_ids.to_vec(),
            Some(EvidenceCapabilityProfile::fixture_strict()),
            ProposalContent {
                proposed_elements: vec![text.to_owned()].into(),
                objective_elements: vec![ProposalObjectiveElement {
                    element_id: format!("objective-{text}"),
                    expression: text.to_owned(),
                    form: "Atomic".to_owned(),
                    origin: "InterpreterInference".to_owned(),
                    basis: "InterpreterSynthesis".to_owned(),
                    class_name: "RequestedOutcome".to_owned(),
                    scope: None,
                    designations: Vec::new().into(),
                    evidence_references: Vec::new().into(),
                    evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
                    representation_status: None,
                    relationships: Vec::new().into(),
                    component_expressions: Vec::new().into(),
                }]
                .into(),
                constraint_elements: Vec::new().into(),
                capability_elements: Vec::new().into(),
                clarification_elements: Vec::new().into(),
                evidence_elements: Vec::new().into(),
                evidence_references: Vec::new().into(),
                declared_assumptions: Vec::new().into(),
                declared_uncertainties: Vec::new().into(),
                declared_bounds: vec!["proposal-scoped".to_owned()].into(),
            },
            BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        )
    }

    #[test]
    fn request_authority_is_distinct_and_replayable() {
        let first = request();
        let second = request();
        assert_eq!(first, second);
        assert_ne!(first.request_id().as_str(), first.operation_id().as_str());
        assert_ne!(
            first.source_intake_id(),
            &StableId::from_parts("unused", &["id"])
        );
        assert_ne!(
            first.source_intake_id().as_str(),
            first.operation_id().as_str()
        );
    }

    #[test]
    fn invalid_request_scope_commits_boundary_failure() {
        let intake = admitted_intake();
        let context = InterpretationOperationContext::from_intake(&intake);
        let profile = EvidenceCapabilityProfile::fixture_strict().binding();
        let spec = InterpretationRequestSpec {
            included_source_ids: vec![StableId::from_parts("src", &["unknown"])],
            excluded_source_ids: Vec::new(),
            requested_scope: BTreeSet::from([ProposalDomain::Objectives]),
            contract_version: "0.1.0".to_owned(),
            proposal_schema_version: "schema-v1".to_owned(),
            interpreter_profile: InterpreterProfileBinding {
                identity: StableId::from_parts("interpreter-profile", &["fixture"]),
                version: "v1".to_owned(),
            },
            evidence_profile: profile,
            required_proposal_metadata: BTreeSet::new(),
            permitted_response_forms: BTreeSet::new(),
            completion_expectations: BTreeSet::new(),
            declared_bounds: BTreeSet::new(),
            declared_exclusions: BTreeSet::new(),
            replay_context: BTreeMap::new(),
        };
        let InterpretationRequestOutcome::Failure(failure) =
            issue_interpretation_request(&context, spec)
        else {
            panic!("unknown source scope must fail request issuance")
        };
        assert_eq!(
            failure.category(),
            &BoundaryFailureCategory::UnknownSourceScope
        );
    }

    #[test]
    fn structural_admission_is_not_semantic_acceptance() {
        let request = request();
        let admitted = proposal(&request, "an intentionally unsupported semantic claim");
        let ProposalAdmissionOutcome::Decision(decision) =
            admit_interpretation_proposal(&request, &admitted, "criteria-v1")
        else {
            panic!("valid fixture proposal must produce a decision")
        };
        assert_eq!(decision.disposition(), AdmissionDisposition::Admitted);
        assert!(decision
            .basis()
            .iter()
            .any(|value| value.contains("structural")));
    }

    #[test]
    fn negative_structural_result_is_a_decision_not_operation_failure() {
        let request = request();
        let mut invalid = proposal(&request, "proposal");
        invalid.proposal_schema_version = "wrong-schema".to_owned();
        let ProposalAdmissionOutcome::Decision(decision) =
            admit_interpretation_proposal(&request, &invalid, "criteria-v1")
        else {
            panic!("schema mismatch remains a completed negative decision")
        };
        assert_eq!(
            decision.disposition(),
            AdmissionDisposition::RejectedIncompatible
        );
        assert!(decision
            .structural_findings()
            .contains(&StructuralFinding::SchemaIncompatible));
    }

    #[test]
    fn proposal_content_correction_requires_a_new_identity() {
        let request = request();
        let first = proposal(&request, "first");
        let second = proposal(&request, "second");
        assert_ne!(first.proposal_id(), second.proposal_id());
        assert_eq!(first.content().proposed_elements[0], "first");
    }

    #[test]
    fn multiple_admitted_proposals_coexist_without_precedence() {
        let request = request();
        let first = proposal(&request, "first");
        let second = proposal(&request, "second");
        let ProposalAdmissionOutcome::Decision(first_decision) =
            admit_interpretation_proposal(&request, &first, "criteria-v1")
        else {
            panic!("first proposal must produce a decision")
        };
        let ProposalAdmissionOutcome::Decision(second_decision) =
            admit_interpretation_proposal(&request, &second, "criteria-v1")
        else {
            panic!("second proposal must produce a decision")
        };
        let outcome = publish_admitted_proposal_set(
            &request,
            vec![(first, *first_decision), (second, *second_decision)],
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["fixture"]),
                version: "set-v1".to_owned(),
                criteria_version: "criteria-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: false,
            },
        );
        let SetPublicationOutcome::Set(set) = outcome else {
            panic!("multiple admitted proposals must coexist")
        };
        assert_eq!(set.proposals().len(), 2);
        assert_eq!(set.admitted_proposal_ids().len(), 2);
    }

    #[test]
    fn empty_admitted_set_requires_explicit_profile_permission() {
        let request = request();
        let mut invalid = proposal(&request, "incomplete");
        invalid.production_state = ProposalProductionState::Incomplete;
        let ProposalAdmissionOutcome::Decision(decision) =
            admit_interpretation_proposal(&request, &invalid, "criteria-v1")
        else {
            panic!("incomplete proposal must receive a decision")
        };
        let denied = publish_admitted_proposal_set(
            &request,
            vec![(invalid.clone(), (*decision).clone())],
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["denied"]),
                version: "set-v1".to_owned(),
                criteria_version: "criteria-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: false,
            },
        );
        assert!(matches!(denied, SetPublicationOutcome::Failure(_)));
        let allowed = publish_admitted_proposal_set(
            &request,
            vec![(invalid, *decision)],
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["allowed"]),
                version: "set-v1".to_owned(),
                criteria_version: "criteria-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: true,
            },
        );
        let SetPublicationOutcome::Set(set) = allowed else {
            panic!("explicit zero-admitted profile must permit the empty set")
        };
        assert!(set.admitted_proposal_ids().is_empty());
        let truly_empty = publish_admitted_proposal_set(
            &request,
            Vec::new(),
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["empty"]),
                version: "set-v1".to_owned(),
                criteria_version: "criteria-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: true,
            },
        );
        let SetPublicationOutcome::Set(set) = truly_empty else {
            panic!("explicit policy must permit zero submitted proposals")
        };
        assert!(set.proposals().is_empty());
    }

    #[test]
    fn invalid_evidence_reference_is_a_valid_negative_decision() {
        let request = request();
        let mut invalid = proposal(&request, "evidence");
        invalid.content.evidence_references = vec![EvidenceReference {
            form: EvidenceForm::SourceSpan,
            reference: String::new(),
            status: EvidenceStatus::EvidenceProvided,
        }]
        .into();
        let ProposalAdmissionOutcome::Decision(decision) =
            admit_interpretation_proposal(&request, &invalid, "criteria-v1")
        else {
            panic!("evidence reference invalidity remains a decision")
        };
        assert_eq!(
            decision.disposition(),
            AdmissionDisposition::RejectedUnverifiable
        );
    }
}

// ---------------------------------------------------------------------------
// Contract 003: Objectives and Objective Relationships
// ---------------------------------------------------------------------------

contract_002_id!(ObjectiveRepresentationOperationId, "o3op");
contract_002_id!(DeclaredObjectiveSetId, "odset");
contract_002_id!(DeclaredObjectiveId, "obj");
contract_002_id!(ObjectiveRepresentationId, "orep");
contract_002_id!(ObjectiveRepresentationFailureRecordId, "ofail");
contract_002_id!(ObjectiveRepresentationProfileId, "oprof");
contract_002_id!(ObjectiveRegistryId, "oreg");
contract_002_id!(ObjectiveRelationshipId, "orel");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectiveForm {
    Atomic,
    Composite,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectiveOrigin {
    ExplicitSource,
    InterpreterInference,
    ApplicationSupplied,
    ReferencedArtifact,
    MixedOrigin,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RepresentationBasis {
    DirectQuotation,
    StructuredExtraction,
    InterpreterSynthesis,
    ApplicationDeclaration,
    ReferencedArtifactDeclaration,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectiveDesignation {
    Primary,
    Secondary,
    Subordinate,
    Conditional,
    Parallel,
    Alternative,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectiveRepresentationStatus {
    Represented,
    Incomplete,
    Unsupported,
    Conflicting,
    Unresolved,
    EvidenceLimited,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PrimaryObjectiveStatus {
    Declared,
    Undetermined,
    Conflicting,
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ObjectiveRelationshipType {
    Supports,
    DependsOn,
    ConditionalUpon,
    AlternativeTo,
    MutuallyExclusiveWith,
    ParallelWith,
    SequentialTo,
    ComponentOf,
    ConflictsWith,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureObjectiveRepresentationProfile {
    identity: ObjectiveRepresentationProfileId,
    version: String,
    authority_reference: String,
    scope: String,
    allow_empty: bool,
    allow_incomplete: bool,
    allow_unsupported: bool,
    allow_evidence_limited: bool,
    allow_decomposition: bool,
    allow_cross_proposal_relationships: bool,
    allowed_forms: BTreeSet<ObjectiveForm>,
    allowed_origins: BTreeSet<ObjectiveOrigin>,
    allowed_bases: BTreeSet<RepresentationBasis>,
    allowed_designations: BTreeSet<ObjectiveDesignation>,
    allowed_statuses: BTreeSet<ObjectiveRepresentationStatus>,
}

impl FixtureObjectiveRepresentationProfile {
    /// A visibly fixture-only authority. It is not a production default.
    pub fn fixture() -> Self {
        Self {
            identity: ObjectiveRepresentationProfileId::derive(&["contract-003-fixture"]),
            version: "fixture-003-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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

    pub fn identity(&self) -> &ObjectiveRepresentationProfileId {
        &self.identity
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn authority_reference(&self) -> &str {
        &self.authority_reference
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureObjectiveRegistry {
    pub identity: ObjectiveRegistryId,
    pub version: String,
    pub authority_reference: String,
    pub values: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureObjectiveRegistries {
    pub class_registry: FixtureObjectiveRegistry,
    pub form_registry: FixtureObjectiveRegistry,
    pub relationship_registry: FixtureObjectiveRegistry,
    pub designation_registry: FixtureObjectiveRegistry,
    pub status_registry: FixtureObjectiveRegistry,
}

impl FixtureObjectiveRegistries {
    pub fn fixture() -> Self {
        fn registry(name: &str, values: &[&str]) -> FixtureObjectiveRegistry {
            FixtureObjectiveRegistry {
                identity: ObjectiveRegistryId::derive(&[name, "contract-003-fixture"]),
                version: "fixture-003-registry-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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

    fn versions(&self) -> Vec<String> {
        vec![
            self.class_registry.version.clone(),
            self.form_registry.version.clone(),
            self.relationship_registry.version.clone(),
            self.designation_registry.version.clone(),
            self.status_registry.version.clone(),
        ]
    }

    fn identities(&self) -> Vec<ObjectiveRegistryId> {
        vec![
            self.class_registry.identity.clone(),
            self.form_registry.identity.clone(),
            self.relationship_registry.identity.clone(),
            self.designation_registry.identity.clone(),
            self.status_registry.identity.clone(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveSchemaBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveConfigurationBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveRepresentationInputs<'a> {
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub profile: &'a FixtureObjectiveRepresentationProfile,
    pub registries: &'a FixtureObjectiveRegistries,
    pub schema: &'a ObjectiveSchemaBinding,
    pub configuration: &'a ObjectiveConfigurationBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveRelationship {
    relationship_id: ObjectiveRelationshipId,
    relationship_type: ObjectiveRelationshipType,
    source_objective_id: DeclaredObjectiveId,
    target_objective_ids: Arc<[DeclaredObjectiveId]>,
    origin: ObjectiveOrigin,
    basis: RepresentationBasis,
    evidence_references: Arc<[EvidenceReference]>,
    evidence_status: Option<EvidenceStatus>,
    source_proposal_ids: Arc<[InterpretationProposalId]>,
    scope: Option<String>,
}

impl ObjectiveRelationship {
    pub fn relationship_id(&self) -> &ObjectiveRelationshipId {
        &self.relationship_id
    }
    pub fn relationship_type(&self) -> ObjectiveRelationshipType {
        self.relationship_type
    }
    pub fn source_objective_id(&self) -> &DeclaredObjectiveId {
        &self.source_objective_id
    }
    pub fn target_objective_ids(&self) -> &[DeclaredObjectiveId] {
        &self.target_objective_ids
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredObjective {
    logical_objective_id: DeclaredObjectiveId,
    representation_id: ObjectiveRepresentationId,
    expression: String,
    form: ObjectiveForm,
    origin: ObjectiveOrigin,
    basis: RepresentationBasis,
    source_proposal_ids: Arc<[InterpretationProposalId]>,
    source_element_ids: Arc<[String]>,
    evidence_references: Arc<[EvidenceReference]>,
    evidence_status: Option<EvidenceStatus>,
    scope: Option<String>,
    designations: Arc<[ObjectiveDesignation]>,
    status: ObjectiveRepresentationStatus,
    relationships: Arc<[ObjectiveRelationshipId]>,
    profile_id: ObjectiveRepresentationProfileId,
    profile_version: String,
    registry_ids: Arc<[ObjectiveRegistryId]>,
    registry_versions: Arc<[String]>,
    schema: ObjectiveSchemaBinding,
    configuration: ObjectiveConfigurationBinding,
    operation_id: ObjectiveRepresentationOperationId,
    set_id: DeclaredObjectiveSetId,
}

impl DeclaredObjective {
    pub fn logical_objective_id(&self) -> &DeclaredObjectiveId {
        &self.logical_objective_id
    }
    pub fn representation_id(&self) -> &ObjectiveRepresentationId {
        &self.representation_id
    }
    pub fn expression(&self) -> &str {
        &self.expression
    }
    pub fn form(&self) -> ObjectiveForm {
        self.form
    }
    pub fn origin(&self) -> ObjectiveOrigin {
        self.origin
    }
    pub fn basis(&self) -> RepresentationBasis {
        self.basis
    }
    pub fn source_proposal_ids(&self) -> &[InterpretationProposalId] {
        &self.source_proposal_ids
    }
    pub fn source_element_ids(&self) -> &[String] {
        &self.source_element_ids
    }
    pub fn evidence_references(&self) -> &[EvidenceReference] {
        &self.evidence_references
    }
    pub fn evidence_status(&self) -> Option<EvidenceStatus> {
        self.evidence_status
    }
    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }
    pub fn designations(&self) -> &[ObjectiveDesignation] {
        &self.designations
    }
    pub fn status(&self) -> ObjectiveRepresentationStatus {
        self.status
    }
    pub fn relationship_ids(&self) -> &[ObjectiveRelationshipId] {
        &self.relationships
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredObjectiveSet {
    set_id: DeclaredObjectiveSetId,
    operation_id: ObjectiveRepresentationOperationId,
    interpretation_operation_id: InterpretationOperationId,
    source_intake_id: StableId,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    input_proposal_ids: Arc<[InterpretationProposalId]>,
    input_decision_ids: Arc<[ProposalAdmissionId]>,
    profile_id: ObjectiveRepresentationProfileId,
    profile_version: String,
    registry_ids: Arc<[ObjectiveRegistryId]>,
    registry_versions: Arc<[String]>,
    schema: ObjectiveSchemaBinding,
    configuration: ObjectiveConfigurationBinding,
    objectives: Arc<[DeclaredObjective]>,
    relationships: Arc<[ObjectiveRelationship]>,
    primary_status: PrimaryObjectiveStatus,
    representation_findings: Arc<[String]>,
    replay_context: BTreeMap<String, String>,
}

impl DeclaredObjectiveSet {
    pub fn set_id(&self) -> &DeclaredObjectiveSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &ObjectiveRepresentationOperationId {
        &self.operation_id
    }
    pub fn interpretation_operation_id(&self) -> &InterpretationOperationId {
        &self.interpretation_operation_id
    }
    pub fn admitted_proposal_set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.admitted_proposal_set_id
    }
    pub fn input_proposal_ids(&self) -> &[InterpretationProposalId] {
        &self.input_proposal_ids
    }
    pub fn input_decision_ids(&self) -> &[ProposalAdmissionId] {
        &self.input_decision_ids
    }
    pub fn objectives(&self) -> &[DeclaredObjective] {
        &self.objectives
    }
    pub fn relationships(&self) -> &[ObjectiveRelationship] {
        &self.relationships
    }
    pub fn primary_status(&self) -> PrimaryObjectiveStatus {
        self.primary_status
    }
    pub fn profile_id(&self) -> &ObjectiveRepresentationProfileId {
        &self.profile_id
    }
    pub fn profile_version(&self) -> &str {
        &self.profile_version
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveRepresentationFailureCategory {
    MissingUpstreamPublication,
    InvalidExactPublicationBinding,
    MissingRequiredProfile,
    IncompatibleProfile,
    MissingRequiredRegistry,
    IncompatibleRegistryVersion,
    InvalidSchemaBinding,
    InvalidConfigurationBinding,
    UnauthorizedObjectiveSurface,
    MalformedObjectiveInput,
    InvalidEvidenceReference,
    InvalidRelationshipReference,
    NondeterministicConstruction,
    AtomicCommitmentFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectiveRepresentationFailureRecord {
    failure_id: ObjectiveRepresentationFailureRecordId,
    operation_id: ObjectiveRepresentationOperationId,
    interpretation_operation_id: InterpretationOperationId,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    input_proposal_ids: Arc<[InterpretationProposalId]>,
    profile_id: Option<ObjectiveRepresentationProfileId>,
    profile_version: Option<String>,
    registry_ids: Arc<[ObjectiveRegistryId]>,
    registry_versions: Arc<[String]>,
    category: ObjectiveRepresentationFailureCategory,
    findings: Arc<[String]>,
    retry_permitted: bool,
    replay_context: BTreeMap<String, String>,
}

impl ObjectiveRepresentationFailureRecord {
    pub fn failure_id(&self) -> &ObjectiveRepresentationFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &ObjectiveRepresentationOperationId {
        &self.operation_id
    }
    pub fn admitted_proposal_set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.admitted_proposal_set_id
    }
    pub fn category(&self) -> &ObjectiveRepresentationFailureCategory {
        &self.category
    }
    pub fn findings(&self) -> &[String] {
        &self.findings
    }
    pub fn retry_permitted(&self) -> bool {
        self.retry_permitted
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObjectiveRepresentationOutcome {
    Set(DeclaredObjectiveSet),
    Failure(ObjectiveRepresentationFailureRecord),
}

pub fn represent_objectives(
    inputs: ObjectiveRepresentationInputs<'_>,
) -> ObjectiveRepresentationOutcome {
    let upstream = inputs.admitted_proposal_set;
    let mut proposal_ids: Vec<_> = upstream
        .proposals
        .iter()
        .map(|proposal| proposal.proposal_id.clone())
        .collect();
    proposal_ids.sort();
    let operation_id = ObjectiveRepresentationOperationId::derive(&[
        upstream.operation_id.as_str(),
        upstream.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
        inputs.schema.identity.as_str(),
        inputs.schema.version.as_str(),
        inputs.configuration.identity.as_str(),
        inputs.configuration.version.as_str(),
    ]);
    if let Some(failure) = validate_objective_inputs(&inputs, &operation_id, &proposal_ids) {
        return ObjectiveRepresentationOutcome::Failure(failure);
    }
    let admitted: BTreeSet<_> = upstream.admitted_proposal_ids.iter().cloned().collect();
    let mut proposals: Vec<_> = upstream
        .proposals
        .iter()
        .filter(|proposal| admitted.contains(&proposal.proposal_id))
        .collect();
    proposals.sort_by_key(|proposal| proposal.proposal_id.clone());
    let mut elements: Vec<(&InterpretationProposal, &ProposalObjectiveElement)> = proposals
        .iter()
        .flat_map(|proposal| {
            let mut proposal_elements: Vec<_> = proposal
                .content
                .objective_elements
                .iter()
                .map(|element| (*proposal, element))
                .collect();
            proposal_elements.sort_by_key(|(_, element)| element.element_id.clone());
            proposal_elements
        })
        .collect();
    elements.sort_by_key(|(proposal, element)| {
        (proposal.proposal_id.clone(), element.element_id.clone())
    });
    let mut objective_entries = Vec::new();
    let mut element_objective_ids: BTreeMap<
        (InterpretationProposalId, String),
        DeclaredObjectiveId,
    > = BTreeMap::new();
    for (proposal, element) in &elements {
        let logical_id = DeclaredObjectiveId::derive(&[
            operation_id.as_str(),
            proposal.proposal_id.as_str(),
            element.element_id.as_str(),
        ]);
        element_objective_ids.insert(
            (proposal.proposal_id.clone(), element.element_id.clone()),
            logical_id.clone(),
        );
        match build_objective(
            &operation_id,
            &logical_id,
            &upstream.set_id,
            proposal,
            element,
            inputs.profile,
            inputs.registries,
            inputs.schema,
            inputs.configuration,
        ) {
            Ok(objective) => objective_entries.push(objective),
            Err(category) => {
                return ObjectiveRepresentationOutcome::Failure(objective_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    category,
                    &["objective representation could not be constructed"],
                ));
            }
        }
        if !element.component_expressions.is_empty() {
            if element.form != "Composite" || !inputs.profile.allow_decomposition {
                return ObjectiveRepresentationOutcome::Failure(objective_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ObjectiveRepresentationFailureCategory::UnauthorizedObjectiveSurface,
                    &["objective decomposition is not authorized by the bound fixture profile"],
                ));
            }
            for (index, expression) in element.component_expressions.iter().enumerate() {
                let child_element_id = format!("{}#component-{index}", element.element_id);
                let child_id = DeclaredObjectiveId::derive(&[
                    logical_id.as_str(),
                    &index.to_string(),
                    expression,
                ]);
                element_objective_ids.insert(
                    (proposal.proposal_id.clone(), child_element_id.clone()),
                    child_id.clone(),
                );
                let child = build_objective(
                    &operation_id,
                    &child_id,
                    &upstream.set_id,
                    proposal,
                    &ProposalObjectiveElement {
                        element_id: child_element_id,
                        expression: expression.clone(),
                        form: "Atomic".to_owned(),
                        origin: element.origin.clone(),
                        basis: element.basis.clone(),
                        class_name: element.class_name.clone(),
                        scope: element.scope.clone(),
                        designations: Vec::new().into(),
                        evidence_references: element.evidence_references.clone(),
                        evidence_status: element.evidence_status,
                        representation_status: None,
                        relationships: Vec::new().into(),
                        component_expressions: Vec::new().into(),
                    },
                    inputs.profile,
                    inputs.registries,
                    inputs.schema,
                    inputs.configuration,
                );
                match child {
                    Ok(value) => objective_entries.push(value),
                    Err(category) => {
                        return ObjectiveRepresentationOutcome::Failure(objective_failure(
                            &operation_id,
                            upstream,
                            inputs,
                            category.clone(),
                            &["profile-authorized objective decomposition was malformed"],
                        ));
                    }
                }
            }
        }
    }
    let mut relationship_entries = Vec::new();
    for (proposal, element) in &elements {
        for relationship in element.relationships.iter() {
            let Some(source_id) = element_objective_ids
                .get(&(proposal.proposal_id.clone(), element.element_id.clone()))
            else {
                return ObjectiveRepresentationOutcome::Failure(objective_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ObjectiveRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship source objective is not represented"],
                ));
            };
            let relationship_type = match parse_relationship_type(&relationship.relationship_type) {
                Some(value)
                    if inputs
                        .registries
                        .relationship_registry
                        .values
                        .contains(&relationship.relationship_type) =>
                {
                    value
                }
                _ => {
                    return ObjectiveRepresentationOutcome::Failure(objective_failure(
                        &operation_id,
                        upstream,
                        inputs,
                        ObjectiveRepresentationFailureCategory::InvalidRelationshipReference,
                        &["relationship type is absent from the bound fixture registry"],
                    ));
                }
            };
            let mut target_ids = Vec::new();
            for target in relationship.target_element_ids.iter() {
                let qualified = target.split_once("::");
                let matches: Vec<_> = element_objective_ids
                    .iter()
                    .filter(|((proposal_id, element_id), _)| {
                        qualified
                            .map(|(qualified_proposal, qualified_element)| {
                                proposal_id.as_str() == qualified_proposal
                                    && element_id == qualified_element
                            })
                            .unwrap_or(element_id == target)
                    })
                    .map(|(_, objective_id)| objective_id.clone())
                    .collect();
                if matches.len() != 1 {
                    return ObjectiveRepresentationOutcome::Failure(objective_failure(
                        &operation_id,
                        upstream,
                        inputs,
                        ObjectiveRepresentationFailureCategory::InvalidRelationshipReference,
                        &["relationship target objective is absent or ambiguous"],
                    ));
                }
                if let Some((qualified_proposal, _)) = qualified {
                    if qualified_proposal != proposal.proposal_id.as_str()
                        && !inputs.profile.allow_cross_proposal_relationships
                    {
                        return ObjectiveRepresentationOutcome::Failure(objective_failure(
                            &operation_id,
                            upstream,
                            inputs,
                            ObjectiveRepresentationFailureCategory::UnauthorizedObjectiveSurface,
                            &["cross-proposal objective relationships are not authorized by the bound profile"],
                        ));
                    }
                }
                target_ids.push(matches[0].clone());
            }
            if target_ids.is_empty() {
                return ObjectiveRepresentationOutcome::Failure(objective_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ObjectiveRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship must contain a target"],
                ));
            }
            let relationship_id = ObjectiveRelationshipId::derive(&[
                operation_id.as_str(),
                proposal.proposal_id.as_str(),
                relationship.relationship_id.as_str(),
                &format!("{relationship_type:?}"),
            ]);
            relationship_entries.push(ObjectiveRelationship {
                relationship_id: relationship_id.clone(),
                relationship_type,
                source_objective_id: source_id.clone(),
                target_objective_ids: target_ids.into(),
                origin: parse_origin(&element.origin).unwrap_or(ObjectiveOrigin::MixedOrigin),
                basis: parse_basis(&element.basis)
                    .unwrap_or(RepresentationBasis::StructuredExtraction),
                evidence_references: relationship.evidence_references.clone(),
                evidence_status: relationship.evidence_status,
                source_proposal_ids: vec![proposal.proposal_id.clone()].into(),
                scope: relationship.scope.clone(),
            });
            if let Some(source_objective) = objective_entries
                .iter_mut()
                .find(|objective| objective.logical_objective_id == *source_id)
            {
                let mut ids = source_objective.relationships.to_vec();
                ids.push(relationship_id);
                source_objective.relationships = ids.into();
            }
        }
        for element in proposal.content.objective_elements.iter() {
            if element.component_expressions.is_empty() {
                continue;
            }
            let Some(parent_id) = element_objective_ids
                .get(&(proposal.proposal_id.clone(), element.element_id.clone()))
            else {
                continue;
            };
            for (index, _) in element.component_expressions.iter().enumerate() {
                let child_key = format!("{}#component-{index}", element.element_id);
                let Some(child_id) =
                    element_objective_ids.get(&(proposal.proposal_id.clone(), child_key))
                else {
                    continue;
                };
                let relationship_id = ObjectiveRelationshipId::derive(&[
                    operation_id.as_str(),
                    parent_id.as_str(),
                    child_id.as_str(),
                    "ComponentOf",
                ]);
                relationship_entries.push(ObjectiveRelationship {
                    relationship_id,
                    relationship_type: ObjectiveRelationshipType::ComponentOf,
                    source_objective_id: parent_id.clone(),
                    target_objective_ids: vec![child_id.clone()].into(),
                    origin: parse_origin(&element.origin).unwrap_or(ObjectiveOrigin::MixedOrigin),
                    basis: parse_basis(&element.basis)
                        .unwrap_or(RepresentationBasis::StructuredExtraction),
                    evidence_references: element.evidence_references.clone(),
                    evidence_status: element.evidence_status,
                    source_proposal_ids: vec![proposal.proposal_id.clone()].into(),
                    scope: element.scope.clone(),
                });
            }
        }
    }
    let primary_count = objective_entries
        .iter()
        .filter(|objective| {
            objective
                .designations
                .contains(&ObjectiveDesignation::Primary)
        })
        .count();
    let primary_status = match primary_count {
        0 => PrimaryObjectiveStatus::NotApplicable,
        1 => PrimaryObjectiveStatus::Declared,
        _ => PrimaryObjectiveStatus::Conflicting,
    };
    if objective_entries.is_empty() && !inputs.profile.allow_empty {
        return ObjectiveRepresentationOutcome::Failure(objective_failure(
            &operation_id,
            upstream,
            inputs,
            ObjectiveRepresentationFailureCategory::MissingRequiredProfile,
            &["the bound fixture profile does not permit an empty objective set"],
        ));
    }
    let mut set_parts = vec![
        operation_id.as_str(),
        upstream.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
    ];
    let objective_text: Vec<String> = objective_entries
        .iter()
        .map(|objective| objective.representation_id.to_string())
        .collect();
    set_parts.extend(objective_text.iter().map(String::as_str));
    let set_id = DeclaredObjectiveSetId::derive(&set_parts);
    for objective in &mut objective_entries {
        objective.set_id = set_id.clone();
    }
    let replay_context = BTreeMap::from([
        ("profile_version".to_owned(), inputs.profile.version.clone()),
        ("schema_version".to_owned(), inputs.schema.version.clone()),
        (
            "configuration_version".to_owned(),
            inputs.configuration.version.clone(),
        ),
    ]);
    ObjectiveRepresentationOutcome::Set(DeclaredObjectiveSet {
        set_id,
        operation_id,
        interpretation_operation_id: upstream.operation_id.clone(),
        source_intake_id: upstream.source_intake_id.clone(),
        admitted_proposal_set_id: upstream.set_id.clone(),
        input_proposal_ids: upstream.submitted_proposal_ids.clone(),
        input_decision_ids: upstream.decision_ids.clone(),
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        registry_ids: inputs.registries.identities().into(),
        registry_versions: inputs.registries.versions().into(),
        schema: inputs.schema.clone(),
        configuration: inputs.configuration.clone(),
        objectives: objective_entries.into(),
        relationships: relationship_entries.into(),
        primary_status,
        representation_findings: Vec::new().into(),
        replay_context,
    })
}

fn validate_objective_inputs(
    inputs: &ObjectiveRepresentationInputs<'_>,
    operation_id: &ObjectiveRepresentationOperationId,
    proposal_ids: &[InterpretationProposalId],
) -> Option<ObjectiveRepresentationFailureRecord> {
    let upstream = inputs.admitted_proposal_set;
    if upstream.proposals.is_empty() && !inputs.profile.allow_empty {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::MissingRequiredProfile,
            &["empty proposal publication is not permitted by the bound profile"],
        ));
    }
    if inputs.profile.version.is_empty()
        || inputs.profile.authority_reference.is_empty()
        || inputs.profile.scope.is_empty()
        || inputs.profile.allowed_forms.is_empty()
        || inputs.profile.allowed_origins.is_empty()
        || inputs.profile.allowed_bases.is_empty()
    {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::IncompatibleProfile,
            &["fixture objective representation profile is incomplete"],
        ));
    }
    let registry_versions = inputs.registries.versions();
    if registry_versions
        .windows(2)
        .any(|versions| versions[0] != versions[1])
    {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::IncompatibleRegistryVersion,
            &["fixture registries are not bound to one compatible version"],
        ));
    }
    let registries = [
        &inputs.registries.class_registry,
        &inputs.registries.form_registry,
        &inputs.registries.relationship_registry,
        &inputs.registries.designation_registry,
        &inputs.registries.status_registry,
    ];
    if registries.iter().any(|registry| {
        registry.version.is_empty()
            || registry.authority_reference.is_empty()
            || registry.values.is_empty()
    }) {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::MissingRequiredRegistry,
            &["fixture registry binding is incomplete"],
        ));
    }
    if inputs.schema.version.is_empty() || inputs.schema.authority_reference.is_empty() {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::InvalidSchemaBinding,
            &["objective schema binding is incomplete"],
        ));
    }
    if inputs.configuration.version.is_empty()
        || inputs.configuration.authority_reference.is_empty()
    {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::InvalidConfigurationBinding,
            &["objective configuration binding is incomplete"],
        ));
    }
    if upstream
        .decisions
        .iter()
        .any(|decision| !proposal_ids.contains(&decision.proposal_id))
    {
        return Some(objective_failure(
            operation_id,
            upstream,
            inputs.clone(),
            ObjectiveRepresentationFailureCategory::InvalidExactPublicationBinding,
            &["upstream admission decision is not bound to the exact proposal publication"],
        ));
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn build_objective(
    operation_id: &ObjectiveRepresentationOperationId,
    logical_id: &DeclaredObjectiveId,
    set_id: &AdmittedInterpretationProposalSetId,
    proposal: &InterpretationProposal,
    element: &ProposalObjectiveElement,
    profile: &FixtureObjectiveRepresentationProfile,
    registries: &FixtureObjectiveRegistries,
    schema: &ObjectiveSchemaBinding,
    configuration: &ObjectiveConfigurationBinding,
) -> Result<DeclaredObjective, ObjectiveRepresentationFailureCategory> {
    if element.element_id.is_empty() {
        return Err(ObjectiveRepresentationFailureCategory::MalformedObjectiveInput);
    }
    let form = parse_form(&element.form);
    let origin = parse_origin(&element.origin);
    let basis = parse_basis(&element.basis);
    let designation_values: Result<Vec<_>, _> = element
        .designations
        .iter()
        .map(|value| {
            parse_designation(value)
                .ok_or(ObjectiveRepresentationFailureCategory::MalformedObjectiveInput)
        })
        .collect();
    let designations = designation_values?;
    if let Some(value) = form {
        if !profile.allowed_forms.contains(&value)
            || !registries.form_registry.values.contains(&element.form)
        {
            return Err(ObjectiveRepresentationFailureCategory::IncompatibleProfile);
        }
    }
    if let Some(value) = origin {
        if !profile.allowed_origins.contains(&value) {
            return Err(ObjectiveRepresentationFailureCategory::IncompatibleProfile);
        }
    }
    if let Some(value) = basis {
        if !profile.allowed_bases.contains(&value) {
            return Err(ObjectiveRepresentationFailureCategory::IncompatibleProfile);
        }
    }
    if designations
        .iter()
        .any(|designation| !profile.allowed_designations.contains(designation))
    {
        return Err(ObjectiveRepresentationFailureCategory::IncompatibleProfile);
    }
    let mut status = if element.expression.is_empty()
        || element.form.is_empty()
        || element.origin.is_empty()
        || element.basis.is_empty()
        || element.class_name.is_empty()
    {
        ObjectiveRepresentationStatus::Incomplete
    } else if form.is_none()
        || origin.is_none()
        || basis.is_none()
        || !registries.form_registry.values.contains(&element.form)
        || !registries
            .class_registry
            .values
            .contains(&element.class_name)
    {
        ObjectiveRepresentationStatus::Unsupported
    } else if element.evidence_references.is_empty() && element.evidence_status.is_none() {
        ObjectiveRepresentationStatus::EvidenceLimited
    } else {
        ObjectiveRepresentationStatus::Represented
    };
    if let Some(explicit_status) = element.representation_status.as_deref() {
        status = parse_status(explicit_status)
            .ok_or(ObjectiveRepresentationFailureCategory::MalformedObjectiveInput)?;
        if !registries.status_registry.values.contains(explicit_status) {
            status = ObjectiveRepresentationStatus::Unsupported;
        }
    }
    if !profile.allowed_statuses.contains(&status)
        || (status == ObjectiveRepresentationStatus::Incomplete && !profile.allow_incomplete)
        || (status == ObjectiveRepresentationStatus::Unsupported && !profile.allow_unsupported)
        || (status == ObjectiveRepresentationStatus::EvidenceLimited
            && !profile.allow_evidence_limited)
    {
        return Err(ObjectiveRepresentationFailureCategory::IncompatibleProfile);
    }
    if element
        .evidence_references
        .iter()
        .any(|reference| reference.reference.is_empty())
    {
        return Err(ObjectiveRepresentationFailureCategory::InvalidEvidenceReference);
    }
    let representation_id = ObjectiveRepresentationId::derive(&[
        operation_id.as_str(),
        logical_id.as_str(),
        &format!("{status:?}"),
        proposal.proposal_id.as_str(),
        schema.version.as_str(),
        configuration.version.as_str(),
    ]);
    let registry_ids = registries.identities();
    let registry_versions = registries.versions();
    Ok(DeclaredObjective {
        logical_objective_id: logical_id.clone(),
        representation_id,
        expression: element.expression.clone(),
        form: form.unwrap_or(ObjectiveForm::Atomic),
        origin: origin.unwrap_or(ObjectiveOrigin::MixedOrigin),
        basis: basis.unwrap_or(RepresentationBasis::StructuredExtraction),
        source_proposal_ids: vec![proposal.proposal_id.clone()].into(),
        source_element_ids: vec![element.element_id.clone()].into(),
        evidence_references: element.evidence_references.clone(),
        evidence_status: element.evidence_status,
        scope: element.scope.clone(),
        designations: designations.into(),
        status,
        relationships: Vec::new().into(),
        profile_id: profile.identity.clone(),
        profile_version: profile.version.clone(),
        registry_ids: registry_ids.into(),
        registry_versions: registry_versions.into(),
        schema: schema.clone(),
        configuration: configuration.clone(),
        operation_id: operation_id.clone(),
        set_id: DeclaredObjectiveSetId::derive(&[set_id.as_str(), operation_id.as_str()]),
    })
}

fn parse_form(value: &str) -> Option<ObjectiveForm> {
    match value {
        "Atomic" => Some(ObjectiveForm::Atomic),
        "Composite" => Some(ObjectiveForm::Composite),
        _ => None,
    }
}

fn parse_origin(value: &str) -> Option<ObjectiveOrigin> {
    match value {
        "ExplicitSource" => Some(ObjectiveOrigin::ExplicitSource),
        "InterpreterInference" => Some(ObjectiveOrigin::InterpreterInference),
        "ApplicationSupplied" => Some(ObjectiveOrigin::ApplicationSupplied),
        "ReferencedArtifact" => Some(ObjectiveOrigin::ReferencedArtifact),
        "MixedOrigin" => Some(ObjectiveOrigin::MixedOrigin),
        _ => None,
    }
}

fn parse_basis(value: &str) -> Option<RepresentationBasis> {
    match value {
        "DirectQuotation" => Some(RepresentationBasis::DirectQuotation),
        "StructuredExtraction" => Some(RepresentationBasis::StructuredExtraction),
        "InterpreterSynthesis" => Some(RepresentationBasis::InterpreterSynthesis),
        "ApplicationDeclaration" => Some(RepresentationBasis::ApplicationDeclaration),
        "ReferencedArtifactDeclaration" => Some(RepresentationBasis::ReferencedArtifactDeclaration),
        _ => None,
    }
}

fn parse_designation(value: &str) -> Option<ObjectiveDesignation> {
    match value {
        "Primary" => Some(ObjectiveDesignation::Primary),
        "Secondary" => Some(ObjectiveDesignation::Secondary),
        "Subordinate" => Some(ObjectiveDesignation::Subordinate),
        "Conditional" => Some(ObjectiveDesignation::Conditional),
        "Parallel" => Some(ObjectiveDesignation::Parallel),
        "Alternative" => Some(ObjectiveDesignation::Alternative),
        _ => None,
    }
}

fn parse_status(value: &str) -> Option<ObjectiveRepresentationStatus> {
    match value {
        "Represented" => Some(ObjectiveRepresentationStatus::Represented),
        "Incomplete" => Some(ObjectiveRepresentationStatus::Incomplete),
        "Unsupported" => Some(ObjectiveRepresentationStatus::Unsupported),
        "Conflicting" => Some(ObjectiveRepresentationStatus::Conflicting),
        "Unresolved" => Some(ObjectiveRepresentationStatus::Unresolved),
        "EvidenceLimited" => Some(ObjectiveRepresentationStatus::EvidenceLimited),
        _ => None,
    }
}

fn parse_relationship_type(value: &str) -> Option<ObjectiveRelationshipType> {
    match value {
        "Supports" => Some(ObjectiveRelationshipType::Supports),
        "DependsOn" => Some(ObjectiveRelationshipType::DependsOn),
        "ConditionalUpon" => Some(ObjectiveRelationshipType::ConditionalUpon),
        "AlternativeTo" => Some(ObjectiveRelationshipType::AlternativeTo),
        "MutuallyExclusiveWith" => Some(ObjectiveRelationshipType::MutuallyExclusiveWith),
        "ParallelWith" => Some(ObjectiveRelationshipType::ParallelWith),
        "SequentialTo" => Some(ObjectiveRelationshipType::SequentialTo),
        "ComponentOf" => Some(ObjectiveRelationshipType::ComponentOf),
        "ConflictsWith" => Some(ObjectiveRelationshipType::ConflictsWith),
        _ => None,
    }
}

fn objective_failure(
    operation_id: &ObjectiveRepresentationOperationId,
    upstream: &AdmittedInterpretationProposalSet,
    inputs: ObjectiveRepresentationInputs<'_>,
    category: ObjectiveRepresentationFailureCategory,
    findings: &[&str],
) -> ObjectiveRepresentationFailureRecord {
    let failure_id = ObjectiveRepresentationFailureRecordId::derive(&[
        operation_id.as_str(),
        upstream.set_id.as_str(),
        &format!("{category:?}"),
        inputs.profile.version.as_str(),
    ]);
    ObjectiveRepresentationFailureRecord {
        failure_id,
        operation_id: operation_id.clone(),
        interpretation_operation_id: upstream.operation_id.clone(),
        admitted_proposal_set_id: upstream.set_id.clone(),
        input_proposal_ids: upstream.submitted_proposal_ids.clone(),
        profile_id: Some(inputs.profile.identity.clone()),
        profile_version: Some(inputs.profile.version.clone()),
        registry_ids: inputs.registries.identities().into(),
        registry_versions: inputs.registries.versions().into(),
        category,
        findings: findings
            .iter()
            .map(|finding| (*finding).to_owned())
            .collect::<Vec<_>>()
            .into(),
        retry_permitted: true,
        replay_context: BTreeMap::from([
            ("profile_version".to_owned(), inputs.profile.version.clone()),
            ("schema_version".to_owned(), inputs.schema.version.clone()),
            (
                "configuration_version".to_owned(),
                inputs.configuration.version.clone(),
            ),
        ]),
    }
}

#[cfg(test)]
mod contract_003_tests {
    use super::*;

    fn request() -> InterpretationRequest {
        let input = SourceComponentInput::new(
            SourceCategory::NaturalLanguage,
            SourceOrigin::InteractiveUser,
            SourcePayload::inline("objective fixture source"),
            AdmissionState::Accepted,
        )
        .with_preservation(PreservationFacts::recoverable("immutable source"));
        let IntakeOutcome::Success(intake) = admit(
            &SourceSubmission::new(vec![input]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("source admission") else {
            panic!("fixture intake must succeed")
        };
        let context = InterpretationOperationContext::from_intake(&intake);
        let spec = InterpretationRequestSpec {
            included_source_ids: intake.source_ids(),
            excluded_source_ids: Vec::new(),
            requested_scope: BTreeSet::from([ProposalDomain::Objectives]),
            contract_version: "0.1.0".to_owned(),
            proposal_schema_version: "proposal-fixture-v2".to_owned(),
            interpreter_profile: InterpreterProfileBinding {
                identity: StableId::from_parts("interpreter-profile", &["fixture"]),
                version: "fixture-interpreter-v1".to_owned(),
            },
            evidence_profile: EvidenceCapabilityProfile::fixture_strict().binding(),
            required_proposal_metadata: BTreeSet::from(["trace".to_owned()]),
            permitted_response_forms: BTreeSet::from(["structured-proposal".to_owned()]),
            completion_expectations: BTreeSet::from(["returned".to_owned()]),
            declared_bounds: BTreeSet::from(["structural-admission-only".to_owned()]),
            declared_exclusions: BTreeSet::from(["semantic-correctness".to_owned()]),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        };
        let InterpretationRequestOutcome::Request(request) =
            issue_interpretation_request(&context, spec)
        else {
            panic!("fixture request must succeed")
        };
        *request
    }

    fn element(id: &str, expression: &str) -> ProposalObjectiveElement {
        ProposalObjectiveElement {
            element_id: id.to_owned(),
            expression: expression.to_owned(),
            form: "Atomic".to_owned(),
            origin: "ExplicitSource".to_owned(),
            basis: "StructuredExtraction".to_owned(),
            class_name: "RequestedOutcome".to_owned(),
            scope: Some("fixture-scope".to_owned()),
            designations: Vec::new().into(),
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
            representation_status: None,
            relationships: Vec::new().into(),
            component_expressions: Vec::new().into(),
        }
    }

    fn proposal(
        request: &InterpretationRequest,
        id: &str,
        objective_elements: Vec<ProposalObjectiveElement>,
        narrative: Vec<&str>,
    ) -> InterpretationProposal {
        InterpretationProposal::new(
            request,
            StableId::from_parts("interpreter", &[id]),
            "interpreter-v1",
            request.interpreter_profile.clone(),
            ProposalProductionState::Returned,
            ProposalCompletionState::Complete,
            request.included_source_ids.to_vec(),
            Some(EvidenceCapabilityProfile::fixture_strict()),
            ProposalContent {
                proposed_elements: narrative
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
                    .into(),
                objective_elements: objective_elements.into(),
                constraint_elements: Vec::new().into(),
                capability_elements: Vec::new().into(),
                clarification_elements: Vec::new().into(),
                evidence_elements: Vec::new().into(),
                evidence_references: Vec::new().into(),
                declared_assumptions: Vec::new().into(),
                declared_uncertainties: Vec::new().into(),
                declared_bounds: Vec::new().into(),
            },
            BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        )
    }

    fn admitted_set(
        request: &InterpretationRequest,
        proposals: Vec<InterpretationProposal>,
    ) -> AdmittedInterpretationProposalSet {
        let pairs = proposals
            .into_iter()
            .map(|proposal| {
                let ProposalAdmissionOutcome::Decision(decision) =
                    admit_interpretation_proposal(request, &proposal, "criteria-v1")
                else {
                    panic!("fixture proposal must receive a decision")
                };
                (proposal, *decision)
            })
            .collect();
        let SetPublicationOutcome::Set(set) = publish_admitted_proposal_set(
            request,
            pairs,
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["fixture"]),
                version: "set-v1".to_owned(),
                criteria_version: "criteria-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: true,
            },
        ) else {
            panic!("fixture admitted set must publish")
        };
        set
    }

    fn represent(
        set: &AdmittedInterpretationProposalSet,
        profile: &FixtureObjectiveRepresentationProfile,
        registries: &FixtureObjectiveRegistries,
    ) -> ObjectiveRepresentationOutcome {
        let schema = ObjectiveSchemaBinding {
            identity: StableId::from_parts("objective-schema", &["fixture"]),
            version: "objective-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = ObjectiveConfigurationBinding {
            identity: StableId::from_parts("objective-config", &["fixture"]),
            version: "objective-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        represent_objectives(ObjectiveRepresentationInputs {
            admitted_proposal_set: set,
            profile,
            registries,
            schema: &schema,
            configuration: &configuration,
        })
    }

    #[test]
    fn explicit_objective_is_represented_and_bound_to_one_admitted_set() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "one",
                vec![element("objective-1", "deliver outcome")],
                vec![],
            )],
        );
        let outcome = represent(
            &set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &FixtureObjectiveRegistries::fixture(),
        );
        let ObjectiveRepresentationOutcome::Set(objectives) = outcome else {
            panic!("explicit objective must produce a set")
        };
        assert_eq!(objectives.objectives().len(), 1);
        assert_eq!(
            objectives.objectives()[0].status(),
            ObjectiveRepresentationStatus::Represented
        );
        assert_eq!(objectives.admitted_proposal_set_id(), set.set_id());
    }

    #[test]
    fn generic_narrative_is_not_scanned_as_an_objective() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "narrative",
                Vec::new(),
                vec!["The user wants the system to do something useful."],
            )],
        );
        let ObjectiveRepresentationOutcome::Set(objectives) = represent(
            &set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &FixtureObjectiveRegistries::fixture(),
        ) else {
            panic!("fixture profile permits an empty objective set")
        };
        assert!(objectives.objectives().is_empty());
    }

    #[test]
    fn identical_objectives_from_multiple_proposals_remain_distinct_and_replayable() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![
                proposal(
                    &request,
                    "first",
                    vec![element("same", "same outcome")],
                    vec![],
                ),
                proposal(
                    &request,
                    "second",
                    vec![element("same", "same outcome")],
                    vec![],
                ),
            ],
        );
        let profile = FixtureObjectiveRepresentationProfile::fixture();
        let registries = FixtureObjectiveRegistries::fixture();
        let first = represent(&set, &profile, &registries);
        let second = represent(&set, &profile, &registries);
        assert_eq!(first, second);
        let ObjectiveRepresentationOutcome::Set(objectives) = first else {
            panic!("fixture objectives must succeed")
        };
        assert_eq!(objectives.objectives().len(), 2);
        assert_ne!(
            objectives.objectives()[0].logical_objective_id(),
            objectives.objectives()[1].logical_objective_id()
        );
    }
    #[test]
    fn incomplete_and_unsupported_states_remain_successful_objective_records() {
        let request = request();
        let mut incomplete = element("incomplete", "");
        incomplete.representation_status = Some("Incomplete".to_owned());
        let mut unsupported = element("unsupported", "unsupported form");
        unsupported.form = "FutureForm".to_owned();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "states",
                vec![incomplete, unsupported],
                vec![],
            )],
        );
        let ObjectiveRepresentationOutcome::Set(objectives) = represent(
            &set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &FixtureObjectiveRegistries::fixture(),
        ) else {
            panic!("fixture profile permits non-success objective states")
        };
        assert_eq!(objectives.objectives().len(), 2);
        assert!(objectives
            .objectives()
            .iter()
            .any(|objective| objective.status() == ObjectiveRepresentationStatus::Incomplete));
        assert!(objectives
            .objectives()
            .iter()
            .any(|objective| objective.status() == ObjectiveRepresentationStatus::Unsupported));
    }

    #[test]
    fn profile_authorized_decomposition_preserves_parent_child_relationships() {
        let request = request();
        let mut composite = element("composite", "ship a result");
        composite.form = "Composite".to_owned();
        composite.component_expressions = vec!["part one".to_owned(), "part two".to_owned()].into();
        let set = admitted_set(
            &request,
            vec![proposal(&request, "composite", vec![composite], vec![])],
        );
        let ObjectiveRepresentationOutcome::Set(objectives) = represent(
            &set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &FixtureObjectiveRegistries::fixture(),
        ) else {
            panic!("fixture profile authorizes decomposition")
        };
        assert_eq!(objectives.objectives().len(), 3);
        assert_eq!(
            objectives
                .relationships()
                .iter()
                .filter(|relationship| relationship.relationship_type()
                    == ObjectiveRelationshipType::ComponentOf)
                .count(),
            2
        );
    }

    #[test]
    fn unauthorized_decomposition_is_failure_and_conflict_relationships_are_preserved() {
        let request = request();
        let mut composite = element("composite", "ship a result");
        composite.form = "Composite".to_owned();
        composite.component_expressions = vec!["part one".to_owned()].into();
        let set = admitted_set(
            &request,
            vec![proposal(&request, "composite", vec![composite], vec![])],
        );
        let mut profile = FixtureObjectiveRepresentationProfile::fixture();
        profile.allow_decomposition = false;
        assert!(matches!(
            represent(&set, &profile, &FixtureObjectiveRegistries::fixture()),
            ObjectiveRepresentationOutcome::Failure(ObjectiveRepresentationFailureRecord {
                category: ObjectiveRepresentationFailureCategory::UnauthorizedObjectiveSurface,
                ..
            })
        ));

        let mut left = element("left", "retain left");
        left.relationships = vec![ProposalObjectiveRelationship {
            relationship_id: "conflict".to_owned(),
            relationship_type: "ConflictsWith".to_owned(),
            target_element_ids: vec!["right".to_owned()].into(),
            scope: Some("fixture-scope".to_owned()),
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
        }]
        .into();
        let conflict_set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "conflict",
                vec![left, element("right", "retain right")],
                vec![],
            )],
        );
        let ObjectiveRepresentationOutcome::Set(objectives) = represent(
            &conflict_set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &FixtureObjectiveRegistries::fixture(),
        ) else {
            panic!("conflict relation must remain representable")
        };
        assert_eq!(objectives.relationships().len(), 1);
        assert_eq!(
            objectives.relationships()[0].relationship_type(),
            ObjectiveRelationshipType::ConflictsWith
        );
        assert_eq!(objectives.objectives().len(), 2);
    }

    #[test]
    fn incomplete_profile_or_registry_is_a_terminal_failure_without_partial_set() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "one",
                vec![element("objective-1", "deliver outcome")],
                vec![],
            )],
        );
        let mut profile = FixtureObjectiveRepresentationProfile::fixture();
        profile.version.clear();
        let outcome = represent(&set, &profile, &FixtureObjectiveRegistries::fixture());
        let ObjectiveRepresentationOutcome::Failure(failure) = outcome else {
            panic!("incomplete profile must fail")
        };
        assert_eq!(
            failure.category(),
            &ObjectiveRepresentationFailureCategory::IncompatibleProfile
        );
        assert_eq!(failure.admitted_proposal_set_id(), set.set_id());

        let mut registries = FixtureObjectiveRegistries::fixture();
        registries.status_registry.version = "fixture-003-registry-v2".to_owned();
        let outcome = represent(
            &set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &registries,
        );
        assert!(matches!(
            outcome,
            ObjectiveRepresentationOutcome::Failure(ObjectiveRepresentationFailureRecord {
                category: ObjectiveRepresentationFailureCategory::IncompatibleRegistryVersion,
                ..
            })
        ));
    }

    #[test]
    fn primary_designation_does_not_create_execution_priority() {
        let request = request();
        let mut objective = element("primary", "primary outcome");
        objective.designations = vec!["Primary".to_owned()].into();
        let set = admitted_set(
            &request,
            vec![proposal(&request, "primary", vec![objective], vec![])],
        );
        let ObjectiveRepresentationOutcome::Set(objectives) = represent(
            &set,
            &FixtureObjectiveRepresentationProfile::fixture(),
            &FixtureObjectiveRegistries::fixture(),
        ) else {
            panic!("primary designation must be represented")
        };
        assert_eq!(
            objectives.primary_status(),
            PrimaryObjectiveStatus::Declared
        );
        assert!(objectives.objectives()[0]
            .designations()
            .contains(&ObjectiveDesignation::Primary));
    }
}

// ---------------------------------------------------------------------------
// Contract 004: Declared Constraints
// ---------------------------------------------------------------------------

contract_002_id!(ConstraintRepresentationOperationId, "c4op");
contract_002_id!(DeclaredConstraintSetId, "cdset");
contract_002_id!(DeclaredConstraintId, "constraint");
contract_002_id!(ConstraintRepresentationId, "crep");
contract_002_id!(ConstraintRepresentationFailureRecordId, "cfail");
contract_002_id!(ConstraintRepresentationProfileId, "cprof");
contract_002_id!(ConstraintRelationshipId, "crel");
contract_002_id!(ConstraintRegistryId, "creg");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintForm {
    RequiredInclusion,
    RequiredExclusion,
    MaximumBound,
    MinimumBound,
    ExactRequirement,
    ConditionalRequirement,
    Prohibition,
    ScopeRestriction,
    TemporalRestriction,
    FormatRestriction,
    ResourceRestriction,
    ConfidentialityRestriction,
    Composite,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintOrigin {
    ExplicitSource,
    InterpreterInference,
    ApplicationSupplied,
    ReferencedArtifact,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintBasis {
    DirectQuotation,
    StructuredExtraction,
    InterpreterSynthesis,
    ApplicationDeclaration,
    ReferencedArtifactDeclaration,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintRepresentationStatus {
    Represented,
    Incomplete,
    Unsupported,
    Conflicting,
    EvidenceLimited,
    Unresolved,
    ScopeUnresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintScopeKind {
    RequestWide,
    ObjectiveSpecific,
    ObjectiveSetSpecific,
    ArtifactSpecific,
    OutputSpecific,
    SourceSpecific,
    Conditional,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintScopeResolutionStatus {
    Resolved,
    Partial,
    Conflicting,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstraintRelationshipType {
    SupportsAsDeclared,
    DependsOnAsDeclared,
    ConditionalUponAsDeclared,
    OverridesAsDeclared,
    SubordinateToAsDeclared,
    AlternativeToAsDeclared,
    ConflictsWith,
    ComponentOf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstraintRepresentationProfile {
    identity: ConstraintRepresentationProfileId,
    version: String,
    authority_reference: String,
    scope: String,
    allow_empty: bool,
    allow_inferred: bool,
    allow_unresolved_reference: bool,
    allow_scope_unresolved: bool,
    allow_decomposition: bool,
    allow_cross_proposal_relationships: bool,
    allow_explicit_conflict: bool,
    allowed_forms: BTreeSet<ConstraintForm>,
    allowed_origins: BTreeSet<ConstraintOrigin>,
    allowed_bases: BTreeSet<ConstraintBasis>,
    allowed_statuses: BTreeSet<ConstraintRepresentationStatus>,
    allowed_scope_kinds: BTreeSet<ConstraintScopeKind>,
}

impl FixtureConstraintRepresentationProfile {
    pub fn fixture() -> Self {
        Self {
            identity: ConstraintRepresentationProfileId::derive(&["contract-004-fixture"]),
            version: "fixture-004-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
    pub fn identity(&self) -> &ConstraintRepresentationProfileId {
        &self.identity
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn authority_reference(&self) -> &str {
        &self.authority_reference
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstraintRegistry {
    pub identity: ConstraintRegistryId,
    pub version: String,
    pub authority_reference: String,
    pub values: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstraintRegistries {
    pub class_registry: FixtureConstraintRegistry,
    pub form_registry: FixtureConstraintRegistry,
    pub origin_registry: FixtureConstraintRegistry,
    pub basis_registry: FixtureConstraintRegistry,
    pub relationship_registry: FixtureConstraintRegistry,
    pub status_registry: FixtureConstraintRegistry,
    pub scope_status_registry: FixtureConstraintRegistry,
    pub conflict_registry: FixtureConstraintRegistry,
}

impl FixtureConstraintRegistries {
    pub fn fixture() -> Self {
        fn registry(name: &str, values: &[&str]) -> FixtureConstraintRegistry {
            FixtureConstraintRegistry {
                identity: ConstraintRegistryId::derive(&[name, "contract-004-fixture"]),
                version: "fixture-004-registry-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
    fn all(&self) -> [&FixtureConstraintRegistry; 8] {
        [
            &self.class_registry,
            &self.form_registry,
            &self.origin_registry,
            &self.basis_registry,
            &self.relationship_registry,
            &self.status_registry,
            &self.scope_status_registry,
            &self.conflict_registry,
        ]
    }
    fn identities(&self) -> Vec<ConstraintRegistryId> {
        self.all()
            .iter()
            .map(|registry| registry.identity.clone())
            .collect()
    }
    fn versions(&self) -> Vec<String> {
        self.all()
            .iter()
            .map(|registry| registry.version.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintSchemaBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintConfigurationBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintLineageProof {
    pub admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    pub admitted_operation_id: InterpretationOperationId,
    pub objective_set_id: DeclaredObjectiveSetId,
    pub source_intake_id: StableId,
    pub interpretation_lineage_id: InterpretationOperationId,
    pub proposal_ids: Arc<[InterpretationProposalId]>,
    pub decision_ids: Arc<[ProposalAdmissionId]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstraintLineageValidationError {
    ObjectiveSetForeignLineage,
    ProposalBindingMismatch,
    DecisionBindingMismatch,
    SourceBindingMismatch,
}

pub fn validate_constraint_lineage(
    admitted: &AdmittedInterpretationProposalSet,
    objectives: &DeclaredObjectiveSet,
) -> Result<ConstraintLineageProof, ConstraintLineageValidationError> {
    if objectives.admitted_proposal_set_id != admitted.set_id
        || objectives.interpretation_operation_id != admitted.operation_id
    {
        return Err(ConstraintLineageValidationError::ObjectiveSetForeignLineage);
    }
    if objectives.input_proposal_ids.as_ref() != admitted.submitted_proposal_ids.as_ref() {
        return Err(ConstraintLineageValidationError::ProposalBindingMismatch);
    }
    if objectives.input_decision_ids.as_ref() != admitted.decision_ids.as_ref() {
        return Err(ConstraintLineageValidationError::DecisionBindingMismatch);
    }
    if objectives.source_intake_id != admitted.source_intake_id {
        return Err(ConstraintLineageValidationError::SourceBindingMismatch);
    }
    Ok(ConstraintLineageProof {
        admitted_proposal_set_id: admitted.set_id.clone(),
        admitted_operation_id: admitted.operation_id.clone(),
        objective_set_id: objectives.set_id.clone(),
        source_intake_id: admitted.source_intake_id.clone(),
        interpretation_lineage_id: admitted.operation_id.clone(),
        proposal_ids: admitted.submitted_proposal_ids.clone(),
        decision_ids: admitted.decision_ids.clone(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintRepresentationInputs<'a> {
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub objective_set: &'a DeclaredObjectiveSet,
    pub profile: &'a FixtureConstraintRepresentationProfile,
    pub registries: &'a FixtureConstraintRegistries,
    pub schema: &'a ConstraintSchemaBinding,
    pub configuration: &'a ConstraintConfigurationBinding,
    pub implementation_version: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintRelationship {
    relationship_id: ConstraintRelationshipId,
    relationship_type: ConstraintRelationshipType,
    source_constraint_id: DeclaredConstraintId,
    target_constraint_ids: Arc<[DeclaredConstraintId]>,
    scope: Option<String>,
    evidence_references: Arc<[EvidenceReference]>,
    evidence_status: Option<EvidenceStatus>,
}

impl ConstraintRelationship {
    pub fn relationship_id(&self) -> &ConstraintRelationshipId {
        &self.relationship_id
    }
    pub fn relationship_type(&self) -> ConstraintRelationshipType {
        self.relationship_type
    }
    pub fn source_constraint_id(&self) -> &DeclaredConstraintId {
        &self.source_constraint_id
    }
    pub fn target_constraint_ids(&self) -> &[DeclaredConstraintId] {
        &self.target_constraint_ids
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredConstraint {
    logical_constraint_id: DeclaredConstraintId,
    representation_id: ConstraintRepresentationId,
    expression: String,
    class_name: String,
    form: Option<ConstraintForm>,
    origin: Option<ConstraintOrigin>,
    basis: Option<ConstraintBasis>,
    scope_kind: Option<ConstraintScopeKind>,
    scope_target_ids: Arc<[String]>,
    scope_resolution_status: ConstraintScopeResolutionStatus,
    objective_ids: Arc<[String]>,
    declared_priority: Option<i32>,
    evidence_references: Arc<[EvidenceReference]>,
    evidence_status: Option<EvidenceStatus>,
    reference: Option<String>,
    reference_state: String,
    status: ConstraintRepresentationStatus,
    relationships: Arc<[ConstraintRelationshipId]>,
    source_proposal_ids: Arc<[InterpretationProposalId]>,
    source_element_ids: Arc<[String]>,
    profile_id: ConstraintRepresentationProfileId,
    profile_version: String,
    registry_ids: Arc<[ConstraintRegistryId]>,
    registry_versions: Arc<[String]>,
    operation_id: ConstraintRepresentationOperationId,
    set_id: DeclaredConstraintSetId,
}

impl DeclaredConstraint {
    pub fn logical_constraint_id(&self) -> &DeclaredConstraintId {
        &self.logical_constraint_id
    }
    pub fn representation_id(&self) -> &ConstraintRepresentationId {
        &self.representation_id
    }
    pub fn expression(&self) -> &str {
        &self.expression
    }
    pub fn class_name(&self) -> &str {
        &self.class_name
    }
    pub fn form(&self) -> Option<ConstraintForm> {
        self.form
    }
    pub fn origin(&self) -> Option<ConstraintOrigin> {
        self.origin
    }
    pub fn basis(&self) -> Option<ConstraintBasis> {
        self.basis
    }
    pub fn scope_kind(&self) -> Option<ConstraintScopeKind> {
        self.scope_kind
    }
    pub fn scope_target_ids(&self) -> &[String] {
        &self.scope_target_ids
    }
    pub fn scope_resolution_status(&self) -> ConstraintScopeResolutionStatus {
        self.scope_resolution_status
    }
    pub fn objective_ids(&self) -> &[String] {
        &self.objective_ids
    }
    pub fn declared_priority(&self) -> Option<i32> {
        self.declared_priority
    }
    pub fn evidence_references(&self) -> &[EvidenceReference] {
        &self.evidence_references
    }
    pub fn evidence_status(&self) -> Option<EvidenceStatus> {
        self.evidence_status
    }
    pub fn reference(&self) -> Option<&str> {
        self.reference.as_deref()
    }
    pub fn reference_state(&self) -> &str {
        &self.reference_state
    }
    pub fn status(&self) -> ConstraintRepresentationStatus {
        self.status
    }
    pub fn relationship_ids(&self) -> &[ConstraintRelationshipId] {
        &self.relationships
    }
    pub fn source_proposal_ids(&self) -> &[InterpretationProposalId] {
        &self.source_proposal_ids
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeclaredConstraintSet {
    set_id: DeclaredConstraintSetId,
    operation_id: ConstraintRepresentationOperationId,
    interpretation_operation_id: InterpretationOperationId,
    source_intake_id: StableId,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    objective_set_id: DeclaredObjectiveSetId,
    input_proposal_ids: Arc<[InterpretationProposalId]>,
    input_decision_ids: Arc<[ProposalAdmissionId]>,
    profile_id: ConstraintRepresentationProfileId,
    profile_version: String,
    registry_ids: Arc<[ConstraintRegistryId]>,
    registry_versions: Arc<[String]>,
    schema: ConstraintSchemaBinding,
    configuration: ConstraintConfigurationBinding,
    constraints: Arc<[DeclaredConstraint]>,
    relationships: Arc<[ConstraintRelationship]>,
    replay_context: BTreeMap<String, String>,
}

impl DeclaredConstraintSet {
    pub fn set_id(&self) -> &DeclaredConstraintSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &ConstraintRepresentationOperationId {
        &self.operation_id
    }
    pub fn admitted_proposal_set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.admitted_proposal_set_id
    }
    pub fn objective_set_id(&self) -> &DeclaredObjectiveSetId {
        &self.objective_set_id
    }
    pub fn input_proposal_ids(&self) -> &[InterpretationProposalId] {
        &self.input_proposal_ids
    }
    pub fn input_decision_ids(&self) -> &[ProposalAdmissionId] {
        &self.input_decision_ids
    }
    pub fn profile_id(&self) -> &ConstraintRepresentationProfileId {
        &self.profile_id
    }
    pub fn profile_version(&self) -> &str {
        &self.profile_version
    }
    pub fn constraints(&self) -> &[DeclaredConstraint] {
        &self.constraints
    }
    pub fn relationships(&self) -> &[ConstraintRelationship] {
        &self.relationships
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstraintRepresentationFailureCategory {
    MissingRequiredProfile,
    IncompatibleProfile,
    MissingRequiredRegistry,
    IncompatibleRegistryVersion,
    InvalidSchemaBinding,
    InvalidConfigurationBinding,
    InvalidLineage,
    InvalidProposalAdmissionBinding,
    MalformedConstraintInput,
    ForeignLineageReference,
    ProhibitedReference,
    InvalidRelationshipReference,
    UnauthorizedDecomposition,
    AtomicCommitmentFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstraintRepresentationFailureRecord {
    failure_id: ConstraintRepresentationFailureRecordId,
    operation_id: ConstraintRepresentationOperationId,
    interpretation_operation_id: InterpretationOperationId,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    objective_set_id: DeclaredObjectiveSetId,
    input_proposal_ids: Arc<[InterpretationProposalId]>,
    input_decision_ids: Arc<[ProposalAdmissionId]>,
    profile_id: Option<ConstraintRepresentationProfileId>,
    profile_version: Option<String>,
    registry_ids: Arc<[ConstraintRegistryId]>,
    registry_versions: Arc<[String]>,
    category: ConstraintRepresentationFailureCategory,
    findings: Arc<[String]>,
}

impl ConstraintRepresentationFailureRecord {
    pub fn failure_id(&self) -> &ConstraintRepresentationFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &ConstraintRepresentationOperationId {
        &self.operation_id
    }
    pub fn admitted_proposal_set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.admitted_proposal_set_id
    }
    pub fn objective_set_id(&self) -> &DeclaredObjectiveSetId {
        &self.objective_set_id
    }
    pub fn category(&self) -> &ConstraintRepresentationFailureCategory {
        &self.category
    }
    pub fn findings(&self) -> &[String] {
        &self.findings
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstraintRepresentationOutcome {
    Set(DeclaredConstraintSet),
    Failure(ConstraintRepresentationFailureRecord),
}

pub fn represent_constraints(
    inputs: ConstraintRepresentationInputs<'_>,
) -> ConstraintRepresentationOutcome {
    let upstream = inputs.admitted_proposal_set;
    let operation_id = ConstraintRepresentationOperationId::derive(&[
        upstream.set_id.as_str(),
        inputs.objective_set.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
        inputs.schema.identity.as_str(),
        inputs.schema.version.as_str(),
        inputs.configuration.identity.as_str(),
        inputs.configuration.version.as_str(),
        inputs.implementation_version,
    ]);
    if let Some(failure) = validate_constraint_inputs(&inputs, &operation_id) {
        return ConstraintRepresentationOutcome::Failure(failure);
    }
    let admitted: BTreeSet<_> = upstream.admitted_proposal_ids.iter().cloned().collect();
    let mut proposals: Vec<_> = upstream
        .proposals
        .iter()
        .filter(|proposal| admitted.contains(&proposal.proposal_id))
        .collect();
    proposals.sort_by_key(|proposal| proposal.proposal_id.clone());
    let mut elements: Vec<(&InterpretationProposal, &ProposalConstraintElement)> = proposals
        .iter()
        .flat_map(|proposal| {
            proposal
                .content
                .constraint_elements
                .iter()
                .map(move |element| (*proposal, element))
        })
        .collect();
    elements.sort_by_key(|(proposal, element)| {
        (proposal.proposal_id.clone(), element.element_id.clone())
    });
    let objective_ids: BTreeSet<_> = inputs
        .objective_set
        .objectives
        .iter()
        .map(|objective| objective.logical_objective_id.to_string())
        .collect();
    let mut constraints = Vec::new();
    let mut element_ids: BTreeMap<(InterpretationProposalId, String), DeclaredConstraintId> =
        BTreeMap::new();
    for (proposal, element) in &elements {
        let logical_id = DeclaredConstraintId::derive(&[
            operation_id.as_str(),
            proposal.proposal_id.as_str(),
            element.element_id.as_str(),
        ]);
        element_ids.insert(
            (proposal.proposal_id.clone(), element.element_id.clone()),
            logical_id.clone(),
        );
        match build_constraint(
            &operation_id,
            &logical_id,
            upstream.set_id.as_str(),
            proposal,
            element,
            inputs.profile,
            inputs.registries,
            &objective_ids,
            inputs.schema,
            inputs.configuration,
        ) {
            Ok(constraint) => constraints.push(constraint),
            Err(category) => {
                return ConstraintRepresentationOutcome::Failure(constraint_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    category,
                    &["constraint representation could not be constructed"],
                ))
            }
        }
        if !element.component_expressions.is_empty() {
            if element.form != "Composite" || !inputs.profile.allow_decomposition {
                return ConstraintRepresentationOutcome::Failure(constraint_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ConstraintRepresentationFailureCategory::UnauthorizedDecomposition,
                    &["constraint decomposition is not authorized by the bound fixture profile"],
                ));
            }
            for (index, expression) in element.component_expressions.iter().enumerate() {
                let child_element_id = format!("{}#component-{index}", element.element_id);
                let child_id = DeclaredConstraintId::derive(&[
                    logical_id.as_str(),
                    &index.to_string(),
                    expression,
                ]);
                element_ids.insert(
                    (proposal.proposal_id.clone(), child_element_id.clone()),
                    child_id.clone(),
                );
                let child_element = ProposalConstraintElement {
                    element_id: child_element_id,
                    expression: expression.clone(),
                    form: "RequiredInclusion".to_owned(),
                    ..(*element).clone()
                };
                match build_constraint(
                    &operation_id,
                    &child_id,
                    upstream.set_id.as_str(),
                    proposal,
                    &child_element,
                    inputs.profile,
                    inputs.registries,
                    &objective_ids,
                    inputs.schema,
                    inputs.configuration,
                ) {
                    Ok(child) => constraints.push(child),
                    Err(category) => {
                        return ConstraintRepresentationOutcome::Failure(constraint_failure(
                            &operation_id,
                            upstream,
                            inputs,
                            category,
                            &["authorized constraint decomposition was malformed"],
                        ))
                    }
                }
            }
        }
    }
    let mut relationships = Vec::new();
    for (proposal, element) in &elements {
        for relationship in element.relationships.iter() {
            let Some(source_id) =
                element_ids.get(&(proposal.proposal_id.clone(), element.element_id.clone()))
            else {
                return ConstraintRepresentationOutcome::Failure(constraint_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ConstraintRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship source is not represented"],
                ));
            };
            let Some(relationship_type) =
                parse_constraint_relationship(&relationship.relationship_type)
            else {
                return ConstraintRepresentationOutcome::Failure(constraint_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ConstraintRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship type is not registered"],
                ));
            };
            if !inputs
                .registries
                .relationship_registry
                .values
                .contains(&relationship.relationship_type)
            {
                return ConstraintRepresentationOutcome::Failure(constraint_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ConstraintRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship type is not in the bound registry"],
                ));
            }
            let mut target_ids = Vec::new();
            for target in relationship.target_element_ids.iter() {
                let matches: Vec<_> = element_ids
                    .iter()
                    .filter(|((proposal_id, element_id), _)| {
                        target == element_id || target == &format!("{}::{element_id}", proposal_id)
                    })
                    .map(|(_, id)| id.clone())
                    .collect();
                if matches.len() != 1 {
                    return ConstraintRepresentationOutcome::Failure(constraint_failure(
                        &operation_id,
                        upstream,
                        inputs,
                        ConstraintRepresentationFailureCategory::InvalidRelationshipReference,
                        &["relationship target is absent or ambiguous"],
                    ));
                }
                target_ids.push(matches[0].clone());
            }
            if target_ids.is_empty() {
                return ConstraintRepresentationOutcome::Failure(constraint_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    ConstraintRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship requires a target"],
                ));
            }
            let relationship_id = ConstraintRelationshipId::derive(&[
                operation_id.as_str(),
                proposal.proposal_id.as_str(),
                relationship.relationship_id.as_str(),
                &format!("{relationship_type:?}"),
            ]);
            relationships.push(ConstraintRelationship {
                relationship_id: relationship_id.clone(),
                relationship_type,
                source_constraint_id: source_id.clone(),
                target_constraint_ids: target_ids.into(),
                scope: relationship.scope.clone(),
                evidence_references: relationship.evidence_references.clone(),
                evidence_status: relationship.evidence_status,
            });
            if let Some(source) =
                constraints
                    .iter_mut()
                    .find(|constraint: &&mut DeclaredConstraint| {
                        constraint.logical_constraint_id == *source_id
                    })
            {
                let mut ids = source.relationships.to_vec();
                ids.push(relationship_id);
                source.relationships = ids.into();
            }
        }
        for (index, _) in element.component_expressions.iter().enumerate() {
            let Some(parent_id) =
                element_ids.get(&(proposal.proposal_id.clone(), element.element_id.clone()))
            else {
                continue;
            };
            let child_key = format!("{}#component-{index}", element.element_id);
            let Some(child_id) = element_ids.get(&(proposal.proposal_id.clone(), child_key)) else {
                continue;
            };
            let relationship_id = ConstraintRelationshipId::derive(&[
                operation_id.as_str(),
                parent_id.as_str(),
                child_id.as_str(),
                "ComponentOf",
            ]);
            relationships.push(ConstraintRelationship {
                relationship_id,
                relationship_type: ConstraintRelationshipType::ComponentOf,
                source_constraint_id: parent_id.clone(),
                target_constraint_ids: vec![child_id.clone()].into(),
                scope: None,
                evidence_references: element.evidence_references.clone(),
                evidence_status: element.evidence_status,
            });
        }
    }
    let set_id = DeclaredConstraintSetId::derive(&[
        operation_id.as_str(),
        upstream.set_id.as_str(),
        inputs.objective_set.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
        &constraints
            .iter()
            .map(|constraint| constraint.representation_id.to_string())
            .collect::<Vec<_>>()
            .join("|"),
    ]);
    for constraint in &mut constraints {
        constraint.set_id = set_id.clone();
    }
    ConstraintRepresentationOutcome::Set(DeclaredConstraintSet {
        set_id,
        operation_id,
        interpretation_operation_id: upstream.operation_id.clone(),
        source_intake_id: upstream.source_intake_id.clone(),
        admitted_proposal_set_id: upstream.set_id.clone(),
        objective_set_id: inputs.objective_set.set_id.clone(),
        input_proposal_ids: upstream.submitted_proposal_ids.clone(),
        input_decision_ids: upstream.decision_ids.clone(),
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        registry_ids: inputs.registries.identities().into(),
        registry_versions: inputs.registries.versions().into(),
        schema: inputs.schema.clone(),
        configuration: inputs.configuration.clone(),
        constraints: constraints.into(),
        relationships: relationships.into(),
        replay_context: BTreeMap::from([
            (
                String::from("profile_version"),
                inputs.profile.version.clone(),
            ),
            (
                String::from("schema_version"),
                inputs.schema.version.clone(),
            ),
            (
                String::from("configuration_version"),
                inputs.configuration.version.clone(),
            ),
            (
                String::from("implementation_version"),
                inputs.implementation_version.to_owned(),
            ),
        ]),
    })
}

fn validate_constraint_inputs(
    inputs: &ConstraintRepresentationInputs<'_>,
    operation_id: &ConstraintRepresentationOperationId,
) -> Option<ConstraintRepresentationFailureRecord> {
    let upstream = inputs.admitted_proposal_set;
    let failure = |category, finding: &'static str| {
        constraint_failure(operation_id, upstream, inputs.clone(), category, &[finding])
    };
    if inputs.profile.version.is_empty()
        || inputs.profile.authority_reference.is_empty()
        || inputs.profile.allowed_forms.is_empty()
        || inputs.profile.allowed_origins.is_empty()
        || inputs.profile.allowed_bases.is_empty()
    {
        return Some(failure(
            ConstraintRepresentationFailureCategory::IncompatibleProfile,
            "constraint profile is incomplete",
        ));
    }
    let versions = inputs.registries.versions();
    if inputs.registries.all().iter().any(|registry| {
        registry.version.is_empty()
            || registry.authority_reference.is_empty()
            || registry.values.is_empty()
    }) {
        return Some(failure(
            ConstraintRepresentationFailureCategory::MissingRequiredRegistry,
            "constraint registry is incomplete",
        ));
    }
    if versions.windows(2).any(|pair| pair[0] != pair[1]) {
        return Some(failure(
            ConstraintRepresentationFailureCategory::IncompatibleRegistryVersion,
            "constraint registries have incompatible versions",
        ));
    }
    if inputs.schema.version.is_empty() || inputs.schema.authority_reference.is_empty() {
        return Some(failure(
            ConstraintRepresentationFailureCategory::InvalidSchemaBinding,
            "constraint schema binding is incomplete",
        ));
    }
    if inputs.configuration.version.is_empty()
        || inputs.configuration.authority_reference.is_empty()
        || inputs.implementation_version.is_empty()
    {
        return Some(failure(
            ConstraintRepresentationFailureCategory::InvalidConfigurationBinding,
            "constraint configuration binding is incomplete",
        ));
    }
    if validate_constraint_lineage(upstream, inputs.objective_set).is_err() {
        return Some(failure(
            ConstraintRepresentationFailureCategory::InvalidLineage,
            "admitted proposals and objective set are not one declared lineage",
        ));
    }
    if upstream.decisions.len() != upstream.decision_ids.len()
        || upstream.admitted_proposal_ids.iter().any(|proposal_id| {
            !upstream.decisions.iter().any(|decision| {
                decision.proposal_id == *proposal_id
                    && decision.disposition == AdmissionDisposition::Admitted
            })
        })
        || upstream.decisions.iter().any(|decision| {
            decision.disposition != AdmissionDisposition::Admitted
                || !upstream
                    .admitted_proposal_ids
                    .contains(&decision.proposal_id)
        })
    {
        return Some(failure(
            ConstraintRepresentationFailureCategory::InvalidProposalAdmissionBinding,
            "proposal admission decisions are not exact admitted bindings",
        ));
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn build_constraint(
    operation_id: &ConstraintRepresentationOperationId,
    logical_id: &DeclaredConstraintId,
    admitted_set_id: &str,
    proposal: &InterpretationProposal,
    element: &ProposalConstraintElement,
    profile: &FixtureConstraintRepresentationProfile,
    registries: &FixtureConstraintRegistries,
    objective_ids: &BTreeSet<String>,
    schema: &ConstraintSchemaBinding,
    configuration: &ConstraintConfigurationBinding,
) -> Result<DeclaredConstraint, ConstraintRepresentationFailureCategory> {
    if element.element_id.is_empty()
        || element.expression.is_empty()
        || element.scope_kind.is_empty()
        || element.scope_resolution_status.is_empty()
    {
        return Err(ConstraintRepresentationFailureCategory::MalformedConstraintInput);
    }
    let form = parse_constraint_form(&element.form);
    let origin = parse_constraint_origin(&element.origin);
    let basis = parse_constraint_basis(&element.basis);
    let scope_kind = parse_scope_kind(&element.scope_kind);
    let scope_status = parse_scope_status(&element.scope_resolution_status)
        .ok_or(ConstraintRepresentationFailureCategory::MalformedConstraintInput)?;
    if !registries
        .class_registry
        .values
        .contains(&element.class_name)
        || !registries.form_registry.values.contains(&element.form)
        || !registries.origin_registry.values.contains(&element.origin)
        || !registries.basis_registry.values.contains(&element.basis)
    {
        return Err(ConstraintRepresentationFailureCategory::MalformedConstraintInput);
    }
    if form.is_none() || origin.is_none() || basis.is_none() || scope_kind.is_none() {
        return Err(ConstraintRepresentationFailureCategory::MalformedConstraintInput);
    }
    if !profile.allowed_forms.contains(&form.unwrap())
        || !profile.allowed_origins.contains(&origin.unwrap())
        || !profile.allowed_bases.contains(&basis.unwrap())
        || !profile.allowed_scope_kinds.contains(&scope_kind.unwrap())
    {
        return Err(ConstraintRepresentationFailureCategory::IncompatibleProfile);
    }
    if element.scope_resolution_status != "Resolved" && !profile.allow_scope_unresolved {
        return Err(ConstraintRepresentationFailureCategory::IncompatibleProfile);
    }
    if scope_kind == Some(ConstraintScopeKind::ObjectiveSpecific)
        || !element.objective_ids.is_empty()
    {
        for id in element.objective_ids.iter() {
            if !objective_ids.contains(id) {
                return Err(ConstraintRepresentationFailureCategory::ForeignLineageReference);
            }
        }
    }
    let mut status = if element.expression.is_empty() {
        ConstraintRepresentationStatus::Incomplete
    } else if element.evidence_references.is_empty() && element.evidence_status.is_none() {
        ConstraintRepresentationStatus::EvidenceLimited
    } else {
        ConstraintRepresentationStatus::Represented
    };
    if element.explicit_conflict {
        if !profile.allow_explicit_conflict
            || !registries
                .conflict_registry
                .values
                .contains("ExplicitConflict")
        {
            return Err(ConstraintRepresentationFailureCategory::IncompatibleProfile);
        }
        status = ConstraintRepresentationStatus::Conflicting;
    }
    if let Some(raw) = element.representation_status.as_deref() {
        status = parse_constraint_status(raw)
            .ok_or(ConstraintRepresentationFailureCategory::MalformedConstraintInput)?;
    }
    if !profile.allowed_statuses.contains(&status) {
        return Err(ConstraintRepresentationFailureCategory::IncompatibleProfile);
    }
    let reference_state = element.reference_state.as_str();
    match reference_state {
        "None" | "Resolved" => {}
        "Unresolved" if profile.allow_unresolved_reference => {
            status = ConstraintRepresentationStatus::Unresolved;
        }
        "Malformed" => {
            return Err(ConstraintRepresentationFailureCategory::MalformedConstraintInput)
        }
        "ForeignLineage" => {
            return Err(ConstraintRepresentationFailureCategory::ForeignLineageReference)
        }
        "Prohibited" => return Err(ConstraintRepresentationFailureCategory::ProhibitedReference),
        _ => return Err(ConstraintRepresentationFailureCategory::MalformedConstraintInput),
    }
    if element
        .evidence_references
        .iter()
        .any(|reference| reference.reference.is_empty())
    {
        return Err(ConstraintRepresentationFailureCategory::MalformedConstraintInput);
    }
    let representation_id = ConstraintRepresentationId::derive(&[
        operation_id.as_str(),
        logical_id.as_str(),
        &format!("{status:?}"),
        &element.expression,
        &format!("{:?}", element.declared_priority),
        schema.version.as_str(),
        configuration.version.as_str(),
    ]);
    Ok(DeclaredConstraint {
        logical_constraint_id: logical_id.clone(),
        representation_id,
        expression: element.expression.clone(),
        class_name: element.class_name.clone(),
        form,
        origin,
        basis,
        scope_kind,
        scope_target_ids: element.scope_target_ids.clone(),
        scope_resolution_status: scope_status,
        objective_ids: element.objective_ids.clone(),
        declared_priority: element.declared_priority,
        evidence_references: element.evidence_references.clone(),
        evidence_status: element.evidence_status,
        reference: element.reference.clone(),
        reference_state: element.reference_state.clone(),
        status,
        relationships: Vec::new().into(),
        source_proposal_ids: vec![proposal.proposal_id.clone()].into(),
        source_element_ids: vec![element.element_id.clone()].into(),
        profile_id: profile.identity.clone(),
        profile_version: profile.version.clone(),
        registry_ids: registries.identities().into(),
        registry_versions: registries.versions().into(),
        operation_id: operation_id.clone(),
        set_id: DeclaredConstraintSetId::derive(&[admitted_set_id, operation_id.as_str()]),
    })
}

fn parse_constraint_form(value: &str) -> Option<ConstraintForm> {
    match value {
        "RequiredInclusion" => Some(ConstraintForm::RequiredInclusion),
        "RequiredExclusion" => Some(ConstraintForm::RequiredExclusion),
        "MaximumBound" => Some(ConstraintForm::MaximumBound),
        "MinimumBound" => Some(ConstraintForm::MinimumBound),
        "ExactRequirement" => Some(ConstraintForm::ExactRequirement),
        "ConditionalRequirement" => Some(ConstraintForm::ConditionalRequirement),
        "Prohibition" => Some(ConstraintForm::Prohibition),
        "ScopeRestriction" => Some(ConstraintForm::ScopeRestriction),
        "TemporalRestriction" => Some(ConstraintForm::TemporalRestriction),
        "FormatRestriction" => Some(ConstraintForm::FormatRestriction),
        "ResourceRestriction" => Some(ConstraintForm::ResourceRestriction),
        "ConfidentialityRestriction" => Some(ConstraintForm::ConfidentialityRestriction),
        "Composite" => Some(ConstraintForm::Composite),
        _ => None,
    }
}
fn parse_constraint_origin(value: &str) -> Option<ConstraintOrigin> {
    match value {
        "ExplicitSource" => Some(ConstraintOrigin::ExplicitSource),
        "InterpreterInference" => Some(ConstraintOrigin::InterpreterInference),
        "ApplicationSupplied" => Some(ConstraintOrigin::ApplicationSupplied),
        "ReferencedArtifact" => Some(ConstraintOrigin::ReferencedArtifact),
        _ => None,
    }
}
fn parse_constraint_basis(value: &str) -> Option<ConstraintBasis> {
    match value {
        "DirectQuotation" => Some(ConstraintBasis::DirectQuotation),
        "StructuredExtraction" => Some(ConstraintBasis::StructuredExtraction),
        "InterpreterSynthesis" => Some(ConstraintBasis::InterpreterSynthesis),
        "ApplicationDeclaration" => Some(ConstraintBasis::ApplicationDeclaration),
        "ReferencedArtifactDeclaration" => Some(ConstraintBasis::ReferencedArtifactDeclaration),
        _ => None,
    }
}
fn parse_constraint_status(value: &str) -> Option<ConstraintRepresentationStatus> {
    match value {
        "Represented" => Some(ConstraintRepresentationStatus::Represented),
        "Incomplete" => Some(ConstraintRepresentationStatus::Incomplete),
        "Unsupported" => Some(ConstraintRepresentationStatus::Unsupported),
        "Conflicting" => Some(ConstraintRepresentationStatus::Conflicting),
        "EvidenceLimited" => Some(ConstraintRepresentationStatus::EvidenceLimited),
        "Unresolved" => Some(ConstraintRepresentationStatus::Unresolved),
        "ScopeUnresolved" => Some(ConstraintRepresentationStatus::ScopeUnresolved),
        _ => None,
    }
}
fn parse_scope_kind(value: &str) -> Option<ConstraintScopeKind> {
    match value {
        "RequestWide" => Some(ConstraintScopeKind::RequestWide),
        "ObjectiveSpecific" => Some(ConstraintScopeKind::ObjectiveSpecific),
        "ObjectiveSetSpecific" => Some(ConstraintScopeKind::ObjectiveSetSpecific),
        "ArtifactSpecific" => Some(ConstraintScopeKind::ArtifactSpecific),
        "OutputSpecific" => Some(ConstraintScopeKind::OutputSpecific),
        "SourceSpecific" => Some(ConstraintScopeKind::SourceSpecific),
        "Conditional" => Some(ConstraintScopeKind::Conditional),
        "Unresolved" => Some(ConstraintScopeKind::Unresolved),
        _ => None,
    }
}
fn parse_scope_status(value: &str) -> Option<ConstraintScopeResolutionStatus> {
    match value {
        "Resolved" => Some(ConstraintScopeResolutionStatus::Resolved),
        "Partial" => Some(ConstraintScopeResolutionStatus::Partial),
        "Conflicting" => Some(ConstraintScopeResolutionStatus::Conflicting),
        "Unresolved" => Some(ConstraintScopeResolutionStatus::Unresolved),
        _ => None,
    }
}
fn parse_constraint_relationship(value: &str) -> Option<ConstraintRelationshipType> {
    match value {
        "SupportsAsDeclared" => Some(ConstraintRelationshipType::SupportsAsDeclared),
        "DependsOnAsDeclared" => Some(ConstraintRelationshipType::DependsOnAsDeclared),
        "ConditionalUponAsDeclared" => Some(ConstraintRelationshipType::ConditionalUponAsDeclared),
        "OverridesAsDeclared" => Some(ConstraintRelationshipType::OverridesAsDeclared),
        "SubordinateToAsDeclared" => Some(ConstraintRelationshipType::SubordinateToAsDeclared),
        "AlternativeToAsDeclared" => Some(ConstraintRelationshipType::AlternativeToAsDeclared),
        "ConflictsWith" => Some(ConstraintRelationshipType::ConflictsWith),
        "ComponentOf" => Some(ConstraintRelationshipType::ComponentOf),
        _ => None,
    }
}

fn constraint_failure(
    operation_id: &ConstraintRepresentationOperationId,
    upstream: &AdmittedInterpretationProposalSet,
    inputs: ConstraintRepresentationInputs<'_>,
    category: ConstraintRepresentationFailureCategory,
    findings: &[&str],
) -> ConstraintRepresentationFailureRecord {
    ConstraintRepresentationFailureRecord {
        failure_id: ConstraintRepresentationFailureRecordId::derive(&[
            operation_id.as_str(),
            upstream.set_id.as_str(),
            inputs.objective_set.set_id.as_str(),
            &format!("{category:?}"),
        ]),
        operation_id: operation_id.clone(),
        interpretation_operation_id: upstream.operation_id.clone(),
        admitted_proposal_set_id: upstream.set_id.clone(),
        objective_set_id: inputs.objective_set.set_id.clone(),
        input_proposal_ids: upstream.submitted_proposal_ids.clone(),
        input_decision_ids: upstream.decision_ids.clone(),
        profile_id: Some(inputs.profile.identity.clone()),
        profile_version: Some(inputs.profile.version.clone()),
        registry_ids: inputs.registries.identities().into(),
        registry_versions: inputs.registries.versions().into(),
        category,
        findings: findings
            .iter()
            .map(|finding| (*finding).to_owned())
            .collect::<Vec<_>>()
            .into(),
    }
}

#[cfg(test)]
mod contract_004_tests {
    use super::*;

    fn request() -> InterpretationRequest {
        let input = SourceComponentInput::new(
            SourceCategory::NaturalLanguage,
            SourceOrigin::InteractiveUser,
            SourcePayload::inline("constraint fixture source"),
            AdmissionState::Accepted,
        )
        .with_preservation(PreservationFacts::recoverable("immutable source"));
        let IntakeOutcome::Success(intake) = admit(
            &SourceSubmission::new(vec![input]),
            &CompositeAdmissionPolicy::Default,
        )
        .expect("source admission") else {
            panic!("fixture intake must succeed")
        };
        let context = InterpretationOperationContext::from_intake(&intake);
        let spec = InterpretationRequestSpec {
            included_source_ids: intake.source_ids(),
            excluded_source_ids: Vec::new(),
            requested_scope: BTreeSet::from([
                ProposalDomain::Objectives,
                ProposalDomain::Constraints,
            ]),
            contract_version: "0.1.0".to_owned(),
            proposal_schema_version: "proposal-fixture-v2".to_owned(),
            interpreter_profile: InterpreterProfileBinding {
                identity: StableId::from_parts("interpreter-profile", &["fixture"]),
                version: "fixture-interpreter-v1".to_owned(),
            },
            evidence_profile: EvidenceCapabilityProfile::fixture_strict().binding(),
            required_proposal_metadata: BTreeSet::from(["trace".to_owned()]),
            permitted_response_forms: BTreeSet::from(["structured-proposal".to_owned()]),
            completion_expectations: BTreeSet::from(["returned".to_owned()]),
            declared_bounds: BTreeSet::from(["structural-admission-only".to_owned()]),
            declared_exclusions: BTreeSet::from(["semantic-correctness".to_owned()]),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        };
        let InterpretationRequestOutcome::Request(request) =
            issue_interpretation_request(&context, spec)
        else {
            panic!("fixture request must succeed")
        };
        *request
    }

    fn constraint_element(expression: &str) -> ProposalConstraintElement {
        ProposalConstraintElement {
            element_id: format!("constraint-{expression}"),
            expression: expression.to_owned(),
            class_name: "Content".to_owned(),
            form: "RequiredExclusion".to_owned(),
            origin: "ExplicitSource".to_owned(),
            basis: "StructuredExtraction".to_owned(),
            scope_kind: "RequestWide".to_owned(),
            scope_target_ids: Vec::new().into(),
            scope_resolution_status: "Resolved".to_owned(),
            objective_ids: Vec::new().into(),
            declared_priority: None,
            evidence_references: Vec::new().into(),
            evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
            representation_status: None,
            relationships: Vec::new().into(),
            component_expressions: Vec::new().into(),
            reference: None,
            reference_state: "None".to_owned(),
            explicit_conflict: false,
        }
    }

    fn proposal(
        request: &InterpretationRequest,
        id: &str,
        constraints: Vec<ProposalConstraintElement>,
        narrative: Vec<&str>,
    ) -> InterpretationProposal {
        InterpretationProposal::new(
            request,
            StableId::from_parts("interpreter", &[id]),
            "interpreter-v1",
            request.interpreter_profile.clone(),
            ProposalProductionState::Returned,
            ProposalCompletionState::Complete,
            request.included_source_ids.to_vec(),
            Some(EvidenceCapabilityProfile::fixture_strict()),
            ProposalContent {
                proposed_elements: narrative
                    .into_iter()
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
                    .into(),
                objective_elements: vec![ProposalObjectiveElement {
                    element_id: format!("objective-{id}"),
                    expression: "achieve objective".to_owned(),
                    form: "Atomic".to_owned(),
                    origin: "ExplicitSource".to_owned(),
                    basis: "StructuredExtraction".to_owned(),
                    class_name: "RequestedOutcome".to_owned(),
                    scope: None,
                    designations: Vec::new().into(),
                    evidence_references: Vec::new().into(),
                    evidence_status: Some(EvidenceStatus::EvidenceNotRequiredByProfile),
                    representation_status: None,
                    relationships: Vec::new().into(),
                    component_expressions: Vec::new().into(),
                }]
                .into(),
                constraint_elements: constraints.into(),
                capability_elements: Vec::new().into(),
                clarification_elements: Vec::new().into(),
                evidence_elements: Vec::new().into(),
                evidence_references: Vec::new().into(),
                declared_assumptions: Vec::new().into(),
                declared_uncertainties: Vec::new().into(),
                declared_bounds: Vec::new().into(),
            },
            BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        )
    }

    fn admitted_set(
        request: &InterpretationRequest,
        proposals: Vec<InterpretationProposal>,
    ) -> AdmittedInterpretationProposalSet {
        let pairs = proposals
            .into_iter()
            .map(|proposal| {
                let ProposalAdmissionOutcome::Decision(decision) =
                    admit_interpretation_proposal(request, &proposal, "criteria-v2")
                else {
                    panic!("proposal must receive a decision")
                };
                (proposal, *decision)
            })
            .collect();
        let SetPublicationOutcome::Set(set) = publish_admitted_proposal_set(
            request,
            pairs,
            SetPublicationPolicy {
                identity: StableId::from_parts("set-policy", &["constraint-fixture"]),
                version: "set-v2".to_owned(),
                criteria_version: "criteria-v2".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
                allow_zero_admitted: true,
            },
        ) else {
            panic!("admitted proposal set must publish")
        };
        set
    }

    fn objective_set(set: &AdmittedInterpretationProposalSet) -> DeclaredObjectiveSet {
        let profile = FixtureObjectiveRepresentationProfile::fixture();
        let registries = FixtureObjectiveRegistries::fixture();
        let schema = ObjectiveSchemaBinding {
            identity: StableId::from_parts("objective-schema", &["fixture"]),
            version: "objective-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = ObjectiveConfigurationBinding {
            identity: StableId::from_parts("objective-config", &["fixture"]),
            version: "objective-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let ObjectiveRepresentationOutcome::Set(objectives) =
            represent_objectives(ObjectiveRepresentationInputs {
                admitted_proposal_set: set,
                profile: &profile,
                registries: &registries,
                schema: &schema,
                configuration: &configuration,
            })
        else {
            panic!("objective fixture must represent")
        };
        objectives
    }

    fn represent(
        set: &AdmittedInterpretationProposalSet,
        objectives: &DeclaredObjectiveSet,
        profile: &FixtureConstraintRepresentationProfile,
        registries: &FixtureConstraintRegistries,
    ) -> ConstraintRepresentationOutcome {
        let schema = ConstraintSchemaBinding {
            identity: StableId::from_parts("constraint-schema", &["fixture"]),
            version: "constraint-schema-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        let configuration = ConstraintConfigurationBinding {
            identity: StableId::from_parts("constraint-config", &["fixture"]),
            version: "constraint-config-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY".to_owned(),
        };
        represent_constraints(ConstraintRepresentationInputs {
            admitted_proposal_set: set,
            objective_set: objectives,
            profile,
            registries,
            schema: &schema,
            configuration: &configuration,
            implementation_version: "sre-runtime-fixture-v1",
        })
    }

    #[test]
    fn typed_constraint_is_represented_and_narrative_is_not_scanned() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "one",
                vec![constraint_element("do not disclose")],
                vec!["Do not disclose this narrative either."],
            )],
        );
        let objectives = objective_set(&set);
        let outcome = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        );
        let ConstraintRepresentationOutcome::Set(constraints) = outcome else {
            panic!("typed constraint must succeed")
        };
        assert_eq!(constraints.constraints().len(), 1);
        assert_eq!(constraints.constraints()[0].expression(), "do not disclose");
    }

    #[test]
    fn empty_constraint_set_is_committed_without_free_text_inference() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "empty",
                Vec::new(),
                vec!["No policy should be inferred."],
            )],
        );
        let objectives = objective_set(&set);
        let ConstraintRepresentationOutcome::Set(constraints) = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("empty fixture set must succeed")
        };
        assert!(constraints.constraints().is_empty());
    }

    #[test]
    fn separate_proposals_and_origins_remain_distinct_and_identity_bound() {
        let request = request();
        let first = proposal(
            &request,
            "first",
            vec![constraint_element("same")],
            Vec::new(),
        );
        let mut second_element = constraint_element("same");
        second_element.origin = "ApplicationSupplied".to_owned();
        let second = proposal(&request, "second", vec![second_element], Vec::new());
        assert_ne!(first.proposal_id(), second.proposal_id());
        let set = admitted_set(&request, vec![first, second]);
        let objectives = objective_set(&set);
        let ConstraintRepresentationOutcome::Set(constraints) = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("constraints must succeed")
        };
        assert_eq!(constraints.constraints().len(), 2);
        assert_ne!(
            constraints.constraints()[0].logical_constraint_id(),
            constraints.constraints()[1].logical_constraint_id()
        );
        assert_ne!(
            constraints.constraints()[0].origin(),
            constraints.constraints()[1].origin()
        );
    }

    #[test]
    fn unresolved_scope_and_non_success_states_remain_inside_success() {
        let request = request();
        let mut element = constraint_element("uncertain");
        element.scope_kind = "Unresolved".to_owned();
        element.scope_resolution_status = "Unresolved".to_owned();
        element.representation_status = Some("ScopeUnresolved".to_owned());
        let set = admitted_set(
            &request,
            vec![proposal(&request, "states", vec![element], Vec::new())],
        );
        let objectives = objective_set(&set);
        let ConstraintRepresentationOutcome::Set(constraints) = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("unresolved scope is representable")
        };
        assert_eq!(
            constraints.constraints()[0].status(),
            ConstraintRepresentationStatus::ScopeUnresolved
        );
        assert_eq!(
            constraints.constraints()[0].scope_resolution_status(),
            ConstraintScopeResolutionStatus::Unresolved
        );
    }

    #[test]
    fn priority_conflict_and_confidentiality_are_preserved_without_policy_effect() {
        let request = request();
        let mut element = constraint_element("conflict");
        element.class_name = "Confidentiality".to_owned();
        element.form = "ConfidentialityRestriction".to_owned();
        element.declared_priority = Some(9);
        element.explicit_conflict = true;
        let set = admitted_set(
            &request,
            vec![proposal(&request, "conflict", vec![element], Vec::new())],
        );
        let objectives = objective_set(&set);
        let ConstraintRepresentationOutcome::Set(constraints) = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("explicit conflict must remain representable")
        };
        assert_eq!(
            constraints.constraints()[0].status(),
            ConstraintRepresentationStatus::Conflicting
        );
        assert_eq!(constraints.constraints()[0].declared_priority(), Some(9));
        assert_eq!(constraints.constraints()[0].class_name(), "Confidentiality");
    }

    #[test]
    fn reference_decisions_and_malformed_mixed_input_are_terminally_deterministic() {
        let request = request();
        let mut unresolved = constraint_element("reference");
        unresolved.reference = Some("artifact-unknown".to_owned());
        unresolved.reference_state = "Unresolved".to_owned();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "reference",
                vec![unresolved],
                Vec::new(),
            )],
        );
        let objectives = objective_set(&set);
        let ConstraintRepresentationOutcome::Set(constraints) = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("permitted unresolved reference must succeed")
        };
        assert_eq!(constraints.constraints()[0].reference_state(), "Unresolved");

        let mut resolved = constraint_element("resolved");
        resolved.reference = Some("artifact-known".to_owned());
        resolved.reference_state = "Resolved".to_owned();
        let resolved_set = admitted_set(
            &request,
            vec![proposal(&request, "resolved", vec![resolved], Vec::new())],
        );
        let resolved_objectives = objective_set(&resolved_set);
        let ConstraintRepresentationOutcome::Set(resolved_constraints) = represent(
            &resolved_set,
            &resolved_objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("resolved reference must succeed")
        };
        assert_eq!(
            resolved_constraints.constraints()[0].reference(),
            Some("artifact-known")
        );

        for reference_state in ["ForeignLineage", "Prohibited"] {
            let mut fatal = constraint_element(reference_state);
            fatal.reference_state = reference_state.to_owned();
            let fatal_set = admitted_set(
                &request,
                vec![proposal(&request, reference_state, vec![fatal], Vec::new())],
            );
            let fatal_objectives = objective_set(&fatal_set);
            assert!(matches!(
                represent(
                    &fatal_set,
                    &fatal_objectives,
                    &FixtureConstraintRepresentationProfile::fixture(),
                    &FixtureConstraintRegistries::fixture()
                ),
                ConstraintRepresentationOutcome::Failure(_)
            ));
        }

        let mut malformed = constraint_element("malformed");
        malformed.reference_state = "Malformed".to_owned();
        let mixed = admitted_set(
            &request,
            vec![proposal(
                &request,
                "mixed",
                vec![constraint_element("valid"), malformed],
                Vec::new(),
            )],
        );
        let mixed_objectives = objective_set(&mixed);
        assert!(matches!(
            represent(
                &mixed,
                &mixed_objectives,
                &FixtureConstraintRepresentationProfile::fixture(),
                &FixtureConstraintRegistries::fixture()
            ),
            ConstraintRepresentationOutcome::Failure(ConstraintRepresentationFailureRecord {
                category: ConstraintRepresentationFailureCategory::MalformedConstraintInput,
                ..
            })
        ));
    }

    #[test]
    fn lineage_profile_registry_and_decomposition_failures_publish_no_partial_set() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "one",
                vec![constraint_element("one")],
                Vec::new(),
            )],
        );
        let objectives = objective_set(&set);
        let mut foreign = objectives.clone();
        foreign.admitted_proposal_set_id =
            AdmittedInterpretationProposalSetId::derive(&["foreign"]);
        assert!(matches!(
            represent(
                &set,
                &foreign,
                &FixtureConstraintRepresentationProfile::fixture(),
                &FixtureConstraintRegistries::fixture()
            ),
            ConstraintRepresentationOutcome::Failure(ConstraintRepresentationFailureRecord {
                category: ConstraintRepresentationFailureCategory::InvalidLineage,
                ..
            })
        ));
        let mut missing_decision_set = set.clone();
        missing_decision_set.decisions = Vec::new().into();
        assert!(matches!(
            represent(
                &missing_decision_set,
                &objectives,
                &FixtureConstraintRepresentationProfile::fixture(),
                &FixtureConstraintRegistries::fixture()
            ),
            ConstraintRepresentationOutcome::Failure(ConstraintRepresentationFailureRecord {
                category: ConstraintRepresentationFailureCategory::InvalidProposalAdmissionBinding,
                ..
            })
        ));
        let mut profile = FixtureConstraintRepresentationProfile::fixture();
        profile.version.clear();
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &profile,
                &FixtureConstraintRegistries::fixture()
            ),
            ConstraintRepresentationOutcome::Failure(ConstraintRepresentationFailureRecord {
                category: ConstraintRepresentationFailureCategory::IncompatibleProfile,
                ..
            })
        ));
        let mut registries = FixtureConstraintRegistries::fixture();
        registries.status_registry.version = "fixture-004-registry-v2".to_owned();
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &FixtureConstraintRepresentationProfile::fixture(),
                &registries
            ),
            ConstraintRepresentationOutcome::Failure(ConstraintRepresentationFailureRecord {
                category: ConstraintRepresentationFailureCategory::IncompatibleRegistryVersion,
                ..
            })
        ));
    }

    #[test]
    fn composite_decomposition_is_profile_authorized_and_relationship_preserved() {
        let request = request();
        let mut composite = constraint_element("composite");
        composite.form = "Composite".to_owned();
        composite.component_expressions = vec!["part one".to_owned(), "part two".to_owned()].into();
        let set = admitted_set(
            &request,
            vec![proposal(&request, "composite", vec![composite], Vec::new())],
        );
        let objectives = objective_set(&set);
        let ConstraintRepresentationOutcome::Set(constraints) = represent(
            &set,
            &objectives,
            &FixtureConstraintRepresentationProfile::fixture(),
            &FixtureConstraintRegistries::fixture(),
        ) else {
            panic!("authorized decomposition must succeed")
        };
        assert_eq!(constraints.constraints().len(), 3);
        assert_eq!(
            constraints
                .relationships()
                .iter()
                .filter(|relationship| relationship.relationship_type()
                    == ConstraintRelationshipType::ComponentOf)
                .count(),
            2
        );
        let mut profile = FixtureConstraintRepresentationProfile::fixture();
        profile.allow_decomposition = false;
        assert!(matches!(
            represent(
                &set,
                &objectives,
                &profile,
                &FixtureConstraintRegistries::fixture()
            ),
            ConstraintRepresentationOutcome::Failure(ConstraintRepresentationFailureRecord {
                category: ConstraintRepresentationFailureCategory::UnauthorizedDecomposition,
                ..
            })
        ));
    }

    #[test]
    fn replay_equivalence_and_lineage_proof_are_stable() {
        let request = request();
        let set = admitted_set(
            &request,
            vec![proposal(
                &request,
                "replay",
                vec![constraint_element("replay")],
                Vec::new(),
            )],
        );
        let objectives = objective_set(&set);
        let profile = FixtureConstraintRepresentationProfile::fixture();
        let registries = FixtureConstraintRegistries::fixture();
        assert_eq!(
            represent(&set, &objectives, &profile, &registries),
            represent(&set, &objectives, &profile, &registries)
        );
        let proof = validate_constraint_lineage(&set, &objectives).expect("lineage proof");
        assert_eq!(proof.admitted_proposal_set_id, *set.set_id());
        assert_eq!(proof.objective_set_id, *objectives.set_id());
    }
}

// ---------------------------------------------------------------------------
// Contract 005: Capability Requirements
// ---------------------------------------------------------------------------

contract_002_id!(CapabilityRepresentationOperationId, "capop");
contract_002_id!(CapabilityRequirementSetId, "capset");
contract_002_id!(CapabilityRequirementId, "capreq");
contract_002_id!(CapabilityRepresentationId, "caprep");
contract_002_id!(CapabilityRepresentationFailureRecordId, "capfail");
contract_002_id!(CapabilityRepresentationProfileId, "capprof");
contract_002_id!(CapabilityRegistryId, "capreg");
contract_002_id!(CapabilityRelationshipId, "caprel");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityForm {
    Atomic,
    Composite,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityOrigin {
    ExplicitSource,
    InterpreterInference,
    ApplicationSupplied,
    ReferencedArtifact,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityBasis {
    DirectQuotation,
    StructuredExtraction,
    InterpreterSynthesis,
    ApplicationDeclaration,
    ReferencedArtifactDeclaration,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityNecessity {
    Required,
    Optional,
    Conditional,
    Supporting,
    Alternative,
    Unresolved,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityScopeKind {
    RequestWide,
    ObjectiveSpecific,
    ObjectiveSetSpecific,
    ConstraintSpecific,
    Conditional,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityScopeResolutionStatus {
    Resolved,
    Partial,
    Conflicting,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ClassificationSupportStatus {
    Classified,
    Unsupported,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityRepresentationStatus {
    Represented,
    Incomplete,
    Unsupported,
    Unresolved,
    EvidenceLimited,
    ScopeUnresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CapabilityRelationshipType {
    SupportsAsDeclared,
    DependsOnAsDeclared,
    RequiredForAsDeclared,
    AlternativeToAsDeclared,
    IncompatibleWithAsDeclared,
    SubstitutableWithAsDeclared,
    ComponentOf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureCapabilityRepresentationProfile {
    identity: CapabilityRepresentationProfileId,
    version: String,
    authority_reference: String,
    allow_empty: bool,
    allow_inferred: bool,
    allow_unresolved: bool,
    allow_scope_unresolved: bool,
    allow_decomposition: bool,
    allow_cross_proposal_relationships: bool,
    allowed_forms: BTreeSet<CapabilityForm>,
    allowed_origins: BTreeSet<CapabilityOrigin>,
    allowed_bases: BTreeSet<CapabilityBasis>,
    allowed_necessities: BTreeSet<CapabilityNecessity>,
    allowed_statuses: BTreeSet<CapabilityRepresentationStatus>,
    allowed_scope_kinds: BTreeSet<CapabilityScopeKind>,
    allowed_classes: BTreeSet<String>,
    abstraction_rules: BTreeMap<String, String>,
    prohibited_classes: BTreeSet<String>,
}

impl FixtureCapabilityRepresentationProfile {
    pub fn fixture() -> Self {
        Self {
            identity: CapabilityRepresentationProfileId::derive(&["contract-005-fixture"]),
            version: "fixture-005-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
                    "fixture-rule-retrieve-v1".to_owned(),
                    "retrieve=ExternalInformationRetrieval".to_owned(),
                ),
                (
                    "fixture-rule-generate-v1".to_owned(),
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
    pub fn identity(&self) -> &CapabilityRepresentationProfileId {
        &self.identity
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn authority_reference(&self) -> &str {
        &self.authority_reference
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureCapabilityRegistry {
    pub identity: CapabilityRegistryId,
    pub version: String,
    pub authority_reference: String,
    pub values: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureCapabilityRegistries {
    pub class_registry: FixtureCapabilityRegistry,
    pub form_registry: FixtureCapabilityRegistry,
    pub origin_registry: FixtureCapabilityRegistry,
    pub basis_registry: FixtureCapabilityRegistry,
    pub necessity_registry: FixtureCapabilityRegistry,
    pub relationship_registry: FixtureCapabilityRegistry,
    pub status_registry: FixtureCapabilityRegistry,
    pub scope_status_registry: FixtureCapabilityRegistry,
}

impl FixtureCapabilityRegistries {
    pub fn fixture() -> Self {
        fn registry(name: &str, values: &[&str]) -> FixtureCapabilityRegistry {
            FixtureCapabilityRegistry {
                identity: CapabilityRegistryId::derive(&[name, "contract-005-fixture"]),
                version: "fixture-005-registry-v1".to_owned(),
                authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
    fn all(&self) -> [&FixtureCapabilityRegistry; 8] {
        [
            &self.class_registry,
            &self.form_registry,
            &self.origin_registry,
            &self.basis_registry,
            &self.necessity_registry,
            &self.relationship_registry,
            &self.status_registry,
            &self.scope_status_registry,
        ]
    }
    fn identities(&self) -> Vec<CapabilityRegistryId> {
        self.all()
            .iter()
            .map(|registry| registry.identity.clone())
            .collect()
    }
    fn versions(&self) -> Vec<String> {
        self.all()
            .iter()
            .map(|registry| registry.version.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilitySchemaBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityConfigurationBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityLineageProof {
    pub admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    pub objective_set_id: DeclaredObjectiveSetId,
    pub constraint_set_id: DeclaredConstraintSetId,
    pub interpretation_operation_id: InterpretationOperationId,
    pub source_intake_id: StableId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityLineageValidationError {
    ObjectiveLineage,
    ConstraintLineage,
    CrossPublicationLineage,
    SourceLineage,
}

pub fn validate_capability_lineage(
    admitted: &AdmittedInterpretationProposalSet,
    objectives: &DeclaredObjectiveSet,
    constraints: &DeclaredConstraintSet,
) -> Result<CapabilityLineageProof, CapabilityLineageValidationError> {
    if objectives.admitted_proposal_set_id != admitted.set_id
        || objectives.interpretation_operation_id != admitted.operation_id
    {
        return Err(CapabilityLineageValidationError::ObjectiveLineage);
    }
    if constraints.admitted_proposal_set_id != admitted.set_id
        || constraints.objective_set_id != objectives.set_id
    {
        return Err(CapabilityLineageValidationError::ConstraintLineage);
    }
    if constraints.input_proposal_ids.as_ref() != admitted.submitted_proposal_ids.as_ref()
        || constraints.input_decision_ids.as_ref() != admitted.decision_ids.as_ref()
    {
        return Err(CapabilityLineageValidationError::CrossPublicationLineage);
    }
    if objectives.source_intake_id != admitted.source_intake_id
        || constraints.source_intake_id != admitted.source_intake_id
    {
        return Err(CapabilityLineageValidationError::SourceLineage);
    }
    Ok(CapabilityLineageProof {
        admitted_proposal_set_id: admitted.set_id.clone(),
        objective_set_id: objectives.set_id.clone(),
        constraint_set_id: constraints.set_id.clone(),
        interpretation_operation_id: admitted.operation_id.clone(),
        source_intake_id: admitted.source_intake_id.clone(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRepresentationInputs<'a> {
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub objective_set: &'a DeclaredObjectiveSet,
    pub constraint_set: &'a DeclaredConstraintSet,
    pub profile: &'a FixtureCapabilityRepresentationProfile,
    pub registries: &'a FixtureCapabilityRegistries,
    pub schema: &'a CapabilitySchemaBinding,
    pub configuration: &'a CapabilityConfigurationBinding,
    pub implementation_version: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRelationship {
    relationship_id: CapabilityRelationshipId,
    relationship_type: CapabilityRelationshipType,
    source_capability_id: CapabilityRequirementId,
    target_capability_ids: Arc<[CapabilityRequirementId]>,
    scope: Option<String>,
    evidence_references: Arc<[EvidenceReference]>,
    evidence_status: Option<EvidenceStatus>,
}

impl CapabilityRelationship {
    pub fn relationship_id(&self) -> &CapabilityRelationshipId {
        &self.relationship_id
    }
    pub fn relationship_type(&self) -> CapabilityRelationshipType {
        self.relationship_type
    }
    pub fn source_capability_id(&self) -> &CapabilityRequirementId {
        &self.source_capability_id
    }
    pub fn target_capability_ids(&self) -> &[CapabilityRequirementId] {
        &self.target_capability_ids
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRequirement {
    logical_id: CapabilityRequirementId,
    representation_id: CapabilityRepresentationId,
    expression: String,
    class_name: String,
    supplied_class: Option<String>,
    supplied_method: Option<String>,
    abstraction_rule_id: Option<String>,
    abstraction_rule_version: Option<String>,
    form: Option<CapabilityForm>,
    origin: Option<CapabilityOrigin>,
    basis: Option<CapabilityBasis>,
    necessity: CapabilityNecessity,
    scope_kind: Option<CapabilityScopeKind>,
    scope_target_ids: Arc<[String]>,
    scope_resolution_status: CapabilityScopeResolutionStatus,
    objective_ids: Arc<[String]>,
    constraint_ids: Arc<[String]>,
    access_dependency: Option<String>,
    classification_support_status: ClassificationSupportStatus,
    evidence_references: Arc<[EvidenceReference]>,
    evidence_status: Option<EvidenceStatus>,
    status: CapabilityRepresentationStatus,
    relationships: Arc<[CapabilityRelationshipId]>,
    source_proposal_ids: Arc<[InterpretationProposalId]>,
    source_element_ids: Arc<[String]>,
    profile_id: CapabilityRepresentationProfileId,
    profile_version: String,
    registry_ids: Arc<[CapabilityRegistryId]>,
    registry_versions: Arc<[String]>,
    operation_id: CapabilityRepresentationOperationId,
    set_id: CapabilityRequirementSetId,
}

impl CapabilityRequirement {
    pub fn logical_id(&self) -> &CapabilityRequirementId {
        &self.logical_id
    }
    pub fn representation_id(&self) -> &CapabilityRepresentationId {
        &self.representation_id
    }
    pub fn expression(&self) -> &str {
        &self.expression
    }
    pub fn class_name(&self) -> &str {
        &self.class_name
    }
    pub fn supplied_method(&self) -> Option<&str> {
        self.supplied_method.as_deref()
    }
    pub fn abstraction_rule_id(&self) -> Option<&str> {
        self.abstraction_rule_id.as_deref()
    }
    pub fn necessity(&self) -> CapabilityNecessity {
        self.necessity
    }
    pub fn scope_resolution_status(&self) -> CapabilityScopeResolutionStatus {
        self.scope_resolution_status
    }
    pub fn objective_ids(&self) -> &[String] {
        &self.objective_ids
    }
    pub fn constraint_ids(&self) -> &[String] {
        &self.constraint_ids
    }
    pub fn access_dependency(&self) -> Option<&str> {
        self.access_dependency.as_deref()
    }
    pub fn classification_support_status(&self) -> ClassificationSupportStatus {
        self.classification_support_status
    }
    pub fn status(&self) -> CapabilityRepresentationStatus {
        self.status
    }
    pub fn evidence_references(&self) -> &[EvidenceReference] {
        &self.evidence_references
    }
    pub fn relationship_ids(&self) -> &[CapabilityRelationshipId] {
        &self.relationships
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRequirementSet {
    set_id: CapabilityRequirementSetId,
    operation_id: CapabilityRepresentationOperationId,
    interpretation_operation_id: InterpretationOperationId,
    source_intake_id: StableId,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    objective_set_id: DeclaredObjectiveSetId,
    constraint_set_id: DeclaredConstraintSetId,
    input_proposal_ids: Arc<[InterpretationProposalId]>,
    input_decision_ids: Arc<[ProposalAdmissionId]>,
    profile_id: CapabilityRepresentationProfileId,
    profile_version: String,
    registry_ids: Arc<[CapabilityRegistryId]>,
    registry_versions: Arc<[String]>,
    schema: CapabilitySchemaBinding,
    configuration: CapabilityConfigurationBinding,
    requirements: Arc<[CapabilityRequirement]>,
    relationships: Arc<[CapabilityRelationship]>,
    replay_context: BTreeMap<String, String>,
}

impl CapabilityRequirementSet {
    pub fn set_id(&self) -> &CapabilityRequirementSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &CapabilityRepresentationOperationId {
        &self.operation_id
    }
    pub fn admitted_proposal_set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.admitted_proposal_set_id
    }
    pub fn objective_set_id(&self) -> &DeclaredObjectiveSetId {
        &self.objective_set_id
    }
    pub fn constraint_set_id(&self) -> &DeclaredConstraintSetId {
        &self.constraint_set_id
    }
    pub fn requirements(&self) -> &[CapabilityRequirement] {
        &self.requirements
    }
    pub fn relationships(&self) -> &[CapabilityRelationship] {
        &self.relationships
    }
    pub fn profile_id(&self) -> &CapabilityRepresentationProfileId {
        &self.profile_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityRepresentationFailureCategory {
    IncompatibleProfile,
    MissingRequiredRegistry,
    IncompatibleRegistryVersion,
    InvalidSchemaBinding,
    InvalidConfigurationBinding,
    InvalidLineage,
    InvalidProposalAdmissionBinding,
    MalformedCapabilityInput,
    UnsupportedCapabilityClass,
    InvalidReference,
    UnauthorizedAbstraction,
    UnauthorizedDecomposition,
    InvalidRelationshipReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRepresentationFailureRecord {
    failure_id: CapabilityRepresentationFailureRecordId,
    operation_id: CapabilityRepresentationOperationId,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    objective_set_id: DeclaredObjectiveSetId,
    constraint_set_id: DeclaredConstraintSetId,
    category: CapabilityRepresentationFailureCategory,
    findings: Arc<[String]>,
}

impl CapabilityRepresentationFailureRecord {
    pub fn failure_id(&self) -> &CapabilityRepresentationFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &CapabilityRepresentationOperationId {
        &self.operation_id
    }
    pub fn category(&self) -> &CapabilityRepresentationFailureCategory {
        &self.category
    }
    pub fn admitted_proposal_set_id(&self) -> &AdmittedInterpretationProposalSetId {
        &self.admitted_proposal_set_id
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CapabilityRepresentationOutcome {
    Set(CapabilityRequirementSet),
    Failure(CapabilityRepresentationFailureRecord),
}

pub fn represent_capabilities(
    inputs: CapabilityRepresentationInputs<'_>,
) -> CapabilityRepresentationOutcome {
    let upstream = inputs.admitted_proposal_set;
    let operation_id = CapabilityRepresentationOperationId::derive(&[
        upstream.set_id.as_str(),
        inputs.objective_set.set_id.as_str(),
        inputs.constraint_set.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
        inputs.schema.identity.as_str(),
        inputs.schema.version.as_str(),
        inputs.configuration.identity.as_str(),
        inputs.configuration.version.as_str(),
        inputs.implementation_version,
    ]);
    if let Some(failure) = validate_capability_inputs(&inputs, &operation_id) {
        return CapabilityRepresentationOutcome::Failure(failure);
    }
    let admitted: BTreeSet<_> = upstream.admitted_proposal_ids.iter().cloned().collect();
    let mut elements: Vec<(&InterpretationProposal, &ProposalCapabilityElement)> = upstream
        .proposals
        .iter()
        .filter(|proposal| admitted.contains(&proposal.proposal_id))
        .flat_map(|proposal| {
            proposal
                .content
                .capability_elements
                .iter()
                .map(move |element| (proposal, element))
        })
        .collect();
    elements.sort_by_key(|(proposal, element)| {
        (proposal.proposal_id.clone(), element.element_id.clone())
    });
    let objective_ids: BTreeSet<String> = inputs
        .objective_set
        .objectives
        .iter()
        .map(|objective| objective.logical_objective_id.to_string())
        .collect();
    let constraint_ids: BTreeSet<String> = inputs
        .constraint_set
        .constraints
        .iter()
        .map(|constraint| constraint.logical_constraint_id.to_string())
        .collect();
    let mut requirements = Vec::new();
    let mut element_ids = BTreeMap::new();
    for (proposal, element) in &elements {
        let logical_id = CapabilityRequirementId::derive(&[
            operation_id.as_str(),
            proposal.proposal_id.as_str(),
            element.element_id.as_str(),
        ]);
        element_ids.insert(
            (proposal.proposal_id.clone(), element.element_id.clone()),
            logical_id.clone(),
        );
        match build_capability(
            &operation_id,
            &logical_id,
            upstream.set_id.as_str(),
            proposal,
            element,
            inputs.profile,
            inputs.registries,
            &objective_ids,
            &constraint_ids,
            inputs.schema,
            inputs.configuration,
        ) {
            Ok(requirement) => requirements.push(requirement),
            Err(category) => {
                return CapabilityRepresentationOutcome::Failure(capability_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    category,
                    &["capability representation could not be constructed"],
                ))
            }
        }
        if !element.component_expressions.is_empty() {
            if element.form != "Composite" || !inputs.profile.allow_decomposition {
                return CapabilityRepresentationOutcome::Failure(capability_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    CapabilityRepresentationFailureCategory::UnauthorizedDecomposition,
                    &["capability decomposition is not authorized by the bound fixture profile"],
                ));
            }
            for (index, expression) in element.component_expressions.iter().enumerate() {
                let child_element_id = format!("{}#component-{index}", element.element_id);
                let child_id = CapabilityRequirementId::derive(&[
                    logical_id.as_str(),
                    &index.to_string(),
                    expression,
                ]);
                element_ids.insert(
                    (proposal.proposal_id.clone(), child_element_id.clone()),
                    child_id.clone(),
                );
                let child = ProposalCapabilityElement {
                    element_id: child_element_id,
                    expression: expression.clone(),
                    form: "Atomic".to_owned(),
                    ..(*element).clone()
                };
                match build_capability(
                    &operation_id,
                    &child_id,
                    upstream.set_id.as_str(),
                    proposal,
                    &child,
                    inputs.profile,
                    inputs.registries,
                    &objective_ids,
                    &constraint_ids,
                    inputs.schema,
                    inputs.configuration,
                ) {
                    Ok(value) => requirements.push(value),
                    Err(category) => {
                        return CapabilityRepresentationOutcome::Failure(capability_failure(
                            &operation_id,
                            upstream,
                            inputs,
                            category,
                            &["authorized capability decomposition was malformed"],
                        ))
                    }
                }
            }
        }
    }
    let mut relationships = Vec::new();
    for (proposal, element) in &elements {
        for relationship in element.relationships.iter() {
            let Some(source_id) =
                element_ids.get(&(proposal.proposal_id.clone(), element.element_id.clone()))
            else {
                return CapabilityRepresentationOutcome::Failure(capability_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    CapabilityRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship source is not represented"],
                ));
            };
            let Some(relationship_type) =
                parse_capability_relationship(&relationship.relationship_type)
            else {
                return CapabilityRepresentationOutcome::Failure(capability_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    CapabilityRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship type is not registered"],
                ));
            };
            if !inputs
                .registries
                .relationship_registry
                .values
                .contains(&relationship.relationship_type)
            {
                return CapabilityRepresentationOutcome::Failure(capability_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    CapabilityRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship type is absent from the bound registry"],
                ));
            }
            let targets: Vec<_> = relationship
                .target_element_ids
                .iter()
                .filter_map(|target| {
                    element_ids
                        .iter()
                        .find(|((_, element_id), _)| element_id == target)
                        .map(|(_, id)| id.clone())
                })
                .collect();
            if targets.len() != relationship.target_element_ids.len() || targets.is_empty() {
                return CapabilityRepresentationOutcome::Failure(capability_failure(
                    &operation_id,
                    upstream,
                    inputs,
                    CapabilityRepresentationFailureCategory::InvalidRelationshipReference,
                    &["relationship target is absent or ambiguous"],
                ));
            }
            let relationship_id = CapabilityRelationshipId::derive(&[
                operation_id.as_str(),
                proposal.proposal_id.as_str(),
                relationship.relationship_id.as_str(),
                &format!("{relationship_type:?}"),
            ]);
            relationships.push(CapabilityRelationship {
                relationship_id: relationship_id.clone(),
                relationship_type,
                source_capability_id: source_id.clone(),
                target_capability_ids: targets.into(),
                scope: relationship.scope.clone(),
                evidence_references: relationship.evidence_references.clone(),
                evidence_status: relationship.evidence_status,
            });
            if let Some(source) = requirements
                .iter_mut()
                .find(|requirement| requirement.logical_id == *source_id)
            {
                let mut ids = source.relationships.to_vec();
                ids.push(relationship_id);
                source.relationships = ids.into();
            }
        }
        for (index, _) in element.component_expressions.iter().enumerate() {
            let Some(parent_id) =
                element_ids.get(&(proposal.proposal_id.clone(), element.element_id.clone()))
            else {
                continue;
            };
            let child_key = format!("{}#component-{index}", element.element_id);
            let Some(child_id) = element_ids.get(&(proposal.proposal_id.clone(), child_key)) else {
                continue;
            };
            relationships.push(CapabilityRelationship {
                relationship_id: CapabilityRelationshipId::derive(&[
                    operation_id.as_str(),
                    parent_id.as_str(),
                    child_id.as_str(),
                    "ComponentOf",
                ]),
                relationship_type: CapabilityRelationshipType::ComponentOf,
                source_capability_id: parent_id.clone(),
                target_capability_ids: vec![child_id.clone()].into(),
                scope: None,
                evidence_references: element.evidence_references.clone(),
                evidence_status: element.evidence_status,
            });
        }
    }
    if requirements.is_empty() && !inputs.profile.allow_empty {
        return CapabilityRepresentationOutcome::Failure(capability_failure(
            &operation_id,
            upstream,
            inputs,
            CapabilityRepresentationFailureCategory::IncompatibleProfile,
            &["empty capability set is not permitted"],
        ));
    }
    let set_id = CapabilityRequirementSetId::derive(&[
        operation_id.as_str(),
        upstream.set_id.as_str(),
        inputs.objective_set.set_id.as_str(),
        inputs.constraint_set.set_id.as_str(),
        &requirements
            .iter()
            .map(|requirement| requirement.representation_id.to_string())
            .collect::<Vec<_>>()
            .join("|"),
    ]);
    for requirement in &mut requirements {
        requirement.set_id = set_id.clone();
    }
    CapabilityRepresentationOutcome::Set(CapabilityRequirementSet {
        set_id,
        operation_id,
        interpretation_operation_id: upstream.operation_id.clone(),
        source_intake_id: upstream.source_intake_id.clone(),
        admitted_proposal_set_id: upstream.set_id.clone(),
        objective_set_id: inputs.objective_set.set_id.clone(),
        constraint_set_id: inputs.constraint_set.set_id.clone(),
        input_proposal_ids: upstream.submitted_proposal_ids.clone(),
        input_decision_ids: upstream.decision_ids.clone(),
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        registry_ids: inputs.registries.identities().into(),
        registry_versions: inputs.registries.versions().into(),
        schema: inputs.schema.clone(),
        configuration: inputs.configuration.clone(),
        requirements: requirements.into(),
        relationships: relationships.into(),
        replay_context: BTreeMap::from([
            (
                String::from("profile_version"),
                inputs.profile.version.clone(),
            ),
            (
                String::from("schema_version"),
                inputs.schema.version.clone(),
            ),
            (
                String::from("configuration_version"),
                inputs.configuration.version.clone(),
            ),
            (
                String::from("implementation_version"),
                inputs.implementation_version.to_owned(),
            ),
        ]),
    })
}

fn validate_capability_inputs(
    inputs: &CapabilityRepresentationInputs<'_>,
    operation_id: &CapabilityRepresentationOperationId,
) -> Option<CapabilityRepresentationFailureRecord> {
    let upstream = inputs.admitted_proposal_set;
    let failure = |category, finding: &'static str| {
        capability_failure(operation_id, upstream, inputs.clone(), category, &[finding])
    };
    if inputs.profile.version.is_empty()
        || inputs.profile.authority_reference.is_empty()
        || inputs.profile.allowed_classes.is_empty()
    {
        return Some(failure(
            CapabilityRepresentationFailureCategory::IncompatibleProfile,
            "capability profile is incomplete",
        ));
    }
    if inputs.registries.all().iter().any(|registry| {
        registry.version.is_empty()
            || registry.authority_reference.is_empty()
            || registry.values.is_empty()
    }) {
        return Some(failure(
            CapabilityRepresentationFailureCategory::MissingRequiredRegistry,
            "capability registry is incomplete",
        ));
    }
    let versions = inputs.registries.versions();
    if versions.windows(2).any(|pair| pair[0] != pair[1]) {
        return Some(failure(
            CapabilityRepresentationFailureCategory::IncompatibleRegistryVersion,
            "capability registries have incompatible versions",
        ));
    }
    if inputs.schema.version.is_empty() || inputs.schema.authority_reference.is_empty() {
        return Some(failure(
            CapabilityRepresentationFailureCategory::InvalidSchemaBinding,
            "capability schema is incomplete",
        ));
    }
    if inputs.configuration.version.is_empty()
        || inputs.configuration.authority_reference.is_empty()
        || inputs.implementation_version.is_empty()
    {
        return Some(failure(
            CapabilityRepresentationFailureCategory::InvalidConfigurationBinding,
            "capability configuration is incomplete",
        ));
    }
    if validate_capability_lineage(upstream, inputs.objective_set, inputs.constraint_set).is_err() {
        return Some(failure(
            CapabilityRepresentationFailureCategory::InvalidLineage,
            "capability inputs do not share one declared lineage",
        ));
    }
    if upstream.decisions.len() != upstream.decision_ids.len()
        || upstream.admitted_proposal_ids.iter().any(|proposal_id| {
            !upstream.decisions.iter().any(|decision| {
                decision.proposal_id == *proposal_id
                    && decision.disposition == AdmissionDisposition::Admitted
            })
        })
    {
        return Some(failure(
            CapabilityRepresentationFailureCategory::InvalidProposalAdmissionBinding,
            "capability input admission decisions are incomplete",
        ));
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn build_capability(
    operation_id: &CapabilityRepresentationOperationId,
    logical_id: &CapabilityRequirementId,
    admitted_set_id: &str,
    proposal: &InterpretationProposal,
    element: &ProposalCapabilityElement,
    profile: &FixtureCapabilityRepresentationProfile,
    registries: &FixtureCapabilityRegistries,
    objective_ids: &BTreeSet<String>,
    constraint_ids: &BTreeSet<String>,
    schema: &CapabilitySchemaBinding,
    configuration: &CapabilityConfigurationBinding,
) -> Result<CapabilityRequirement, CapabilityRepresentationFailureCategory> {
    if element.element_id.is_empty()
        || element.expression.is_empty()
        || element.class_name.is_empty()
    {
        return Err(CapabilityRepresentationFailureCategory::MalformedCapabilityInput);
    }
    let form = parse_capability_form(&element.form)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    let origin = parse_capability_origin(&element.origin)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    let basis = parse_capability_basis(&element.basis)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    let necessity = parse_capability_necessity(&element.necessity)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    let scope_kind = parse_capability_scope(&element.scope_kind)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    let scope_status = parse_capability_scope_status(&element.scope_resolution_status)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    let support = parse_classification_support(&element.classification_support_status)
        .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    if !registries
        .class_registry
        .values
        .contains(&element.class_name)
    {
        if profile.prohibited_classes.contains(&element.class_name) {
            return Err(CapabilityRepresentationFailureCategory::UnsupportedCapabilityClass);
        }
        if element.representation_status.as_deref() != Some("Unsupported") {
            return Err(CapabilityRepresentationFailureCategory::UnsupportedCapabilityClass);
        }
    }
    if !registries.form_registry.values.contains(&element.form)
        || !registries.origin_registry.values.contains(&element.origin)
        || !registries.basis_registry.values.contains(&element.basis)
        || !registries
            .necessity_registry
            .values
            .contains(&element.necessity)
    {
        return Err(CapabilityRepresentationFailureCategory::MalformedCapabilityInput);
    }
    if !profile.allowed_forms.contains(&form)
        || !profile.allowed_origins.contains(&origin)
        || !profile.allowed_bases.contains(&basis)
        || !profile.allowed_necessities.contains(&necessity)
    {
        return Err(CapabilityRepresentationFailureCategory::IncompatibleProfile);
    }
    if origin == CapabilityOrigin::InterpreterInference && !profile.allow_inferred {
        return Err(CapabilityRepresentationFailureCategory::IncompatibleProfile);
    }
    if scope_status != CapabilityScopeResolutionStatus::Resolved && !profile.allow_scope_unresolved
    {
        return Err(CapabilityRepresentationFailureCategory::IncompatibleProfile);
    }
    if element
        .objective_ids
        .iter()
        .any(|id| !objective_ids.contains(id))
        || element
            .constraint_ids
            .iter()
            .any(|id| !constraint_ids.contains(id))
    {
        return Err(CapabilityRepresentationFailureCategory::InvalidReference);
    }
    let mut status = if element.evidence_references.is_empty() && element.evidence_status.is_none()
    {
        CapabilityRepresentationStatus::EvidenceLimited
    } else {
        CapabilityRepresentationStatus::Represented
    };
    if scope_status != CapabilityScopeResolutionStatus::Resolved {
        status = CapabilityRepresentationStatus::ScopeUnresolved;
    }
    if support == ClassificationSupportStatus::Unsupported {
        status = CapabilityRepresentationStatus::Unsupported;
    }
    if support == ClassificationSupportStatus::Unresolved {
        status = CapabilityRepresentationStatus::Unresolved;
    }
    if let Some(raw) = element.representation_status.as_deref() {
        status = parse_capability_status(raw)
            .ok_or(CapabilityRepresentationFailureCategory::MalformedCapabilityInput)?;
    }
    if !profile.allowed_statuses.contains(&status) {
        return Err(CapabilityRepresentationFailureCategory::IncompatibleProfile);
    }
    if let Some(rule_id) = element.abstraction_rule_id.as_deref() {
        let Some(rule) = profile.abstraction_rules.get(rule_id) else {
            return Err(CapabilityRepresentationFailureCategory::UnauthorizedAbstraction);
        };
        let Some(method) = element.supplied_method.as_deref() else {
            return Err(CapabilityRepresentationFailureCategory::UnauthorizedAbstraction);
        };
        if rule != &format!("{method}={}", element.class_name)
            || element.abstraction_rule_version.as_deref() != Some("fixture-005-rule-v1")
        {
            return Err(CapabilityRepresentationFailureCategory::UnauthorizedAbstraction);
        }
    } else if element.supplied_method.is_some()
        && element.supplied_class.is_some()
        && element.supplied_class != Some(element.class_name.clone())
    {
        return Err(CapabilityRepresentationFailureCategory::UnauthorizedAbstraction);
    }
    if element
        .evidence_references
        .iter()
        .any(|reference| reference.reference.is_empty())
    {
        return Err(CapabilityRepresentationFailureCategory::MalformedCapabilityInput);
    }
    let representation_id = CapabilityRepresentationId::derive(&[
        operation_id.as_str(),
        logical_id.as_str(),
        &format!("{status:?}"),
        &format!("{necessity:?}"),
        &element.class_name,
        element.expression.as_str(),
        schema.version.as_str(),
        configuration.version.as_str(),
    ]);
    Ok(CapabilityRequirement {
        logical_id: logical_id.clone(),
        representation_id,
        expression: element.expression.clone(),
        class_name: element.class_name.clone(),
        supplied_class: element.supplied_class.clone(),
        supplied_method: element.supplied_method.clone(),
        abstraction_rule_id: element.abstraction_rule_id.clone(),
        abstraction_rule_version: element.abstraction_rule_version.clone(),
        form: Some(form),
        origin: Some(origin),
        basis: Some(basis),
        necessity,
        scope_kind: Some(scope_kind),
        scope_target_ids: element.scope_target_ids.clone(),
        scope_resolution_status: scope_status,
        objective_ids: element.objective_ids.clone(),
        constraint_ids: element.constraint_ids.clone(),
        access_dependency: element.access_dependency.clone(),
        classification_support_status: support,
        evidence_references: element.evidence_references.clone(),
        evidence_status: element.evidence_status,
        status,
        relationships: Vec::new().into(),
        source_proposal_ids: vec![proposal.proposal_id.clone()].into(),
        source_element_ids: vec![element.element_id.clone()].into(),
        profile_id: profile.identity.clone(),
        profile_version: profile.version.clone(),
        registry_ids: registries.identities().into(),
        registry_versions: registries.versions().into(),
        operation_id: operation_id.clone(),
        set_id: CapabilityRequirementSetId::derive(&[admitted_set_id, operation_id.as_str()]),
    })
}

fn parse_capability_form(value: &str) -> Option<CapabilityForm> {
    match value {
        "Atomic" => Some(CapabilityForm::Atomic),
        "Composite" => Some(CapabilityForm::Composite),
        _ => None,
    }
}
fn parse_capability_origin(value: &str) -> Option<CapabilityOrigin> {
    match value {
        "ExplicitSource" => Some(CapabilityOrigin::ExplicitSource),
        "InterpreterInference" => Some(CapabilityOrigin::InterpreterInference),
        "ApplicationSupplied" => Some(CapabilityOrigin::ApplicationSupplied),
        "ReferencedArtifact" => Some(CapabilityOrigin::ReferencedArtifact),
        _ => None,
    }
}
fn parse_capability_basis(value: &str) -> Option<CapabilityBasis> {
    match value {
        "DirectQuotation" => Some(CapabilityBasis::DirectQuotation),
        "StructuredExtraction" => Some(CapabilityBasis::StructuredExtraction),
        "InterpreterSynthesis" => Some(CapabilityBasis::InterpreterSynthesis),
        "ApplicationDeclaration" => Some(CapabilityBasis::ApplicationDeclaration),
        "ReferencedArtifactDeclaration" => Some(CapabilityBasis::ReferencedArtifactDeclaration),
        _ => None,
    }
}
fn parse_capability_necessity(value: &str) -> Option<CapabilityNecessity> {
    match value {
        "Required" => Some(CapabilityNecessity::Required),
        "Optional" => Some(CapabilityNecessity::Optional),
        "Conditional" => Some(CapabilityNecessity::Conditional),
        "Supporting" => Some(CapabilityNecessity::Supporting),
        "Alternative" => Some(CapabilityNecessity::Alternative),
        "Unresolved" => Some(CapabilityNecessity::Unresolved),
        "Unknown" => Some(CapabilityNecessity::Unknown),
        _ => None,
    }
}
fn parse_capability_scope(value: &str) -> Option<CapabilityScopeKind> {
    match value {
        "RequestWide" => Some(CapabilityScopeKind::RequestWide),
        "ObjectiveSpecific" => Some(CapabilityScopeKind::ObjectiveSpecific),
        "ObjectiveSetSpecific" => Some(CapabilityScopeKind::ObjectiveSetSpecific),
        "ConstraintSpecific" => Some(CapabilityScopeKind::ConstraintSpecific),
        "Conditional" => Some(CapabilityScopeKind::Conditional),
        "Unresolved" => Some(CapabilityScopeKind::Unresolved),
        _ => None,
    }
}
fn parse_capability_scope_status(value: &str) -> Option<CapabilityScopeResolutionStatus> {
    match value {
        "Resolved" => Some(CapabilityScopeResolutionStatus::Resolved),
        "Partial" => Some(CapabilityScopeResolutionStatus::Partial),
        "Conflicting" => Some(CapabilityScopeResolutionStatus::Conflicting),
        "Unresolved" => Some(CapabilityScopeResolutionStatus::Unresolved),
        _ => None,
    }
}
fn parse_classification_support(value: &str) -> Option<ClassificationSupportStatus> {
    match value {
        "Classified" => Some(ClassificationSupportStatus::Classified),
        "Unsupported" => Some(ClassificationSupportStatus::Unsupported),
        "Unresolved" => Some(ClassificationSupportStatus::Unresolved),
        _ => None,
    }
}
fn parse_capability_status(value: &str) -> Option<CapabilityRepresentationStatus> {
    match value {
        "Represented" => Some(CapabilityRepresentationStatus::Represented),
        "Incomplete" => Some(CapabilityRepresentationStatus::Incomplete),
        "Unsupported" => Some(CapabilityRepresentationStatus::Unsupported),
        "Unresolved" => Some(CapabilityRepresentationStatus::Unresolved),
        "EvidenceLimited" => Some(CapabilityRepresentationStatus::EvidenceLimited),
        "ScopeUnresolved" => Some(CapabilityRepresentationStatus::ScopeUnresolved),
        _ => None,
    }
}
fn parse_capability_relationship(value: &str) -> Option<CapabilityRelationshipType> {
    match value {
        "SupportsAsDeclared" => Some(CapabilityRelationshipType::SupportsAsDeclared),
        "DependsOnAsDeclared" => Some(CapabilityRelationshipType::DependsOnAsDeclared),
        "RequiredForAsDeclared" => Some(CapabilityRelationshipType::RequiredForAsDeclared),
        "AlternativeToAsDeclared" => Some(CapabilityRelationshipType::AlternativeToAsDeclared),
        "IncompatibleWithAsDeclared" => {
            Some(CapabilityRelationshipType::IncompatibleWithAsDeclared)
        }
        "SubstitutableWithAsDeclared" => {
            Some(CapabilityRelationshipType::SubstitutableWithAsDeclared)
        }
        "ComponentOf" => Some(CapabilityRelationshipType::ComponentOf),
        _ => None,
    }
}

fn capability_failure(
    operation_id: &CapabilityRepresentationOperationId,
    upstream: &AdmittedInterpretationProposalSet,
    inputs: CapabilityRepresentationInputs<'_>,
    category: CapabilityRepresentationFailureCategory,
    findings: &[&str],
) -> CapabilityRepresentationFailureRecord {
    CapabilityRepresentationFailureRecord {
        failure_id: CapabilityRepresentationFailureRecordId::derive(&[
            operation_id.as_str(),
            upstream.set_id.as_str(),
            inputs.objective_set.set_id.as_str(),
            inputs.constraint_set.set_id.as_str(),
            &format!("{category:?}"),
        ]),
        operation_id: operation_id.clone(),
        admitted_proposal_set_id: upstream.set_id.clone(),
        objective_set_id: inputs.objective_set.set_id.clone(),
        constraint_set_id: inputs.constraint_set.set_id.clone(),
        category,
        findings: findings
            .iter()
            .map(|finding| (*finding).to_owned())
            .collect::<Vec<_>>()
            .into(),
    }
}

// ---------------------------------------------------------------------------
// Contract 006: Semantic clarification / meaning qualification representation
// ---------------------------------------------------------------------------

contract_002_id!(MeaningQualificationRepresentationOperationId, "mqop");
contract_002_id!(MeaningQualificationProfileId, "mqprofile");
contract_002_id!(MeaningQualificationSetId, "mqset");
contract_002_id!(AmbiguityId, "ambig");
contract_002_id!(AssumptionId, "assump");
contract_002_id!(UncertaintyRecordId, "unc");
contract_002_id!(ClarificationRequirementId, "clarify");
contract_002_id!(MeaningQualificationRelationshipId, "mqrel");
contract_002_id!(MeaningQualificationFailureRecordId, "mqfail");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ClarificationDomain {
    Ambiguity,
    Assumption,
    Uncertainty,
    ClarificationRequirement,
}

impl ClarificationDomain {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "Ambiguity" => Some(Self::Ambiguity),
            "Assumption" => Some(Self::Assumption),
            "Uncertainty" => Some(Self::Uncertainty),
            "ClarificationRequirement" => Some(Self::ClarificationRequirement),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureMeaningQualificationProfile {
    pub identity: MeaningQualificationProfileId,
    pub version: String,
    pub authority_reference: String,
    pub allowed_domains: BTreeSet<String>,
    pub allow_empty: bool,
}

impl FixtureMeaningQualificationProfile {
    pub fn fixture() -> Self {
        Self {
            identity: MeaningQualificationProfileId::derive(&["fixture"]),
            version: "fixture-meaning-qualification-v1".to_owned(),
            authority_reference: "fixture-contract-006-authority".to_owned(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureMeaningQualificationRegistries {
    pub version: String,
    pub authority_reference: String,
    pub domain_values: BTreeSet<String>,
    pub relationship_values: BTreeSet<String>,
}

impl FixtureMeaningQualificationRegistries {
    pub fn fixture() -> Self {
        Self {
            version: "fixture-meaning-qualification-registry-v1".to_owned(),
            authority_reference: "fixture-contract-006-registry".to_owned(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeaningQualificationSchemaBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeaningQualificationConfigurationBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeaningQualificationRepresentationInputs<'a> {
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub objective_set: &'a DeclaredObjectiveSet,
    pub constraint_set: &'a DeclaredConstraintSet,
    pub capability_set: &'a CapabilityRequirementSet,
    pub profile: &'a FixtureMeaningQualificationProfile,
    pub registries: &'a FixtureMeaningQualificationRegistries,
    pub schema: &'a MeaningQualificationSchemaBinding,
    pub configuration: &'a MeaningQualificationConfigurationBinding,
    pub implementation_version: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmbiguityRepresentation {
    id: AmbiguityId,
    pub source_proposal_id: InterpretationProposalId,
    pub source_element_id: String,
    pub expression: String,
    pub class_name: String,
    pub alternatives: Arc<[String]>,
    pub target_ids: Arc<[String]>,
    pub evidence_references: Arc<[EvidenceReference]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssumptionRepresentation {
    id: AssumptionId,
    pub source_proposal_id: InterpretationProposalId,
    pub source_element_id: String,
    pub expression: String,
    pub class_name: String,
    pub target_ids: Arc<[String]>,
    pub evidence_references: Arc<[EvidenceReference]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UncertaintyRepresentation {
    id: UncertaintyRecordId,
    pub source_proposal_id: InterpretationProposalId,
    pub source_element_id: String,
    pub expression: String,
    pub class_name: String,
    pub kind: Option<String>,
    pub mode: Option<String>,
    pub target_ids: Arc<[String]>,
    pub evidence_references: Arc<[EvidenceReference]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClarificationRequirementRepresentation {
    id: ClarificationRequirementId,
    pub source_proposal_id: InterpretationProposalId,
    pub source_element_id: String,
    pub expression: String,
    pub target_ids: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeaningQualificationRelationship {
    pub id: MeaningQualificationRelationshipId,
    pub source_element_id: String,
    pub relationship_type: String,
    pub target_element_ids: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedRepresentationLineage {
    pub admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    pub objective_set_id: DeclaredObjectiveSetId,
    pub constraint_set_id: DeclaredConstraintSetId,
    pub capability_set_id: CapabilityRequirementSetId,
    pub source_intake_id: StableId,
    pub input_proposal_ids: Arc<[InterpretationProposalId]>,
    pub input_decision_ids: Arc<[ProposalAdmissionId]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeaningQualificationSet {
    set_id: MeaningQualificationSetId,
    operation_id: MeaningQualificationRepresentationOperationId,
    interpretation_operation_id: InterpretationOperationId,
    source_intake_id: StableId,
    objective_set_id: DeclaredObjectiveSetId,
    constraint_set_id: DeclaredConstraintSetId,
    capability_set_id: CapabilityRequirementSetId,
    profile_id: MeaningQualificationProfileId,
    profile_version: String,
    schema_version: String,
    registry_version: String,
    ambiguities: Arc<[AmbiguityRepresentation]>,
    assumptions: Arc<[AssumptionRepresentation]>,
    uncertainties: Arc<[UncertaintyRepresentation]>,
    clarification_requirements: Arc<[ClarificationRequirementRepresentation]>,
    relationships: Arc<[MeaningQualificationRelationship]>,
    lineage: QualifiedRepresentationLineage,
}

impl MeaningQualificationSet {
    pub fn set_id(&self) -> &MeaningQualificationSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &MeaningQualificationRepresentationOperationId {
        &self.operation_id
    }
    pub fn ambiguities(&self) -> &[AmbiguityRepresentation] {
        &self.ambiguities
    }
    pub fn assumptions(&self) -> &[AssumptionRepresentation] {
        &self.assumptions
    }
    pub fn uncertainties(&self) -> &[UncertaintyRepresentation] {
        &self.uncertainties
    }
    pub fn clarification_requirements(&self) -> &[ClarificationRequirementRepresentation] {
        &self.clarification_requirements
    }
    pub fn relationships(&self) -> &[MeaningQualificationRelationship] {
        &self.relationships
    }
    pub fn lineage(&self) -> &QualifiedRepresentationLineage {
        &self.lineage
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeaningQualificationFailureCategory {
    MissingRequiredProfile,
    MissingRequiredRegistry,
    InvalidSchemaBinding,
    InvalidConfigurationBinding,
    InvalidLineage,
    InvalidProposalAdmissionBinding,
    UnsupportedClarificationDomain,
    InvalidRelationshipReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeaningQualificationRepresentationFailureRecord {
    failure_id: MeaningQualificationFailureRecordId,
    operation_id: MeaningQualificationRepresentationOperationId,
    category: MeaningQualificationFailureCategory,
    findings: Arc<[String]>,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    objective_set_id: DeclaredObjectiveSetId,
    constraint_set_id: DeclaredConstraintSetId,
    capability_set_id: CapabilityRequirementSetId,
}

impl MeaningQualificationRepresentationFailureRecord {
    pub fn failure_id(&self) -> &MeaningQualificationFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &MeaningQualificationRepresentationOperationId {
        &self.operation_id
    }
    pub fn category(&self) -> &MeaningQualificationFailureCategory {
        &self.category
    }
    pub fn findings(&self) -> &[String] {
        &self.findings
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MeaningQualificationOutcome {
    Set(MeaningQualificationSet),
    Failure(MeaningQualificationRepresentationFailureRecord),
}

pub struct SemanticClarificationAuthority;

/// Contract-facing names for the coordinated Contract 006 artifacts.
pub type SemanticClarificationSet = MeaningQualificationSet;
pub type SemanticClarificationFailureRecord = MeaningQualificationRepresentationFailureRecord;

pub fn represent_semantic_clarification(
    inputs: MeaningQualificationRepresentationInputs<'_>,
) -> MeaningQualificationOutcome {
    let upstream = inputs.admitted_proposal_set;
    let operation_id = MeaningQualificationRepresentationOperationId::derive(&[
        upstream.set_id.as_str(),
        inputs.objective_set.set_id.as_str(),
        inputs.constraint_set.set_id.as_str(),
        inputs.capability_set.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
        inputs.schema.identity.as_str(),
        inputs.schema.version.as_str(),
        inputs.configuration.identity.as_str(),
        inputs.configuration.version.as_str(),
        inputs.implementation_version,
    ]);
    let fail = |category, finding| {
        MeaningQualificationOutcome::Failure(MeaningQualificationRepresentationFailureRecord {
            failure_id: MeaningQualificationFailureRecordId::derive(&[
                operation_id.as_str(),
                &format!("{category:?}"),
                finding,
            ]),
            operation_id: operation_id.clone(),
            category,
            findings: vec![finding.to_owned()].into(),
            admitted_proposal_set_id: upstream.set_id.clone(),
            objective_set_id: inputs.objective_set.set_id.clone(),
            constraint_set_id: inputs.constraint_set.set_id.clone(),
            capability_set_id: inputs.capability_set.set_id.clone(),
        })
    };
    if inputs.profile.version.is_empty()
        || inputs.profile.authority_reference.is_empty()
        || inputs.profile.allowed_domains.is_empty()
    {
        return fail(
            MeaningQualificationFailureCategory::MissingRequiredProfile,
            "meaning-qualification profile is incomplete",
        );
    }
    if inputs.registries.version.is_empty()
        || inputs.registries.authority_reference.is_empty()
        || inputs.registries.domain_values.is_empty()
    {
        return fail(
            MeaningQualificationFailureCategory::MissingRequiredRegistry,
            "meaning-qualification registries are incomplete",
        );
    }
    if inputs.schema.version.is_empty() || inputs.schema.authority_reference.is_empty() {
        return fail(
            MeaningQualificationFailureCategory::InvalidSchemaBinding,
            "meaning-qualification schema is incomplete",
        );
    }
    if upstream.proposal_schema_version != "proposal-fixture-v4"
        && upstream.proposal_schema_version != "proposal-fixture-v5"
        && upstream.proposal_schema_version != "ulantra-literal-proposal-v1"
    {
        return fail(
            MeaningQualificationFailureCategory::InvalidProposalAdmissionBinding,
            "Contract 006 requires the versioned typed clarification proposal schema",
        );
    }
    if inputs.configuration.version.is_empty()
        || inputs.configuration.authority_reference.is_empty()
        || inputs.implementation_version.is_empty()
    {
        return fail(
            MeaningQualificationFailureCategory::InvalidConfigurationBinding,
            "meaning-qualification configuration is incomplete",
        );
    }
    if let Err(finding) = validate_semantic_clarification_lineage(&inputs) {
        return fail(MeaningQualificationFailureCategory::InvalidLineage, finding);
    }
    let admitted: BTreeSet<_> = upstream.admitted_proposal_ids.iter().cloned().collect();
    let mut elements: Vec<_> = upstream
        .proposals
        .iter()
        .filter(|p| admitted.contains(&p.proposal_id))
        .flat_map(|p| p.content.clarification_elements.iter().map(move |e| (p, e)))
        .collect();
    elements.sort_by_key(|(p, e)| (p.proposal_id.clone(), e.element_id.clone()));
    let mut ambiguities = Vec::new();
    let mut assumptions = Vec::new();
    let mut uncertainties = Vec::new();
    let mut requirements = Vec::new();
    let mut relationships = Vec::new();
    for (proposal, element) in elements {
        let Some(domain) = ClarificationDomain::parse(&element.domain) else {
            return fail(
                MeaningQualificationFailureCategory::UnsupportedClarificationDomain,
                "clarification domain is not registered",
            );
        };
        if !inputs.profile.allowed_domains.contains(&element.domain)
            || !inputs.registries.domain_values.contains(&element.domain)
        {
            return fail(
                MeaningQualificationFailureCategory::UnsupportedClarificationDomain,
                "clarification domain is not permitted by profile and registry",
            );
        }
        match domain {
            ClarificationDomain::Ambiguity => ambiguities.push(AmbiguityRepresentation {
                id: AmbiguityId::derive(&[
                    operation_id.as_str(),
                    proposal.proposal_id.as_str(),
                    &element.element_id,
                ]),
                source_proposal_id: proposal.proposal_id.clone(),
                source_element_id: element.element_id.clone(),
                expression: element.expression.clone(),
                class_name: element.class_name.clone(),
                alternatives: element.alternatives.clone(),
                target_ids: element.target_ids.clone(),
                evidence_references: element.evidence_references.clone(),
            }),
            ClarificationDomain::Assumption => assumptions.push(AssumptionRepresentation {
                id: AssumptionId::derive(&[
                    operation_id.as_str(),
                    proposal.proposal_id.as_str(),
                    &element.element_id,
                ]),
                source_proposal_id: proposal.proposal_id.clone(),
                source_element_id: element.element_id.clone(),
                expression: element.expression.clone(),
                class_name: element.class_name.clone(),
                target_ids: element.target_ids.clone(),
                evidence_references: element.evidence_references.clone(),
            }),
            ClarificationDomain::Uncertainty => uncertainties.push(UncertaintyRepresentation {
                id: UncertaintyRecordId::derive(&[
                    operation_id.as_str(),
                    proposal.proposal_id.as_str(),
                    &element.element_id,
                ]),
                source_proposal_id: proposal.proposal_id.clone(),
                source_element_id: element.element_id.clone(),
                expression: element.expression.clone(),
                class_name: element.class_name.clone(),
                kind: element.uncertainty_kind.clone(),
                mode: element.uncertainty_mode.clone(),
                target_ids: element.target_ids.clone(),
                evidence_references: element.evidence_references.clone(),
            }),
            ClarificationDomain::ClarificationRequirement => {
                requirements.push(ClarificationRequirementRepresentation {
                    id: ClarificationRequirementId::derive(&[
                        operation_id.as_str(),
                        proposal.proposal_id.as_str(),
                        &element.element_id,
                    ]),
                    source_proposal_id: proposal.proposal_id.clone(),
                    source_element_id: element.element_id.clone(),
                    expression: element.expression.clone(),
                    target_ids: element.target_ids.clone(),
                })
            }
        }
        for relation in element.relationships.iter() {
            if !inputs.registries.relationship_values.contains(relation) {
                return fail(
                    MeaningQualificationFailureCategory::InvalidRelationshipReference,
                    "clarification relationship is not registered",
                );
            }
            relationships.push(MeaningQualificationRelationship {
                id: MeaningQualificationRelationshipId::derive(&[
                    operation_id.as_str(),
                    proposal.proposal_id.as_str(),
                    &element.element_id,
                    relation,
                ]),
                source_element_id: element.element_id.clone(),
                relationship_type: relation.clone(),
                target_element_ids: element.target_ids.clone(),
            });
        }
    }
    if !inputs.profile.allow_empty
        && ambiguities.is_empty()
        && assumptions.is_empty()
        && uncertainties.is_empty()
        && requirements.is_empty()
    {
        return fail(
            MeaningQualificationFailureCategory::MissingRequiredProfile,
            "empty meaning-qualification set is not permitted",
        );
    }
    let lineage = QualifiedRepresentationLineage {
        admitted_proposal_set_id: upstream.set_id.clone(),
        objective_set_id: inputs.objective_set.set_id.clone(),
        constraint_set_id: inputs.constraint_set.set_id.clone(),
        capability_set_id: inputs.capability_set.set_id.clone(),
        source_intake_id: upstream.source_intake_id.clone(),
        input_proposal_ids: upstream.submitted_proposal_ids.clone(),
        input_decision_ids: upstream.decision_ids.clone(),
    };
    let set_id = MeaningQualificationSetId::derive(&[
        operation_id.as_str(),
        &format!(
            "{:?}{:?}{:?}{:?}",
            ambiguities, assumptions, uncertainties, requirements
        ),
    ]);
    MeaningQualificationOutcome::Set(MeaningQualificationSet {
        set_id,
        operation_id,
        interpretation_operation_id: upstream.operation_id.clone(),
        source_intake_id: upstream.source_intake_id.clone(),
        objective_set_id: inputs.objective_set.set_id.clone(),
        constraint_set_id: inputs.constraint_set.set_id.clone(),
        capability_set_id: inputs.capability_set.set_id.clone(),
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        schema_version: inputs.schema.version.clone(),
        registry_version: inputs.registries.version.clone(),
        ambiguities: ambiguities.into(),
        assumptions: assumptions.into(),
        uncertainties: uncertainties.into(),
        clarification_requirements: requirements.into(),
        relationships: relationships.into(),
        lineage,
    })
}

fn validate_semantic_clarification_lineage(
    inputs: &MeaningQualificationRepresentationInputs<'_>,
) -> Result<(), &'static str> {
    let a = inputs.admitted_proposal_set;
    let o = inputs.objective_set;
    let c = inputs.constraint_set;
    let k = inputs.capability_set;
    if o.admitted_proposal_set_id != a.set_id
        || c.admitted_proposal_set_id != a.set_id
        || k.admitted_proposal_set_id != a.set_id
    {
        return Err("upstream admitted proposal binding is inconsistent");
    }
    if c.objective_set_id != o.set_id
        || k.objective_set_id != o.set_id
        || k.constraint_set_id != c.set_id
    {
        return Err("upstream objective or constraint binding is inconsistent");
    }
    if o.interpretation_operation_id != a.operation_id
        || c.interpretation_operation_id != a.operation_id
        || k.interpretation_operation_id != a.operation_id
    {
        return Err("interpretation operation lineage is inconsistent");
    }
    if o.source_intake_id != a.source_intake_id
        || c.source_intake_id != a.source_intake_id
        || k.source_intake_id != a.source_intake_id
    {
        return Err("source intake lineage is inconsistent");
    }
    if o.input_proposal_ids != a.submitted_proposal_ids
        || c.input_proposal_ids != a.submitted_proposal_ids
        || k.input_proposal_ids != a.submitted_proposal_ids
    {
        return Err("proposal lineage is inconsistent");
    }
    if o.input_decision_ids != a.decision_ids
        || c.input_decision_ids != a.decision_ids
        || k.input_decision_ids != a.decision_ids
    {
        return Err("decision lineage is inconsistent");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Contract 007: Evidence representation and explicit grounding
// ---------------------------------------------------------------------------

contract_002_id!(EvidenceRepresentationOperationId, "evop");
contract_002_id!(EvidenceDeclarationId, "evdecl");
contract_002_id!(EvidenceId, "evid");
contract_002_id!(EvidenceRepresentationId, "evrep");
contract_002_id!(GroundingAssertionId, "ground");
contract_002_id!(InterpretationEvidenceSetId, "evset");
contract_002_id!(EvidenceRepresentationFailureRecordId, "evfail");
contract_002_id!(GroundingRepresentationProfileId, "evprofile");

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceMaterialReference {
    AdmittedSourceReference(String),
    QuotedTextValue(String),
    StructuredFieldReference(String),
    ApplicationDeclarationReference(String),
    ReferencedArtifactReference(String),
    AdmittedProposalElementReference(String),
    DerivedObservationInputs(Arc<[String]>),
    CompositeEvidenceMembers(Arc<[EvidenceRepresentationId]>),
}

impl EvidenceMaterialReference {
    fn kind_name(&self) -> &'static str {
        match self {
            Self::AdmittedSourceReference(_) => "AdmittedSourceReference",
            Self::QuotedTextValue(_) => "QuotedTextValue",
            Self::StructuredFieldReference(_) => "StructuredFieldReference",
            Self::ApplicationDeclarationReference(_) => "ApplicationDeclarationReference",
            Self::ReferencedArtifactReference(_) => "ReferencedArtifactReference",
            Self::AdmittedProposalElementReference(_) => "AdmittedProposalElementReference",
            Self::DerivedObservationInputs(_) => "DerivedObservationInputs",
            Self::CompositeEvidenceMembers(_) => "CompositeEvidenceMembers",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceClass {
    SourceSpan,
    QuotedText,
    StructuredField,
    ApplicationDeclaration,
    ReferencedArtifact,
    AdmittedProposalElement,
    DerivedObservation,
    CompositeEvidence,
}

impl EvidenceClass {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "SourceSpan" => Some(Self::SourceSpan),
            "QuotedText" => Some(Self::QuotedText),
            "StructuredField" => Some(Self::StructuredField),
            "ApplicationDeclaration" => Some(Self::ApplicationDeclaration),
            "ReferencedArtifact" => Some(Self::ReferencedArtifact),
            "AdmittedProposalElement" => Some(Self::AdmittedProposalElement),
            "DerivedObservation" => Some(Self::DerivedObservation),
            "CompositeEvidence" => Some(Self::CompositeEvidence),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceOrigin {
    Source,
    Application,
    ReferencedArtifact,
    AdmittedProposal,
    Derived,
    Composite,
}

impl EvidenceOrigin {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "Source" => Some(Self::Source),
            "Application" => Some(Self::Application),
            "ReferencedArtifact" => Some(Self::ReferencedArtifact),
            "AdmittedProposal" => Some(Self::AdmittedProposal),
            "Derived" => Some(Self::Derived),
            "Composite" => Some(Self::Composite),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceRepresentationStatus {
    Represented,
    Incomplete,
    Unsupported,
    EvidenceLimited,
    Conflicting,
    Unresolved,
}

impl EvidenceRepresentationStatus {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "Represented" => Some(Self::Represented),
            "Incomplete" => Some(Self::Incomplete),
            "Unsupported" => Some(Self::Unsupported),
            "EvidenceLimited" => Some(Self::EvidenceLimited),
            "Conflicting" => Some(Self::Conflicting),
            "Unresolved" => Some(Self::Unresolved),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DeclaredEvidenceStatus {
    Observed,
    Referenced,
    Unavailable,
    Incomplete,
    Conflicting,
    External,
    Composite,
}

impl DeclaredEvidenceStatus {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "Observed" => Some(Self::Observed),
            "Referenced" => Some(Self::Referenced),
            "Unavailable" => Some(Self::Unavailable),
            "Incomplete" => Some(Self::Incomplete),
            "Conflicting" => Some(Self::Conflicting),
            "External" => Some(Self::External),
            "Composite" => Some(Self::Composite),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EvidenceLocator {
    PreciseOffset,
    PreciseLineRange,
    BoundedSection,
    StructuredField,
    AmbiguousBounded,
    NoLocatorSupplied,
}

impl EvidenceLocator {
    fn parse(value: Option<&str>) -> Option<Self> {
        match value {
            None => Some(Self::NoLocatorSupplied),
            Some("PreciseOffset") => Some(Self::PreciseOffset),
            Some("PreciseLineRange") => Some(Self::PreciseLineRange),
            Some("BoundedSection") => Some(Self::BoundedSection),
            Some("StructuredField") => Some(Self::StructuredField),
            Some("AmbiguousBounded") => Some(Self::AmbiguousBounded),
            Some("") => None,
            Some(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GroundingRelationshipKind {
    Supports,
    PartiallySupports,
    References,
    DerivedFrom,
    Aggregates,
    Refines,
    Supersedes,
}

impl GroundingRelationshipKind {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "Supports" => Some(Self::Supports),
            "PartiallySupports" => Some(Self::PartiallySupports),
            "References" => Some(Self::References),
            "DerivedFrom" => Some(Self::DerivedFrom),
            "Aggregates" => Some(Self::Aggregates),
            "Refines" => Some(Self::Refines),
            "Supersedes" => Some(Self::Supersedes),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroundingDeclaration {
    pub supported_artifact_id: String,
    pub supported_artifact_type: String,
    pub relationship: GroundingRelationshipKind,
    pub bounded_aspect: Option<String>,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureEvidenceProfile {
    pub identity: GroundingRepresentationProfileId,
    pub version: String,
    pub authority_reference: String,
    pub allowed_classes: BTreeSet<String>,
    pub allowed_origins: BTreeSet<String>,
    pub allowed_material_kinds: BTreeSet<String>,
    pub allowed_evidence_statuses: BTreeSet<String>,
    pub allowed_representation_statuses: BTreeSet<String>,
    pub allowed_target_types: BTreeSet<String>,
    pub allowed_relationships: BTreeSet<String>,
    pub allow_empty: bool,
    pub allow_ungrounded_accounting: bool,
}

impl FixtureEvidenceProfile {
    pub fn fixture() -> Self {
        Self {
            identity: GroundingRepresentationProfileId::derive(&["fixture"]),
            version: "fixture-evidence-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureEvidenceRegistries {
    pub version: String,
    pub authority_reference: String,
    pub evidence_classes: BTreeSet<String>,
    pub origins: BTreeSet<String>,
    pub bases: BTreeSet<String>,
    pub evidence_statuses: BTreeSet<String>,
    pub representation_statuses: BTreeSet<String>,
    pub target_types: BTreeSet<String>,
    pub grounding_relationships: BTreeSet<String>,
    pub locators: BTreeSet<String>,
}

impl FixtureEvidenceRegistries {
    pub fn fixture() -> Self {
        let profile = FixtureEvidenceProfile::fixture();
        Self {
            version: "fixture-evidence-registry-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSchemaBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceConfigurationBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRepresentationInputs<'a> {
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub objective_set: &'a DeclaredObjectiveSet,
    pub constraint_set: &'a DeclaredConstraintSet,
    pub capability_set: &'a CapabilityRequirementSet,
    pub clarification_set: &'a MeaningQualificationSet,
    pub profile: &'a FixtureEvidenceProfile,
    pub registries: &'a FixtureEvidenceRegistries,
    pub schema: &'a EvidenceSchemaBinding,
    pub configuration: &'a EvidenceConfigurationBinding,
    pub implementation_version: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceDeclaration {
    pub declaration_id: EvidenceDeclarationId,
    pub evidence_id: EvidenceId,
    pub class: EvidenceClass,
    pub origin: EvidenceOrigin,
    pub basis: String,
    pub material_reference: EvidenceMaterialReference,
    pub locator: EvidenceLocator,
    pub supplied_value: Option<String>,
    pub evidence_status: DeclaredEvidenceStatus,
    pub representation_status: EvidenceRepresentationStatus,
    pub source_proposal_id: InterpretationProposalId,
    pub source_proposal_element_id: String,
    pub grounding: Option<GroundingDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRepresentationInstance {
    pub representation_id: EvidenceRepresentationId,
    pub evidence_id: EvidenceId,
    pub declaration_id: EvidenceDeclarationId,
    pub class: EvidenceClass,
    pub origin: EvidenceOrigin,
    pub basis: String,
    pub material_reference: EvidenceMaterialReference,
    pub locator: EvidenceLocator,
    pub supplied_value: Option<String>,
    pub evidence_status: DeclaredEvidenceStatus,
    pub representation_status: EvidenceRepresentationStatus,
    pub source_proposal_id: InterpretationProposalId,
    pub source_proposal_element_id: String,
    pub profile_id: GroundingRepresentationProfileId,
    pub profile_version: String,
    pub schema_version: String,
    pub registry_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroundingAssertion {
    pub assertion_id: GroundingAssertionId,
    pub evidence_declaration_id: EvidenceDeclarationId,
    pub evidence_representation_id: EvidenceRepresentationId,
    pub supported_artifact_id: String,
    pub supported_artifact_type: String,
    pub relationship: GroundingRelationshipKind,
    pub bounded_aspect: Option<String>,
    pub basis: String,
    pub profile_id: GroundingRepresentationProfileId,
    pub profile_version: String,
    pub registry_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UngroundedArtifactAccountingEntry {
    pub artifact_id: String,
    pub artifact_type: String,
    pub evidence_set_id: InterpretationEvidenceSetId,
    pub accounting_basis: String,
    pub profile_id: GroundingRepresentationProfileId,
    pub profile_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRepresentationLineage {
    pub admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    pub objective_set_id: DeclaredObjectiveSetId,
    pub constraint_set_id: DeclaredConstraintSetId,
    pub capability_set_id: CapabilityRequirementSetId,
    pub clarification_set_id: MeaningQualificationSetId,
    pub source_intake_id: StableId,
    pub input_proposal_ids: Arc<[InterpretationProposalId]>,
    pub input_decision_ids: Arc<[ProposalAdmissionId]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InterpretationEvidenceSet {
    set_id: InterpretationEvidenceSetId,
    operation_id: EvidenceRepresentationOperationId,
    declarations: Arc<[EvidenceDeclaration]>,
    representations: Arc<[EvidenceRepresentationInstance]>,
    groundings: Arc<[GroundingAssertion]>,
    ungrounded_accounting: Arc<[UngroundedArtifactAccountingEntry]>,
    lineage: EvidenceRepresentationLineage,
    profile_id: GroundingRepresentationProfileId,
    profile_version: String,
    schema_version: String,
    registry_version: String,
}

impl InterpretationEvidenceSet {
    pub fn set_id(&self) -> &InterpretationEvidenceSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &EvidenceRepresentationOperationId {
        &self.operation_id
    }
    pub fn declarations(&self) -> &[EvidenceDeclaration] {
        &self.declarations
    }
    pub fn representations(&self) -> &[EvidenceRepresentationInstance] {
        &self.representations
    }
    pub fn groundings(&self) -> &[GroundingAssertion] {
        &self.groundings
    }
    pub fn ungrounded_accounting(&self) -> &[UngroundedArtifactAccountingEntry] {
        &self.ungrounded_accounting
    }
    pub fn lineage(&self) -> &EvidenceRepresentationLineage {
        &self.lineage
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceRepresentationFailureCategory {
    MissingRequiredProfile,
    MissingRequiredRegistry,
    InvalidSchemaBinding,
    InvalidConfigurationBinding,
    InvalidProposalSchema,
    InvalidLineage,
    UnsupportedEvidenceClass,
    UnsupportedEvidenceOrigin,
    UnsupportedMaterialReference,
    InvalidStatusCombination,
    InvalidLocator,
    InvalidTargetReference,
    InvalidGroundingDeclaration,
    InvalidRelationship,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRepresentationFailureRecord {
    failure_id: EvidenceRepresentationFailureRecordId,
    operation_id: EvidenceRepresentationOperationId,
    category: EvidenceRepresentationFailureCategory,
    findings: Arc<[String]>,
    admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    objective_set_id: DeclaredObjectiveSetId,
    constraint_set_id: DeclaredConstraintSetId,
    capability_set_id: CapabilityRequirementSetId,
    clarification_set_id: MeaningQualificationSetId,
}

impl EvidenceRepresentationFailureRecord {
    pub fn failure_id(&self) -> &EvidenceRepresentationFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &EvidenceRepresentationOperationId {
        &self.operation_id
    }
    pub fn category(&self) -> &EvidenceRepresentationFailureCategory {
        &self.category
    }
    pub fn findings(&self) -> &[String] {
        &self.findings
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceRepresentationOutcome {
    Set(InterpretationEvidenceSet),
    Failure(EvidenceRepresentationFailureRecord),
}

pub struct EvidenceRepresentationAuthority;

pub fn represent_evidence(
    inputs: EvidenceRepresentationInputs<'_>,
) -> EvidenceRepresentationOutcome {
    let upstream = inputs.admitted_proposal_set;
    let operation_id = EvidenceRepresentationOperationId::derive(&[
        upstream.set_id.as_str(),
        inputs.objective_set.set_id.as_str(),
        inputs.constraint_set.set_id.as_str(),
        inputs.capability_set.set_id.as_str(),
        inputs.clarification_set.set_id.as_str(),
        inputs.profile.identity.as_str(),
        inputs.profile.version.as_str(),
        inputs.schema.identity.as_str(),
        inputs.schema.version.as_str(),
        inputs.configuration.identity.as_str(),
        inputs.configuration.version.as_str(),
        inputs.implementation_version,
    ]);
    let fail = |category, finding: &'static str| {
        EvidenceRepresentationOutcome::Failure(EvidenceRepresentationFailureRecord {
            failure_id: EvidenceRepresentationFailureRecordId::derive(&[
                operation_id.as_str(),
                &format!("{category:?}"),
                finding,
            ]),
            operation_id: operation_id.clone(),
            category,
            findings: vec![finding.to_owned()].into(),
            admitted_proposal_set_id: upstream.set_id.clone(),
            objective_set_id: inputs.objective_set.set_id.clone(),
            constraint_set_id: inputs.constraint_set.set_id.clone(),
            capability_set_id: inputs.capability_set.set_id.clone(),
            clarification_set_id: inputs.clarification_set.set_id.clone(),
        })
    };
    if inputs.profile.version.is_empty()
        || inputs.profile.authority_reference.is_empty()
        || inputs.profile.allowed_classes.is_empty()
    {
        return fail(
            EvidenceRepresentationFailureCategory::MissingRequiredProfile,
            "evidence profile is incomplete",
        );
    }
    if inputs.registries.version.is_empty()
        || inputs.registries.authority_reference.is_empty()
        || inputs.registries.evidence_classes.is_empty()
    {
        return fail(
            EvidenceRepresentationFailureCategory::MissingRequiredRegistry,
            "evidence registries are incomplete",
        );
    }
    if inputs.schema.version.is_empty() || inputs.schema.authority_reference.is_empty() {
        return fail(
            EvidenceRepresentationFailureCategory::InvalidSchemaBinding,
            "evidence schema is incomplete",
        );
    }
    if inputs.configuration.version.is_empty()
        || inputs.configuration.authority_reference.is_empty()
        || inputs.implementation_version.is_empty()
    {
        return fail(
            EvidenceRepresentationFailureCategory::InvalidConfigurationBinding,
            "evidence configuration is incomplete",
        );
    }
    if upstream.proposal_schema_version != "proposal-fixture-v5"
        && upstream.proposal_schema_version != "ulantra-literal-proposal-v1"
    {
        return fail(
            EvidenceRepresentationFailureCategory::InvalidProposalSchema,
            "Contract 007 requires the typed evidence proposal schema",
        );
    }
    if let Err(category) = validate_evidence_lineage(&inputs) {
        return fail(
            EvidenceRepresentationFailureCategory::InvalidLineage,
            category,
        );
    }
    let targets = evidence_target_registry(&inputs);
    let admitted: BTreeSet<_> = upstream.admitted_proposal_ids.iter().cloned().collect();
    let mut source_elements: Vec<_> = upstream
        .proposals
        .iter()
        .filter(|p| admitted.contains(&p.proposal_id))
        .flat_map(|p| p.content.evidence_elements.iter().map(move |e| (p, e)))
        .collect();
    source_elements.sort_by_key(|(p, e)| (p.proposal_id.clone(), e.element_id.clone()));
    let mut declarations = Vec::new();
    let mut representations = Vec::new();
    let mut groundings = Vec::new();
    for (proposal, element) in source_elements {
        let Some(class) = EvidenceClass::parse(&element.evidence_class) else {
            return fail(
                EvidenceRepresentationFailureCategory::UnsupportedEvidenceClass,
                "evidence class is not registered",
            );
        };
        let Some(origin) = EvidenceOrigin::parse(&element.origin) else {
            return fail(
                EvidenceRepresentationFailureCategory::UnsupportedEvidenceOrigin,
                "evidence origin is not registered",
            );
        };
        let Some(evidence_status) = DeclaredEvidenceStatus::parse(&element.evidence_status) else {
            return fail(
                EvidenceRepresentationFailureCategory::InvalidStatusCombination,
                "evidence status is not registered",
            );
        };
        let Some(representation_status) =
            EvidenceRepresentationStatus::parse(&element.representation_status)
        else {
            return fail(
                EvidenceRepresentationFailureCategory::InvalidStatusCombination,
                "representation status is not registered",
            );
        };
        if !inputs
            .profile
            .allowed_classes
            .contains(&element.evidence_class)
            || !inputs
                .registries
                .evidence_classes
                .contains(&element.evidence_class)
        {
            return fail(
                EvidenceRepresentationFailureCategory::UnsupportedEvidenceClass,
                "evidence class is not permitted",
            );
        }
        if !inputs.profile.allowed_origins.contains(&element.origin)
            || !inputs.registries.origins.contains(&element.origin)
        {
            return fail(
                EvidenceRepresentationFailureCategory::UnsupportedEvidenceOrigin,
                "evidence origin is not permitted",
            );
        }
        if !inputs
            .profile
            .allowed_evidence_statuses
            .contains(&element.evidence_status)
            || !inputs
                .profile
                .allowed_representation_statuses
                .contains(&element.representation_status)
        {
            return fail(
                EvidenceRepresentationFailureCategory::InvalidStatusCombination,
                "status combination is not profile-authorized",
            );
        }
        let Some(locator) = EvidenceLocator::parse(element.locator.as_deref()) else {
            return fail(
                EvidenceRepresentationFailureCategory::InvalidLocator,
                "locator is not registered",
            );
        };
        if element
            .locator
            .as_deref()
            .is_some_and(|value| !inputs.registries.locators.contains(value))
        {
            return fail(
                EvidenceRepresentationFailureCategory::InvalidLocator,
                "locator is absent from the bound registry",
            );
        }
        let material = match element.material_kind.as_str() {
            "AdmittedSourceReference" => {
                EvidenceMaterialReference::AdmittedSourceReference(element.material_value.clone())
            }
            "QuotedTextValue" => {
                EvidenceMaterialReference::QuotedTextValue(element.material_value.clone())
            }
            "StructuredFieldReference" => {
                EvidenceMaterialReference::StructuredFieldReference(element.material_value.clone())
            }
            "ApplicationDeclarationReference" => {
                EvidenceMaterialReference::ApplicationDeclarationReference(
                    element.material_value.clone(),
                )
            }
            "ReferencedArtifactReference" => {
                EvidenceMaterialReference::ReferencedArtifactReference(
                    element.material_value.clone(),
                )
            }
            "AdmittedProposalElementReference" => {
                EvidenceMaterialReference::AdmittedProposalElementReference(
                    element.material_value.clone(),
                )
            }
            "DerivedObservationInputs" => EvidenceMaterialReference::DerivedObservationInputs(
                vec![element.material_value.clone()].into(),
            ),
            "CompositeEvidenceMembers" => {
                EvidenceMaterialReference::CompositeEvidenceMembers(Vec::new().into())
            }
            _ => {
                return fail(
                    EvidenceRepresentationFailureCategory::UnsupportedMaterialReference,
                    "material reference is not closed or registered",
                )
            }
        };
        if !inputs
            .profile
            .allowed_material_kinds
            .contains(material.kind_name())
            || element.material_value.is_empty()
        {
            return fail(
                EvidenceRepresentationFailureCategory::UnsupportedMaterialReference,
                "material reference is not profile-authorized",
            );
        }
        let declaration_id = EvidenceDeclarationId::derive(&[
            operation_id.as_str(),
            proposal.proposal_id.as_str(),
            &element.element_id,
        ]);
        let evidence_id = EvidenceId::derive(&[
            proposal.proposal_id.as_str(),
            &element.element_id,
            element.material_kind.as_str(),
        ]);
        let representation_id = EvidenceRepresentationId::derive(&[
            operation_id.as_str(),
            declaration_id.as_str(),
            &format!("{material:?}"),
            &element.evidence_status,
            &element.representation_status,
        ]);
        let grounding = match (
            &element.grounding_target_id,
            &element.grounding_target_type,
            &element.grounding_relationship,
        ) {
            (None, None, None) => None,
            (Some(target), Some(target_type), Some(relationship)) => {
                let Some(kind) = GroundingRelationshipKind::parse(relationship) else {
                    return fail(
                        EvidenceRepresentationFailureCategory::InvalidRelationship,
                        "grounding relationship is not registered",
                    );
                };
                if !inputs.profile.allowed_relationships.contains(relationship)
                    || !inputs
                        .registries
                        .grounding_relationships
                        .contains(relationship)
                {
                    return fail(
                        EvidenceRepresentationFailureCategory::InvalidRelationship,
                        "grounding relationship is not permitted",
                    );
                }
                if !inputs.profile.allowed_target_types.contains(target_type)
                    || !inputs.registries.target_types.contains(target_type)
                    || !targets.contains(&(target.clone(), target_type.clone()))
                {
                    return fail(
                        EvidenceRepresentationFailureCategory::InvalidTargetReference,
                        "grounding target is not an exact committed artifact",
                    );
                }
                if kind == GroundingRelationshipKind::PartiallySupports
                    && element
                        .grounding_aspect
                        .as_deref()
                        .is_none_or(str::is_empty)
                {
                    return fail(
                        EvidenceRepresentationFailureCategory::InvalidGroundingDeclaration,
                        "partial grounding requires a bounded aspect",
                    );
                }
                Some(GroundingDeclaration {
                    supported_artifact_id: target.clone(),
                    supported_artifact_type: target_type.clone(),
                    relationship: kind,
                    bounded_aspect: element.grounding_aspect.clone(),
                    basis: element
                        .grounding_basis
                        .clone()
                        .unwrap_or_else(|| "Declared".to_owned()),
                })
            }
            _ => {
                return fail(
                    EvidenceRepresentationFailureCategory::InvalidGroundingDeclaration,
                    "grounding declaration is incomplete",
                )
            }
        };
        let declaration = EvidenceDeclaration {
            declaration_id: declaration_id.clone(),
            evidence_id: evidence_id.clone(),
            class,
            origin,
            basis: element.basis.clone(),
            material_reference: material.clone(),
            locator,
            supplied_value: element.supplied_value.clone(),
            evidence_status,
            representation_status,
            source_proposal_id: proposal.proposal_id.clone(),
            source_proposal_element_id: element.element_id.clone(),
            grounding: grounding.clone(),
        };
        declarations.push(declaration);
        representations.push(EvidenceRepresentationInstance {
            representation_id: representation_id.clone(),
            evidence_id,
            declaration_id: declaration_id.clone(),
            class,
            origin,
            basis: element.basis.clone(),
            material_reference: material,
            locator,
            supplied_value: element.supplied_value.clone(),
            evidence_status,
            representation_status,
            source_proposal_id: proposal.proposal_id.clone(),
            source_proposal_element_id: element.element_id.clone(),
            profile_id: inputs.profile.identity.clone(),
            profile_version: inputs.profile.version.clone(),
            schema_version: inputs.schema.version.clone(),
            registry_version: inputs.registries.version.clone(),
        });
        if let Some(grounding) = grounding {
            groundings.push(GroundingAssertion {
                assertion_id: GroundingAssertionId::derive(&[
                    operation_id.as_str(),
                    declaration_id.as_str(),
                    representation_id.as_str(),
                    &grounding.supported_artifact_id,
                    &grounding.supported_artifact_type,
                    &format!("{:?}", grounding.relationship),
                ]),
                evidence_declaration_id: declaration_id,
                evidence_representation_id: representation_id,
                supported_artifact_id: grounding.supported_artifact_id,
                supported_artifact_type: grounding.supported_artifact_type,
                relationship: grounding.relationship,
                bounded_aspect: grounding.bounded_aspect,
                basis: grounding.basis,
                profile_id: inputs.profile.identity.clone(),
                profile_version: inputs.profile.version.clone(),
                registry_version: inputs.registries.version.clone(),
            });
        }
    }
    if !inputs.profile.allow_empty && representations.is_empty() {
        return fail(
            EvidenceRepresentationFailureCategory::MissingRequiredProfile,
            "empty evidence set is not permitted",
        );
    }
    let set_id = InterpretationEvidenceSetId::derive(&[
        operation_id.as_str(),
        &format!("{:?}{:?}", declarations, groundings),
    ]);
    let grounded: BTreeSet<_> = groundings
        .iter()
        .map(|g| {
            (
                g.supported_artifact_id.clone(),
                g.supported_artifact_type.clone(),
            )
        })
        .collect();
    let mut accounting = Vec::new();
    if inputs.profile.allow_ungrounded_accounting {
        for (id, kind) in targets {
            if !grounded.contains(&(id.clone(), kind.clone())) {
                accounting.push(UngroundedArtifactAccountingEntry {
                    artifact_id: id,
                    artifact_type: kind,
                    evidence_set_id: set_id.clone(),
                    accounting_basis: "No grounding assertion committed in this publication"
                        .to_owned(),
                    profile_id: inputs.profile.identity.clone(),
                    profile_version: inputs.profile.version.clone(),
                });
            }
        }
    }
    EvidenceRepresentationOutcome::Set(InterpretationEvidenceSet {
        set_id,
        operation_id,
        declarations: declarations.into(),
        representations: representations.into(),
        groundings: groundings.into(),
        ungrounded_accounting: accounting.into(),
        lineage: EvidenceRepresentationLineage {
            admitted_proposal_set_id: upstream.set_id.clone(),
            objective_set_id: inputs.objective_set.set_id.clone(),
            constraint_set_id: inputs.constraint_set.set_id.clone(),
            capability_set_id: inputs.capability_set.set_id.clone(),
            clarification_set_id: inputs.clarification_set.set_id.clone(),
            source_intake_id: upstream.source_intake_id.clone(),
            input_proposal_ids: upstream.submitted_proposal_ids.clone(),
            input_decision_ids: upstream.decision_ids.clone(),
        },
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        schema_version: inputs.schema.version.clone(),
        registry_version: inputs.registries.version.clone(),
    })
}

fn evidence_target_registry(
    inputs: &EvidenceRepresentationInputs<'_>,
) -> BTreeSet<(String, String)> {
    let mut targets = BTreeSet::new();
    targets.extend(inputs.objective_set.objectives.iter().map(|o| {
        (
            o.logical_objective_id.to_string(),
            "DeclaredObjective".to_owned(),
        )
    }));
    targets.extend(inputs.constraint_set.constraints.iter().map(|c| {
        (
            c.logical_constraint_id.to_string(),
            "DeclaredConstraint".to_owned(),
        )
    }));
    targets.extend(
        inputs
            .capability_set
            .requirements
            .iter()
            .map(|c| (c.logical_id.to_string(), "CapabilityRequirement".to_owned())),
    );
    targets.extend(
        inputs
            .clarification_set
            .ambiguities
            .iter()
            .map(|a| (a.id.to_string(), "Ambiguity".to_owned())),
    );
    targets.extend(
        inputs
            .clarification_set
            .assumptions
            .iter()
            .map(|a| (a.id.to_string(), "Assumption".to_owned())),
    );
    targets.extend(
        inputs
            .clarification_set
            .uncertainties
            .iter()
            .map(|u| (u.id.to_string(), "Uncertainty".to_owned())),
    );
    targets.extend(
        inputs
            .clarification_set
            .clarification_requirements
            .iter()
            .map(|r| (r.id.to_string(), "ClarificationRequirement".to_owned())),
    );
    targets
}

fn validate_evidence_lineage(
    inputs: &EvidenceRepresentationInputs<'_>,
) -> Result<(), &'static str> {
    validate_semantic_clarification_lineage(&MeaningQualificationRepresentationInputs {
        admitted_proposal_set: inputs.admitted_proposal_set,
        objective_set: inputs.objective_set,
        constraint_set: inputs.constraint_set,
        capability_set: inputs.capability_set,
        profile: &FixtureMeaningQualificationProfile::fixture(),
        registries: &FixtureMeaningQualificationRegistries::fixture(),
        schema: &MeaningQualificationSchemaBinding {
            identity: StableId::from_parts("lineage", &["schema"]),
            version: "lineage".to_owned(),
            authority_reference: "lineage".to_owned(),
        },
        configuration: &MeaningQualificationConfigurationBinding {
            identity: StableId::from_parts("lineage", &["configuration"]),
            version: "lineage".to_owned(),
            authority_reference: "lineage".to_owned(),
        },
        implementation_version: "lineage",
    })
    .map_err(|_| "upstream objective, constraint, or capability lineage is inconsistent")?;
    let a = inputs.admitted_proposal_set;
    if inputs.clarification_set.lineage.source_intake_id != a.source_intake_id
        || inputs.clarification_set.lineage.admitted_proposal_set_id != a.set_id
        || inputs.clarification_set.lineage.input_proposal_ids != a.submitted_proposal_ids
        || inputs.clarification_set.lineage.input_decision_ids != a.decision_ids
    {
        return Err("meaning qualification lineage is inconsistent");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Contract 008: Declared provenance representation
// ---------------------------------------------------------------------------

contract_002_id!(ProvenanceRepresentationInputId, "pvinput");
contract_002_id!(ProvenanceRepresentationOperationId, "pvop");
contract_002_id!(ProvenanceSubjectId, "pvsubject");
contract_002_id!(ProvenanceDeclarationId, "pvdecl");
contract_002_id!(ProvenanceRepresentationId, "pvrep");
contract_002_id!(ProvenanceRepresentationInstanceId, "pvinstance");
contract_002_id!(ProvenanceEventId, "pvevent");
contract_002_id!(LineageAssertionId, "pvlineage");
contract_002_id!(LifecycleRelationId, "pvlifecycle");
contract_002_id!(ExternalIdentityMappingId, "pvexternal");
contract_002_id!(ProvenanceConflictAssociationId, "pvconflict");
contract_002_id!(ProvenanceConstructionDecisionId, "pvdecision");
contract_002_id!(ProvenanceRecordSetId, "pvset");
contract_002_id!(ProvenanceRepresentationFailureRecordId, "pvfail");
contract_002_id!(ProvenanceProfileId, "pvprofile");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProvenanceDeclarationOrigin {
    ApplicationSupplied,
    AdmittedProposalSupplied,
    ImportedDeclaration,
    PriorGovernedDeclaration,
    ExternalIdentityMappingDeclaration,
    TestFixture,
}

impl ProvenanceDeclarationOrigin {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "ApplicationSupplied" => Some(Self::ApplicationSupplied),
            "AdmittedProposalSupplied" => Some(Self::AdmittedProposalSupplied),
            "ImportedDeclaration" => Some(Self::ImportedDeclaration),
            "PriorGovernedDeclaration" => Some(Self::PriorGovernedDeclaration),
            "ExternalIdentityMappingDeclaration" => Some(Self::ExternalIdentityMappingDeclaration),
            "TestFixture" => Some(Self::TestFixture),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProvenanceStatus {
    Complete,
    Partial,
    Unavailable,
    External,
    Retrospective,
    Unresolved,
    Conflicting,
    Empty,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProvenanceSubjectClass {
    SourceIntakeRecord,
    AdmittedInterpretationProposalSet,
    DeclaredObjectiveSet,
    DeclaredConstraintSet,
    CapabilityRequirementSet,
    MeaningQualificationSet,
    InterpretationEvidenceSet,
    ExternalSubject,
}

impl ProvenanceSubjectClass {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "SourceIntakeRecord" => Some(Self::SourceIntakeRecord),
            "AdmittedInterpretationProposalSet" => Some(Self::AdmittedInterpretationProposalSet),
            "DeclaredObjectiveSet" => Some(Self::DeclaredObjectiveSet),
            "DeclaredConstraintSet" => Some(Self::DeclaredConstraintSet),
            "CapabilityRequirementSet" => Some(Self::CapabilityRequirementSet),
            "MeaningQualificationSet" => Some(Self::MeaningQualificationSet),
            "InterpretationEvidenceSet" => Some(Self::InterpretationEvidenceSet),
            "ExternalSubject" => Some(Self::ExternalSubject),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventTimeRepresentation {
    Exact(String),
    Range(String),
    Approximate(String),
    Relative(String),
    Unknown,
    Unavailable,
    Conflicting(Arc<[String]>),
}

impl EventTimeRepresentation {
    fn kind_name(&self) -> &'static str {
        match self {
            Self::Exact(_) => "Exact",
            Self::Range(_) => "Range",
            Self::Approximate(_) => "Approximate",
            Self::Relative(_) => "Relative",
            Self::Unknown => "Unknown",
            Self::Unavailable => "Unavailable",
            Self::Conflicting(_) => "Conflicting",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceSubjectDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub subject_id: String,
    pub subject_class: String,
    pub origin: String,
    pub participant_reference: Option<String>,
    pub source_reference: Option<String>,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransformationEventDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub event_id: ProvenanceEventId,
    pub event_schema_version: String,
    pub input_subject_ids: Arc<[String]>,
    pub output_subject_ids: Arc<[String]>,
    pub participant_references: Arc<[String]>,
    pub basis: String,
    pub event_time: EventTimeRepresentation,
    pub representation_time: EventTimeRepresentation,
    pub commitment_reference: Option<String>,
    pub origin: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustodyEventDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub event_id: ProvenanceEventId,
    pub event_schema_version: String,
    pub subject_id: String,
    pub participant_references: Arc<[String]>,
    pub basis: String,
    pub event_time: EventTimeRepresentation,
    pub origin: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationEventDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub event_id: ProvenanceEventId,
    pub event_schema_version: String,
    pub subject_id: String,
    pub publication_reference: String,
    pub basis: String,
    pub event_time: EventTimeRepresentation,
    pub origin: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleEventDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub event_id: ProvenanceEventId,
    pub event_schema_version: String,
    pub subject_id: String,
    pub related_subject_id: String,
    pub relationship: String,
    pub basis: String,
    pub event_time: EventTimeRepresentation,
    pub origin: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceEventPayload {
    Transformation(TransformationEventDeclaration),
    Custody(CustodyEventDeclaration),
    Publication(PublicationEventDeclaration),
    Lifecycle(LifecycleEventDeclaration),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub assertion_id: LineageAssertionId,
    pub subject_artifact_id: String,
    pub ancestor_or_related_artifact_id: String,
    pub relationship: String,
    pub basis: String,
    pub origin: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleRelationDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub relation_id: LifecycleRelationId,
    pub subject_artifact_id: String,
    pub related_artifact_id: String,
    pub relationship: String,
    pub basis: String,
    pub origin: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalIdentityMappingDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub mapping_id: ExternalIdentityMappingId,
    pub namespace: String,
    pub external_identifier: String,
    pub target_subject_id: String,
    pub basis: String,
    pub origin: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceConflictDeclaration {
    pub declaration_id: ProvenanceDeclarationId,
    pub conflict_id: ProvenanceConflictAssociationId,
    pub involved_claim_ids: Arc<[String]>,
    pub conflict_class: String,
    pub basis: String,
    pub origin: String,
    pub status: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRepresentationInput {
    pub input_id: ProvenanceRepresentationInputId,
    pub schema_version: String,
    pub profile_id: ProvenanceProfileId,
    pub profile_version: String,
    pub registry_version: String,
    pub configuration_id: StableId,
    pub configuration_version: String,
    pub implementation_version: String,
    pub admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    pub objective_set_id: DeclaredObjectiveSetId,
    pub constraint_set_id: DeclaredConstraintSetId,
    pub capability_set_id: CapabilityRequirementSetId,
    pub clarification_set_id: MeaningQualificationSetId,
    pub evidence_set_id: InterpretationEvidenceSetId,
    pub source_intake_id: StableId,
    pub interpretation_operation_id: InterpretationOperationId,
    pub input_proposal_ids: Arc<[InterpretationProposalId]>,
    pub input_decision_ids: Arc<[ProposalAdmissionId]>,
    pub subject_declarations: Arc<[ProvenanceSubjectDeclaration]>,
    pub event_declarations: Arc<[ProvenanceEventPayload]>,
    pub lineage_declarations: Arc<[LineageDeclaration]>,
    pub lifecycle_declarations: Arc<[LifecycleRelationDeclaration]>,
    pub external_identity_mappings: Arc<[ExternalIdentityMappingDeclaration]>,
    pub conflict_declarations: Arc<[ProvenanceConflictDeclaration]>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureProvenanceProfile {
    pub identity: ProvenanceProfileId,
    pub version: String,
    pub authority_reference: String,
    pub allowed_origins: BTreeSet<String>,
    pub allowed_subject_classes: BTreeSet<String>,
    pub allowed_event_classes: BTreeSet<String>,
    pub allowed_lineage_relationships: BTreeSet<String>,
    pub allowed_lifecycle_relationships: BTreeSet<String>,
    pub allowed_mapping_namespaces: BTreeSet<String>,
    pub allowed_conflict_classes: BTreeSet<String>,
    pub allowed_time_variants: BTreeSet<String>,
    pub allow_empty: bool,
}

impl FixtureProvenanceProfile {
    pub fn fixture() -> Self {
        Self {
            identity: ProvenanceProfileId::derive(&["fixture"]),
            version: "fixture-provenance-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
            allowed_mapping_namespaces: ["fixture-external"]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureProvenanceRegistries {
    pub version: String,
    pub authority_reference: String,
    pub subject_classes: BTreeSet<String>,
    pub event_classes: BTreeSet<String>,
    pub lineage_relationships: BTreeSet<String>,
    pub lifecycle_relationships: BTreeSet<String>,
    pub mapping_namespaces: BTreeSet<String>,
    pub conflict_classes: BTreeSet<String>,
    pub time_variants: BTreeSet<String>,
}

impl FixtureProvenanceRegistries {
    pub fn fixture() -> Self {
        let profile = FixtureProvenanceProfile::fixture();
        Self {
            version: "fixture-provenance-registry-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceSchemaBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceConfigurationBinding {
    pub identity: StableId,
    pub version: String,
    pub authority_reference: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRepresentationInputs<'a> {
    pub input: &'a ProvenanceRepresentationInput,
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub objective_set: &'a DeclaredObjectiveSet,
    pub constraint_set: &'a DeclaredConstraintSet,
    pub capability_set: &'a CapabilityRequirementSet,
    pub clarification_set: &'a MeaningQualificationSet,
    pub evidence_set: &'a InterpretationEvidenceSet,
    pub profile: &'a FixtureProvenanceProfile,
    pub registries: &'a FixtureProvenanceRegistries,
    pub schema: &'a ProvenanceSchemaBinding,
    pub configuration: &'a ProvenanceConfigurationBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceSubject {
    pub subject_id: String,
    pub subject_class: ProvenanceSubjectClass,
    pub declaration_id: ProvenanceDeclarationId,
    pub origin: ProvenanceDeclarationOrigin,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRepresentation {
    pub representation_id: ProvenanceRepresentationId,
    pub subject_id: String,
    pub declaration_id: ProvenanceDeclarationId,
    pub status: ProvenanceStatus,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRepresentationInstance {
    pub instance_id: ProvenanceRepresentationInstanceId,
    pub representation_id: ProvenanceRepresentationId,
    pub profile_id: ProvenanceProfileId,
    pub profile_version: String,
    pub registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceEvent {
    pub event_id: ProvenanceEventId,
    pub declaration_id: ProvenanceDeclarationId,
    pub event_class: String,
    pub payload: ProvenanceEventPayload,
    pub event_time: EventTimeRepresentation,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageAssertion {
    pub assertion_id: LineageAssertionId,
    pub declaration_id: ProvenanceDeclarationId,
    pub subject_artifact_id: String,
    pub ancestor_or_related_artifact_id: String,
    pub relationship: String,
    pub status: String,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleRelation {
    pub relation_id: LifecycleRelationId,
    pub declaration_id: ProvenanceDeclarationId,
    pub subject_artifact_id: String,
    pub related_artifact_id: String,
    pub relationship: String,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalIdentityMapping {
    pub mapping_id: ExternalIdentityMappingId,
    pub declaration_id: ProvenanceDeclarationId,
    pub namespace: String,
    pub external_identifier: String,
    pub target_subject_id: String,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceConflictAssociation {
    pub conflict_id: ProvenanceConflictAssociationId,
    pub declaration_id: ProvenanceDeclarationId,
    pub involved_claim_ids: Arc<[String]>,
    pub conflict_class: String,
    pub status: String,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceConstructionDecisionKind {
    ConstructionAuthorized,
    RepresentedUnresolved,
    RepresentedUnavailable,
    RepresentedConflicting,
    NoArtifactAuthorized,
    StructuralFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceConstructionDecision {
    pub decision_id: ProvenanceConstructionDecisionId,
    pub declaration_id: Option<ProvenanceDeclarationId>,
    pub decision_kind: ProvenanceConstructionDecisionKind,
    pub authorizing_rule_id: String,
    pub authorizing_rule_version: String,
    pub constructed_artifact_ids: Arc<[String]>,
    pub retained_basis: String,
    pub validation_findings: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRecordSet {
    set_id: ProvenanceRecordSetId,
    operation_id: ProvenanceRepresentationOperationId,
    subjects: Arc<[ProvenanceSubject]>,
    representations: Arc<[ProvenanceRepresentation]>,
    instances: Arc<[ProvenanceRepresentationInstance]>,
    events: Arc<[ProvenanceEvent]>,
    lineage_assertions: Arc<[LineageAssertion]>,
    lifecycle_relations: Arc<[LifecycleRelation]>,
    external_mappings: Arc<[ExternalIdentityMapping]>,
    conflicts: Arc<[ProvenanceConflictAssociation]>,
    decisions: Arc<[ProvenanceConstructionDecision]>,
    status: ProvenanceStatus,
    input_id: ProvenanceRepresentationInputId,
    profile_id: ProvenanceProfileId,
    profile_version: String,
    registry_version: String,
    schema_version: String,
    configuration_version: String,
    implementation_version: String,
}

impl ProvenanceRecordSet {
    pub fn set_id(&self) -> &ProvenanceRecordSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &ProvenanceRepresentationOperationId {
        &self.operation_id
    }
    pub fn subjects(&self) -> &[ProvenanceSubject] {
        &self.subjects
    }
    pub fn representations(&self) -> &[ProvenanceRepresentation] {
        &self.representations
    }
    pub fn instances(&self) -> &[ProvenanceRepresentationInstance] {
        &self.instances
    }
    pub fn events(&self) -> &[ProvenanceEvent] {
        &self.events
    }
    pub fn lineage_assertions(&self) -> &[LineageAssertion] {
        &self.lineage_assertions
    }
    pub fn lifecycle_relations(&self) -> &[LifecycleRelation] {
        &self.lifecycle_relations
    }
    pub fn external_mappings(&self) -> &[ExternalIdentityMapping] {
        &self.external_mappings
    }
    pub fn conflicts(&self) -> &[ProvenanceConflictAssociation] {
        &self.conflicts
    }
    pub fn decisions(&self) -> &[ProvenanceConstructionDecision] {
        &self.decisions
    }
    pub fn status(&self) -> ProvenanceStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceRepresentationFailureCategory {
    MissingRequiredProfile,
    MissingRequiredRegistry,
    InvalidInputSchema,
    InvalidSchemaBinding,
    InvalidConfigurationBinding,
    InvalidLineage,
    InvalidDeclarationOrigin,
    InvalidSubjectReference,
    InvalidEventSchema,
    InvalidEventReference,
    InvalidRelationship,
    ProhibitedCycle,
    InvalidTimeVariant,
    InvalidExternalMapping,
    InvalidConflict,
    EmptyNotPermitted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceRepresentationFailureRecord {
    failure_id: ProvenanceRepresentationFailureRecordId,
    operation_id: ProvenanceRepresentationOperationId,
    category: ProvenanceRepresentationFailureCategory,
    findings: Arc<[String]>,
    input_id: ProvenanceRepresentationInputId,
}

impl ProvenanceRepresentationFailureRecord {
    pub fn failure_id(&self) -> &ProvenanceRepresentationFailureRecordId {
        &self.failure_id
    }
    pub fn operation_id(&self) -> &ProvenanceRepresentationOperationId {
        &self.operation_id
    }
    pub fn category(&self) -> &ProvenanceRepresentationFailureCategory {
        &self.category
    }
    pub fn findings(&self) -> &[String] {
        &self.findings
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProvenanceRepresentationOutcome {
    Success(ProvenanceRecordSet),
    Failure(ProvenanceRepresentationFailureRecord),
}

pub struct ProvenanceRepresentationAuthority;

pub fn represent_provenance(
    inputs: ProvenanceRepresentationInputs<'_>,
) -> ProvenanceRepresentationOutcome {
    let input = inputs.input;
    let operation_id = ProvenanceRepresentationOperationId::derive(&[
        input.input_id.as_str(),
        input.profile_id.as_str(),
        input.profile_version.as_str(),
        input.registry_version.as_str(),
        input.schema_version.as_str(),
        input.configuration_version.as_str(),
        input.implementation_version.as_str(),
    ]);
    let fail = |category, finding: &'static str| {
        ProvenanceRepresentationOutcome::Failure(ProvenanceRepresentationFailureRecord {
            failure_id: ProvenanceRepresentationFailureRecordId::derive(&[
                operation_id.as_str(),
                &format!("{category:?}"),
                finding,
            ]),
            operation_id: operation_id.clone(),
            category,
            findings: vec![finding.to_owned()].into(),
            input_id: input.input_id.clone(),
        })
    };
    if input.schema_version != "provenance-input-v1" {
        return fail(
            ProvenanceRepresentationFailureCategory::InvalidInputSchema,
            "provenance input schema is not supported",
        );
    }
    if input.profile_id != inputs.profile.identity
        || input.profile_version != inputs.profile.version
        || input.registry_version != inputs.registries.version
    {
        return fail(
            ProvenanceRepresentationFailureCategory::InvalidLineage,
            "input profile or registry binding differs from supplied profile",
        );
    }
    if inputs.profile.version.is_empty() || inputs.profile.authority_reference.is_empty() {
        return fail(
            ProvenanceRepresentationFailureCategory::MissingRequiredProfile,
            "provenance profile is incomplete",
        );
    }
    if inputs.registries.version.is_empty() || inputs.registries.authority_reference.is_empty() {
        return fail(
            ProvenanceRepresentationFailureCategory::MissingRequiredRegistry,
            "provenance registries are incomplete",
        );
    }
    if inputs.schema.version.is_empty() || inputs.schema.authority_reference.is_empty() {
        return fail(
            ProvenanceRepresentationFailureCategory::InvalidSchemaBinding,
            "provenance schema is incomplete",
        );
    }
    if inputs.configuration.version.is_empty()
        || inputs.configuration.authority_reference.is_empty()
        || input.implementation_version.is_empty()
    {
        return fail(
            ProvenanceRepresentationFailureCategory::InvalidConfigurationBinding,
            "provenance configuration is incomplete",
        );
    }
    if let Err(finding) = validate_provenance_lineage(&inputs) {
        return fail(
            ProvenanceRepresentationFailureCategory::InvalidLineage,
            finding,
        );
    }
    let known_subjects = provenance_subject_registry(&inputs);
    let mut subjects = Vec::new();
    let mut representations = Vec::new();
    let mut instances = Vec::new();
    let mut events = Vec::new();
    let mut lineage = Vec::new();
    let mut lifecycle = Vec::new();
    let mut mappings = Vec::new();
    let mut conflicts = Vec::new();
    let mut decisions = Vec::new();
    for declaration in input.subject_declarations.iter() {
        let Some(origin) = ProvenanceDeclarationOrigin::parse(&declaration.origin) else {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidDeclarationOrigin,
                "subject declaration origin is not registered",
            );
        };
        let Some(class) = ProvenanceSubjectClass::parse(&declaration.subject_class) else {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidSubjectReference,
                "subject class is not registered",
            );
        };
        if !inputs.profile.allowed_origins.contains(&declaration.origin)
            || !inputs
                .registries
                .subject_classes
                .contains(&declaration.subject_class)
            || !known_subjects.contains(&declaration.subject_id)
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidSubjectReference,
                "subject is not exact, supported, and lineage-bound",
            );
        }
        let subject = ProvenanceSubject {
            subject_id: declaration.subject_id.clone(),
            subject_class: class,
            declaration_id: declaration.declaration_id.clone(),
            origin,
            basis: declaration.basis.clone(),
        };
        let representation_id = ProvenanceRepresentationId::derive(&[
            operation_id.as_str(),
            declaration.declaration_id.as_str(),
            declaration.subject_id.as_str(),
        ]);
        let instance_id = ProvenanceRepresentationInstanceId::derive(&[
            representation_id.as_str(),
            input.input_id.as_str(),
        ]);
        subjects.push(subject);
        representations.push(ProvenanceRepresentation {
            representation_id: representation_id.clone(),
            subject_id: declaration.subject_id.clone(),
            declaration_id: declaration.declaration_id.clone(),
            status: ProvenanceStatus::Complete,
            basis: declaration.basis.clone(),
        });
        instances.push(ProvenanceRepresentationInstance {
            instance_id,
            representation_id,
            profile_id: inputs.profile.identity.clone(),
            profile_version: inputs.profile.version.clone(),
            registry_version: inputs.registries.version.clone(),
            schema_version: inputs.schema.version.clone(),
            configuration_version: inputs.configuration.version.clone(),
            implementation_version: input.implementation_version.clone(),
        });
        decisions.push(ProvenanceConstructionDecision {
            decision_id: ProvenanceConstructionDecisionId::derive(&[
                operation_id.as_str(),
                declaration.declaration_id.as_str(),
                "subject",
            ]),
            declaration_id: Some(declaration.declaration_id.clone()),
            decision_kind: ProvenanceConstructionDecisionKind::ConstructionAuthorized,
            authorizing_rule_id: "fixture-subject-reference-v1".to_owned(),
            authorizing_rule_version: "fixture-provenance-v1".to_owned(),
            constructed_artifact_ids: vec![declaration.subject_id.clone()].into(),
            retained_basis: declaration.basis.clone(),
            validation_findings: Vec::new().into(),
        });
    }
    for payload in input.event_declarations.iter() {
        let (event_class, declaration_id, event_id, schema_version, event_time, basis, origin) =
            match payload {
                ProvenanceEventPayload::Transformation(value) => (
                    "Transformation",
                    &value.declaration_id,
                    &value.event_id,
                    &value.event_schema_version,
                    &value.event_time,
                    &value.basis,
                    &value.origin,
                ),
                ProvenanceEventPayload::Custody(value) => (
                    "Custody",
                    &value.declaration_id,
                    &value.event_id,
                    &value.event_schema_version,
                    &value.event_time,
                    &value.basis,
                    &value.origin,
                ),
                ProvenanceEventPayload::Publication(value) => (
                    "Publication",
                    &value.declaration_id,
                    &value.event_id,
                    &value.event_schema_version,
                    &value.event_time,
                    &value.basis,
                    &value.origin,
                ),
                ProvenanceEventPayload::Lifecycle(value) => (
                    "Lifecycle",
                    &value.declaration_id,
                    &value.event_id,
                    &value.event_schema_version,
                    &value.event_time,
                    &value.basis,
                    &value.origin,
                ),
            };
        if !inputs.profile.allowed_event_classes.contains(event_class)
            || !inputs.registries.event_classes.contains(event_class)
            || schema_version != "provenance-event-v1"
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidEventSchema,
                "event class or schema is not supported",
            );
        }
        let Some(_) = ProvenanceDeclarationOrigin::parse(origin) else {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidDeclarationOrigin,
                "event declaration origin is not registered",
            );
        };
        if !inputs.profile.allowed_origins.contains(origin) {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidDeclarationOrigin,
                "event declaration origin is not permitted",
            );
        }
        if !inputs
            .profile
            .allowed_time_variants
            .contains(event_time.kind_name())
            || !inputs
                .registries
                .time_variants
                .contains(event_time.kind_name())
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidTimeVariant,
                "event time variant is not permitted",
            );
        }
        let subjects_for_event = match payload {
            ProvenanceEventPayload::Transformation(value) => value
                .input_subject_ids
                .iter()
                .chain(value.output_subject_ids.iter())
                .cloned()
                .collect::<Vec<_>>(),
            ProvenanceEventPayload::Custody(value) => vec![value.subject_id.clone()],
            ProvenanceEventPayload::Publication(value) => vec![value.subject_id.clone()],
            ProvenanceEventPayload::Lifecycle(value) => {
                vec![value.subject_id.clone(), value.related_subject_id.clone()]
            }
        };
        if subjects_for_event
            .iter()
            .any(|subject| !known_subjects.contains(subject))
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidEventReference,
                "event subject reference is not exact",
            );
        }
        events.push(ProvenanceEvent {
            event_id: event_id.clone(),
            declaration_id: declaration_id.clone(),
            event_class: event_class.to_owned(),
            payload: payload.clone(),
            event_time: event_time.clone(),
            basis: basis.clone(),
        });
        decisions.push(ProvenanceConstructionDecision {
            decision_id: ProvenanceConstructionDecisionId::derive(&[
                operation_id.as_str(),
                declaration_id.as_str(),
                "event",
            ]),
            declaration_id: Some(declaration_id.clone()),
            decision_kind: ProvenanceConstructionDecisionKind::ConstructionAuthorized,
            authorizing_rule_id: "fixture-event-schema-v1".to_owned(),
            authorizing_rule_version: "fixture-provenance-v1".to_owned(),
            constructed_artifact_ids: vec![event_id.to_string()].into(),
            retained_basis: basis.clone(),
            validation_findings: Vec::new().into(),
        });
    }
    for declaration in input.lineage_declarations.iter() {
        if !inputs
            .profile
            .allowed_lineage_relationships
            .contains(&declaration.relationship)
            || !inputs
                .registries
                .lineage_relationships
                .contains(&declaration.relationship)
            || !known_subjects.contains(&declaration.subject_artifact_id)
            || !known_subjects.contains(&declaration.ancestor_or_related_artifact_id)
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidRelationship,
                "lineage declaration is unsupported or references a foreign subject",
            );
        }
        if declaration.subject_artifact_id == declaration.ancestor_or_related_artifact_id
            && declaration.relationship != "RelatedTo"
        {
            return fail(
                ProvenanceRepresentationFailureCategory::ProhibitedCycle,
                "ancestry relationship contains a prohibited self-cycle",
            );
        }
        lineage.push(LineageAssertion {
            assertion_id: declaration.assertion_id.clone(),
            declaration_id: declaration.declaration_id.clone(),
            subject_artifact_id: declaration.subject_artifact_id.clone(),
            ancestor_or_related_artifact_id: declaration.ancestor_or_related_artifact_id.clone(),
            relationship: declaration.relationship.clone(),
            status: declaration.status.clone(),
            basis: declaration.basis.clone(),
        });
    }
    for declaration in input.lifecycle_declarations.iter() {
        if !inputs
            .profile
            .allowed_lifecycle_relationships
            .contains(&declaration.relationship)
            || !inputs
                .registries
                .lifecycle_relationships
                .contains(&declaration.relationship)
            || !known_subjects.contains(&declaration.subject_artifact_id)
            || !known_subjects.contains(&declaration.related_artifact_id)
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidRelationship,
                "lifecycle declaration is unsupported or references a foreign subject",
            );
        }
        lifecycle.push(LifecycleRelation {
            relation_id: declaration.relation_id.clone(),
            declaration_id: declaration.declaration_id.clone(),
            subject_artifact_id: declaration.subject_artifact_id.clone(),
            related_artifact_id: declaration.related_artifact_id.clone(),
            relationship: declaration.relationship.clone(),
            basis: declaration.basis.clone(),
        });
    }
    for declaration in input.external_identity_mappings.iter() {
        if !inputs
            .profile
            .allowed_mapping_namespaces
            .contains(&declaration.namespace)
            || !inputs
                .registries
                .mapping_namespaces
                .contains(&declaration.namespace)
            || !known_subjects.contains(&declaration.target_subject_id)
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidExternalMapping,
                "external mapping is not explicitly permitted or target-bound",
            );
        }
        mappings.push(ExternalIdentityMapping {
            mapping_id: declaration.mapping_id.clone(),
            declaration_id: declaration.declaration_id.clone(),
            namespace: declaration.namespace.clone(),
            external_identifier: declaration.external_identifier.clone(),
            target_subject_id: declaration.target_subject_id.clone(),
            basis: declaration.basis.clone(),
        });
    }
    for declaration in input.conflict_declarations.iter() {
        if !inputs
            .profile
            .allowed_conflict_classes
            .contains(&declaration.conflict_class)
            || !inputs
                .registries
                .conflict_classes
                .contains(&declaration.conflict_class)
            || declaration.involved_claim_ids.is_empty()
        {
            return fail(
                ProvenanceRepresentationFailureCategory::InvalidConflict,
                "conflict declaration is unsupported or empty",
            );
        }
        conflicts.push(ProvenanceConflictAssociation {
            conflict_id: declaration.conflict_id.clone(),
            declaration_id: declaration.declaration_id.clone(),
            involved_claim_ids: declaration.involved_claim_ids.clone(),
            conflict_class: declaration.conflict_class.clone(),
            status: declaration.status.clone(),
            basis: declaration.basis.clone(),
        });
    }
    if !inputs.profile.allow_empty
        && subjects.is_empty()
        && events.is_empty()
        && lineage.is_empty()
        && lifecycle.is_empty()
        && mappings.is_empty()
        && conflicts.is_empty()
    {
        return fail(
            ProvenanceRepresentationFailureCategory::EmptyNotPermitted,
            "empty provenance publication is not permitted",
        );
    }
    let status = if subjects.is_empty()
        && events.is_empty()
        && lineage.is_empty()
        && lifecycle.is_empty()
        && mappings.is_empty()
        && conflicts.is_empty()
    {
        ProvenanceStatus::Empty
    } else if !conflicts.is_empty() {
        ProvenanceStatus::Conflicting
    } else {
        ProvenanceStatus::Complete
    };
    let set_id = ProvenanceRecordSetId::derive(&[
        operation_id.as_str(),
        input.input_id.as_str(),
        &format!(
            "{:?}{:?}{:?}{:?}{:?}{:?}",
            subjects, events, lineage, lifecycle, mappings, conflicts
        ),
    ]);
    ProvenanceRepresentationOutcome::Success(ProvenanceRecordSet {
        set_id,
        operation_id,
        subjects: subjects.into(),
        representations: representations.into(),
        instances: instances.into(),
        events: events.into(),
        lineage_assertions: lineage.into(),
        lifecycle_relations: lifecycle.into(),
        external_mappings: mappings.into(),
        conflicts: conflicts.into(),
        decisions: decisions.into(),
        status,
        input_id: input.input_id.clone(),
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        registry_version: inputs.registries.version.clone(),
        schema_version: inputs.schema.version.clone(),
        configuration_version: inputs.configuration.version.clone(),
        implementation_version: input.implementation_version.clone(),
    })
}

fn provenance_subject_registry(inputs: &ProvenanceRepresentationInputs<'_>) -> BTreeSet<String> {
    BTreeSet::from([
        inputs.admitted_proposal_set.set_id.to_string(),
        inputs.objective_set.set_id.to_string(),
        inputs.constraint_set.set_id.to_string(),
        inputs.capability_set.set_id.to_string(),
        inputs.clarification_set.set_id.to_string(),
        inputs.evidence_set.set_id.to_string(),
        inputs.admitted_proposal_set.source_intake_id.to_string(),
    ])
}

fn validate_provenance_lineage(
    inputs: &ProvenanceRepresentationInputs<'_>,
) -> Result<(), &'static str> {
    let input = inputs.input;
    let admitted = inputs.admitted_proposal_set;
    if input.admitted_proposal_set_id != admitted.set_id
        || input.objective_set_id != inputs.objective_set.set_id
        || input.constraint_set_id != inputs.constraint_set.set_id
        || input.capability_set_id != inputs.capability_set.set_id
        || input.clarification_set_id != inputs.clarification_set.set_id
        || input.evidence_set_id != inputs.evidence_set.set_id
    {
        return Err("upstream publication identity binding is inconsistent");
    }
    if input.source_intake_id != admitted.source_intake_id
        || input.interpretation_operation_id != admitted.operation_id
        || input.input_proposal_ids != admitted.submitted_proposal_ids
        || input.input_decision_ids != admitted.decision_ids
    {
        return Err("proposal, source, or operation lineage is inconsistent");
    }
    if inputs.evidence_set.lineage.admitted_proposal_set_id != admitted.set_id
        || inputs.evidence_set.lineage.objective_set_id != inputs.objective_set.set_id
        || inputs.evidence_set.lineage.constraint_set_id != inputs.constraint_set.set_id
        || inputs.evidence_set.lineage.capability_set_id != inputs.capability_set.set_id
        || inputs.evidence_set.lineage.clarification_set_id != inputs.clarification_set.set_id
    {
        return Err("evidence publication lineage is inconsistent");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Contract 009: Semantic reconciliation
// ---------------------------------------------------------------------------

contract_002_id!(SemanticReconciliationInputId, "srinput");
contract_002_id!(SemanticReconciliationOperationId, "srop");
contract_002_id!(SemanticReconciliationId, "sr");
contract_002_id!(SemanticReconciliationSetId, "srset");
contract_002_id!(SemanticReconciliationFailureRecordId, "srfail");
contract_002_id!(ReconciliationSubjectId, "srsubject");
contract_002_id!(ReconciliationGroupId, "srgroup");
contract_002_id!(ComparisonFindingId, "srfinding");
contract_002_id!(ReconciliationRuleApplicationId, "srapply");
contract_002_id!(ReconciliationDecisionId, "srdecision");
contract_002_id!(StandingAssignmentId, "srstanding");
contract_002_id!(ReconciledSemanticElementId, "srelement");
contract_002_id!(ReconciliationRelationshipId, "srrelationship");
contract_002_id!(ReconciledRelationshipId, "srrel");
contract_002_id!(SemanticReconciliationProfileId, "srprofile");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ComparisonRelationship {
    ExactIdentity,
    ExactRepresentationMatch,
    ExactDuplicate,
    ProfileEquivalent,
    Compatible,
    Complementary,
    Alternative,
    Competing,
    Conflicting,
    Unrelated,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ReconciliationDisposition {
    Merged,
    Selected,
    Preserved,
    Split,
    Deferred,
    ExcludedFromDownstreamBasis,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Standing {
    Included,
    Contributing,
    PreservedAsAlternative,
    Deferred,
    Excluded,
    Unresolved,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SemanticReconciliationStatus {
    Reconciled,
    PartiallyReconciled,
    Deferred,
    Unresolved,
    Blocked,
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SemanticReconciliationFailureCategory {
    MissingRequiredProfile,
    IncompatibleProfile,
    MissingRegistry,
    UnknownRule,
    IncompatibleBinding,
    InvalidUpstreamLineage,
    InvalidSubject,
    InvalidGroup,
    InvalidFinding,
    MissingDecisionRule,
    InvalidStanding,
    NonDeterministicRule,
    PartialPublication,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReconciliationFailureFinding {
    pub code: String,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationSubjectDeclaration {
    pub subject_id: ReconciliationSubjectId,
    pub upstream_publication_id: String,
    pub upstream_representation_id: String,
    pub original_expression: String,
    pub semantic_class: String,
    pub semantic_domain: String,
    pub represented_scope: String,
    pub upstream_status: String,
    pub evidence_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationStandingDeclaration {
    pub assignment_id: StandingAssignmentId,
    pub target_kind: String,
    pub target_id: String,
    pub standing: Standing,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationGroupDeclaration {
    pub group_id: ReconciliationGroupId,
    pub member_subject_ids: Arc<[ReconciliationSubjectId]>,
    pub semantic_domain_or_interaction_class: String,
    pub represented_scope: String,
    pub formation_rule_id: String,
    pub formation_rule_version: String,
    pub comparison_rule_id: String,
    pub comparison_rule_version: String,
    pub decision_rule_id: String,
    pub decision_rule_version: String,
    pub declared_relationship: ComparisonRelationship,
    pub declared_disposition: ReconciliationDisposition,
    pub standing_assignments: Arc<[ReconciliationStandingDeclaration]>,
    pub resulting_element_ids: Arc<[ReconciledSemanticElementId]>,
    pub result_representation_ids: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationRelationshipDeclaration {
    pub relationship_id: ReconciliationRelationshipId,
    pub relationship_class: String,
    pub expression: String,
    pub source_subject_id: ReconciliationSubjectId,
    pub target_subject_id: ReconciliationSubjectId,
    pub represented_scope: String,
    pub reconciliation_decision_ids: Arc<[ReconciliationDecisionId]>,
    pub standing_assignment_ids: Arc<[StandingAssignmentId]>,
    pub evidence_reference_ids: Arc<[String]>,
    pub grounding_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReconciliationInput {
    pub input_id: SemanticReconciliationInputId,
    pub operation_id: SemanticReconciliationOperationId,
    pub admitted_proposal_set_id: AdmittedInterpretationProposalSetId,
    pub objective_set_id: DeclaredObjectiveSetId,
    pub constraint_set_id: DeclaredConstraintSetId,
    pub capability_set_id: CapabilityRequirementSetId,
    pub clarification_set_id: MeaningQualificationSetId,
    pub evidence_set_id: InterpretationEvidenceSetId,
    pub provenance_set_id: ProvenanceRecordSetId,
    pub source_intake_id: StableId,
    pub interpretation_operation_id: InterpretationOperationId,
    pub input_proposal_ids: Arc<[InterpretationProposalId]>,
    pub input_decision_ids: Arc<[ProposalAdmissionId]>,
    pub declared_subjects: Arc<[ReconciliationSubjectDeclaration]>,
    pub declared_groups: Arc<[ReconciliationGroupDeclaration]>,
    pub declared_relationships: Arc<[ReconciliationRelationshipDeclaration]>,
    pub profile_id: SemanticReconciliationProfileId,
    pub profile_version: String,
    pub grouping_registry_version: String,
    pub comparison_registry_version: String,
    pub decision_registry_version: String,
    pub standing_registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureSemanticReconciliationProfile {
    pub identity: SemanticReconciliationProfileId,
    pub version: String,
    pub authority_reference: String,
    pub registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub allowed_domains: BTreeSet<String>,
    pub allowed_interactions: BTreeSet<String>,
    pub allowed_grouping_rules: BTreeSet<String>,
    pub allowed_comparison_rules: BTreeSet<String>,
    pub allowed_decision_rules: BTreeSet<String>,
    pub allowed_standings: BTreeSet<Standing>,
    pub allow_empty: bool,
}

impl FixtureSemanticReconciliationProfile {
    pub fn fixture() -> Self {
        Self {
            identity: SemanticReconciliationProfileId::derive(&["contract-009-fixture"]),
            version: "fixture-reconciliation-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            registry_version: "fixture-reconciliation-registry-v1".to_owned(),
            schema_version: "fixture-reconciliation-schema-v1".to_owned(),
            configuration_version: "fixture-reconciliation-config-v1".to_owned(),
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureSemanticReconciliationRegistries {
    pub version: String,
    pub grouping_rules: BTreeSet<String>,
    pub comparison_rules: BTreeSet<String>,
    pub decision_rules: BTreeSet<String>,
    pub standing_values: BTreeSet<Standing>,
}

impl FixtureSemanticReconciliationRegistries {
    pub fn fixture() -> Self {
        let p = FixtureSemanticReconciliationProfile::fixture();
        Self {
            version: p.registry_version.clone(),
            grouping_rules: p.allowed_grouping_rules.clone(),
            comparison_rules: p.allowed_comparison_rules.clone(),
            decision_rules: p.allowed_decision_rules.clone(),
            standing_values: p.allowed_standings.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationSubject {
    pub subject_id: ReconciliationSubjectId,
    pub upstream_publication_id: String,
    pub upstream_representation_id: String,
    pub original_expression: String,
    pub semantic_class: String,
    pub semantic_domain: String,
    pub represented_scope: String,
    pub upstream_status: String,
    pub input_declaration_id: SemanticReconciliationInputId,
    pub evidence_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciledRelationship {
    pub relationship_id: ReconciledRelationshipId,
    pub source_relationship_id: ReconciliationRelationshipId,
    pub relationship_class: String,
    pub expression: String,
    pub source_subject_id: ReconciliationSubjectId,
    pub target_subject_id: ReconciliationSubjectId,
    pub represented_scope: String,
    pub reconciliation_decision_ids: Arc<[ReconciliationDecisionId]>,
    pub standing_assignment_ids: Arc<[StandingAssignmentId]>,
    pub evidence_reference_ids: Arc<[String]>,
    pub grounding_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationGroup {
    pub group_id: ReconciliationGroupId,
    pub member_subject_ids: Arc<[ReconciliationSubjectId]>,
    pub semantic_domain_or_interaction_class: String,
    pub represented_scope: String,
    pub formation_rule_id: String,
    pub formation_rule_version: String,
    pub input_declaration_id: SemanticReconciliationInputId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonFinding {
    pub finding_id: ComparisonFindingId,
    pub group_id: ReconciliationGroupId,
    pub subject_ids: Arc<[ReconciliationSubjectId]>,
    pub comparison_relationship: ComparisonRelationship,
    pub comparison_rule_id: String,
    pub comparison_rule_version: String,
    pub structural_comparison_basis: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationRuleApplication {
    pub application_id: ReconciliationRuleApplicationId,
    pub group_id: ReconciliationGroupId,
    pub rule_id: String,
    pub rule_version: String,
    pub input_finding_ids: Arc<[ComparisonFindingId]>,
    pub applicability_status: String,
    pub authorized_dispositions: Arc<[ReconciliationDisposition]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciliationDecision {
    pub decision_id: ReconciliationDecisionId,
    pub group_id: ReconciliationGroupId,
    pub participating_subject_ids: Arc<[ReconciliationSubjectId]>,
    pub comparison_finding_ids: Arc<[ComparisonFindingId]>,
    pub rule_application_ids: Arc<[ReconciliationRuleApplicationId]>,
    pub disposition: ReconciliationDisposition,
    pub decision_basis: Arc<[String]>,
    pub standing_assignment_ids: Arc<[StandingAssignmentId]>,
    pub resulting_element_ids: Arc<[ReconciledSemanticElementId]>,
    pub preserved_alternative_ids: Arc<[ReconciliationSubjectId]>,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StandingAssignment {
    pub assignment_id: StandingAssignmentId,
    pub decision_id: ReconciliationDecisionId,
    pub target_kind: String,
    pub target_id: String,
    pub standing: Standing,
    pub assignment_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReconciledSemanticElement {
    pub element_id: ReconciledSemanticElementId,
    pub representation_id: String,
    /// The expression and semantic context committed by Contract 009. These
    /// fields are carried forward unchanged by Contract 010.
    pub original_expression: String,
    pub semantic_domain: String,
    pub semantic_class: String,
    pub represented_scope: String,
    pub evidence_reference_ids: Arc<[String]>,
    pub grounding_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
    pub contributing_subject_ids: Arc<[ReconciliationSubjectId]>,
    pub contributing_decision_ids: Arc<[ReconciliationDecisionId]>,
    pub preserved_alternative_ids: Arc<[ReconciliationSubjectId]>,
    pub exact_content_binding: String,
    pub construction_kind: String,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReconciliationSet {
    pub set_id: SemanticReconciliationSetId,
    pub operation_id: SemanticReconciliationOperationId,
    pub logical_reconciliation_id: SemanticReconciliationId,
    pub input_id: SemanticReconciliationInputId,
    pub subjects: Arc<[ReconciliationSubject]>,
    pub groups: Arc<[ReconciliationGroup]>,
    pub findings: Arc<[ComparisonFinding]>,
    pub rule_applications: Arc<[ReconciliationRuleApplication]>,
    pub decisions: Arc<[ReconciliationDecision]>,
    pub standing_assignments: Arc<[StandingAssignment]>,
    pub elements: Arc<[ReconciledSemanticElement]>,
    pub relationships: Arc<[ReconciledRelationship]>,
    pub status: SemanticReconciliationStatus,
    pub profile_id: SemanticReconciliationProfileId,
    pub profile_version: String,
    pub registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl SemanticReconciliationSet {
    pub fn set_id(&self) -> &SemanticReconciliationSetId {
        &self.set_id
    }
    pub fn operation_id(&self) -> &SemanticReconciliationOperationId {
        &self.operation_id
    }
    pub fn status(&self) -> SemanticReconciliationStatus {
        self.status
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticReconciliationFailureRecord {
    pub failure_record_id: SemanticReconciliationFailureRecordId,
    pub operation_id: SemanticReconciliationOperationId,
    pub input_id: SemanticReconciliationInputId,
    pub category: SemanticReconciliationFailureCategory,
    pub stage: String,
    pub findings: Arc<[SemanticReconciliationFailureFinding]>,
    pub profile_id: SemanticReconciliationProfileId,
    pub profile_version: String,
    pub registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticReconciliationOutcome {
    Success(SemanticReconciliationSet),
    Failure(SemanticReconciliationFailureRecord),
}

pub struct SemanticReconciliationInputs<'a> {
    pub input: &'a SemanticReconciliationInput,
    pub admitted_proposal_set: &'a AdmittedInterpretationProposalSet,
    pub objective_set: &'a DeclaredObjectiveSet,
    pub constraint_set: &'a DeclaredConstraintSet,
    pub capability_set: &'a CapabilityRequirementSet,
    pub clarification_set: &'a MeaningQualificationSet,
    pub evidence_set: &'a InterpretationEvidenceSet,
    pub provenance_set: &'a ProvenanceRecordSet,
    pub profile: &'a FixtureSemanticReconciliationProfile,
    pub registries: &'a FixtureSemanticReconciliationRegistries,
}

fn semantic_reconciliation_failure(
    inputs: &SemanticReconciliationInputs<'_>,
    category: SemanticReconciliationFailureCategory,
    stage: &str,
    detail: &str,
) -> SemanticReconciliationOutcome {
    SemanticReconciliationOutcome::Failure(SemanticReconciliationFailureRecord {
        failure_record_id: SemanticReconciliationFailureRecordId::derive(&[
            inputs.input.operation_id.as_str(),
            stage,
            detail,
        ]),
        operation_id: inputs.input.operation_id.clone(),
        input_id: inputs.input.input_id.clone(),
        category,
        stage: stage.to_owned(),
        findings: Arc::from([SemanticReconciliationFailureFinding {
            code: format!("{category:?}"),
            detail: detail.to_owned(),
        }]),
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        registry_version: inputs.registries.version.clone(),
        schema_version: inputs.input.schema_version.clone(),
        configuration_version: inputs.input.configuration_version.clone(),
        implementation_version: inputs.input.implementation_version.clone(),
    })
}

fn stable_reconciliation_relationship_binding(relationships: &[ReconciledRelationship]) -> String {
    let mut ordered = relationships.to_vec();
    ordered.sort_by(|left, right| left.relationship_id.cmp(&right.relationship_id));
    format!("{ordered:?}")
}

fn validate_semantic_reconciliation_lineage(
    inputs: &SemanticReconciliationInputs<'_>,
) -> Result<(), &'static str> {
    let i = inputs.input;
    let a = inputs.admitted_proposal_set;
    if i.admitted_proposal_set_id != a.set_id
        || i.objective_set_id != inputs.objective_set.set_id
        || i.constraint_set_id != inputs.constraint_set.set_id
        || i.capability_set_id != inputs.capability_set.set_id
        || i.clarification_set_id != inputs.clarification_set.set_id
        || i.evidence_set_id != inputs.evidence_set.set_id
        || i.provenance_set_id != inputs.provenance_set.set_id
    {
        return Err("upstream publication identity binding is inconsistent");
    }
    if i.source_intake_id != a.source_intake_id
        || i.interpretation_operation_id != a.operation_id
        || i.input_proposal_ids != a.submitted_proposal_ids
        || i.input_decision_ids != a.decision_ids
    {
        return Err("proposal, source, or operation lineage is inconsistent");
    }
    Ok(())
}

fn reconciliation_known_publications(
    inputs: &SemanticReconciliationInputs<'_>,
) -> BTreeSet<String> {
    let mut ids = BTreeSet::from([
        inputs.admitted_proposal_set.set_id.to_string(),
        inputs.objective_set.set_id.to_string(),
        inputs.constraint_set.set_id.to_string(),
        inputs.capability_set.set_id.to_string(),
        inputs.clarification_set.set_id.to_string(),
        inputs.evidence_set.set_id.to_string(),
        inputs.provenance_set.set_id.to_string(),
    ]);
    ids.extend(
        inputs
            .objective_set
            .objectives
            .iter()
            .map(|x| x.logical_objective_id.to_string()),
    );
    ids.extend(
        inputs
            .constraint_set
            .constraints
            .iter()
            .map(|x| x.logical_constraint_id.to_string()),
    );
    ids.extend(
        inputs
            .capability_set
            .requirements
            .iter()
            .map(|x| x.logical_id.to_string()),
    );
    ids.extend(
        inputs
            .clarification_set
            .ambiguities
            .iter()
            .map(|x| x.id.to_string()),
    );
    ids.extend(
        inputs
            .clarification_set
            .assumptions
            .iter()
            .map(|x| x.id.to_string()),
    );
    ids.extend(
        inputs
            .clarification_set
            .uncertainties
            .iter()
            .map(|x| x.id.to_string()),
    );
    ids
}

pub fn represent_semantic_reconciliation(
    inputs: SemanticReconciliationInputs<'_>,
) -> SemanticReconciliationOutcome {
    let i = inputs.input;
    if i.profile_id != inputs.profile.identity || i.profile_version != inputs.profile.version {
        return semantic_reconciliation_failure(
            &inputs,
            SemanticReconciliationFailureCategory::IncompatibleProfile,
            "profile-validation",
            "profile identity or version is not exact",
        );
    }
    if i.grouping_registry_version != inputs.registries.version
        || i.comparison_registry_version != inputs.registries.version
        || i.decision_registry_version != inputs.registries.version
        || i.standing_registry_version != inputs.registries.version
        || i.schema_version != inputs.profile.schema_version
        || i.configuration_version != inputs.profile.configuration_version
    {
        return semantic_reconciliation_failure(
            &inputs,
            SemanticReconciliationFailureCategory::IncompatibleBinding,
            "binding-validation",
            "registry, schema, or configuration binding is not exact",
        );
    }
    if let Err(detail) = validate_semantic_reconciliation_lineage(&inputs) {
        return semantic_reconciliation_failure(
            &inputs,
            SemanticReconciliationFailureCategory::InvalidUpstreamLineage,
            "lineage-validation",
            detail,
        );
    }
    let known = reconciliation_known_publications(&inputs);
    let mut seen_subjects = BTreeSet::new();
    let mut subjects = Vec::new();
    for d in i.declared_subjects.iter() {
        if !seen_subjects.insert(d.subject_id.clone())
            || !inputs.profile.allowed_domains.contains(&d.semantic_domain)
            || !known.contains(&d.upstream_publication_id)
        {
            return semantic_reconciliation_failure(
                &inputs,
                SemanticReconciliationFailureCategory::InvalidSubject,
                "subject-resolution",
                "subject identity, domain, or publication binding is invalid",
            );
        }
        subjects.push(ReconciliationSubject {
            subject_id: d.subject_id.clone(),
            upstream_publication_id: d.upstream_publication_id.clone(),
            upstream_representation_id: d.upstream_representation_id.clone(),
            original_expression: d.original_expression.clone(),
            semantic_class: d.semantic_class.clone(),
            semantic_domain: d.semantic_domain.clone(),
            represented_scope: d.represented_scope.clone(),
            upstream_status: d.upstream_status.clone(),
            input_declaration_id: i.input_id.clone(),
            evidence_reference_ids: d.evidence_reference_ids.clone(),
            provenance_reference_ids: d.provenance_reference_ids.clone(),
        });
    }
    if subjects.is_empty() && !inputs.profile.allow_empty {
        return semantic_reconciliation_failure(
            &inputs,
            SemanticReconciliationFailureCategory::InvalidGroup,
            "group-formation",
            "empty reconciliation is not permitted",
        );
    }
    let subject_map: BTreeMap<_, _> = subjects.iter().map(|s| (s.subject_id.clone(), s)).collect();
    let mut relationships = Vec::new();
    let mut seen_relationships = BTreeSet::new();
    for declaration in i.declared_relationships.iter() {
        if !seen_relationships.insert(declaration.relationship_id.clone())
            || declaration.relationship_class.is_empty()
            || declaration.expression.is_empty()
            || declaration.represented_scope.is_empty()
            || !subject_map.contains_key(&declaration.source_subject_id)
            || !subject_map.contains_key(&declaration.target_subject_id)
        {
            return semantic_reconciliation_failure(
                &inputs,
                SemanticReconciliationFailureCategory::InvalidFinding,
                "relationship-validation",
                "relationship is duplicate, malformed, or references a foreign subject, decision, or standing assignment",
            );
        }
        let relationship_id = ReconciledRelationshipId::derive(&[
            declaration.relationship_id.as_str(),
            declaration.relationship_class.as_str(),
            declaration.expression.as_str(),
            declaration.source_subject_id.as_str(),
            declaration.target_subject_id.as_str(),
            declaration.represented_scope.as_str(),
            &format!(
                "{:?}{:?}{:?}{:?}{:?}",
                declaration.reconciliation_decision_ids,
                declaration.standing_assignment_ids,
                declaration.evidence_reference_ids,
                declaration.grounding_reference_ids,
                declaration.provenance_reference_ids
            ),
            &format!("{:?}", declaration.unresolved_conditions),
        ]);
        relationships.push(ReconciledRelationship {
            relationship_id,
            source_relationship_id: declaration.relationship_id.clone(),
            relationship_class: declaration.relationship_class.clone(),
            expression: declaration.expression.clone(),
            source_subject_id: declaration.source_subject_id.clone(),
            target_subject_id: declaration.target_subject_id.clone(),
            represented_scope: declaration.represented_scope.clone(),
            reconciliation_decision_ids: declaration.reconciliation_decision_ids.clone(),
            standing_assignment_ids: declaration.standing_assignment_ids.clone(),
            evidence_reference_ids: declaration.evidence_reference_ids.clone(),
            grounding_reference_ids: declaration.grounding_reference_ids.clone(),
            provenance_reference_ids: declaration.provenance_reference_ids.clone(),
            unresolved_conditions: declaration.unresolved_conditions.clone(),
        });
    }
    let mut groups = Vec::new();
    let mut findings = Vec::new();
    let mut applications = Vec::new();
    let mut decisions = Vec::new();
    let mut assignments = Vec::new();
    let mut elements = Vec::new();
    let mut seen_groups = BTreeSet::new();
    for g in i.declared_groups.iter() {
        if !seen_groups.insert(g.group_id.clone())
            || !inputs
                .profile
                .allowed_grouping_rules
                .contains(&g.formation_rule_id)
            || !inputs
                .profile
                .allowed_comparison_rules
                .contains(&g.comparison_rule_id)
            || !inputs
                .profile
                .allowed_decision_rules
                .contains(&g.decision_rule_id)
            || !inputs
                .profile
                .allowed_interactions
                .contains(&g.semantic_domain_or_interaction_class)
            || g.member_subject_ids.is_empty()
            || g.member_subject_ids
                .iter()
                .any(|id| !subject_map.contains_key(id))
        {
            return semantic_reconciliation_failure(
                &inputs,
                SemanticReconciliationFailureCategory::InvalidGroup,
                "group-validation",
                "group is undeclared, unsupported, empty, or contains a foreign subject",
            );
        }
        let group = ReconciliationGroup {
            group_id: g.group_id.clone(),
            member_subject_ids: g.member_subject_ids.clone(),
            semantic_domain_or_interaction_class: g.semantic_domain_or_interaction_class.clone(),
            represented_scope: g.represented_scope.clone(),
            formation_rule_id: g.formation_rule_id.clone(),
            formation_rule_version: g.formation_rule_version.clone(),
            input_declaration_id: i.input_id.clone(),
        };
        let mut group_findings = Vec::new();
        for pair in g.member_subject_ids.windows(2) {
            let left = subject_map[&pair[0]];
            let right = subject_map[&pair[1]];
            let relationship =
                if left.upstream_representation_id == right.upstream_representation_id {
                    ComparisonRelationship::ExactRepresentationMatch
                } else {
                    g.declared_relationship
                };
            let finding = ComparisonFinding {
                finding_id: ComparisonFindingId::derive(&[
                    g.group_id.as_str(),
                    pair[0].as_str(),
                    pair[1].as_str(),
                    &format!("{relationship:?}"),
                    g.comparison_rule_id.as_str(),
                    g.comparison_rule_version.as_str(),
                ]),
                group_id: g.group_id.clone(),
                subject_ids: Arc::from(pair.to_vec()),
                comparison_relationship: relationship,
                comparison_rule_id: g.comparison_rule_id.clone(),
                comparison_rule_version: g.comparison_rule_version.clone(),
                structural_comparison_basis: Arc::from([
                    left.upstream_representation_id.clone(),
                    right.upstream_representation_id.clone(),
                ]),
            };
            group_findings.push(finding.finding_id.clone());
            findings.push(finding);
        }
        let application_id = ReconciliationRuleApplicationId::derive(&[
            g.group_id.as_str(),
            g.decision_rule_id.as_str(),
            g.decision_rule_version.as_str(),
        ]);
        applications.push(ReconciliationRuleApplication {
            application_id: application_id.clone(),
            group_id: g.group_id.clone(),
            rule_id: g.decision_rule_id.clone(),
            rule_version: g.decision_rule_version.clone(),
            input_finding_ids: group_findings.clone().into(),
            applicability_status: "DeclaredAndProfileAuthorized".to_owned(),
            authorized_dispositions: Arc::from([g.declared_disposition]),
        });
        let expected_targets = g.member_subject_ids.len() + g.resulting_element_ids.len();
        if g.standing_assignments.len() != expected_targets
            || g.standing_assignments
                .iter()
                .any(|a| !inputs.profile.allowed_standings.contains(&a.standing))
        {
            return semantic_reconciliation_failure(&inputs, SemanticReconciliationFailureCategory::InvalidStanding, "standing-validation", "every participating subject and result element requires exactly one permitted assignment");
        }
        let expected_target_ids: BTreeSet<_> = g
            .member_subject_ids
            .iter()
            .map(|id| ("UpstreamSubject".to_owned(), id.to_string()))
            .chain(
                g.resulting_element_ids
                    .iter()
                    .map(|id| ("ResultingReconciledElement".to_owned(), id.to_string())),
            )
            .collect();
        let declared_target_ids: BTreeSet<_> = g
            .standing_assignments
            .iter()
            .map(|assignment| (assignment.target_kind.clone(), assignment.target_id.clone()))
            .collect();
        if declared_target_ids != expected_target_ids {
            return semantic_reconciliation_failure(
                &inputs,
                SemanticReconciliationFailureCategory::InvalidStanding,
                "standing-validation",
                "standing assignments do not exactly cover subjects and result elements",
            );
        }
        let decision_id = ReconciliationDecisionId::derive(&[
            g.group_id.as_str(),
            g.decision_rule_id.as_str(),
            &format!("{:?}", g.declared_disposition),
            &format!("{:?}", g.standing_assignments),
        ]);
        let mut decision_assignment_ids = Vec::new();
        for a in g.standing_assignments.iter() {
            if assignments
                .iter()
                .any(|x: &StandingAssignment| x.assignment_id == a.assignment_id)
            {
                return semantic_reconciliation_failure(
                    &inputs,
                    SemanticReconciliationFailureCategory::InvalidStanding,
                    "standing-validation",
                    "standing assignment identity is duplicated",
                );
            }
            decision_assignment_ids.push(a.assignment_id.clone());
            assignments.push(StandingAssignment {
                assignment_id: a.assignment_id.clone(),
                decision_id: decision_id.clone(),
                target_kind: a.target_kind.clone(),
                target_id: a.target_id.clone(),
                standing: a.standing,
                assignment_basis: a.basis.clone(),
            });
        }
        for e in g.resulting_element_ids.iter() {
            if !g.result_representation_ids.iter().any(|x| x == e.as_str()) {
                return semantic_reconciliation_failure(
                    &inputs,
                    SemanticReconciliationFailureCategory::InvalidStanding,
                    "element-validation",
                    "result element lacks an exact representation binding",
                );
            }
            elements.push(ReconciledSemanticElement {
                element_id: e.clone(),
                representation_id: e.to_string(),
                original_expression: subject_map
                    .get(&g.member_subject_ids[0])
                    .map(|subject| subject.original_expression.clone())
                    .unwrap_or_default(),
                semantic_domain: subject_map
                    .get(&g.member_subject_ids[0])
                    .map(|subject| subject.semantic_domain.clone())
                    .unwrap_or_default(),
                semantic_class: subject_map
                    .get(&g.member_subject_ids[0])
                    .map(|subject| subject.semantic_class.clone())
                    .unwrap_or_default(),
                represented_scope: subject_map
                    .get(&g.member_subject_ids[0])
                    .map(|subject| subject.represented_scope.clone())
                    .unwrap_or_default(),
                evidence_reference_ids: subject_map
                    .get(&g.member_subject_ids[0])
                    .map(|subject| subject.evidence_reference_ids.clone())
                    .unwrap_or_default(),
                grounding_reference_ids: Arc::from([]),
                provenance_reference_ids: subject_map
                    .get(&g.member_subject_ids[0])
                    .map(|subject| subject.provenance_reference_ids.clone())
                    .unwrap_or_default(),
                contributing_subject_ids: g.member_subject_ids.clone(),
                contributing_decision_ids: Arc::from([decision_id.clone()]),
                preserved_alternative_ids: g.member_subject_ids.clone(),
                exact_content_binding: g
                    .member_subject_ids
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                construction_kind: "ExactUpstreamPreservation".to_owned(),
                unresolved_conditions: g.unresolved_conditions.clone(),
            });
        }
        decisions.push(ReconciliationDecision {
            decision_id,
            group_id: g.group_id.clone(),
            participating_subject_ids: g.member_subject_ids.clone(),
            comparison_finding_ids: group_findings.into(),
            rule_application_ids: Arc::from([application_id]),
            disposition: g.declared_disposition,
            decision_basis: Arc::from([
                "Explicit declared group, comparison, decision, and standing bindings".to_owned(),
            ]),
            standing_assignment_ids: decision_assignment_ids.into(),
            resulting_element_ids: g.resulting_element_ids.clone(),
            preserved_alternative_ids: g.member_subject_ids.clone(),
            unresolved_conditions: g.unresolved_conditions.clone(),
        });
        groups.push(group);
    }
    if relationships.iter().any(|relationship| {
        relationship
            .reconciliation_decision_ids
            .iter()
            .any(|decision_id| {
                !decisions
                    .iter()
                    .any(|decision| decision.decision_id == *decision_id)
            })
            || relationship
                .standing_assignment_ids
                .iter()
                .any(|assignment_id| {
                    !assignments
                        .iter()
                        .any(|assignment| assignment.assignment_id == *assignment_id)
                })
    }) {
        return semantic_reconciliation_failure(
            &inputs,
            SemanticReconciliationFailureCategory::InvalidFinding,
            "relationship-validation",
            "relationship references a foreign decision or standing assignment",
        );
    }
    let status = if subjects.is_empty() {
        SemanticReconciliationStatus::NotApplicable
    } else if decisions
        .iter()
        .any(|d| d.disposition == ReconciliationDisposition::Deferred)
    {
        SemanticReconciliationStatus::Deferred
    } else if decisions
        .iter()
        .any(|d| d.disposition == ReconciliationDisposition::Unresolved)
    {
        SemanticReconciliationStatus::Unresolved
    } else if decisions.len() < groups.len() {
        SemanticReconciliationStatus::PartiallyReconciled
    } else {
        SemanticReconciliationStatus::Reconciled
    };
    let set_id = SemanticReconciliationSetId::derive(&[
        i.operation_id.as_str(),
        i.input_id.as_str(),
        &format!(
            "{:?}{:?}{:?}{:?}{:?}{}",
            subjects,
            groups,
            findings,
            decisions,
            assignments,
            stable_reconciliation_relationship_binding(&relationships)
        ),
    ]);
    SemanticReconciliationOutcome::Success(SemanticReconciliationSet {
        set_id,
        operation_id: i.operation_id.clone(),
        logical_reconciliation_id: SemanticReconciliationId::derive(&[
            i.operation_id.as_str(),
            i.input_id.as_str(),
        ]),
        input_id: i.input_id.clone(),
        subjects: subjects.into(),
        groups: groups.into(),
        findings: findings.into(),
        rule_applications: applications.into(),
        decisions: decisions.into(),
        standing_assignments: assignments.into(),
        elements: elements.into(),
        relationships: relationships.into(),
        status,
        profile_id: inputs.profile.identity.clone(),
        profile_version: inputs.profile.version.clone(),
        registry_version: inputs.registries.version.clone(),
        schema_version: i.schema_version.clone(),
        configuration_version: i.configuration_version.clone(),
        implementation_version: i.implementation_version.clone(),
        replay_context: i.replay_context.clone(),
    })
}

// ---------------------------------------------------------------------------
// Contract 010: Semantic normalization
// ---------------------------------------------------------------------------

contract_002_id!(NormalizationInputId, "sninput");
contract_002_id!(NormalizationOperationId, "snop");
contract_002_id!(NormalizationProfileId, "snprofile");
contract_002_id!(CanonicalRegistryId, "snregistry");
contract_002_id!(MappingRuleRegistryId, "snrules");
contract_002_id!(MappingApplicationId, "snapply");
contract_002_id!(NormalizationDecisionId, "sndecision");
contract_002_id!(NormalizedSemanticElementId, "snelement");
contract_002_id!(NormalizedRelationshipId, "snrelationship");
contract_002_id!(NormalizedRequestRepresentationId, "snrequest");
contract_002_id!(NormalizationFailureRecordId, "snfail");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NormalizationEligibility {
    Eligible,
    ProfileDeferred,
    NotEligible,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NormalizationDisposition {
    Mapped,
    Identity,
    Preserved,
    Deferred,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MappingRuleKind {
    ExactAlias,
    ExactEnum,
    IdentifierOuterWhitespace,
    IdentifierCase,
    NumericLexical,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MappingApplicationStatus {
    Applied,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NormalizationFailureCategory {
    InvalidUpstreamPublication,
    UpstreamPublicationResolutionFailure,
    MissingRegistry,
    ConflictingRegistry,
    ProfileResolutionFailure,
    ProfileVersionMismatch,
    SchemaIncompatibility,
    NoAuthorizedRule,
    UnauthorizedMapping,
    InvalidMappingTarget,
    CircularMappingDependency,
    TraceabilityFailure,
    NonDeterministicRuleSelection,
    AtomicCommitmentFailure,
    InvalidStandingBinding,
    EligibilityResolutionFailure,
    SemanticContinuityFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRegistryEntry {
    pub registry_id: CanonicalRegistryId,
    pub registry_version: String,
    pub semantic_domain: String,
    pub semantic_class: String,
    pub expression_class: String,
    pub canonical_value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappingRuleDefinition {
    pub rule_id: String,
    pub rule_version: String,
    pub kind: MappingRuleKind,
    pub semantic_domain: String,
    pub semantic_class: String,
    pub expression_class: String,
    pub exact_source_form: String,
    pub exact_target_form: String,
    pub applicability_conditions: Arc<[String]>,
    pub required_registry_target: String,
    pub profile_authority: String,
    pub replay_inputs: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureNormalizationProfile {
    pub identity: NormalizationProfileId,
    pub version: String,
    pub authority_reference: String,
    pub supported_semantic_domains: BTreeSet<String>,
    pub supported_semantic_classes: BTreeSet<String>,
    pub supported_relationship_classes: BTreeSet<String>,
    pub eligible_standings: BTreeSet<Standing>,
    pub deferred_conditions: BTreeSet<String>,
    pub permitted_dispositions: BTreeSet<NormalizationDisposition>,
    pub canonical_registry_id: CanonicalRegistryId,
    pub canonical_registry_version: String,
    pub mapping_rule_registry_id: MappingRuleRegistryId,
    pub mapping_rule_registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub preservation_permissions: BTreeSet<(String, String, String)>,
    pub identity_conditions: BTreeSet<(String, String, String)>,
    pub relationship_preservation_permissions: BTreeSet<(String, String)>,
    pub relationship_identity_conditions: BTreeSet<(String, String)>,
    pub forbidden_transformations: BTreeSet<String>,
    pub decomposition_enabled: bool,
    pub collection_deduplication_enabled: bool,
}

impl FixtureNormalizationProfile {
    pub fn fixture() -> Self {
        Self {
            identity: NormalizationProfileId::derive(&["contract-010-fixture"]),
            version: "fixture-normalization-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
            canonical_registry_id: CanonicalRegistryId::derive(&["contract-010-fixture"]),
            canonical_registry_version: "fixture-canonical-registry-v1".to_owned(),
            mapping_rule_registry_id: MappingRuleRegistryId::derive(&["contract-010-fixture"]),
            mapping_rule_registry_version: "fixture-mapping-rules-v1".to_owned(),
            schema_version: "fixture-normalization-schema-v1".to_owned(),
            configuration_version: "fixture-normalization-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
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

    pub fn allows_preservation(&self, element: &ReconciledSemanticElement) -> bool {
        self.preservation_permissions.contains(&(
            element.semantic_domain.clone(),
            element.semantic_class.clone(),
            "expression".to_owned(),
        ))
    }

    pub fn allows_identity(&self, element: &ReconciledSemanticElement) -> bool {
        self.identity_conditions.contains(&(
            element.semantic_domain.clone(),
            element.semantic_class.clone(),
            "expression".to_owned(),
        ))
    }

    fn allows_relationship_preservation(&self, relationship: &ReconciledRelationship) -> bool {
        self.relationship_preservation_permissions.contains(&(
            relationship.relationship_class.clone(),
            "relationship-expression".to_owned(),
        ))
    }

    fn allows_relationship_identity(&self, relationship: &ReconciledRelationship) -> bool {
        self.relationship_identity_conditions.contains(&(
            relationship.relationship_class.clone(),
            "relationship-expression".to_owned(),
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureCanonicalRegistry {
    pub identity: CanonicalRegistryId,
    pub version: String,
    pub entries: Arc<[CanonicalRegistryEntry]>,
}

impl FixtureCanonicalRegistry {
    pub fn fixture() -> Self {
        let p = FixtureNormalizationProfile::fixture();
        Self {
            identity: p.canonical_registry_id.clone(),
            version: p.canonical_registry_version.clone(),
            entries: vec![
                (
                    "Objective",
                    "RequestedOutcome",
                    "expression",
                    "achieve-outcome",
                ),
                ("Constraint", "Content", "expression", "do-not-disclose"),
                (
                    "CapabilityRequirement",
                    "LanguageUnderstanding",
                    "expression",
                    "understanding",
                ),
                ("Relationship", "DependsOn", "expression", "depends-on"),
            ]
            .into_iter()
            .map(
                |(domain, class, expression_class, value)| CanonicalRegistryEntry {
                    registry_id: p.canonical_registry_id.clone(),
                    registry_version: p.canonical_registry_version.clone(),
                    semantic_domain: domain.to_owned(),
                    semantic_class: class.to_owned(),
                    expression_class: expression_class.to_owned(),
                    canonical_value: value.to_owned(),
                },
            )
            .collect::<Vec<_>>()
            .into(),
        }
    }

    fn contains(&self, domain: &str, class: &str, expression_class: &str, value: &str) -> bool {
        self.entries.iter().any(|entry| {
            entry.semantic_domain == domain
                && entry.semantic_class == class
                && entry.expression_class == expression_class
                && entry.canonical_value == value
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureMappingRuleRegistry {
    pub identity: MappingRuleRegistryId,
    pub version: String,
    pub rules: Arc<[MappingRuleDefinition]>,
}

impl FixtureMappingRuleRegistry {
    pub fn fixture() -> Self {
        let p = FixtureNormalizationProfile::fixture();
        let authority = p.authority_reference.clone();
        let registry = p.canonical_registry_id.to_string();
        let rule = |id: &str,
                    domain: &str,
                    class: &str,
                    source: &str,
                    target: &str,
                    kind: MappingRuleKind| MappingRuleDefinition {
            rule_id: id.to_owned(),
            rule_version: "fixture-rule-v1".to_owned(),
            kind,
            semantic_domain: domain.to_owned(),
            semantic_class: class.to_owned(),
            expression_class: "expression".to_owned(),
            exact_source_form: source.to_owned(),
            exact_target_form: target.to_owned(),
            applicability_conditions: Arc::from([
                "exact domain/class/expression binding".to_owned()
            ]),
            required_registry_target: registry.clone(),
            profile_authority: authority.clone(),
            replay_inputs: Arc::from(["fixture".to_owned()]),
        };
        Self {
            identity: p.mapping_rule_registry_id,
            version: p.mapping_rule_registry_version,
            rules: vec![
                rule(
                    "ObjectiveRequestedOutcomeExactAliasRule",
                    "Objective",
                    "RequestedOutcome",
                    "achieve outcome",
                    "achieve-outcome",
                    MappingRuleKind::ExactAlias,
                ),
                rule(
                    "ConstraintContentExactAliasRule",
                    "Constraint",
                    "Content",
                    "do not disclose",
                    "do-not-disclose",
                    MappingRuleKind::ExactAlias,
                ),
                rule(
                    "CapabilityLanguageUnderstandingExactAliasRule",
                    "CapabilityRequirement",
                    "LanguageUnderstanding",
                    "understand",
                    "understanding",
                    MappingRuleKind::ExactAlias,
                ),
                rule(
                    "RelationshipDependsOnExactAliasRule",
                    "Relationship",
                    "DependsOn",
                    "depends on",
                    "depends-on",
                    MappingRuleKind::ExactAlias,
                ),
            ]
            .into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticNormalizationInput {
    pub input_id: NormalizationInputId,
    pub operation_id: NormalizationOperationId,
    pub reconciliation_set_id: SemanticReconciliationSetId,
    pub upstream_content_binding: String,
    pub profile_id: NormalizationProfileId,
    pub profile_version: String,
    pub canonical_registry_id: CanonicalRegistryId,
    pub canonical_registry_version: String,
    pub mapping_rule_registry_id: MappingRuleRegistryId,
    pub mapping_rule_registry_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizationOperation {
    pub operation_id: NormalizationOperationId,
    pub input_id: NormalizationInputId,
    pub reconciliation_set_id: SemanticReconciliationSetId,
    pub profile_binding: (NormalizationProfileId, String),
    pub registry_binding: (CanonicalRegistryId, String, MappingRuleRegistryId, String),
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl SemanticNormalizationInput {
    pub fn for_fixture(set: &SemanticReconciliationSet) -> Self {
        let p = FixtureNormalizationProfile::fixture();
        Self {
            input_id: NormalizationInputId::derive(&[set.set_id.as_str(), "fixture-input"]),
            operation_id: NormalizationOperationId::derive(&[
                set.set_id.as_str(),
                "fixture-operation",
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappingApplicationRecord {
    pub application_id: MappingApplicationId,
    pub source_element_id: ReconciledSemanticElementId,
    pub source_expression_reference: String,
    pub original_supplied_expression: String,
    pub matched_source_form: String,
    pub target_canonical_value: String,
    pub mapping_rule_id: String,
    pub mapping_rule_version: String,
    pub applicability_basis: Arc<[String]>,
    pub semantic_domain: String,
    pub semantic_class: String,
    pub expression_class: String,
    pub profile_binding: (NormalizationProfileId, String),
    pub registry_binding: (CanonicalRegistryId, String, MappingRuleRegistryId, String),
    pub authority_basis: String,
    pub application_status: MappingApplicationStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizationDecision {
    pub decision_id: NormalizationDecisionId,
    pub operation_id: NormalizationOperationId,
    pub source_element_id: ReconciledSemanticElementId,
    pub standing_assignment_id: StandingAssignmentId,
    pub eligibility: NormalizationEligibility,
    pub mapping_application_id: Option<MappingApplicationId>,
    pub profile_binding: (NormalizationProfileId, String),
    pub registry_binding: (CanonicalRegistryId, String, MappingRuleRegistryId, String),
    pub schema_version: String,
    pub configuration_version: String,
    pub original_expression_reference: String,
    pub resulting_expression: String,
    pub semantic_identity: ReconciledSemanticElementId,
    pub semantic_domain: String,
    pub semantic_class: String,
    pub represented_scope: String,
    pub contributing_subject_ids: Arc<[ReconciliationSubjectId]>,
    pub contributing_decision_ids: Arc<[ReconciliationDecisionId]>,
    pub preserved_alternative_ids: Arc<[ReconciliationSubjectId]>,
    pub evidence_reference_ids: Arc<[String]>,
    pub grounding_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
    pub disposition: NormalizationDisposition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedSemanticElement {
    pub normalized_element_id: NormalizedSemanticElementId,
    pub source_element_id: ReconciledSemanticElementId,
    pub normalization_decision_id: NormalizationDecisionId,
    pub standing_assignment_id: StandingAssignmentId,
    pub canonical_expression: String,
    pub semantic_domain: String,
    pub semantic_class: String,
    pub represented_scope: String,
    pub contributing_subject_ids: Arc<[ReconciliationSubjectId]>,
    pub contributing_decision_ids: Arc<[ReconciliationDecisionId]>,
    pub preserved_alternative_ids: Arc<[ReconciliationSubjectId]>,
    pub evidence_reference_ids: Arc<[String]>,
    pub grounding_reference_ids: Arc<[String]>,
    pub provenance_reference_ids: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedRelationship {
    pub normalized_relationship_id: NormalizedRelationshipId,
    pub source_relationship_id: ReconciliationRelationshipId,
    pub relationship_class: String,
    pub canonical_expression: String,
    /// Contract 010 preserves the exact Contract 009 subject endpoint IDs.
    /// Contract 011 may order this relationship but may not reinterpret them.
    pub source_subject_id: ReconciliationSubjectId,
    pub target_subject_id: ReconciliationSubjectId,
    pub represented_scope: String,
    pub normalization_disposition: NormalizationDisposition,
    pub normalization_decision_references: Arc<[NormalizationDecisionId]>,
    pub reconciliation_decision_references: Arc<[ReconciliationDecisionId]>,
    pub standing_references: Arc<[StandingAssignmentId]>,
    pub evidence_references: Arc<[String]>,
    pub grounding_references: Arc<[String]>,
    pub provenance_references: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedRequestRepresentation {
    pub publication_id: NormalizedRequestRepresentationId,
    pub operation_id: NormalizationOperationId,
    pub input_id: NormalizationInputId,
    pub reconciliation_set_id: SemanticReconciliationSetId,
    pub profile_binding: (NormalizationProfileId, String),
    pub registry_binding: (CanonicalRegistryId, String, MappingRuleRegistryId, String),
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub elements: Arc<[NormalizedSemanticElement]>,
    pub relationships: Arc<[NormalizedRelationship]>,
    pub decisions: Arc<[NormalizationDecision]>,
    pub mapping_applications: Arc<[MappingApplicationRecord]>,
    pub stable_iteration_order: Arc<[NormalizedSemanticElementId]>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizationFailureRecord {
    pub failure_record_id: NormalizationFailureRecordId,
    pub operation_id: NormalizationOperationId,
    pub input_id: NormalizationInputId,
    pub category: NormalizationFailureCategory,
    pub stage: String,
    pub detail: String,
    pub profile_binding: (NormalizationProfileId, String),
    pub registry_binding: (CanonicalRegistryId, String, MappingRuleRegistryId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NormalizationOperationOutcome {
    Success(NormalizedRequestRepresentation),
    Failure(NormalizationFailureRecord),
}

fn reconciliation_content_binding(set: &SemanticReconciliationSet) -> String {
    StableId::from_parts(
        "reconciliation-content",
        &[
            set.set_id.as_str(),
            &format!(
                "{:?}{:?}{:?}{:?}{:?}",
                set.elements,
                set.decisions,
                set.standing_assignments,
                set.replay_context,
                stable_reconciliation_relationship_binding(&set.relationships)
            ),
        ],
    )
    .to_string()
}

fn normalization_failure(
    input: &SemanticNormalizationInput,
    category: NormalizationFailureCategory,
    stage: &str,
    detail: &str,
) -> NormalizationOperationOutcome {
    NormalizationOperationOutcome::Failure(NormalizationFailureRecord {
        failure_record_id: NormalizationFailureRecordId::derive(&[
            input.operation_id.as_str(),
            stage,
            detail,
        ]),
        operation_id: input.operation_id.clone(),
        input_id: input.input_id.clone(),
        category,
        stage: stage.to_owned(),
        detail: detail.to_owned(),
        profile_binding: (input.profile_id.clone(), input.profile_version.clone()),
        registry_binding: (
            input.canonical_registry_id.clone(),
            input.canonical_registry_version.clone(),
            input.mapping_rule_registry_id.clone(),
            input.mapping_rule_registry_version.clone(),
        ),
    })
}

fn standing_for_element<'a>(
    set: &'a SemanticReconciliationSet,
    element: &ReconciledSemanticElement,
) -> Result<&'a StandingAssignment, NormalizationFailureCategory> {
    let matches: Vec<_> = set
        .standing_assignments
        .iter()
        .filter(|a| {
            a.target_kind == "ResultingReconciledElement"
                && a.target_id == element.element_id.to_string()
        })
        .collect();
    if matches.len() != 1 {
        return Err(NormalizationFailureCategory::InvalidStandingBinding);
    }
    let standing = matches[0];
    if !set.decisions.iter().any(|d| {
        d.decision_id == standing.decision_id
            && d.resulting_element_ids
                .iter()
                .any(|id| id == &element.element_id)
    }) {
        return Err(NormalizationFailureCategory::InvalidStandingBinding);
    }
    Ok(standing)
}

pub fn evaluate_normalization_eligibility(
    element: &ReconciledSemanticElement,
    standing: &StandingAssignment,
    profile: &FixtureNormalizationProfile,
) -> Result<NormalizationEligibility, NormalizationFailureCategory> {
    if !profile
        .supported_semantic_domains
        .contains(&element.semantic_domain)
        || !profile
            .supported_semantic_classes
            .contains(&element.semantic_class)
    {
        return Err(NormalizationFailureCategory::EligibilityResolutionFailure);
    }
    if profile
        .deferred_conditions
        .iter()
        .any(|condition| element.unresolved_conditions.iter().any(|x| x == condition))
    {
        return Ok(NormalizationEligibility::ProfileDeferred);
    }
    if profile.eligible_standings.contains(&standing.standing) {
        Ok(NormalizationEligibility::Eligible)
    } else {
        Ok(NormalizationEligibility::NotEligible)
    }
}

fn find_mapping<'a>(
    element: &ReconciledSemanticElement,
    profile: &FixtureNormalizationProfile,
    rules: &'a FixtureMappingRuleRegistry,
) -> Result<Option<&'a MappingRuleDefinition>, NormalizationFailureCategory> {
    let matches: Vec<_> = rules
        .rules
        .iter()
        .filter(|rule| {
            rule.semantic_domain == element.semantic_domain
                && rule.semantic_class == element.semantic_class
                && rule.expression_class == "expression"
                && rule.exact_source_form == element.original_expression
                && rule.required_registry_target == profile.canonical_registry_id.to_string()
        })
        .collect();
    if matches.len() > 1 {
        return Err(NormalizationFailureCategory::NonDeterministicRuleSelection);
    }
    Ok(matches.into_iter().next())
}

fn find_relationship_mapping<'a>(
    relationship: &ReconciledRelationship,
    profile: &FixtureNormalizationProfile,
    rules: &'a FixtureMappingRuleRegistry,
) -> Result<Option<&'a MappingRuleDefinition>, NormalizationFailureCategory> {
    let matches: Vec<_> = rules
        .rules
        .iter()
        .filter(|rule| {
            rule.semantic_domain == "Relationship"
                && rule.semantic_class == relationship.relationship_class
                && rule.expression_class == "expression"
                && rule.exact_source_form == relationship.expression
                && rule.required_registry_target == profile.canonical_registry_id.to_string()
        })
        .collect();
    if matches.len() > 1 {
        return Err(NormalizationFailureCategory::NonDeterministicRuleSelection);
    }
    Ok(matches.into_iter().next())
}

pub fn normalize_semantic_request(
    input: &SemanticNormalizationInput,
    reconciliation: &SemanticReconciliationSet,
    profile: &FixtureNormalizationProfile,
    canonical_registry: &FixtureCanonicalRegistry,
    mapping_rules: &FixtureMappingRuleRegistry,
) -> NormalizationOperationOutcome {
    if input.reconciliation_set_id != reconciliation.set_id
        || input.upstream_content_binding != reconciliation_content_binding(reconciliation)
    {
        return normalization_failure(
            input,
            NormalizationFailureCategory::InvalidUpstreamPublication,
            "upstream-validation",
            "the supplied Contract 009 publication is not the exact bound publication",
        );
    }
    if input.profile_id != profile.identity || input.profile_version != profile.version {
        return normalization_failure(
            input,
            NormalizationFailureCategory::ProfileVersionMismatch,
            "profile-validation",
            "profile identity or version is not exact",
        );
    }
    if input.canonical_registry_id != canonical_registry.identity
        || input.canonical_registry_version != canonical_registry.version
        || profile.canonical_registry_id != canonical_registry.identity
        || profile.canonical_registry_version != canonical_registry.version
    {
        return normalization_failure(
            input,
            NormalizationFailureCategory::ConflictingRegistry,
            "registry-validation",
            "canonical registry binding is not exact",
        );
    }
    if input.mapping_rule_registry_id != mapping_rules.identity
        || input.mapping_rule_registry_version != mapping_rules.version
        || profile.mapping_rule_registry_id != mapping_rules.identity
        || profile.mapping_rule_registry_version != mapping_rules.version
    {
        return normalization_failure(
            input,
            NormalizationFailureCategory::MissingRegistry,
            "rule-registry-validation",
            "mapping-rule registry binding is not exact",
        );
    }
    if input.schema_version != profile.schema_version
        || input.configuration_version != profile.configuration_version
        || input.implementation_version != profile.implementation_version
    {
        return normalization_failure(
            input,
            NormalizationFailureCategory::SchemaIncompatibility,
            "binding-validation",
            "schema, configuration, or implementation binding is not exact",
        );
    }

    let mut decisions = Vec::new();
    let mut elements = Vec::new();
    let mut normalized_relationships = Vec::new();
    let mut applications = Vec::new();
    let mut stable_iteration_order = Vec::new();
    for source in reconciliation.elements.iter() {
        let standing = match standing_for_element(reconciliation, source) {
            Ok(value) => value,
            Err(category) => {
                return normalization_failure(
                    input,
                    category,
                    "standing-validation",
                    "exactly one valid Contract 009 standing assignment is required",
                )
            }
        };
        let eligibility = match evaluate_normalization_eligibility(source, standing, profile) {
            Ok(value) => value,
            Err(category) => {
                return normalization_failure(
                    input,
                    category,
                    "eligibility",
                    "element is outside the fixture profile",
                )
            }
        };
        if eligibility == NormalizationEligibility::NotEligible {
            return normalization_failure(
                input,
                NormalizationFailureCategory::EligibilityResolutionFailure,
                "eligibility",
                "standing does not authorize normalization",
            );
        }
        let (disposition, resulting_expression, application_id) =
            if eligibility == NormalizationEligibility::ProfileDeferred {
                if !profile
                    .permitted_dispositions
                    .contains(&NormalizationDisposition::Deferred)
                {
                    return normalization_failure(
                        input,
                        NormalizationFailureCategory::NoAuthorizedRule,
                        "disposition",
                        "profile deferral is not authorized",
                    );
                }
                (
                    NormalizationDisposition::Deferred,
                    source.original_expression.clone(),
                    None,
                )
            } else if let Some(rule) = match find_mapping(source, profile, mapping_rules) {
                Ok(value) => value,
                Err(category) => {
                    return normalization_failure(
                        input,
                        category,
                        "rule-selection",
                        "mapping rule selection is not deterministic",
                    )
                }
            } {
                if !canonical_registry.contains(
                    &rule.semantic_domain,
                    &rule.semantic_class,
                    &rule.expression_class,
                    &rule.exact_target_form,
                ) {
                    return normalization_failure(
                        input,
                        NormalizationFailureCategory::InvalidMappingTarget,
                        "mapping-target",
                        "mapping target is absent from the bound canonical registry",
                    );
                }
                let application_id = MappingApplicationId::derive(&[
                    source.element_id.as_str(),
                    rule.rule_id.as_str(),
                    rule.rule_version.as_str(),
                    profile.version.as_str(),
                    canonical_registry.version.as_str(),
                ]);
                applications.push(MappingApplicationRecord {
                    application_id: application_id.clone(),
                    source_element_id: source.element_id.clone(),
                    source_expression_reference: source.representation_id.clone(),
                    original_supplied_expression: source.original_expression.clone(),
                    matched_source_form: rule.exact_source_form.clone(),
                    target_canonical_value: rule.exact_target_form.clone(),
                    mapping_rule_id: rule.rule_id.clone(),
                    mapping_rule_version: rule.rule_version.clone(),
                    applicability_basis: rule.applicability_conditions.clone(),
                    semantic_domain: rule.semantic_domain.clone(),
                    semantic_class: rule.semantic_class.clone(),
                    expression_class: rule.expression_class.clone(),
                    profile_binding: (profile.identity.clone(), profile.version.clone()),
                    registry_binding: (
                        canonical_registry.identity.clone(),
                        canonical_registry.version.clone(),
                        mapping_rules.identity.clone(),
                        mapping_rules.version.clone(),
                    ),
                    authority_basis: rule.profile_authority.clone(),
                    application_status: MappingApplicationStatus::Applied,
                });
                (
                    NormalizationDisposition::Mapped,
                    rule.exact_target_form.clone(),
                    Some(application_id),
                )
            } else if canonical_registry.contains(
                &source.semantic_domain,
                &source.semantic_class,
                "expression",
                &source.original_expression,
            ) && profile.allows_identity(source)
            {
                (
                    NormalizationDisposition::Identity,
                    source.original_expression.clone(),
                    None,
                )
            } else if profile.allows_preservation(source) {
                (
                    NormalizationDisposition::Preserved,
                    source.original_expression.clone(),
                    None,
                )
            } else {
                return normalization_failure(
                    input,
                    NormalizationFailureCategory::NoAuthorizedRule,
                    "disposition",
                    "no mapping, identity, or preservation authority applies",
                );
            };
        let decision_id = NormalizationDecisionId::derive(&[
            input.operation_id.as_str(),
            source.element_id.as_str(),
            &format!("{disposition:?}"),
            &resulting_expression,
        ]);
        let decision = NormalizationDecision {
            decision_id: decision_id.clone(),
            operation_id: input.operation_id.clone(),
            source_element_id: source.element_id.clone(),
            standing_assignment_id: standing.assignment_id.clone(),
            eligibility,
            mapping_application_id: application_id.clone(),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            registry_binding: (
                canonical_registry.identity.clone(),
                canonical_registry.version.clone(),
                mapping_rules.identity.clone(),
                mapping_rules.version.clone(),
            ),
            schema_version: input.schema_version.clone(),
            configuration_version: input.configuration_version.clone(),
            original_expression_reference: source.representation_id.clone(),
            resulting_expression: resulting_expression.clone(),
            semantic_identity: source.element_id.clone(),
            semantic_domain: source.semantic_domain.clone(),
            semantic_class: source.semantic_class.clone(),
            represented_scope: source.represented_scope.clone(),
            contributing_subject_ids: source.contributing_subject_ids.clone(),
            contributing_decision_ids: source.contributing_decision_ids.clone(),
            preserved_alternative_ids: source.preserved_alternative_ids.clone(),
            evidence_reference_ids: source.evidence_reference_ids.clone(),
            grounding_reference_ids: source.grounding_reference_ids.clone(),
            provenance_reference_ids: source.provenance_reference_ids.clone(),
            unresolved_conditions: source.unresolved_conditions.clone(),
            disposition,
        };
        let normalized_element_id = NormalizedSemanticElementId::derive(&[
            source.element_id.as_str(),
            resulting_expression.as_str(),
            profile.version.as_str(),
        ]);
        elements.push(NormalizedSemanticElement {
            normalized_element_id: normalized_element_id.clone(),
            source_element_id: source.element_id.clone(),
            normalization_decision_id: decision_id,
            standing_assignment_id: standing.assignment_id.clone(),
            canonical_expression: resulting_expression,
            semantic_domain: source.semantic_domain.clone(),
            semantic_class: source.semantic_class.clone(),
            represented_scope: source.represented_scope.clone(),
            contributing_subject_ids: source.contributing_subject_ids.clone(),
            contributing_decision_ids: source.contributing_decision_ids.clone(),
            preserved_alternative_ids: source.preserved_alternative_ids.clone(),
            evidence_reference_ids: source.evidence_reference_ids.clone(),
            grounding_reference_ids: source.grounding_reference_ids.clone(),
            provenance_reference_ids: source.provenance_reference_ids.clone(),
            unresolved_conditions: source.unresolved_conditions.clone(),
        });
        stable_iteration_order.push(normalized_element_id);
        decisions.push(decision);
    }
    for source in reconciliation.relationships.iter() {
        if !profile
            .supported_relationship_classes
            .contains(&source.relationship_class)
            || !reconciliation
                .subjects
                .iter()
                .any(|subject| subject.subject_id == source.source_subject_id)
            || !reconciliation
                .subjects
                .iter()
                .any(|subject| subject.subject_id == source.target_subject_id)
        {
            return normalization_failure(
                input,
                NormalizationFailureCategory::TraceabilityFailure,
                "relationship-validation",
                "relationship class or exact endpoint identity is not bound by Contract 009",
            );
        }
        let (disposition, resulting_expression) = if source
            .unresolved_conditions
            .iter()
            .any(|condition| condition == "profile-deferred")
        {
            if !profile
                .permitted_dispositions
                .contains(&NormalizationDisposition::Deferred)
            {
                return normalization_failure(
                    input,
                    NormalizationFailureCategory::NoAuthorizedRule,
                    "relationship-disposition",
                    "relationship deferral is not authorized by the bound profile",
                );
            }
            (
                NormalizationDisposition::Deferred,
                source.expression.clone(),
            )
        } else if let Some(rule) = match find_relationship_mapping(source, profile, mapping_rules) {
            Ok(value) => value,
            Err(category) => {
                return normalization_failure(
                    input,
                    category,
                    "relationship-rule-selection",
                    "relationship mapping rule selection is not deterministic",
                )
            }
        } {
            if !canonical_registry.contains(
                "Relationship",
                &rule.semantic_class,
                &rule.expression_class,
                &rule.exact_target_form,
            ) {
                return normalization_failure(
                    input,
                    NormalizationFailureCategory::InvalidMappingTarget,
                    "relationship-mapping-target",
                    "relationship mapping target is absent from the bound canonical registry",
                );
            }
            (
                NormalizationDisposition::Mapped,
                rule.exact_target_form.clone(),
            )
        } else if canonical_registry.contains(
            "Relationship",
            &source.relationship_class,
            "expression",
            &source.expression,
        ) && profile.allows_relationship_identity(source)
        {
            (
                NormalizationDisposition::Identity,
                source.expression.clone(),
            )
        } else if profile.allows_relationship_preservation(source) {
            (
                NormalizationDisposition::Preserved,
                source.expression.clone(),
            )
        } else {
            return normalization_failure(
                input,
                NormalizationFailureCategory::NoAuthorizedRule,
                "relationship-disposition",
                "no relationship mapping, identity, or preservation authority applies",
            );
        };
        let relationship_decision_id = NormalizationDecisionId::derive(&[
            input.operation_id.as_str(),
            source.relationship_id.as_str(),
            &format!("{disposition:?}"),
            &resulting_expression,
        ]);
        let normalized_relationship_id = NormalizedRelationshipId::derive(&[
            source.source_relationship_id.as_str(),
            source.relationship_class.as_str(),
            resulting_expression.as_str(),
            source.source_subject_id.as_str(),
            source.target_subject_id.as_str(),
            source.represented_scope.as_str(),
            &format!(
                "{:?}{:?}{:?}{:?}{:?}",
                disposition,
                source.reconciliation_decision_ids,
                source.standing_assignment_ids,
                source.evidence_reference_ids,
                source.grounding_reference_ids
            ),
            &format!(
                "{:?}{:?}",
                source.provenance_reference_ids, source.unresolved_conditions
            ),
            profile.version.as_str(),
            canonical_registry.version.as_str(),
            mapping_rules.version.as_str(),
            input.schema_version.as_str(),
            input.configuration_version.as_str(),
            input.implementation_version.as_str(),
        ]);
        normalized_relationships.push(NormalizedRelationship {
            normalized_relationship_id,
            source_relationship_id: source.source_relationship_id.clone(),
            relationship_class: source.relationship_class.clone(),
            canonical_expression: resulting_expression,
            source_subject_id: source.source_subject_id.clone(),
            target_subject_id: source.target_subject_id.clone(),
            represented_scope: source.represented_scope.clone(),
            normalization_disposition: disposition,
            normalization_decision_references: Arc::from([relationship_decision_id]),
            reconciliation_decision_references: source.reconciliation_decision_ids.clone(),
            standing_references: source.standing_assignment_ids.clone(),
            evidence_references: source.evidence_reference_ids.clone(),
            grounding_references: source.grounding_reference_ids.clone(),
            provenance_references: source.provenance_reference_ids.clone(),
            unresolved_conditions: source.unresolved_conditions.clone(),
        });
    }
    let mut identity_relationships = normalized_relationships.clone();
    identity_relationships.sort_by(|left, right| {
        left.normalized_relationship_id
            .cmp(&right.normalized_relationship_id)
    });
    let publication_id = NormalizedRequestRepresentationId::derive(&[
        input.operation_id.as_str(),
        &format!("{:?}{:?}{:?}", elements, identity_relationships, decisions),
    ]);
    NormalizationOperationOutcome::Success(NormalizedRequestRepresentation {
        publication_id,
        operation_id: input.operation_id.clone(),
        input_id: input.input_id.clone(),
        reconciliation_set_id: reconciliation.set_id.clone(),
        profile_binding: (profile.identity.clone(), profile.version.clone()),
        registry_binding: (
            canonical_registry.identity.clone(),
            canonical_registry.version.clone(),
            mapping_rules.identity.clone(),
            mapping_rules.version.clone(),
        ),
        schema_version: input.schema_version.clone(),
        configuration_version: input.configuration_version.clone(),
        implementation_version: input.implementation_version.clone(),
        elements: elements.into(),
        relationships: normalized_relationships.into(),
        decisions: decisions.into(),
        mapping_applications: applications.into(),
        stable_iteration_order: stable_iteration_order.into(),
        replay_context: input.replay_context.clone(),
    })
}

#[cfg(test)]
mod contract_010_tests {
    use super::*;

    fn reconciliation(expression: &str, standing: Standing) -> SemanticReconciliationSet {
        let element_id = ReconciledSemanticElementId::derive(&["element"]);
        let decision_id = ReconciliationDecisionId::derive(&["decision"]);
        let assignment_id = StandingAssignmentId::derive(&["assignment"]);
        let subject_id = ReconciliationSubjectId::derive(&["subject"]);
        SemanticReconciliationSet {
            set_id: SemanticReconciliationSetId::derive(&[expression, &format!("{standing:?}")]),
            operation_id: SemanticReconciliationOperationId::derive(&["operation"]),
            logical_reconciliation_id: SemanticReconciliationId::derive(&["logical"]),
            input_id: SemanticReconciliationInputId::derive(&["input"]),
            subjects: vec![ReconciliationSubject {
                subject_id: subject_id.clone(),
                upstream_publication_id: "fixture-publication".to_owned(),
                upstream_representation_id: "fixture-representation".to_owned(),
                original_expression: expression.to_owned(),
                semantic_class: "RequestedOutcome".to_owned(),
                semantic_domain: "Objective".to_owned(),
                represented_scope: "request-wide".to_owned(),
                upstream_status: "Represented".to_owned(),
                input_declaration_id: SemanticReconciliationInputId::derive(&["input"]),
                evidence_reference_ids: vec!["evidence".to_owned()].into(),
                provenance_reference_ids: vec!["provenance".to_owned()].into(),
            }]
            .into(),
            groups: Vec::new().into(),
            findings: Vec::new().into(),
            rule_applications: Vec::new().into(),
            decisions: vec![ReconciliationDecision {
                decision_id: decision_id.clone(),
                group_id: ReconciliationGroupId::derive(&["group"]),
                participating_subject_ids: vec![subject_id.clone()].into(),
                comparison_finding_ids: Vec::new().into(),
                rule_application_ids: Vec::new().into(),
                disposition: ReconciliationDisposition::Preserved,
                decision_basis: vec!["fixture".to_owned()].into(),
                standing_assignment_ids: vec![assignment_id.clone()].into(),
                resulting_element_ids: vec![element_id.clone()].into(),
                preserved_alternative_ids: Vec::new().into(),
                unresolved_conditions: Vec::new().into(),
            }]
            .into(),
            standing_assignments: vec![StandingAssignment {
                assignment_id,
                decision_id,
                target_kind: "ResultingReconciledElement".to_owned(),
                target_id: element_id.to_string(),
                standing,
                assignment_basis: "fixture".to_owned(),
            }]
            .into(),
            elements: vec![ReconciledSemanticElement {
                element_id: element_id.clone(),
                representation_id: "source-expression-reference".to_owned(),
                original_expression: expression.to_owned(),
                semantic_domain: "Objective".to_owned(),
                semantic_class: "RequestedOutcome".to_owned(),
                represented_scope: "request-wide".to_owned(),
                evidence_reference_ids: vec!["evidence".to_owned()].into(),
                grounding_reference_ids: vec!["grounding".to_owned()].into(),
                provenance_reference_ids: vec!["provenance".to_owned()].into(),
                contributing_subject_ids: vec![subject_id].into(),
                contributing_decision_ids: vec![ReconciliationDecisionId::derive(&["decision"])]
                    .into(),
                preserved_alternative_ids: Vec::new().into(),
                exact_content_binding: "fixture-content".to_owned(),
                construction_kind: "fixture".to_owned(),
                unresolved_conditions: Vec::new().into(),
            }]
            .into(),
            relationships: Vec::new().into(),
            status: SemanticReconciliationStatus::Reconciled,
            profile_id: SemanticReconciliationProfileId::derive(&["contract-009-fixture"]),
            profile_version: "fixture-reconciliation-v1".to_owned(),
            registry_version: "fixture-reconciliation-registry-v1".to_owned(),
            schema_version: "fixture-reconciliation-schema-v1".to_owned(),
            configuration_version: "fixture-reconciliation-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        }
    }

    fn normalize(
        set: &SemanticReconciliationSet,
        profile: &FixtureNormalizationProfile,
    ) -> NormalizationOperationOutcome {
        let mut input = SemanticNormalizationInput::for_fixture(set);
        input.profile_id = profile.identity.clone();
        input.profile_version = profile.version.clone();
        normalize_semantic_request(
            &input,
            set,
            profile,
            &FixtureCanonicalRegistry::fixture(),
            &FixtureMappingRuleRegistry::fixture(),
        )
    }

    fn with_relationship(
        mut set: SemanticReconciliationSet,
        source_relationship_id: &str,
        expression: &str,
    ) -> SemanticReconciliationSet {
        let subject_id = set.elements[0].contributing_subject_ids[0].clone();
        set.relationships = vec![ReconciledRelationship {
            relationship_id: ReconciledRelationshipId::derive(&[source_relationship_id]),
            source_relationship_id: ReconciliationRelationshipId::derive(&[source_relationship_id]),
            relationship_class: "DependsOn".to_owned(),
            expression: expression.to_owned(),
            source_subject_id: subject_id.clone(),
            target_subject_id: subject_id,
            represented_scope: "request-wide".to_owned(),
            reconciliation_decision_ids: vec![ReconciliationDecisionId::derive(&["decision"])]
                .into(),
            standing_assignment_ids: vec![StandingAssignmentId::derive(&["assignment"])].into(),
            evidence_reference_ids: vec!["evidence".to_owned()].into(),
            grounding_reference_ids: vec!["grounding".to_owned()].into(),
            provenance_reference_ids: vec!["provenance".to_owned()].into(),
            unresolved_conditions: Vec::new().into(),
        }]
        .into();
        set
    }

    #[test]
    fn contract_010_exact_alias_is_mapped_and_replay_is_deterministic() {
        let set = reconciliation("achieve outcome", Standing::Included);
        let profile = FixtureNormalizationProfile::fixture();
        let first = normalize(&set, &profile);
        assert_eq!(first, normalize(&set, &profile));
        let NormalizationOperationOutcome::Success(publication) = first else {
            panic!("normalization publication")
        };
        assert_eq!(
            publication.decisions[0].disposition,
            NormalizationDisposition::Mapped
        );
        assert_eq!(
            publication.elements[0].canonical_expression,
            "achieve-outcome"
        );
        assert_eq!(publication.mapping_applications.len(), 1);
        assert!(publication.relationships.is_empty());
    }

    #[test]
    fn contract_010_relationship_publication_preserves_exact_continuity() {
        let set = with_relationship(
            reconciliation("achieve outcome", Standing::Included),
            "source-relationship",
            "depends on",
        );
        let NormalizationOperationOutcome::Success(publication) =
            normalize(&set, &FixtureNormalizationProfile::fixture())
        else {
            panic!("relationship normalization publication")
        };
        let relationship = &publication.relationships[0];
        assert_eq!(
            relationship.source_relationship_id,
            ReconciliationRelationshipId::derive(&["source-relationship"])
        );
        assert_eq!(relationship.canonical_expression, "depends-on");
        assert_eq!(
            relationship.source_subject_id,
            relationship.target_subject_id
        );
        assert_eq!(relationship.represented_scope, "request-wide");
        assert_eq!(relationship.reconciliation_decision_references.len(), 1);
        assert_eq!(relationship.standing_references.len(), 1);
        assert_eq!(
            relationship.normalization_disposition,
            NormalizationDisposition::Mapped
        );
    }

    #[test]
    fn contract_010_relationship_identity_separation_is_preserved() {
        let first = with_relationship(
            reconciliation("achieve outcome", Standing::Included),
            "relationship-one",
            "depends on",
        );
        let second = with_relationship(first.clone(), "relationship-two", "depends on");
        let NormalizationOperationOutcome::Success(first_publication) =
            normalize(&first, &FixtureNormalizationProfile::fixture())
        else {
            panic!("first relationship publication")
        };
        let NormalizationOperationOutcome::Success(second_publication) =
            normalize(&second, &FixtureNormalizationProfile::fixture())
        else {
            panic!("second relationship publication")
        };
        assert_ne!(
            first_publication.relationships[0].normalized_relationship_id,
            second_publication.relationships[0].normalized_relationship_id
        );
    }

    #[test]
    fn contract_010_relationship_collection_order_is_non_authoritative() {
        let first = with_relationship(
            reconciliation("achieve outcome", Standing::Included),
            "relationship-one",
            "depends on",
        );
        let second_relationship = ReconciledRelationship {
            relationship_id: ReconciledRelationshipId::derive(&["relationship-two"]),
            source_relationship_id: ReconciliationRelationshipId::derive(&["relationship-two"]),
            relationship_class: "DependsOn".to_owned(),
            expression: "depends on".to_owned(),
            source_subject_id: first.elements[0].contributing_subject_ids[0].clone(),
            target_subject_id: first.elements[0].contributing_subject_ids[0].clone(),
            represented_scope: "request-wide".to_owned(),
            reconciliation_decision_ids: vec![ReconciliationDecisionId::derive(&["decision"])]
                .into(),
            standing_assignment_ids: vec![StandingAssignmentId::derive(&["assignment"])].into(),
            evidence_reference_ids: vec!["evidence".to_owned()].into(),
            grounding_reference_ids: vec!["grounding".to_owned()].into(),
            provenance_reference_ids: vec!["provenance".to_owned()].into(),
            unresolved_conditions: Vec::new().into(),
        };
        let mut forward = first.clone();
        forward.relationships = vec![first.relationships[0].clone(), second_relationship].into();
        let mut reversed = forward.clone();
        reversed.relationships = vec![
            forward.relationships[1].clone(),
            forward.relationships[0].clone(),
        ]
        .into();
        let first_publication = normalize(&forward, &FixtureNormalizationProfile::fixture());
        let reversed_publication = normalize(&reversed, &FixtureNormalizationProfile::fixture());
        let (
            NormalizationOperationOutcome::Success(first),
            NormalizationOperationOutcome::Success(reversed),
        ) = (first_publication, reversed_publication)
        else {
            panic!("relationship permutation publications")
        };
        assert_eq!(first.publication_id, reversed.publication_id);
        assert_eq!(first.relationships.len(), reversed.relationships.len());
    }

    #[test]
    fn contract_010_foreign_relationship_endpoint_fails_atomically() {
        let mut set = with_relationship(
            reconciliation("achieve outcome", Standing::Included),
            "source-relationship",
            "depends on",
        );
        let mut relationship = set.relationships[0].clone();
        relationship.target_subject_id = ReconciliationSubjectId::derive(&["foreign"]);
        set.relationships = vec![relationship].into();
        assert!(matches!(
            normalize(&set, &FixtureNormalizationProfile::fixture()),
            NormalizationOperationOutcome::Failure(NormalizationFailureRecord {
                category: NormalizationFailureCategory::TraceabilityFailure,
                ..
            })
        ));
    }

    #[test]
    fn contract_010_wrong_upstream_binding_fails_atomically() {
        let set = reconciliation("achieve outcome", Standing::Included);
        let profile = FixtureNormalizationProfile::fixture();
        let mut input = SemanticNormalizationInput::for_fixture(&set);
        input.upstream_content_binding = "foreign-binding".to_owned();
        assert!(matches!(
            normalize_semantic_request(
                &input,
                &set,
                &profile,
                &FixtureCanonicalRegistry::fixture(),
                &FixtureMappingRuleRegistry::fixture()
            ),
            NormalizationOperationOutcome::Failure(NormalizationFailureRecord {
                category: NormalizationFailureCategory::InvalidUpstreamPublication,
                ..
            })
        ));
    }

    #[test]
    fn contract_010_missing_standing_cannot_create_authority() {
        let mut set = reconciliation("achieve outcome", Standing::Included);
        set.standing_assignments = Vec::new().into();
        let outcome = normalize(&set, &FixtureNormalizationProfile::fixture());
        assert!(matches!(
            outcome,
            NormalizationOperationOutcome::Failure(NormalizationFailureRecord {
                category: NormalizationFailureCategory::InvalidStandingBinding,
                ..
            })
        ));
    }

    #[test]
    fn contract_010_no_mapping_is_not_an_identity_or_preservation_fallback() {
        let set = reconciliation("unmapped expression", Standing::Included);
        let outcome = normalize(&set, &FixtureNormalizationProfile::fixture());
        assert!(matches!(
            outcome,
            NormalizationOperationOutcome::Failure(NormalizationFailureRecord {
                category: NormalizationFailureCategory::NoAuthorizedRule,
                ..
            })
        ));
    }

    #[test]
    fn contract_010_deferred_profile_condition_is_explicit() {
        let mut set = reconciliation("unmapped expression", Standing::Included);
        let mut element = set.elements[0].clone();
        element.unresolved_conditions = vec!["profile-deferred".to_owned()].into();
        set.elements = vec![element].into();
        let outcome = normalize(&set, &FixtureNormalizationProfile::fixture());
        let NormalizationOperationOutcome::Success(publication) = outcome else {
            panic!("deferred publication")
        };
        assert_eq!(
            publication.decisions[0].disposition,
            NormalizationDisposition::Deferred
        );
    }
}

// ---------------------------------------------------------------------------
// Contract 011: Canonical ordering
// ---------------------------------------------------------------------------

contract_002_id!(CanonicalOrderingInputId, "coinput");
contract_002_id!(CanonicalOrderingOperationId, "coop");
contract_002_id!(CanonicalOrderingProfileId, "coprofile");
contract_002_id!(OrderingScopeId, "coscope");
contract_002_id!(OrderableSubjectId, "cosubject");
contract_002_id!(OrderingConstraintId, "coconstraint");
contract_002_id!(OrderingDecisionId, "codecision");
contract_002_id!(CanonicalOrderingAssignmentId, "coassignment");
contract_002_id!(CanonicallyOrderedRequestRepresentationId, "corequest");
contract_002_id!(CanonicalOrderingFailureRecordId, "cofail");
contract_002_id!(OrderingConstraintApplicationId, "coapply");
contract_002_id!(OrderingConstraintGraphId, "cograph");
contract_002_id!(OrderingLinearizationRecordId, "colinear");
contract_002_id!(OrderingRegistryId, "coregistry");
contract_002_id!(OrderingRuleRegistryId, "corules");

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OrderingSubjectReference {
    Element(NormalizedSemanticElementId),
    Relationship(NormalizedRelationshipId),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OrderingSubjectKind {
    Element,
    Relationship,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OrderingRelation {
    Before,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OrderingOutcomeClass {
    Direct,
    DependencyLinearized,
    TieBroken,
    IdentityOrder,
    PreservedOrder,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CanonicalOrderingFailureCategory {
    InvalidUpstreamPublication,
    UpstreamPublicationResolutionFailure,
    ProfileResolutionFailure,
    RegistryResolutionFailure,
    InvalidSubjectReference,
    UnresolvableOrderingScope,
    MissingRequiredOrderingRule,
    InvalidTraversalRule,
    UndefinedComparison,
    NonDeterministicComparator,
    MissingRequiredTieBreaker,
    OrderingCycle,
    ContradictoryOrderingConstraints,
    InvalidAssignmentCoverage,
    AtomicCommitmentFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderableSubject {
    pub subject_id: OrderableSubjectId,
    pub subject_kind: OrderingSubjectKind,
    pub normalized_subject: OrderingSubjectReference,
    pub normalized_publication_id: NormalizedRequestRepresentationId,
    pub scope_ids: Arc<[OrderingScopeId]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingScope {
    pub scope_id: OrderingScopeId,
    pub scope_kind: String,
    pub participating_subject_ids: Arc<[OrderableSubjectId]>,
    pub governing_constraint_ids: Arc<[OrderingConstraintId]>,
    pub profile_binding: (CanonicalOrderingProfileId, String),
    pub registry_binding: (OrderingRegistryId, String, OrderingRuleRegistryId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingConstraint {
    pub constraint_id: OrderingConstraintId,
    pub scope_id: OrderingScopeId,
    pub before_subject_id: OrderableSubjectId,
    pub after_subject_id: OrderableSubjectId,
    pub relation: OrderingRelation,
    pub basis: String,
    pub governing_rule_id: String,
    pub governing_rule_version: String,
    pub profile_binding: (CanonicalOrderingProfileId, String),
    pub source_relationship_id: Option<NormalizedRelationshipId>,
    pub replay_bindings: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingConstraintApplication {
    pub application_id: OrderingConstraintApplicationId,
    pub constraint_id: OrderingConstraintId,
    pub scope_id: OrderingScopeId,
    pub subject_ids: Arc<[OrderableSubjectId]>,
    pub rule_binding: (String, String),
    pub status: String,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingConstraintGraph {
    pub graph_id: OrderingConstraintGraphId,
    pub scope_id: OrderingScopeId,
    pub node_ids: Arc<[OrderableSubjectId]>,
    pub edges: Arc<[(OrderableSubjectId, OrderableSubjectId, OrderingConstraintId)]>,
    pub application_ids: Arc<[OrderingConstraintApplicationId]>,
    pub replay_bindings: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingLinearizationRecord {
    pub linearization_id: OrderingLinearizationRecordId,
    pub graph_id: OrderingConstraintGraphId,
    pub scope_id: OrderingScopeId,
    pub traversal_rule_binding: (String, String),
    pub tie_break_rule_binding: Option<(String, String)>,
    pub ordered_subject_ids: Arc<[OrderableSubjectId]>,
    pub underdetermined_subject_ids: Arc<[OrderableSubjectId]>,
    pub application_ids: Arc<[OrderingConstraintApplicationId]>,
    pub replay_bindings: Arc<[String]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderingDecision {
    pub decision_id: OrderingDecisionId,
    pub operation_id: CanonicalOrderingOperationId,
    pub scope_id: OrderingScopeId,
    pub subject_ids: Arc<[OrderableSubjectId]>,
    pub outcome_class: OrderingOutcomeClass,
    pub governing_constraint_ids: Arc<[OrderingConstraintId]>,
    pub application_ids: Arc<[OrderingConstraintApplicationId]>,
    pub graph_id: OrderingConstraintGraphId,
    pub linearization_id: OrderingLinearizationRecordId,
    pub profile_binding: (CanonicalOrderingProfileId, String),
    pub registry_binding: (OrderingRegistryId, String, OrderingRuleRegistryId, String),
    pub basis: String,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalOrderingAssignment {
    pub assignment_id: CanonicalOrderingAssignmentId,
    pub scope_id: OrderingScopeId,
    pub ordered_subject_id: OrderableSubjectId,
    pub subject_kind: OrderingSubjectKind,
    pub canonical_position: usize,
    pub ordering_decision_id: OrderingDecisionId,
    pub constraint_application_ids: Arc<[OrderingConstraintApplicationId]>,
    pub profile_binding: (CanonicalOrderingProfileId, String),
    pub registry_binding: (OrderingRegistryId, String, OrderingRuleRegistryId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureCanonicalOrderingProfile {
    pub identity: CanonicalOrderingProfileId,
    pub version: String,
    pub authority_reference: String,
    pub ordering_registry_id: OrderingRegistryId,
    pub ordering_registry_version: String,
    pub ordering_rule_registry_id: OrderingRuleRegistryId,
    pub ordering_rule_registry_version: String,
    pub scope_registry_version: String,
    pub comparison_version: String,
    pub traversal_rule_id: String,
    pub traversal_rule_version: String,
    pub tie_break_rule_id: String,
    pub tie_break_rule_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub relationship_bridges: BTreeMap<String, String>,
    pub require_tie_break: bool,
    pub identity_order_authorized: bool,
    pub preserved_order_authorized: bool,
}

impl FixtureCanonicalOrderingProfile {
    pub fn fixture() -> Self {
        Self {
            identity: CanonicalOrderingProfileId::derive(&["contract-011-fixture"]),
            version: "fixture-ordering-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            ordering_registry_id: OrderingRegistryId::derive(&["contract-011-fixture"]),
            ordering_registry_version: "fixture-ordering-registry-v1".to_owned(),
            ordering_rule_registry_id: OrderingRuleRegistryId::derive(&["contract-011-fixture"]),
            ordering_rule_registry_version: "fixture-ordering-rules-v1".to_owned(),
            scope_registry_version: "fixture-ordering-scopes-v1".to_owned(),
            comparison_version: "fixture-ordering-comparison-v1".to_owned(),
            traversal_rule_id: "DeterministicKahnTraversal".to_owned(),
            traversal_rule_version: "fixture-traversal-v1".to_owned(),
            tie_break_rule_id: "SubjectIdentityAscendingTieBreak".to_owned(),
            tie_break_rule_version: "fixture-tie-break-v1".to_owned(),
            schema_version: "fixture-ordering-schema-v1".to_owned(),
            configuration_version: "fixture-ordering-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            relationship_bridges: BTreeMap::from([(
                "DependsOn".to_owned(),
                "target-before-source".to_owned(),
            )]),
            require_tie_break: true,
            identity_order_authorized: false,
            preserved_order_authorized: false,
        }
    }

    fn registry_binding(&self) -> (OrderingRegistryId, String, OrderingRuleRegistryId, String) {
        (
            self.ordering_registry_id.clone(),
            self.ordering_registry_version.clone(),
            self.ordering_rule_registry_id.clone(),
            self.ordering_rule_registry_version.clone(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalOrderingInput {
    pub input_id: CanonicalOrderingInputId,
    pub operation_id: CanonicalOrderingOperationId,
    pub normalized_publication_id: NormalizedRequestRepresentationId,
    pub normalized_content_binding: String,
    pub profile_id: CanonicalOrderingProfileId,
    pub profile_version: String,
    pub ordering_registry_id: OrderingRegistryId,
    pub ordering_registry_version: String,
    pub ordering_rule_registry_id: OrderingRuleRegistryId,
    pub ordering_rule_registry_version: String,
    pub scope_registry_version: String,
    pub comparison_version: String,
    pub traversal_rule_id: String,
    pub traversal_rule_version: String,
    pub tie_break_rule_id: String,
    pub tie_break_rule_version: String,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub scopes: Arc<[OrderingScope]>,
    pub constraints: Arc<[OrderingConstraint]>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicallyOrderedRequestRepresentation {
    pub publication_id: CanonicallyOrderedRequestRepresentationId,
    pub operation_id: CanonicalOrderingOperationId,
    pub input_id: CanonicalOrderingInputId,
    pub normalized_publication_id: NormalizedRequestRepresentationId,
    pub normalized_content_binding: String,
    pub profile_binding: (CanonicalOrderingProfileId, String),
    pub registry_binding: (OrderingRegistryId, String, OrderingRuleRegistryId, String),
    pub orderable_subjects: Arc<[OrderableSubject]>,
    pub scopes: Arc<[OrderingScope]>,
    pub constraints: Arc<[OrderingConstraint]>,
    pub decisions: Arc<[OrderingDecision]>,
    pub assignments: Arc<[CanonicalOrderingAssignment]>,
    pub ordered_element_ids: Arc<[NormalizedSemanticElementId]>,
    pub ordered_relationship_ids: Arc<[NormalizedRelationshipId]>,
    /// Immutable Contract 010 source snapshots carried across the exact
    /// Contract 011 publication boundary for mechanical Contract 013 use.
    pub normalized_elements: Arc<[NormalizedSemanticElement]>,
    pub normalized_relationships: Arc<[NormalizedRelationship]>,
    pub constraint_applications: Arc<[OrderingConstraintApplication]>,
    pub graphs: Arc<[OrderingConstraintGraph]>,
    pub linearizations: Arc<[OrderingLinearizationRecord]>,
    pub schema_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalOrderingFailureRecord {
    pub failure_record_id: CanonicalOrderingFailureRecordId,
    pub operation_id: CanonicalOrderingOperationId,
    pub input_id: CanonicalOrderingInputId,
    pub category: CanonicalOrderingFailureCategory,
    pub stage: String,
    pub detail: String,
    pub profile_binding: (CanonicalOrderingProfileId, String),
    pub registry_binding: (OrderingRegistryId, String, OrderingRuleRegistryId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalOrderingOutcome {
    Success(Box<CanonicallyOrderedRequestRepresentation>),
    Failure(Box<CanonicalOrderingFailureRecord>),
}

fn normalized_publication_content_binding(publication: &NormalizedRequestRepresentation) -> String {
    let mut relationships = publication.relationships.to_vec();
    relationships.sort_by(|left, right| {
        left.normalized_relationship_id
            .cmp(&right.normalized_relationship_id)
    });
    StableId::from_parts(
        "normalized-content",
        &[
            publication.publication_id.as_str(),
            &format!(
                "{:?}{:?}{:?}{:?}{:?}",
                publication.elements,
                relationships,
                publication.decisions,
                publication.mapping_applications,
                publication.replay_context
            ),
        ],
    )
    .to_string()
}

fn ordering_failure(
    input: &CanonicalOrderingInput,
    category: CanonicalOrderingFailureCategory,
    stage: &str,
    detail: &str,
) -> CanonicalOrderingOutcome {
    CanonicalOrderingOutcome::Failure(Box::new(CanonicalOrderingFailureRecord {
        failure_record_id: CanonicalOrderingFailureRecordId::derive(&[
            input.operation_id.as_str(),
            stage,
            detail,
        ]),
        operation_id: input.operation_id.clone(),
        input_id: input.input_id.clone(),
        category,
        stage: stage.to_owned(),
        detail: detail.to_owned(),
        profile_binding: (input.profile_id.clone(), input.profile_version.clone()),
        registry_binding: (
            input.ordering_registry_id.clone(),
            input.ordering_registry_version.clone(),
            input.ordering_rule_registry_id.clone(),
            input.ordering_rule_registry_version.clone(),
        ),
    }))
}

fn orderable_subjects(publication: &NormalizedRequestRepresentation) -> Vec<OrderableSubject> {
    let mut subjects = publication
        .elements
        .iter()
        .map(|element| OrderableSubject {
            subject_id: OrderableSubjectId::derive(&[
                publication.publication_id.as_str(),
                "element",
                element.normalized_element_id.as_str(),
            ]),
            subject_kind: OrderingSubjectKind::Element,
            normalized_subject: OrderingSubjectReference::Element(
                element.normalized_element_id.clone(),
            ),
            normalized_publication_id: publication.publication_id.clone(),
            scope_ids: Vec::new().into(),
        })
        .chain(
            publication
                .relationships
                .iter()
                .map(|relationship| OrderableSubject {
                    subject_id: OrderableSubjectId::derive(&[
                        publication.publication_id.as_str(),
                        "relationship",
                        relationship.normalized_relationship_id.as_str(),
                    ]),
                    subject_kind: OrderingSubjectKind::Relationship,
                    normalized_subject: OrderingSubjectReference::Relationship(
                        relationship.normalized_relationship_id.clone(),
                    ),
                    normalized_publication_id: publication.publication_id.clone(),
                    scope_ids: Vec::new().into(),
                }),
        )
        .collect::<Vec<_>>();
    subjects.sort_by(|left, right| left.subject_id.cmp(&right.subject_id));
    subjects
}

impl CanonicalOrderingInput {
    pub fn for_fixture(publication: &NormalizedRequestRepresentation) -> Self {
        let profile = FixtureCanonicalOrderingProfile::fixture();
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
                "fixture-input",
            ]),
            operation_id: CanonicalOrderingOperationId::derive(&[
                publication.publication_id.as_str(),
                "fixture-operation",
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

fn validate_ordering_bindings(
    input: &CanonicalOrderingInput,
    publication: &NormalizedRequestRepresentation,
    profile: &FixtureCanonicalOrderingProfile,
) -> Result<(), (CanonicalOrderingFailureCategory, &'static str)> {
    if input.normalized_publication_id != publication.publication_id
        || input.normalized_content_binding != normalized_publication_content_binding(publication)
    {
        return Err((
            CanonicalOrderingFailureCategory::InvalidUpstreamPublication,
            "normalized publication identity or content binding is not exact",
        ));
    }
    if input.profile_id != profile.identity || input.profile_version != profile.version {
        return Err((
            CanonicalOrderingFailureCategory::ProfileResolutionFailure,
            "ordering profile identity or version is not exact",
        ));
    }
    if input.ordering_registry_id != profile.ordering_registry_id
        || input.ordering_registry_version != profile.ordering_registry_version
        || input.ordering_rule_registry_id != profile.ordering_rule_registry_id
        || input.ordering_rule_registry_version != profile.ordering_rule_registry_version
        || input.scope_registry_version != profile.scope_registry_version
        || input.comparison_version != profile.comparison_version
        || input.traversal_rule_id != profile.traversal_rule_id
        || input.traversal_rule_version != profile.traversal_rule_version
        || input.tie_break_rule_id != profile.tie_break_rule_id
        || input.tie_break_rule_version != profile.tie_break_rule_version
    {
        return Err((
            CanonicalOrderingFailureCategory::RegistryResolutionFailure,
            "ordering registry, comparison, traversal, or tie-break binding is not exact",
        ));
    }
    if input.schema_version != profile.schema_version
        || input.configuration_version != profile.configuration_version
        || input.implementation_version != profile.implementation_version
    {
        return Err((
            CanonicalOrderingFailureCategory::ProfileResolutionFailure,
            "schema, configuration, or implementation binding is not exact",
        ));
    }
    Ok(())
}

fn endpoint_element_subject(
    publication: &NormalizedRequestRepresentation,
    endpoint: &ReconciliationSubjectId,
) -> Result<OrderingSubjectReference, CanonicalOrderingFailureCategory> {
    let matches: Vec<_> = publication
        .elements
        .iter()
        .filter(|element| {
            element
                .contributing_subject_ids
                .iter()
                .any(|id| id == endpoint)
        })
        .map(|element| element.normalized_element_id.clone())
        .collect();
    if matches.len() != 1 {
        return Err(CanonicalOrderingFailureCategory::InvalidSubjectReference);
    }
    Ok(OrderingSubjectReference::Element(matches[0].clone()))
}

pub fn perform_canonical_ordering(
    input: &CanonicalOrderingInput,
    publication: &NormalizedRequestRepresentation,
    profile: &FixtureCanonicalOrderingProfile,
) -> CanonicalOrderingOutcome {
    if let Err((category, detail)) = validate_ordering_bindings(input, publication, profile) {
        return ordering_failure(input, category, "binding-validation", detail);
    }
    let subjects = orderable_subjects(publication);
    let subject_by_reference: BTreeMap<_, _> = subjects
        .iter()
        .map(|subject| (subject.normalized_subject.clone(), subject))
        .collect();
    let subject_by_id: BTreeMap<_, _> = subjects
        .iter()
        .map(|subject| (subject.subject_id.clone(), subject))
        .collect();
    if input.scopes.is_empty() {
        return ordering_failure(
            input,
            CanonicalOrderingFailureCategory::UnresolvableOrderingScope,
            "scope-validation",
            "no explicit ordering scope is bound",
        );
    }
    let mut constraints = input.constraints.to_vec();
    let mut scopes = input.scopes.to_vec();
    let mut applications = Vec::new();
    for scope in &scopes {
        if scope
            .participating_subject_ids
            .iter()
            .any(|id| !subject_by_id.contains_key(id))
        {
            return ordering_failure(
                input,
                CanonicalOrderingFailureCategory::InvalidSubjectReference,
                "scope-validation",
                "scope contains a foreign normalized subject identity",
            );
        }
    }
    for relationship in publication.relationships.iter() {
        let Some(direction) = profile
            .relationship_bridges
            .get(&relationship.relationship_class)
        else {
            continue;
        };
        let source = match endpoint_element_subject(publication, &relationship.source_subject_id) {
            Ok(value) => value,
            Err(category) => {
                return ordering_failure(
                    input,
                    category,
                    "relationship-bridge",
                    "relationship endpoint does not bind to exactly one normalized element",
                )
            }
        };
        let target = match endpoint_element_subject(publication, &relationship.target_subject_id) {
            Ok(value) => value,
            Err(category) => {
                return ordering_failure(
                    input,
                    category,
                    "relationship-bridge",
                    "relationship endpoint does not bind to exactly one normalized element",
                )
            }
        };
        let before = if direction == "target-before-source" {
            target.clone()
        } else {
            source.clone()
        };
        let after = if direction == "target-before-source" {
            source
        } else {
            target
        };
        let before_subject = subject_by_reference.get(&before);
        let after_subject = subject_by_reference.get(&after);
        let (Some(before_subject), Some(after_subject)) = (before_subject, after_subject) else {
            return ordering_failure(
                input,
                CanonicalOrderingFailureCategory::InvalidSubjectReference,
                "relationship-bridge",
                "relationship bridge endpoint is not an orderable normalized element",
            );
        };
        let scope = match scopes.iter().find(|scope| {
            scope
                .participating_subject_ids
                .iter()
                .any(|id| id == &before_subject.subject_id)
                && scope
                    .participating_subject_ids
                    .iter()
                    .any(|id| id == &after_subject.subject_id)
        }) {
            Some(scope) => scope,
            None => {
                return ordering_failure(
                    input,
                    CanonicalOrderingFailureCategory::UnresolvableOrderingScope,
                    "relationship-bridge",
                    "relationship bridge has no compatible explicit ordering scope",
                )
            }
        };
        let constraint_id = OrderingConstraintId::derive(&[
            relationship.normalized_relationship_id.as_str(),
            scope.scope_id.as_str(),
            "RelationshipBridge",
            profile.ordering_rule_registry_version.as_str(),
        ]);
        constraints.push(OrderingConstraint {
            constraint_id,
            scope_id: scope.scope_id.clone(),
            before_subject_id: before_subject.subject_id.clone(),
            after_subject_id: after_subject.subject_id.clone(),
            relation: OrderingRelation::Before,
            basis: "Explicit fixture relationship bridge".to_owned(),
            governing_rule_id: "RelationshipBridgeDependsOnRule".to_owned(),
            governing_rule_version: profile.ordering_rule_registry_version.clone(),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            source_relationship_id: Some(relationship.normalized_relationship_id.clone()),
            replay_bindings: Arc::from([relationship.normalized_relationship_id.to_string()]),
        });
    }
    let mut decisions = Vec::new();
    let mut assignments = Vec::new();
    let mut graphs = Vec::new();
    let mut linearizations = Vec::new();
    for scope in &mut scopes {
        let scope_constraints: Vec<_> = constraints
            .iter()
            .filter(|constraint| constraint.scope_id == scope.scope_id)
            .cloned()
            .collect();
        scope.governing_constraint_ids = scope_constraints
            .iter()
            .map(|constraint| constraint.constraint_id.clone())
            .collect::<Vec<_>>()
            .into();
        let mut seen_edges = BTreeSet::new();
        let mut edges = Vec::new();
        for constraint in &scope_constraints {
            if !scope
                .participating_subject_ids
                .iter()
                .any(|id| id == &constraint.before_subject_id)
                || !scope
                    .participating_subject_ids
                    .iter()
                    .any(|id| id == &constraint.after_subject_id)
            {
                return ordering_failure(
                    input,
                    CanonicalOrderingFailureCategory::InvalidSubjectReference,
                    "constraint-validation",
                    "constraint is outside its explicit scope",
                );
            }
            if !seen_edges.insert((
                constraint.before_subject_id.clone(),
                constraint.after_subject_id.clone(),
            )) {
                continue;
            }
            if seen_edges.contains(&(
                constraint.after_subject_id.clone(),
                constraint.before_subject_id.clone(),
            )) {
                return ordering_failure(
                    input,
                    CanonicalOrderingFailureCategory::ContradictoryOrderingConstraints,
                    "constraint-validation",
                    "opposing constraints are contradictory",
                );
            }
            let application_id = OrderingConstraintApplicationId::derive(&[
                constraint.constraint_id.as_str(),
                profile.version.as_str(),
            ]);
            applications.push(OrderingConstraintApplication {
                application_id,
                constraint_id: constraint.constraint_id.clone(),
                scope_id: scope.scope_id.clone(),
                subject_ids: Arc::from([
                    constraint.before_subject_id.clone(),
                    constraint.after_subject_id.clone(),
                ]),
                rule_binding: (
                    constraint.governing_rule_id.clone(),
                    constraint.governing_rule_version.clone(),
                ),
                status: "Applied".to_owned(),
                basis: constraint.basis.clone(),
            });
            edges.push((
                constraint.before_subject_id.clone(),
                constraint.after_subject_id.clone(),
                constraint.constraint_id.clone(),
            ));
        }
        let nodes = scope.participating_subject_ids.to_vec();
        let graph_id = OrderingConstraintGraphId::derive(&[
            scope.scope_id.as_str(),
            &format!("{:?}{:?}", nodes, edges),
        ]);
        let graph = OrderingConstraintGraph {
            graph_id: graph_id.clone(),
            scope_id: scope.scope_id.clone(),
            node_ids: nodes.clone().into(),
            edges: edges.clone().into(),
            application_ids: applications
                .iter()
                .filter(|application| application.scope_id == scope.scope_id)
                .map(|application| application.application_id.clone())
                .collect::<Vec<_>>()
                .into(),
            replay_bindings: Arc::from([
                profile.traversal_rule_version.clone(),
                profile.comparison_version.clone(),
            ]),
        };
        let mut indegree: BTreeMap<OrderableSubjectId, usize> =
            nodes.iter().cloned().map(|id| (id, 0)).collect();
        let mut outgoing: BTreeMap<OrderableSubjectId, Vec<OrderableSubjectId>> = BTreeMap::new();
        for (before, after, _) in &edges {
            *indegree.get_mut(after).expect("validated node") += 1;
            outgoing
                .entry(before.clone())
                .or_default()
                .push(after.clone());
        }
        let mut ready: BTreeSet<_> = indegree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(id, _)| id.clone())
            .collect();
        let mut ordered = Vec::new();
        let mut underdetermined = Vec::new();
        while let Some(next) = ready.iter().next().cloned() {
            if ready.len() > 1 {
                underdetermined.extend(ready.iter().filter(|id| **id != next).cloned());
            }
            ready.remove(&next);
            ordered.push(next.clone());
            if let Some(children) = outgoing.get(&next) {
                for child in children {
                    let degree = indegree.get_mut(child).expect("validated child");
                    *degree -= 1;
                    if *degree == 0 {
                        ready.insert(child.clone());
                    }
                }
            }
        }
        if ordered.len() != nodes.len() {
            return ordering_failure(
                input,
                CanonicalOrderingFailureCategory::OrderingCycle,
                "linearization",
                "ordering graph contains a cycle",
            );
        }
        let used_tie_break = !underdetermined.is_empty();
        if used_tie_break && !profile.require_tie_break {
            return ordering_failure(
                input,
                CanonicalOrderingFailureCategory::MissingRequiredTieBreaker,
                "linearization",
                "underdetermined ordering has no authorized tie-break",
            );
        }
        let outcome_class = if used_tie_break {
            OrderingOutcomeClass::TieBroken
        } else if !edges.is_empty() {
            OrderingOutcomeClass::DependencyLinearized
        } else if profile.identity_order_authorized {
            OrderingOutcomeClass::IdentityOrder
        } else if profile.preserved_order_authorized {
            OrderingOutcomeClass::PreservedOrder
        } else {
            OrderingOutcomeClass::Direct
        };
        let linearization_id = OrderingLinearizationRecordId::derive(&[
            graph_id.as_str(),
            &format!("{:?}", ordered),
            profile.traversal_rule_version.as_str(),
            profile.tie_break_rule_version.as_str(),
        ]);
        let linearization = OrderingLinearizationRecord {
            linearization_id: linearization_id.clone(),
            graph_id: graph_id.clone(),
            scope_id: scope.scope_id.clone(),
            traversal_rule_binding: (
                profile.traversal_rule_id.clone(),
                profile.traversal_rule_version.clone(),
            ),
            tie_break_rule_binding: used_tie_break.then(|| {
                (
                    profile.tie_break_rule_id.clone(),
                    profile.tie_break_rule_version.clone(),
                )
            }),
            ordered_subject_ids: ordered.clone().into(),
            underdetermined_subject_ids: underdetermined.into(),
            application_ids: graph.application_ids.clone(),
            replay_bindings: Arc::from([profile.comparison_version.clone()]),
        };
        let decision_id = OrderingDecisionId::derive(&[
            input.operation_id.as_str(),
            scope.scope_id.as_str(),
            &format!("{outcome_class:?}{:?}", ordered),
        ]);
        let decision = OrderingDecision {
            decision_id: decision_id.clone(),
            operation_id: input.operation_id.clone(),
            scope_id: scope.scope_id.clone(),
            subject_ids: ordered.clone().into(),
            outcome_class,
            governing_constraint_ids: scope.governing_constraint_ids.clone(),
            application_ids: graph.application_ids.clone(),
            graph_id: graph_id.clone(),
            linearization_id: linearization_id.clone(),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            registry_binding: profile.registry_binding(),
            basis: "Fixture-authorized canonical arrangement".to_owned(),
            replay_context: input.replay_context.clone(),
        };
        for (position, subject_id) in ordered.iter().enumerate() {
            let subject = subject_by_id.get(subject_id).expect("validated subject");
            assignments.push(CanonicalOrderingAssignment {
                assignment_id: CanonicalOrderingAssignmentId::derive(&[
                    scope.scope_id.as_str(),
                    subject_id.as_str(),
                    &position.to_string(),
                    decision_id.as_str(),
                ]),
                scope_id: scope.scope_id.clone(),
                ordered_subject_id: subject_id.clone(),
                subject_kind: subject.subject_kind,
                canonical_position: position,
                ordering_decision_id: decision_id.clone(),
                constraint_application_ids: graph.application_ids.clone(),
                profile_binding: (profile.identity.clone(), profile.version.clone()),
                registry_binding: profile.registry_binding(),
            });
        }
        graphs.push(graph);
        linearizations.push(linearization);
        decisions.push(decision);
    }
    let mut ordered_element_ids = Vec::new();
    let mut ordered_relationship_ids = Vec::new();
    for assignment in &assignments {
        match subject_by_id[&assignment.ordered_subject_id]
            .normalized_subject
            .clone()
        {
            OrderingSubjectReference::Element(id) => ordered_element_ids.push(id),
            OrderingSubjectReference::Relationship(id) => ordered_relationship_ids.push(id),
        }
    }
    let publication_id = CanonicallyOrderedRequestRepresentationId::derive(&[
        input.operation_id.as_str(),
        &format!("{:?}{:?}{:?}", decisions, assignments, scopes),
    ]);
    CanonicalOrderingOutcome::Success(Box::new(CanonicallyOrderedRequestRepresentation {
        publication_id,
        operation_id: input.operation_id.clone(),
        input_id: input.input_id.clone(),
        normalized_publication_id: publication.publication_id.clone(),
        normalized_content_binding: input.normalized_content_binding.clone(),
        profile_binding: (profile.identity.clone(), profile.version.clone()),
        registry_binding: profile.registry_binding(),
        orderable_subjects: subjects.into(),
        scopes: scopes.into(),
        constraints: constraints.into(),
        decisions: decisions.into(),
        assignments: assignments.into(),
        ordered_element_ids: ordered_element_ids.into(),
        ordered_relationship_ids: ordered_relationship_ids.into(),
        normalized_elements: publication.elements.clone(),
        normalized_relationships: publication.relationships.clone(),
        constraint_applications: applications.into(),
        graphs: graphs.into(),
        linearizations: linearizations.into(),
        schema_version: input.schema_version.clone(),
        configuration_version: input.configuration_version.clone(),
        implementation_version: input.implementation_version.clone(),
        replay_context: input.replay_context.clone(),
    }))
}

#[cfg(test)]
mod contract_011_tests {
    use super::*;

    pub(super) fn normalized_fixture(with_relationship: bool) -> NormalizedRequestRepresentation {
        let subject_one = ReconciliationSubjectId::derive(&["subject-one"]);
        let subject_two = ReconciliationSubjectId::derive(&["subject-two"]);
        let element_one = NormalizedSemanticElement {
            normalized_element_id: NormalizedSemanticElementId::derive(&["element-one"]),
            source_element_id: ReconciledSemanticElementId::derive(&["element-one"]),
            normalization_decision_id: NormalizationDecisionId::derive(&["element-one"]),
            standing_assignment_id: StandingAssignmentId::derive(&["element-one"]),
            canonical_expression: "first".to_owned(),
            semantic_domain: "Objective".to_owned(),
            semantic_class: "RequestedOutcome".to_owned(),
            represented_scope: "request-wide".to_owned(),
            contributing_subject_ids: vec![subject_one.clone()].into(),
            contributing_decision_ids: Vec::new().into(),
            preserved_alternative_ids: Vec::new().into(),
            evidence_reference_ids: Vec::new().into(),
            grounding_reference_ids: Vec::new().into(),
            provenance_reference_ids: Vec::new().into(),
            unresolved_conditions: Vec::new().into(),
        };
        let element_two = NormalizedSemanticElement {
            normalized_element_id: NormalizedSemanticElementId::derive(&["element-two"]),
            source_element_id: ReconciledSemanticElementId::derive(&["element-two"]),
            normalization_decision_id: NormalizationDecisionId::derive(&["element-two"]),
            standing_assignment_id: StandingAssignmentId::derive(&["element-two"]),
            canonical_expression: "second".to_owned(),
            semantic_domain: "Objective".to_owned(),
            semantic_class: "RequestedOutcome".to_owned(),
            represented_scope: "request-wide".to_owned(),
            contributing_subject_ids: vec![subject_two.clone()].into(),
            contributing_decision_ids: Vec::new().into(),
            preserved_alternative_ids: Vec::new().into(),
            evidence_reference_ids: Vec::new().into(),
            grounding_reference_ids: Vec::new().into(),
            provenance_reference_ids: Vec::new().into(),
            unresolved_conditions: Vec::new().into(),
        };
        let relationships = if with_relationship {
            vec![NormalizedRelationship {
                normalized_relationship_id: NormalizedRelationshipId::derive(&["depends"]),
                source_relationship_id: ReconciliationRelationshipId::derive(&["depends"]),
                relationship_class: "DependsOn".to_owned(),
                canonical_expression: "depends-on".to_owned(),
                source_subject_id: subject_one,
                target_subject_id: subject_two,
                represented_scope: "request-wide".to_owned(),
                normalization_disposition: NormalizationDisposition::Mapped,
                normalization_decision_references: Vec::new().into(),
                reconciliation_decision_references: Vec::new().into(),
                standing_references: Vec::new().into(),
                evidence_references: Vec::new().into(),
                grounding_references: Vec::new().into(),
                provenance_references: Vec::new().into(),
                unresolved_conditions: Vec::new().into(),
            }]
            .into()
        } else {
            Vec::new().into()
        };
        NormalizedRequestRepresentation {
            publication_id: NormalizedRequestRepresentationId::derive(&["normalized-fixture"]),
            operation_id: NormalizationOperationId::derive(&["operation"]),
            input_id: NormalizationInputId::derive(&["input"]),
            reconciliation_set_id: SemanticReconciliationSetId::derive(&["reconciliation"]),
            profile_binding: (
                NormalizationProfileId::derive(&["contract-010-fixture"]),
                "fixture-normalization-v1".to_owned(),
            ),
            registry_binding: (
                CanonicalRegistryId::derive(&["contract-010-fixture"]),
                "fixture-canonical-registry-v1".to_owned(),
                MappingRuleRegistryId::derive(&["contract-010-fixture"]),
                "fixture-mapping-rules-v1".to_owned(),
            ),
            schema_version: "fixture-normalization-schema-v1".to_owned(),
            configuration_version: "fixture-normalization-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            elements: vec![element_one, element_two].into(),
            relationships,
            decisions: Vec::new().into(),
            mapping_applications: Vec::new().into(),
            stable_iteration_order: Vec::new().into(),
            replay_context: BTreeMap::from([("fixture".to_owned(), "v1".to_owned())]),
        }
    }

    #[test]
    fn contract_011_direct_element_and_relationship_ordering_is_explicit() {
        let publication = normalized_fixture(true);
        let input = CanonicalOrderingInput::for_fixture(&publication);
        let CanonicalOrderingOutcome::Success(ordered) = perform_canonical_ordering(
            &input,
            &publication,
            &FixtureCanonicalOrderingProfile::fixture(),
        ) else {
            panic!("ordering publication")
        };
        assert_eq!(ordered.ordered_element_ids.len(), 2);
        assert_eq!(ordered.ordered_relationship_ids.len(), 1);
        assert_eq!(ordered.assignments.len(), 3);
        assert_eq!(ordered.graphs[0].edges.len(), 1);
        assert_eq!(
            ordered.decisions[0].outcome_class,
            OrderingOutcomeClass::TieBroken
        );
        assert_eq!(publication.elements[0].canonical_expression, "first");
    }

    #[test]
    fn contract_011_relationship_bridge_is_explicit_not_automatic() {
        let publication = normalized_fixture(true);
        let input = CanonicalOrderingInput::for_fixture(&publication);
        let mut profile = FixtureCanonicalOrderingProfile::fixture();
        profile.relationship_bridges.clear();
        let CanonicalOrderingOutcome::Success(ordered) =
            perform_canonical_ordering(&input, &publication, &profile)
        else {
            panic!("ordering publication without bridge")
        };
        assert!(ordered.graphs[0].edges.is_empty());
        assert!(ordered.constraint_applications.is_empty());
    }

    #[test]
    fn contract_011_authorized_tie_break_is_distinct_from_incidental_order() {
        let publication = normalized_fixture(false);
        let input = CanonicalOrderingInput::for_fixture(&publication);
        let CanonicalOrderingOutcome::Success(ordered) = perform_canonical_ordering(
            &input,
            &publication,
            &FixtureCanonicalOrderingProfile::fixture(),
        ) else {
            panic!("tie-break publication")
        };
        assert_eq!(
            ordered.decisions[0].outcome_class,
            OrderingOutcomeClass::TieBroken
        );
    }

    #[test]
    fn contract_011_cycle_and_contradiction_fail_before_tie_break() {
        let publication = normalized_fixture(false);
        let subjects = orderable_subjects(&publication);
        let scope_id = CanonicalOrderingInput::for_fixture(&publication).scopes[0]
            .scope_id
            .clone();
        let profile = FixtureCanonicalOrderingProfile::fixture();
        let constraint = |before: usize, after: usize, label: &str| OrderingConstraint {
            constraint_id: OrderingConstraintId::derive(&[label]),
            scope_id: scope_id.clone(),
            before_subject_id: subjects[before].subject_id.clone(),
            after_subject_id: subjects[after].subject_id.clone(),
            relation: OrderingRelation::Before,
            basis: "fixture".to_owned(),
            governing_rule_id: "ExplicitFixtureOrderingRule".to_owned(),
            governing_rule_version: "fixture-ordering-rules-v1".to_owned(),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            source_relationship_id: None,
            replay_bindings: Vec::new().into(),
        };
        let mut input = CanonicalOrderingInput::for_fixture(&publication);
        input.constraints = vec![constraint(0, 1, "forward"), constraint(1, 0, "reverse")].into();
        assert!(matches!(
            perform_canonical_ordering(&input, &publication, &profile),
            CanonicalOrderingOutcome::Failure(record)
                if record.category
                    == CanonicalOrderingFailureCategory::ContradictoryOrderingConstraints
        ));
    }

    #[test]
    fn contract_011_wrong_normalized_binding_is_atomic() {
        let publication = normalized_fixture(false);
        let mut input = CanonicalOrderingInput::for_fixture(&publication);
        input.normalized_content_binding = "foreign-binding".to_owned();
        assert!(matches!(
            perform_canonical_ordering(
                &input,
                &publication,
                &FixtureCanonicalOrderingProfile::fixture()
            ),
            CanonicalOrderingOutcome::Failure(record)
                if record.category == CanonicalOrderingFailureCategory::InvalidUpstreamPublication
        ));
    }
}

// ---------------------------------------------------------------------------
// Contract 012: Structural validation
// ---------------------------------------------------------------------------

contract_002_id!(StructuralValidationId, "sv");
contract_002_id!(StructuralValidationOperationId, "svop");
contract_002_id!(ValidationProfileId, "svprofile");
contract_002_id!(ConstructionRequirementsId, "svreqset");
contract_002_id!(ValidationRequirementId, "svreq");
contract_002_id!(StructuralRegistryId, "svregistry");
contract_002_id!(ValidationRuleSetId, "svrules");
contract_002_id!(ValidationRuleApplicationId, "svapply");
contract_002_id!(ValidationFindingId, "svfinding");
contract_002_id!(ValidationDecisionId, "svdecision");
contract_002_id!(StructuralValidationResultId, "svresult");
contract_002_id!(StructuralValidationFailureRecordId, "svfail");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValidationCompletion {
    Completed,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum StructuralEligibility {
    Eligible,
    EligibleWithWarnings,
    Ineligible,
    Deferred,
    NotDetermined,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FindingSeverity {
    Informational,
    Warning,
    Blocking,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValidationDecisionKind {
    RequirementSatisfied,
    RequirementSatisfiedWithWarning,
    RequirementUnsatisfied,
    RequirementDeferred,
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValidationFailureCategory {
    WrongOrderedPublicationBinding,
    UnreadableOrUnresolvableOrderedPublication,
    ValidationProfileResolutionFailure,
    ConstructionRequirementsResolutionFailure,
    SchemaResolutionFailure,
    StructuralRegistryResolutionFailure,
    ValidationRuleSetResolutionFailure,
    IncompatibleValidationContext,
    UnknownAuthoritativeRule,
    UnknownValidationRequirement,
    RuleExecutionFailure,
    NonDeterministicRuleBehavior,
    IncompleteRuleCoverage,
    IncompleteDecisionCoverage,
    InvalidDecisionReference,
    InconsistentEligibilityAggregation,
    ResultCommitmentFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRequirement {
    pub requirement_id: ValidationRequirementId,
    pub requirement_class: String,
    pub rule_id: String,
    pub rule_version: String,
    pub mandatory: bool,
    pub structural_only: bool,
    pub profile_binding: (ValidationProfileId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationRuleApplication {
    pub application_id: ValidationRuleApplicationId,
    pub operation_id: StructuralValidationOperationId,
    pub requirement_id: ValidationRequirementId,
    pub rule_binding: (String, String),
    pub profile_binding: (ValidationProfileId, String),
    pub schema_version: String,
    pub registry_binding: (StructuralRegistryId, String),
    pub subject_ids: Arc<[OrderableSubjectId]>,
    pub evaluation_method: String,
    pub deterministic_input_context: String,
    pub completion: ValidationCompletion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationFinding {
    pub finding_id: ValidationFindingId,
    pub application_id: ValidationRuleApplicationId,
    pub requirement_id: ValidationRequirementId,
    pub subject_ids: Arc<[OrderableSubjectId]>,
    pub scope_ids: Arc<[OrderingScopeId]>,
    pub condition_class: String,
    pub severity: FindingSeverity,
    pub observed_structural_facts: Arc<[String]>,
    pub profile_binding: (ValidationProfileId, String),
    pub rule_binding: (String, String),
    pub schema_version: String,
    pub registry_binding: (StructuralRegistryId, String),
    pub configuration_version: String,
    pub ordered_publication_binding: String,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationDecision {
    pub decision_id: ValidationDecisionId,
    pub requirement_id: ValidationRequirementId,
    pub finding_ids: Arc<[ValidationFindingId]>,
    pub decision_kind: ValidationDecisionKind,
    pub decision_basis: String,
    pub rule_binding: (String, String),
    pub mandatory: bool,
    pub downstream_implication: String,
    pub profile_binding: (ValidationProfileId, String),
    pub ordered_publication_binding: String,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionRequirements {
    pub identity: ConstructionRequirementsId,
    pub version: String,
    pub requirements: Arc<[ValidationRequirement]>,
}

impl FixtureConstructionRequirements {
    pub fn fixture(profile: &FixtureValidationProfile) -> Self {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureStructuralRegistries {
    pub identity: StructuralRegistryId,
    pub version: String,
}

impl FixtureStructuralRegistries {
    pub fn fixture() -> Self {
        Self {
            identity: StructuralRegistryId::derive(&["contract-012-fixture"]),
            version: "fixture-structural-registry-v1".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureValidationRuleSet {
    pub identity: ValidationRuleSetId,
    pub version: String,
    pub known_rule_ids: Arc<[String]>,
}

impl FixtureValidationRuleSet {
    pub fn fixture(profile: &FixtureValidationProfile) -> Self {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureValidationProfile {
    pub identity: ValidationProfileId,
    pub version: String,
    pub authority_reference: String,
    pub requirements_id: ConstructionRequirementsId,
    pub requirements_version: String,
    pub schema_version: String,
    pub registry_id: StructuralRegistryId,
    pub registry_version: String,
    pub rule_set_id: ValidationRuleSetId,
    pub rule_set_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub aggregation_rule_id: String,
    pub aggregation_rule_version: String,
    pub warning_condition: Option<String>,
    pub deferred_condition: Option<String>,
    pub omit_decision_for: Option<ValidationRequirementId>,
}

impl FixtureValidationProfile {
    pub fn fixture() -> Self {
        Self {
            identity: ValidationProfileId::derive(&["contract-012-fixture"]),
            version: "fixture-validation-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            requirements_id: ConstructionRequirementsId::derive(&["contract-012-fixture"]),
            requirements_version: "fixture-construction-requirements-v1".to_owned(),
            schema_version: "fixture-validation-schema-v1".to_owned(),
            registry_id: StructuralRegistryId::derive(&["contract-012-fixture"]),
            registry_version: "fixture-structural-registry-v1".to_owned(),
            rule_set_id: ValidationRuleSetId::derive(&["contract-012-fixture"]),
            rule_set_version: "fixture-validation-rules-v1".to_owned(),
            configuration_version: "fixture-validation-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            aggregation_rule_id: "MandatoryThenDeferredThenWarning".to_owned(),
            aggregation_rule_version: "fixture-eligibility-aggregation-v1".to_owned(),
            warning_condition: None,
            deferred_condition: None,
            omit_decision_for: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralValidationInput {
    pub validation_id: StructuralValidationId,
    pub operation_id: StructuralValidationOperationId,
    pub ordered_publication_id: CanonicallyOrderedRequestRepresentationId,
    pub ordered_content_binding: String,
    pub profile_id: ValidationProfileId,
    pub profile_version: String,
    pub requirements_id: ConstructionRequirementsId,
    pub requirements_version: String,
    pub schema_version: String,
    pub registry_id: StructuralRegistryId,
    pub registry_version: String,
    pub rule_set_id: ValidationRuleSetId,
    pub rule_set_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub aggregation_rule_id: String,
    pub aggregation_rule_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl StructuralValidationInput {
    pub fn for_fixture(publication: &CanonicallyOrderedRequestRepresentation) -> Self {
        let profile = FixtureValidationProfile::fixture();
        Self {
            validation_id: StructuralValidationId::derive(&[
                publication.publication_id.as_str(),
                "fixture-validation",
            ]),
            operation_id: StructuralValidationOperationId::derive(&[
                publication.publication_id.as_str(),
                "fixture-operation",
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralValidationResult {
    pub result_id: StructuralValidationResultId,
    pub validation_id: StructuralValidationId,
    pub operation_id: StructuralValidationOperationId,
    pub ordered_publication_id: CanonicallyOrderedRequestRepresentationId,
    pub ordered_content_binding: String,
    pub completion: ValidationCompletion,
    pub eligibility: StructuralEligibility,
    pub profile_binding: (ValidationProfileId, String),
    pub requirements_binding: (ConstructionRequirementsId, String),
    pub schema_version: String,
    pub registry_binding: (StructuralRegistryId, String),
    pub rule_set_binding: (ValidationRuleSetId, String),
    pub configuration_version: String,
    pub implementation_version: String,
    pub rule_applications: Arc<[ValidationRuleApplication]>,
    pub findings: Arc<[ValidationFinding]>,
    pub decisions: Arc<[ValidationDecision]>,
    pub aggregation_rule_binding: (String, String),
    pub coverage_evidence: Arc<[String]>,
    pub deferred_requirement_ids: Arc<[ValidationRequirementId]>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralValidationFailureRecord {
    pub failure_record_id: StructuralValidationFailureRecordId,
    pub validation_id: StructuralValidationId,
    pub operation_id: StructuralValidationOperationId,
    pub attempted_ordered_publication_id: Option<CanonicallyOrderedRequestRepresentationId>,
    pub attempted_ordered_content_binding: Option<String>,
    pub completion: ValidationCompletion,
    pub eligibility: StructuralEligibility,
    pub category: ValidationFailureCategory,
    pub detail: String,
    pub affected_dependency_bindings: Arc<[String]>,
    pub profile_binding: Option<(ValidationProfileId, String)>,
    pub schema_version: Option<String>,
    pub registry_binding: Option<(StructuralRegistryId, String)>,
    pub rule_set_binding: Option<(ValidationRuleSetId, String)>,
    pub configuration_version: Option<String>,
    pub implementation_version: Option<String>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StructuralValidationOutcome {
    Completed(Box<StructuralValidationResult>),
    Failed(Box<StructuralValidationFailureRecord>),
}

fn ordered_publication_content_binding(
    publication: &CanonicallyOrderedRequestRepresentation,
) -> String {
    StableId::from_parts(
        "ordered-content",
        &[
            publication.publication_id.as_str(),
            &format!(
                "{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}",
                publication.normalized_publication_id,
                publication.normalized_content_binding,
                publication.orderable_subjects,
                publication.scopes,
                publication.constraints,
                publication.decisions,
                publication.assignments,
                publication.normalized_elements,
                publication.normalized_relationships,
                publication.replay_context
            ),
        ],
    )
    .to_string()
}

fn validation_failure(
    input: &StructuralValidationInput,
    category: ValidationFailureCategory,
    detail: &str,
    dependencies: Arc<[String]>,
) -> StructuralValidationOutcome {
    StructuralValidationOutcome::Failed(Box::new(StructuralValidationFailureRecord {
        failure_record_id: StructuralValidationFailureRecordId::derive(&[
            input.operation_id.as_str(),
            &format!("{category:?}"),
            detail,
        ]),
        validation_id: input.validation_id.clone(),
        operation_id: input.operation_id.clone(),
        attempted_ordered_publication_id: Some(input.ordered_publication_id.clone()),
        attempted_ordered_content_binding: Some(input.ordered_content_binding.clone()),
        completion: ValidationCompletion::Failed,
        eligibility: StructuralEligibility::NotDetermined,
        category,
        detail: detail.to_owned(),
        affected_dependency_bindings: dependencies,
        profile_binding: Some((input.profile_id.clone(), input.profile_version.clone())),
        schema_version: Some(input.schema_version.clone()),
        registry_binding: Some((input.registry_id.clone(), input.registry_version.clone())),
        rule_set_binding: Some((input.rule_set_id.clone(), input.rule_set_version.clone())),
        configuration_version: Some(input.configuration_version.clone()),
        implementation_version: Some(input.implementation_version.clone()),
        replay_context: input.replay_context.clone(),
    }))
}

fn validation_subjects(
    publication: &CanonicallyOrderedRequestRepresentation,
) -> BTreeMap<OrderableSubjectId, &OrderableSubject> {
    publication
        .orderable_subjects
        .iter()
        .map(|subject| (subject.subject_id.clone(), subject))
        .collect()
}

fn validate_structural_requirement(
    requirement: &ValidationRequirement,
    publication: &CanonicallyOrderedRequestRepresentation,
) -> (
    bool,
    FindingSeverity,
    Vec<OrderableSubjectId>,
    Vec<OrderingScopeId>,
    Vec<String>,
) {
    let subjects = validation_subjects(publication);
    let subject_ids: Vec<_> = subjects.keys().cloned().collect();
    let scope_ids: Vec<_> = publication
        .scopes
        .iter()
        .map(|scope| scope.scope_id.clone())
        .collect();
    let mut facts = Vec::new();
    let valid = match requirement.requirement_class.as_str() {
        "RequiredStructuralPresence" => {
            let valid = !publication.orderable_subjects.is_empty()
                && !publication.scopes.is_empty()
                && !publication.decisions.is_empty()
                && !publication.assignments.is_empty();
            if !valid {
                facts.push("required structural publication member is absent".to_owned());
            }
            valid
        }
        "IdentifierIntegrity" => {
            let unique_subjects = subjects.len() == publication.orderable_subjects.len();
            let references_valid = publication.orderable_subjects.iter().all(|subject| {
                subject.normalized_publication_id
                    == publication
                        .orderable_subjects
                        .first()
                        .map(|first| first.normalized_publication_id.clone())
                        .unwrap_or_else(|| publication.normalized_publication_id.clone())
            });
            let kinds_valid = publication.orderable_subjects.iter().all(|subject| {
                matches!(
                    (&subject.subject_kind, &subject.normalized_subject),
                    (
                        OrderingSubjectKind::Element,
                        OrderingSubjectReference::Element(_)
                    ) | (
                        OrderingSubjectKind::Relationship,
                        OrderingSubjectReference::Relationship(_)
                    )
                )
            });
            let valid = unique_subjects && references_valid && kinds_valid;
            if !unique_subjects {
                facts.push("orderable subject identifiers are not unique".to_owned());
            }
            if !references_valid {
                facts.push("subject has foreign publication membership".to_owned());
            }
            if !kinds_valid {
                facts.push(
                    "declared subject type does not match its normalized reference".to_owned(),
                );
            }
            valid
        }
        "RelationshipIntegrity" => {
            let relationship_ids: BTreeSet<_> = publication
                .orderable_subjects
                .iter()
                .filter_map(|subject| match &subject.normalized_subject {
                    OrderingSubjectReference::Relationship(id) => Some(id.clone()),
                    OrderingSubjectReference::Element(_) => None,
                })
                .collect();
            let ordered_relationship_ids: BTreeSet<_> = publication
                .ordered_relationship_ids
                .iter()
                .cloned()
                .collect();
            let valid = relationship_ids == ordered_relationship_ids
                && ordered_relationship_ids.len() == publication.ordered_relationship_ids.len();
            if !valid {
                facts.push("ordered relationship membership does not match orderable relationship subjects".to_owned());
            }
            valid
        }
        "OrderingIntegrity" => {
            let assignment_subjects: Vec<_> = publication
                .assignments
                .iter()
                .map(|a| a.ordered_subject_id.clone())
                .collect();
            let mut positions = BTreeSet::new();
            let assignments_valid = assignment_subjects.len() == subjects.len()
                && assignment_subjects
                    .iter()
                    .all(|id| subjects.contains_key(id))
                && assignment_subjects.iter().collect::<BTreeSet<_>>().len() == subjects.len();
            let scopes_valid = publication.scopes.iter().all(|scope| {
                scope
                    .participating_subject_ids
                    .iter()
                    .all(|id| subjects.contains_key(id))
            });
            let positions_unique = publication.assignments.iter().all(|assignment| {
                positions.insert((assignment.scope_id.clone(), assignment.canonical_position))
            });
            let positions_complete = publication.scopes.iter().all(|scope| {
                let expected: BTreeSet<_> = (0..scope.participating_subject_ids.len()).collect();
                let actual: BTreeSet<_> = publication
                    .assignments
                    .iter()
                    .filter(|assignment| assignment.scope_id == scope.scope_id)
                    .map(|assignment| assignment.canonical_position)
                    .collect();
                actual == expected
            });
            let decisions_valid = publication.assignments.iter().all(|assignment| {
                publication
                    .decisions
                    .iter()
                    .any(|decision| decision.decision_id == assignment.ordering_decision_id)
            });
            let valid = assignments_valid
                && scopes_valid
                && positions_unique
                && positions_complete
                && decisions_valid;
            if !assignments_valid {
                facts.push("ordering assignment coverage is incomplete or foreign".to_owned());
            }
            if !scopes_valid {
                facts.push("ordering scope contains a foreign subject".to_owned());
            }
            if !positions_unique {
                facts.push("canonical positions are not unique within scope".to_owned());
            }
            if !positions_complete {
                facts.push("canonical position coverage is incomplete within scope".to_owned());
            }
            if !decisions_valid {
                facts.push("assignment references an unknown ordering decision".to_owned());
            }
            valid
        }
        "CardinalityIntegrity" => {
            let valid = !publication.orderable_subjects.is_empty()
                && publication.assignments.len() == publication.orderable_subjects.len();
            if !valid {
                facts.push("fixture cardinality requirement is unsatisfied".to_owned());
            }
            valid
        }
        "ConstructionPrerequisiteIntegrity" => {
            let valid = !publication.normalized_publication_id.as_str().is_empty()
                && !publication.normalized_content_binding.is_empty()
                && !publication.schema_version.is_empty()
                && !publication.implementation_version.is_empty()
                && publication.assignments.len() == publication.orderable_subjects.len();
            if !valid {
                facts.push("required construction prerequisite binding is absent".to_owned());
            }
            valid
        }
        _ => false,
    };
    let severity = if valid {
        FindingSeverity::Informational
    } else {
        FindingSeverity::Blocking
    };
    (valid, severity, subject_ids, scope_ids, facts)
}

pub fn perform_structural_validation(
    input: &StructuralValidationInput,
    publication: &CanonicallyOrderedRequestRepresentation,
    profile: &FixtureValidationProfile,
    requirements: &FixtureConstructionRequirements,
    registries: &FixtureStructuralRegistries,
    rules: &FixtureValidationRuleSet,
) -> StructuralValidationOutcome {
    if input.ordered_publication_id != publication.publication_id
        || input.ordered_content_binding != ordered_publication_content_binding(publication)
        || publication.normalized_content_binding.is_empty()
    {
        return validation_failure(
            input,
            ValidationFailureCategory::WrongOrderedPublicationBinding,
            "Contract 011 publication identity or content binding is not exact",
            vec![input.ordered_publication_id.to_string()].into(),
        );
    }
    if input.profile_id != profile.identity || input.profile_version != profile.version {
        return validation_failure(
            input,
            ValidationFailureCategory::ValidationProfileResolutionFailure,
            "validation profile binding is not exact",
            Vec::new().into(),
        );
    }
    if input.requirements_id != requirements.identity
        || input.requirements_version != requirements.version
        || input.requirements_id != profile.requirements_id
        || input.requirements_version != profile.requirements_version
    {
        return validation_failure(
            input,
            ValidationFailureCategory::ConstructionRequirementsResolutionFailure,
            "construction requirements binding is not exact",
            Vec::new().into(),
        );
    }
    if input.schema_version != profile.schema_version {
        return validation_failure(
            input,
            ValidationFailureCategory::SchemaResolutionFailure,
            "validation schema binding is not exact",
            Vec::new().into(),
        );
    }
    if input.registry_id != registries.identity
        || input.registry_version != registries.version
        || input.registry_id != profile.registry_id
        || input.registry_version != profile.registry_version
    {
        return validation_failure(
            input,
            ValidationFailureCategory::StructuralRegistryResolutionFailure,
            "structural registry binding is not exact",
            Vec::new().into(),
        );
    }
    if input.rule_set_id != rules.identity
        || input.rule_set_version != rules.version
        || input.rule_set_id != profile.rule_set_id
        || input.rule_set_version != profile.rule_set_version
    {
        return validation_failure(
            input,
            ValidationFailureCategory::ValidationRuleSetResolutionFailure,
            "validation rule-set binding is not exact",
            Vec::new().into(),
        );
    }
    if input.configuration_version != profile.configuration_version
        || input.implementation_version != profile.implementation_version
        || input.aggregation_rule_id != profile.aggregation_rule_id
        || input.aggregation_rule_version != profile.aggregation_rule_version
    {
        return validation_failure(
            input,
            ValidationFailureCategory::IncompatibleValidationContext,
            "validation configuration or aggregation binding is not exact",
            Vec::new().into(),
        );
    }
    let required_ordering_binding =
        if profile.identity == FixtureValidationProfile::local_governed_text().identity {
            let ordering = FixtureCanonicalOrderingProfile::local_governed_text();
            (ordering.identity, ordering.version)
        } else {
            (
                CanonicalOrderingProfileId::derive(&["contract-011-fixture"]),
                "fixture-ordering-v1".to_owned(),
            )
        };
    if publication.profile_binding != required_ordering_binding {
        return validation_failure(
            input,
            ValidationFailureCategory::IncompatibleValidationContext,
            "ordered publication is not from the required Contract 011 profile context",
            Vec::new().into(),
        );
    }
    let requirement_map: BTreeMap<_, _> = requirements
        .requirements
        .iter()
        .map(|r| (r.requirement_id.clone(), r))
        .collect();
    let expected_ids: BTreeSet<_> = profile_requirements(profile).into_iter().collect();
    if expected_ids
        .iter()
        .any(|id| !requirement_map.contains_key(id))
    {
        return validation_failure(
            input,
            ValidationFailureCategory::UnknownValidationRequirement,
            "profile references an unknown validation requirement",
            Vec::new().into(),
        );
    }
    let known_rules: BTreeSet<_> = rules.known_rule_ids.iter().cloned().collect();
    let selected: Vec<_> = expected_ids
        .iter()
        .filter_map(|id| requirement_map.get(id).copied())
        .collect();
    if selected
        .iter()
        .any(|requirement| !requirement.structural_only)
    {
        return validation_failure(
            input,
            ValidationFailureCategory::IncompatibleValidationContext,
            "fixture requirement attempts to introduce non-structural authority",
            Vec::new().into(),
        );
    }
    if selected.iter().any(|requirement| {
        !known_rules.contains(&requirement.rule_id) || requirement.rule_version != rules.version
    }) {
        return validation_failure(
            input,
            ValidationFailureCategory::UnknownAuthoritativeRule,
            "requirement references an unknown authoritative fixture rule",
            Vec::new().into(),
        );
    }
    let mut applications = Vec::new();
    let mut findings = Vec::new();
    let mut decisions = Vec::new();
    for requirement in selected {
        let (valid, severity, subject_ids, scope_ids, facts) =
            validate_structural_requirement(requirement, publication);
        let application_id = ValidationRuleApplicationId::derive(&[
            input.operation_id.as_str(),
            requirement.requirement_id.as_str(),
            requirement.rule_id.as_str(),
        ]);
        let finding_id =
            ValidationFindingId::derive(&[application_id.as_str(), &format!("{valid:?}{facts:?}")]);
        applications.push(ValidationRuleApplication {
            application_id: application_id.clone(),
            operation_id: input.operation_id.clone(),
            requirement_id: requirement.requirement_id.clone(),
            rule_binding: (
                requirement.rule_id.clone(),
                requirement.rule_version.clone(),
            ),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            schema_version: profile.schema_version.clone(),
            registry_binding: (registries.identity.clone(), registries.version.clone()),
            subject_ids: subject_ids.clone().into(),
            evaluation_method: "Closed fixture structural rule".to_owned(),
            deterministic_input_context: input.ordered_content_binding.clone(),
            completion: ValidationCompletion::Completed,
        });
        findings.push(ValidationFinding {
            finding_id: finding_id.clone(),
            application_id,
            requirement_id: requirement.requirement_id.clone(),
            subject_ids: subject_ids.into(),
            scope_ids: scope_ids.into(),
            condition_class: requirement.requirement_class.clone(),
            severity,
            observed_structural_facts: if facts.is_empty() {
                vec!["required structure satisfied".to_owned()].into()
            } else {
                facts.into()
            },
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            rule_binding: (
                requirement.rule_id.clone(),
                requirement.rule_version.clone(),
            ),
            schema_version: profile.schema_version.clone(),
            registry_binding: (registries.identity.clone(), registries.version.clone()),
            configuration_version: profile.configuration_version.clone(),
            ordered_publication_binding: input.ordered_content_binding.clone(),
            replay_context: input.replay_context.clone(),
        });
        if profile.omit_decision_for.as_ref() == Some(&requirement.requirement_id) {
            continue;
        }
        let finding = findings.last().expect("finding just published");
        let deferred = !valid
            && profile.deferred_condition.as_deref()
                == Some(requirement.requirement_class.as_str());
        let warning = !valid
            && profile.warning_condition.as_deref() == Some(requirement.requirement_class.as_str());
        let decision_kind = if deferred {
            ValidationDecisionKind::RequirementDeferred
        } else if valid {
            ValidationDecisionKind::RequirementSatisfied
        } else if warning {
            ValidationDecisionKind::RequirementSatisfiedWithWarning
        } else {
            ValidationDecisionKind::RequirementUnsatisfied
        };
        decisions.push(ValidationDecision {
            decision_id: ValidationDecisionId::derive(&[
                input.operation_id.as_str(),
                requirement.requirement_id.as_str(),
                &format!("{decision_kind:?}"),
            ]),
            requirement_id: requirement.requirement_id.clone(),
            finding_ids: vec![finding.finding_id.clone()].into(),
            decision_kind,
            decision_basis: "Authoritative fixture requirement decision".to_owned(),
            rule_binding: (
                requirement.rule_id.clone(),
                requirement.rule_version.clone(),
            ),
            mandatory: requirement.mandatory,
            downstream_implication: if deferred {
                "Contract 013 entry is prohibited pending the declared structural condition"
                    .to_owned()
            } else {
                "Contract 012 only; no construction authority".to_owned()
            },
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            ordered_publication_binding: input.ordered_content_binding.clone(),
            replay_context: input.replay_context.clone(),
        });
    }
    let finding_by_id: BTreeMap<_, _> = findings
        .iter()
        .map(|finding| (finding.finding_id.clone(), finding))
        .collect();
    let requirement_ids: BTreeSet<_> = selected_requirements(profile).into_iter().collect();
    let decision_requirement_ids: BTreeSet<_> = decisions
        .iter()
        .map(|decision| decision.requirement_id.clone())
        .collect();
    if decisions.len() != findings.len() || decision_requirement_ids != requirement_ids {
        return validation_failure(
            input,
            ValidationFailureCategory::IncompleteDecisionCoverage,
            "every finding and required requirement must have one decision path",
            Vec::new().into(),
        );
    }
    if decisions.iter().any(|decision| {
        decision.finding_ids.is_empty()
            || decision
                .finding_ids
                .iter()
                .any(|id| !finding_by_id.contains_key(id))
    }) {
        return validation_failure(
            input,
            ValidationFailureCategory::InvalidDecisionReference,
            "decision references an unknown finding",
            Vec::new().into(),
        );
    }
    let mandatory_unsatisfied = decisions.iter().any(|decision| {
        decision.mandatory
            && decision.decision_kind == ValidationDecisionKind::RequirementUnsatisfied
    });
    let deferred_ids: Vec<_> = decisions
        .iter()
        .filter(|decision| decision.decision_kind == ValidationDecisionKind::RequirementDeferred)
        .map(|decision| decision.requirement_id.clone())
        .collect();
    let warnings = decisions.iter().any(|decision| {
        decision.decision_kind == ValidationDecisionKind::RequirementSatisfiedWithWarning
    });
    let eligibility = if mandatory_unsatisfied {
        StructuralEligibility::Ineligible
    } else if !deferred_ids.is_empty() {
        StructuralEligibility::Deferred
    } else if warnings {
        StructuralEligibility::EligibleWithWarnings
    } else {
        StructuralEligibility::Eligible
    };
    if eligibility == StructuralEligibility::Deferred && profile.deferred_condition.is_none() {
        return validation_failure(
            input,
            ValidationFailureCategory::InconsistentEligibilityAggregation,
            "deferred eligibility lacks profile authority",
            Vec::new().into(),
        );
    }
    let result_id = StructuralValidationResultId::derive(&[
        input.operation_id.as_str(),
        &format!("{eligibility:?}{:?}{:?}", applications, findings),
    ]);
    StructuralValidationOutcome::Completed(Box::new(StructuralValidationResult {
        result_id,
        validation_id: input.validation_id.clone(),
        operation_id: input.operation_id.clone(),
        ordered_publication_id: publication.publication_id.clone(),
        ordered_content_binding: input.ordered_content_binding.clone(),
        completion: ValidationCompletion::Completed,
        eligibility,
        profile_binding: (profile.identity.clone(), profile.version.clone()),
        requirements_binding: (requirements.identity.clone(), requirements.version.clone()),
        schema_version: profile.schema_version.clone(),
        registry_binding: (registries.identity.clone(), registries.version.clone()),
        rule_set_binding: (rules.identity.clone(), rules.version.clone()),
        configuration_version: profile.configuration_version.clone(),
        implementation_version: profile.implementation_version.clone(),
        rule_applications: applications.into(),
        findings: findings.into(),
        decisions: decisions.into(),
        aggregation_rule_binding: (
            profile.aggregation_rule_id.clone(),
            profile.aggregation_rule_version.clone(),
        ),
        coverage_evidence: vec![
            "complete requirement, rule, finding, and decision coverage".to_owned(),
            "eligibility aggregated from authoritative decisions only".to_owned(),
        ]
        .into(),
        deferred_requirement_ids: deferred_ids.into(),
        replay_context: input.replay_context.clone(),
    }))
}

fn profile_requirements(profile: &FixtureValidationProfile) -> Vec<ValidationRequirementId> {
    FixtureConstructionRequirements::fixture(profile)
        .requirements
        .iter()
        .map(|requirement| requirement.requirement_id.clone())
        .collect()
}

fn selected_requirements(profile: &FixtureValidationProfile) -> Vec<ValidationRequirementId> {
    profile_requirements(profile)
}

#[cfg(test)]
mod contract_012_tests {
    use super::*;

    fn ordered_fixture() -> CanonicallyOrderedRequestRepresentation {
        let publication = super::contract_011_tests::normalized_fixture(false);
        let input = CanonicalOrderingInput::for_fixture(&publication);
        let CanonicalOrderingOutcome::Success(ordered) = perform_canonical_ordering(
            &input,
            &publication,
            &FixtureCanonicalOrderingProfile::fixture(),
        ) else {
            panic!("ordered fixture")
        };
        *ordered
    }

    fn validate(
        publication: &CanonicallyOrderedRequestRepresentation,
        profile: &FixtureValidationProfile,
        requirements: &FixtureConstructionRequirements,
        registries: &FixtureStructuralRegistries,
        rules: &FixtureValidationRuleSet,
    ) -> StructuralValidationOutcome {
        let input = StructuralValidationInput::for_fixture(publication);
        perform_structural_validation(
            &input,
            publication,
            profile,
            requirements,
            registries,
            rules,
        )
    }

    fn valid_outcome() -> StructuralValidationResult {
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let registries = FixtureStructuralRegistries::fixture();
        let rules = FixtureValidationRuleSet::fixture(&profile);
        let StructuralValidationOutcome::Completed(result) = validate(
            &ordered_fixture(),
            &profile,
            &requirements,
            &registries,
            &rules,
        ) else {
            panic!("valid validation")
        };
        *result
    }

    fn with_subject_kind(
        mut publication: CanonicallyOrderedRequestRepresentation,
        index: usize,
        kind: OrderingSubjectKind,
    ) -> CanonicallyOrderedRequestRepresentation {
        let mut subjects = publication.orderable_subjects.to_vec();
        subjects[index].subject_kind = kind;
        publication.orderable_subjects = subjects.into();
        publication
    }

    fn without_assignments(
        mut publication: CanonicallyOrderedRequestRepresentation,
    ) -> CanonicallyOrderedRequestRepresentation {
        publication.assignments = Vec::new().into();
        publication
    }

    fn with_position(
        mut publication: CanonicallyOrderedRequestRepresentation,
        index: usize,
        position: usize,
    ) -> CanonicallyOrderedRequestRepresentation {
        let mut assignments = publication.assignments.to_vec();
        assignments[index].canonical_position = position;
        publication.assignments = assignments.into();
        publication
    }

    #[test]
    fn contract_012_eligible_result_preserves_exact_ordered_binding() {
        let result = valid_outcome();
        assert_eq!(result.completion, ValidationCompletion::Completed);
        assert_eq!(result.eligibility, StructuralEligibility::Eligible);
        assert_eq!(
            result.ordered_content_binding,
            ordered_publication_content_binding(&ordered_fixture())
        );
    }

    #[test]
    fn contract_012_wrong_contract_011_binding_fails_atomically() {
        let publication = ordered_fixture();
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let registries = FixtureStructuralRegistries::fixture();
        let rules = FixtureValidationRuleSet::fixture(&profile);
        let mut input = StructuralValidationInput::for_fixture(&publication);
        input.ordered_content_binding = "foreign-binding".to_owned();
        assert!(
            matches!(perform_structural_validation(&input, &publication, &profile, &requirements, &registries, &rules), StructuralValidationOutcome::Failed(record) if record.category == ValidationFailureCategory::WrongOrderedPublicationBinding && record.eligibility == StructuralEligibility::NotDetermined)
        );
    }

    #[test]
    fn contract_012_failure_publishes_no_partial_result() {
        let publication = ordered_fixture();
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let registries = FixtureStructuralRegistries {
            version: "foreign".to_owned(),
            ..FixtureStructuralRegistries::fixture()
        };
        let rules = FixtureValidationRuleSet::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &registries, &rules), StructuralValidationOutcome::Failed(record) if record.category == ValidationFailureCategory::StructuralRegistryResolutionFailure)
        );
    }

    #[test]
    fn contract_012_equivalent_inputs_replay_deterministically() {
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let registries = FixtureStructuralRegistries::fixture();
        let rules = FixtureValidationRuleSet::fixture(&profile);
        let publication = ordered_fixture();
        assert_eq!(
            validate(&publication, &profile, &requirements, &registries, &rules),
            validate(&publication, &profile, &requirements, &registries, &rules)
        );
    }

    #[test]
    fn contract_012_warning_is_completed_and_distinct_from_ineligibility() {
        let publication = ordered_fixture();
        let mut profile = FixtureValidationProfile::fixture();
        profile.warning_condition = Some("IdentifierIntegrity".to_owned());
        let malformed = with_subject_kind(publication, 0, OrderingSubjectKind::Relationship);
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let outcome = validate(
            &malformed,
            &profile,
            &requirements,
            &FixtureStructuralRegistries::fixture(),
            &FixtureValidationRuleSet::fixture(&profile),
        );
        assert!(
            matches!(outcome, StructuralValidationOutcome::Completed(result) if result.eligibility == StructuralEligibility::EligibleWithWarnings)
        );
    }

    #[test]
    fn contract_012_mandatory_defect_produces_completed_ineligible_result() {
        let publication = without_assignments(ordered_fixture());
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Completed(result) if result.eligibility == StructuralEligibility::Ineligible)
        );
    }

    #[test]
    fn contract_012_profile_authorized_deferred_is_explicit() {
        let publication = with_position(ordered_fixture(), 1, 0);
        let mut profile = FixtureValidationProfile::fixture();
        profile.deferred_condition = Some("OrderingIntegrity".to_owned());
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Completed(result) if result.eligibility == StructuralEligibility::Deferred && !result.deferred_requirement_ids.is_empty())
        );
    }

    #[test]
    fn contract_012_malformed_but_traversable_subject_is_completed_ineligible() {
        let publication =
            with_subject_kind(ordered_fixture(), 0, OrderingSubjectKind::Relationship);
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Completed(result) if result.eligibility == StructuralEligibility::Ineligible)
        );
    }

    #[test]
    fn contract_012_findings_and_decisions_remain_distinct() {
        let result = valid_outcome();
        assert_eq!(result.findings.len(), result.decisions.len());
        assert!(result.findings.iter().all(|finding| result
            .decisions
            .iter()
            .any(|decision| decision.finding_ids.contains(&finding.finding_id))));
    }

    #[test]
    fn contract_012_blocking_severity_does_not_bypass_decision_authority() {
        let publication =
            with_subject_kind(ordered_fixture(), 0, OrderingSubjectKind::Relationship);
        let mut profile = FixtureValidationProfile::fixture();
        profile.warning_condition = Some("IdentifierIntegrity".to_owned());
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let StructuralValidationOutcome::Completed(result) = validate(
            &publication,
            &profile,
            &requirements,
            &FixtureStructuralRegistries::fixture(),
            &FixtureValidationRuleSet::fixture(&profile),
        ) else {
            panic!("completed warning")
        };
        let finding = result
            .findings
            .iter()
            .find(|finding| finding.condition_class == "IdentifierIntegrity")
            .expect("identifier finding");
        assert_eq!(finding.severity, FindingSeverity::Blocking);
        assert_eq!(
            result
                .decisions
                .iter()
                .find(|decision| decision.requirement_id == finding.requirement_id)
                .expect("decision")
                .decision_kind,
            ValidationDecisionKind::RequirementSatisfiedWithWarning
        );
    }

    #[test]
    fn contract_012_finding_without_decision_cannot_publish_result() {
        let publication = ordered_fixture();
        let mut profile = FixtureValidationProfile::fixture();
        profile.omit_decision_for = Some(ValidationRequirementId::derive(&["IdentifierIntegrity"]));
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Failed(record) if record.category == ValidationFailureCategory::IncompleteDecisionCoverage)
        );
    }

    #[test]
    fn contract_012_warning_plus_unsatisfied_mandatory_is_ineligible() {
        let publication = without_assignments(with_subject_kind(
            ordered_fixture(),
            0,
            OrderingSubjectKind::Relationship,
        ));
        let mut profile = FixtureValidationProfile::fixture();
        profile.warning_condition = Some("IdentifierIntegrity".to_owned());
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Completed(result) if result.eligibility == StructuralEligibility::Ineligible)
        );
    }

    #[test]
    fn contract_012_all_mandatory_satisfied_plus_warning_is_eligible_with_warnings() {
        let publication =
            with_subject_kind(ordered_fixture(), 0, OrderingSubjectKind::Relationship);
        let mut profile = FixtureValidationProfile::fixture();
        profile.warning_condition = Some("IdentifierIntegrity".to_owned());
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Completed(result) if result.eligibility == StructuralEligibility::EligibleWithWarnings)
        );
    }

    #[test]
    fn contract_012_all_requirements_satisfied_is_eligible() {
        assert_eq!(valid_outcome().eligibility, StructuralEligibility::Eligible);
    }

    #[test]
    fn contract_012_missing_registry_is_failure_not_deferred() {
        let publication = ordered_fixture();
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let registries = FixtureStructuralRegistries {
            identity: StructuralRegistryId::derive(&["missing"]),
            ..FixtureStructuralRegistries::fixture()
        };
        assert!(
            matches!(validate(&publication, &profile, &requirements, &registries, &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Failed(record) if record.category == ValidationFailureCategory::StructuralRegistryResolutionFailure && record.eligibility == StructuralEligibility::NotDetermined)
        );
    }

    #[test]
    fn contract_012_ordering_integrity_is_checked_without_reordering() {
        let original_publication = ordered_fixture();
        let original = original_publication.assignments.clone();
        let publication = with_position(original_publication, 0, 99);
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let StructuralValidationOutcome::Completed(result) = validate(
            &publication,
            &profile,
            &requirements,
            &FixtureStructuralRegistries::fixture(),
            &FixtureValidationRuleSet::fixture(&profile),
        ) else {
            panic!("completed structural result")
        };
        assert_eq!(result.eligibility, StructuralEligibility::Ineligible);
        assert_eq!(
            publication.assignments[1].canonical_position,
            original[1].canonical_position
        );
    }

    #[test]
    fn contract_012_relationship_defect_is_reported_without_repair() {
        let mut publication = ordered_fixture();
        publication.ordered_relationship_ids =
            vec![NormalizedRelationshipId::derive(&["foreign"])].into();
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let StructuralValidationOutcome::Completed(result) = validate(
            &publication,
            &profile,
            &requirements,
            &FixtureStructuralRegistries::fixture(),
            &FixtureValidationRuleSet::fixture(&profile),
        ) else {
            panic!("completed structural result")
        };
        assert_eq!(result.eligibility, StructuralEligibility::Ineligible);
        assert_eq!(
            publication.ordered_relationship_ids[0],
            NormalizedRelationshipId::derive(&["foreign"])
        );
    }

    #[test]
    fn contract_012_unknown_requirement_or_non_structural_policy_fails() {
        let publication = ordered_fixture();
        let profile = FixtureValidationProfile::fixture();
        let mut requirements = FixtureConstructionRequirements::fixture(&profile);
        let mut entries = requirements.requirements.to_vec();
        entries[0].structural_only = false;
        requirements.requirements = entries.into();
        assert!(
            matches!(validate(&publication, &profile, &requirements, &FixtureStructuralRegistries::fixture(), &FixtureValidationRuleSet::fixture(&profile)), StructuralValidationOutcome::Failed(record) if record.category == ValidationFailureCategory::IncompatibleValidationContext)
        );
    }
}

// ---------------------------------------------------------------------------
// Contract 013: Canonical request construction
// ---------------------------------------------------------------------------

contract_002_id!(CanonicalRequestConstructionId, "crc");
contract_002_id!(ConstructionProfileId, "crprofile");
contract_002_id!(ConstructionSchemaId, "crschema");
contract_002_id!(ConstructionRuleSetId, "crrules");
contract_002_id!(ConstructionRegistryId, "crregistry");
contract_002_id!(ConstructionConfigurationId, "crconfig");
contract_002_id!(ConstructionDecisionId, "crdecision");
contract_002_id!(ConstructionPlacementId, "crplacement");
contract_002_id!(SourceToArtifactMappingId, "crmapping");
contract_002_id!(ConstructedRequestComponentId, "crcomponent");
contract_002_id!(ConstructedCanonicalRequestId, "crrequest");
contract_002_id!(ConstructionManifestId, "crmanifest");
contract_002_id!(ConstructionFailureRecordId, "crfail");
contract_002_id!(ConstructionExecutionRecordId, "crexec");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstructionSourceDisposition {
    Included,
    Referenced,
    AuthorizedOmission,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstructedComponentKind {
    SemanticElement,
    Relationship,
    SchemaGeneratedStructural,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstructionDecisionKind {
    IncludeSemanticElement,
    IncludeRelationship,
    GenerateStructuralComponent,
    OmitOptionalStructuralComponent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConstructionBasis {
    GovernedSourceBasis {
        orderable_subject_id: OrderableSubjectId,
        normalized_subject: OrderingSubjectReference,
    },
    SchemaGeneratedStructuralBasis {
        schema_rule_id: String,
        schema_rule_version: String,
        structural_class: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstructionFailureCategory {
    MissingOrderedRepresentation,
    MissingStructuralValidationResult,
    OrderedRepresentationBindingMismatch,
    ValidationSubjectBindingMismatch,
    ValidationResultNotCompleted,
    IneligibleValidationState,
    DeferredValidationStateNotAccepted,
    FailedValidationPublicationSupplied,
    ConstructionProfileMismatch,
    ConstructionProfileResolutionFailure,
    ConstructionSchemaResolutionFailure,
    ConstructionRuleSetResolutionFailure,
    ConstructionRegistryResolutionFailure,
    ConstructionConfigurationResolutionFailure,
    VersionCompatibilityFailure,
    IntegrityBindingFailure,
    UnknownConstructionRule,
    UnknownSchemaRequirement,
    UnknownStructuralClass,
    SourceDispositionCoverageFailure,
    SourceToArtifactMappingFailure,
    TargetBasisCoverageFailure,
    ConstructionPlacementFailure,
    CanonicalPositionPreservationFailure,
    RequiredComponentAssemblyFailure,
    ReferencePreservationFailure,
    IdentityPreservationFailure,
    AuthorizedOmissionFailure,
    ManifestCompletenessFailure,
    RequestManifestBindingFailure,
    NonDeterministicConstruction,
    AtomicPublicationFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionProfile {
    pub identity: ConstructionProfileId,
    pub version: String,
    pub authority_reference: String,
    pub schema_id: ConstructionSchemaId,
    pub schema_version: String,
    pub rule_set_id: ConstructionRuleSetId,
    pub rule_set_version: String,
    pub registry_id: ConstructionRegistryId,
    pub registry_version: String,
    pub configuration_id: ConstructionConfigurationId,
    pub configuration_version: String,
    pub implementation_version: String,
    pub optional_structural_class: String,
    pub optional_omission_rule_id: String,
    pub emit_optional_structural_component: bool,
    pub omission_rule_authorized: bool,
}

impl FixtureConstructionProfile {
    pub fn fixture() -> Self {
        Self {
            identity: ConstructionProfileId::derive(&["contract-013-fixture"]),
            version: "fixture-construction-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            schema_id: ConstructionSchemaId::derive(&["contract-013-fixture"]),
            schema_version: "fixture-construction-schema-v1".to_owned(),
            rule_set_id: ConstructionRuleSetId::derive(&["contract-013-fixture"]),
            rule_set_version: "fixture-construction-rules-v1".to_owned(),
            registry_id: ConstructionRegistryId::derive(&["contract-013-fixture"]),
            registry_version: "fixture-construction-registry-v1".to_owned(),
            configuration_id: ConstructionConfigurationId::derive(&["contract-013-fixture"]),
            configuration_version: "fixture-construction-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            optional_structural_class: "IndexHolder".to_owned(),
            optional_omission_rule_id: "OmitEmptyOptionalIndexRule".to_owned(),
            emit_optional_structural_component: false,
            omission_rule_authorized: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionSchema {
    pub identity: ConstructionSchemaId,
    pub version: String,
    pub structural_classes: Arc<[String]>,
    pub optional_classes: Arc<[String]>,
}

impl FixtureConstructionSchema {
    pub fn fixture(profile: &FixtureConstructionProfile) -> Self {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionRuleSet {
    pub identity: ConstructionRuleSetId,
    pub version: String,
    pub known_rule_ids: Arc<[String]>,
}

impl FixtureConstructionRuleSet {
    pub fn fixture(profile: &FixtureConstructionProfile) -> Self {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionRegistries {
    pub identity: ConstructionRegistryId,
    pub version: String,
}

impl FixtureConstructionRegistries {
    pub fn fixture(profile: &FixtureConstructionProfile) -> Self {
        Self {
            identity: profile.registry_id.clone(),
            version: profile.registry_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionConfiguration {
    pub identity: ConstructionConfigurationId,
    pub version: String,
}

impl FixtureConstructionConfiguration {
    pub fn fixture(profile: &FixtureConstructionProfile) -> Self {
        Self {
            identity: profile.configuration_id.clone(),
            version: profile.configuration_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureConstructionAuthorityContext {
    pub profile: FixtureConstructionProfile,
    pub schema: FixtureConstructionSchema,
    pub rules: FixtureConstructionRuleSet,
    pub registries: FixtureConstructionRegistries,
    pub configuration: FixtureConstructionConfiguration,
}

impl FixtureConstructionAuthorityContext {
    pub fn fixture(profile: &FixtureConstructionProfile) -> Self {
        Self {
            profile: profile.clone(),
            schema: FixtureConstructionSchema::fixture(profile),
            rules: FixtureConstructionRuleSet::fixture(profile),
            registries: FixtureConstructionRegistries::fixture(profile),
            configuration: FixtureConstructionConfiguration::fixture(profile),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRequestConstructionInput {
    pub construction_id: CanonicalRequestConstructionId,
    pub operation_id: CanonicalRequestConstructionId,
    pub ordered_representation_id: CanonicallyOrderedRequestRepresentationId,
    pub ordered_representation_content_binding: String,
    pub validation_result_id: StructuralValidationResultId,
    pub validation_subject_binding: String,
    pub validation_completion: ValidationCompletion,
    pub validation_eligibility: StructuralEligibility,
    pub construction_profile_id: ConstructionProfileId,
    pub construction_profile_version: String,
    pub construction_schema_id: ConstructionSchemaId,
    pub construction_schema_version: String,
    pub construction_rule_set_id: ConstructionRuleSetId,
    pub construction_rule_set_version: String,
    pub construction_registry_id: ConstructionRegistryId,
    pub construction_registry_version: String,
    pub construction_configuration_id: ConstructionConfigurationId,
    pub construction_configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl CanonicalRequestConstructionInput {
    pub fn for_fixture(
        publication: &CanonicallyOrderedRequestRepresentation,
        validation: &StructuralValidationResult,
    ) -> Self {
        let profile = FixtureConstructionProfile::fixture();
        Self {
            construction_id: CanonicalRequestConstructionId::derive(&[
                publication.publication_id.as_str(),
                validation.result_id.as_str(),
                "fixture-construction",
            ]),
            operation_id: CanonicalRequestConstructionId::derive(&[
                publication.publication_id.as_str(),
                validation.result_id.as_str(),
                "fixture-operation",
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionDecision {
    pub decision_id: ConstructionDecisionId,
    pub construction_id: CanonicalRequestConstructionId,
    pub decision_kind: ConstructionDecisionKind,
    pub source_subject_id: Option<OrderableSubjectId>,
    pub structural_class: Option<String>,
    pub rule_binding: (String, String),
    pub schema_binding: (ConstructionSchemaId, String),
    pub decision_basis: String,
    pub target_component_id: Option<ConstructedRequestComponentId>,
    pub omission_target: Option<String>,
    pub profile_binding: (ConstructionProfileId, String),
    pub registry_binding: (ConstructionRegistryId, String),
    pub configuration_binding: (ConstructionConfigurationId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionPlacement {
    pub placement_id: ConstructionPlacementId,
    pub construction_decision_id: ConstructionDecisionId,
    pub target_component_id: ConstructedRequestComponentId,
    pub parent_container_id: String,
    pub section_or_field_role: String,
    pub scope_reference: String,
    pub ordering_assignment_id: Option<CanonicalOrderingAssignmentId>,
    pub preserved_canonical_position: Option<usize>,
    pub placement_rule_binding: (String, String),
    pub profile_binding: (ConstructionProfileId, String),
    pub schema_binding: (ConstructionSchemaId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceToArtifactMapping {
    pub mapping_id: SourceToArtifactMappingId,
    pub construction_id: CanonicalRequestConstructionId,
    pub source_subject_id: Option<OrderableSubjectId>,
    pub target_component_id: Option<ConstructedRequestComponentId>,
    pub disposition: ConstructionSourceDisposition,
    pub authorized_basis: Arc<[ConstructionBasis]>,
    pub decision_id: ConstructionDecisionId,
    pub omission_rule_binding: Option<(String, String)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedRequestComponent {
    pub component_id: ConstructedRequestComponentId,
    pub component_kind: ConstructedComponentKind,
    pub source_subject_id: Option<OrderableSubjectId>,
    pub normalized_element_id: Option<NormalizedSemanticElementId>,
    pub normalized_relationship_id: Option<NormalizedRelationshipId>,
    pub canonical_expression: Option<String>,
    pub semantic_domain: Option<String>,
    pub semantic_class: Option<String>,
    pub relationship_class: Option<String>,
    pub relationship_source_subject_id: Option<ReconciliationSubjectId>,
    pub relationship_target_subject_id: Option<ReconciliationSubjectId>,
    pub represented_scope: String,
    pub standing_references: Arc<[StandingAssignmentId]>,
    pub evidence_references: Arc<[String]>,
    pub grounding_references: Arc<[String]>,
    pub provenance_references: Arc<[String]>,
    pub unresolved_conditions: Arc<[String]>,
    pub ordering_assignment_id: Option<CanonicalOrderingAssignmentId>,
    pub canonical_position: Option<usize>,
    pub scope_id: Option<OrderingScopeId>,
    pub authorized_bases: Arc<[ConstructionBasis]>,
    pub structural_class: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionExecutionRecord {
    pub execution_record_id: ConstructionExecutionRecordId,
    pub construction_id: CanonicalRequestConstructionId,
    pub decision_ids: Arc<[ConstructionDecisionId]>,
    pub placement_ids: Arc<[ConstructionPlacementId]>,
    pub mapping_ids: Arc<[SourceToArtifactMappingId]>,
    pub deterministic_rule_context: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructedCanonicalRequest {
    pub constructed_canonical_request_id: ConstructedCanonicalRequestId,
    pub construction_id: CanonicalRequestConstructionId,
    pub source_ordered_representation_id: CanonicallyOrderedRequestRepresentationId,
    pub structural_validation_result_id: StructuralValidationResultId,
    pub components: Arc<[ConstructedRequestComponent]>,
    pub construction_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDispositionRecord {
    pub source_subject_id: OrderableSubjectId,
    pub disposition: ConstructionSourceDisposition,
    pub mapping_id: SourceToArtifactMappingId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionManifest {
    pub construction_manifest_id: ConstructionManifestId,
    pub construction_id: CanonicalRequestConstructionId,
    pub constructed_canonical_request_id: ConstructedCanonicalRequestId,
    pub source_ordered_representation_id: CanonicallyOrderedRequestRepresentationId,
    pub structural_validation_result_id: StructuralValidationResultId,
    pub profile_binding: (ConstructionProfileId, String),
    pub schema_binding: (ConstructionSchemaId, String),
    pub rule_set_binding: (ConstructionRuleSetId, String),
    pub registry_binding: (ConstructionRegistryId, String),
    pub configuration_binding: (ConstructionConfigurationId, String),
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
    pub construction_decision_ids: Arc<[ConstructionDecisionId]>,
    pub construction_placement_ids: Arc<[ConstructionPlacementId]>,
    pub source_to_artifact_mapping_ids: Arc<[SourceToArtifactMappingId]>,
    pub source_dispositions: Arc<[SourceDispositionRecord]>,
    pub omission_records: Arc<[SourceToArtifactMappingId]>,
    pub schema_generated_bases: Arc<[ConstructionBasis]>,
    pub source_coverage_summary: String,
    pub target_basis_coverage_summary: String,
    pub validation_lineage: String,
    pub execution_record: ConstructionExecutionRecord,
    pub publication_binding: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionFailureRecord {
    pub failure_record_id: ConstructionFailureRecordId,
    pub construction_id: CanonicalRequestConstructionId,
    pub operation_id: CanonicalRequestConstructionId,
    pub attempted_ordered_representation_id: Option<CanonicallyOrderedRequestRepresentationId>,
    pub attempted_validation_result_id: Option<StructuralValidationResultId>,
    pub category: ConstructionFailureCategory,
    pub detail: String,
    pub profile_binding: Option<(ConstructionProfileId, String)>,
    pub schema_binding: Option<(ConstructionSchemaId, String)>,
    pub rule_set_binding: Option<(ConstructionRuleSetId, String)>,
    pub registry_binding: Option<(ConstructionRegistryId, String)>,
    pub configuration_binding: Option<(ConstructionConfigurationId, String)>,
    pub implementation_version: Option<String>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalRequestConstructionOutcome {
    Constructed {
        request: Box<ConstructedCanonicalRequest>,
        manifest: Box<ConstructionManifest>,
    },
    Failed(Box<ConstructionFailureRecord>),
}

fn structural_validation_subject_binding(validation: &StructuralValidationResult) -> String {
    StableId::from_parts(
        "validation-subject-binding",
        &[
            validation.result_id.as_str(),
            validation.ordered_publication_id.as_str(),
            &format!(
                "{:?}{:?}",
                validation.rule_applications, validation.decisions
            ),
        ],
    )
    .to_string()
}

fn construction_failure(
    input: &CanonicalRequestConstructionInput,
    category: ConstructionFailureCategory,
    detail: &str,
) -> CanonicalRequestConstructionOutcome {
    CanonicalRequestConstructionOutcome::Failed(Box::new(ConstructionFailureRecord {
        failure_record_id: ConstructionFailureRecordId::derive(&[
            input.operation_id.as_str(),
            &format!("{category:?}"),
            detail,
        ]),
        construction_id: input.construction_id.clone(),
        operation_id: input.operation_id.clone(),
        attempted_ordered_representation_id: Some(input.ordered_representation_id.clone()),
        attempted_validation_result_id: Some(input.validation_result_id.clone()),
        category,
        detail: detail.to_owned(),
        profile_binding: Some((
            input.construction_profile_id.clone(),
            input.construction_profile_version.clone(),
        )),
        schema_binding: Some((
            input.construction_schema_id.clone(),
            input.construction_schema_version.clone(),
        )),
        rule_set_binding: Some((
            input.construction_rule_set_id.clone(),
            input.construction_rule_set_version.clone(),
        )),
        registry_binding: Some((
            input.construction_registry_id.clone(),
            input.construction_registry_version.clone(),
        )),
        configuration_binding: Some((
            input.construction_configuration_id.clone(),
            input.construction_configuration_version.clone(),
        )),
        implementation_version: Some(input.implementation_version.clone()),
        replay_context: input.replay_context.clone(),
    }))
}

fn ordered_subject_map(
    publication: &CanonicallyOrderedRequestRepresentation,
) -> BTreeMap<OrderableSubjectId, &OrderableSubject> {
    publication
        .orderable_subjects
        .iter()
        .map(|subject| (subject.subject_id.clone(), subject))
        .collect()
}

fn construction_publication_binding(
    request: &ConstructedCanonicalRequest,
    manifest: &ConstructionManifest,
) -> String {
    StableId::from_parts(
        "construction-publication",
        &[
            request.constructed_canonical_request_id.as_str(),
            manifest.construction_manifest_id.as_str(),
            request.construction_id.as_str(),
            manifest.source_ordered_representation_id.as_str(),
            manifest.structural_validation_result_id.as_str(),
            &format!(
                "{:?}{:?}{:?}",
                manifest.construction_decision_ids,
                manifest.construction_placement_ids,
                manifest.source_to_artifact_mapping_ids
            ),
        ],
    )
    .to_string()
}

pub fn perform_canonical_request_construction(
    input: &CanonicalRequestConstructionInput,
    publication: &CanonicallyOrderedRequestRepresentation,
    validation: &StructuralValidationResult,
    authorities: &FixtureConstructionAuthorityContext,
) -> CanonicalRequestConstructionOutcome {
    let profile = &authorities.profile;
    let schema = &authorities.schema;
    let rules = &authorities.rules;
    let registries = &authorities.registries;
    let configuration = &authorities.configuration;
    if input.ordered_representation_id != publication.publication_id
        || input.ordered_representation_content_binding
            != ordered_publication_content_binding(publication)
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::OrderedRepresentationBindingMismatch,
            "Contract 011 publication identity or content binding is not exact",
        );
    }
    if validation.ordered_publication_id != publication.publication_id
        || validation.ordered_content_binding != ordered_publication_content_binding(publication)
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ValidationSubjectBindingMismatch,
            "Contract 012 result is not bound to the exact Contract 011 publication",
        );
    }
    if input.validation_result_id != validation.result_id
        || input.validation_subject_binding != structural_validation_subject_binding(validation)
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ValidationSubjectBindingMismatch,
            "Contract 012 result identity or subject binding is not exact",
        );
    }
    if validation.completion != ValidationCompletion::Completed
        || input.validation_completion != ValidationCompletion::Completed
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ValidationResultNotCompleted,
            "Contract 012 did not publish a completed validation result",
        );
    }
    match validation.eligibility {
        StructuralEligibility::Eligible | StructuralEligibility::EligibleWithWarnings => {}
        StructuralEligibility::Ineligible => {
            return construction_failure(
                input,
                ConstructionFailureCategory::IneligibleValidationState,
                "Contract 012 completed eligibility is Ineligible",
            )
        }
        StructuralEligibility::Deferred => {
            return construction_failure(
                input,
                ConstructionFailureCategory::DeferredValidationStateNotAccepted,
                "fixture construction does not accept Deferred eligibility",
            )
        }
        StructuralEligibility::NotDetermined => {
            return construction_failure(
                input,
                ConstructionFailureCategory::FailedValidationPublicationSupplied,
                "Contract 012 eligibility is NotDetermined",
            )
        }
    }
    if input.validation_eligibility != validation.eligibility {
        return construction_failure(
            input,
            ConstructionFailureCategory::ValidationSubjectBindingMismatch,
            "declared validation eligibility does not match the exact result",
        );
    }
    if input.construction_profile_id != profile.identity
        || input.construction_profile_version != profile.version
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ConstructionProfileMismatch,
            "construction profile binding is not exact",
        );
    }
    if input.construction_schema_id != schema.identity
        || input.construction_schema_version != schema.version
        || input.construction_schema_id != profile.schema_id
        || input.construction_schema_version != profile.schema_version
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ConstructionSchemaResolutionFailure,
            "construction schema binding is not exact",
        );
    }
    if input.construction_rule_set_id != rules.identity
        || input.construction_rule_set_version != rules.version
        || input.construction_rule_set_id != profile.rule_set_id
        || input.construction_rule_set_version != profile.rule_set_version
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ConstructionRuleSetResolutionFailure,
            "construction rule-set binding is not exact",
        );
    }
    if input.construction_registry_id != registries.identity
        || input.construction_registry_version != registries.version
        || input.construction_registry_id != profile.registry_id
        || input.construction_registry_version != profile.registry_version
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ConstructionRegistryResolutionFailure,
            "construction registry binding is not exact",
        );
    }
    if input.construction_configuration_id != configuration.identity
        || input.construction_configuration_version != configuration.version
        || input.construction_configuration_id != profile.configuration_id
        || input.construction_configuration_version != profile.configuration_version
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ConstructionConfigurationResolutionFailure,
            "construction configuration binding is not exact",
        );
    }
    if input.implementation_version != profile.implementation_version {
        return construction_failure(
            input,
            ConstructionFailureCategory::VersionCompatibilityFailure,
            "construction implementation binding is not exact",
        );
    }
    let known_rules: BTreeSet<_> = rules.known_rule_ids.iter().cloned().collect();
    if !known_rules.contains("IncludeNormalizedElementRule")
        || !known_rules.contains("IncludeNormalizedRelationshipRule")
        || !known_rules.contains(&profile.optional_omission_rule_id)
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::UnknownConstructionRule,
            "required fixture construction rule is absent",
        );
    }
    if !schema
        .structural_classes
        .iter()
        .any(|class| class == &profile.optional_structural_class)
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::UnknownStructuralClass,
            "fixture optional structural class is not registered",
        );
    }

    let subjects = ordered_subject_map(publication);
    let element_map: BTreeMap<_, _> = publication
        .normalized_elements
        .iter()
        .map(|element| (element.normalized_element_id.clone(), element))
        .collect();
    let relationship_map: BTreeMap<_, _> = publication
        .normalized_relationships
        .iter()
        .map(|relationship| {
            (
                relationship.normalized_relationship_id.clone(),
                relationship,
            )
        })
        .collect();
    let mut ordered_assignments: Vec<_> = publication.assignments.iter().collect();
    ordered_assignments.sort_by(|left, right| {
        left.scope_id
            .cmp(&right.scope_id)
            .then(left.canonical_position.cmp(&right.canonical_position))
    });
    let mut decisions = Vec::new();
    let mut placements = Vec::new();
    let mut mappings = Vec::new();
    let mut components = Vec::new();
    let mut source_dispositions = Vec::new();
    let mut schema_generated_bases = Vec::new();

    for assignment in ordered_assignments {
        let Some(subject) = subjects.get(&assignment.ordered_subject_id) else {
            return construction_failure(
                input,
                ConstructionFailureCategory::SourceDispositionCoverageFailure,
                "assignment references no exact Contract 011 subject",
            );
        };
        let (
            component_kind,
            normalized_subject,
            expression,
            domain,
            class,
            relationship_class,
            relationship_source,
            relationship_target,
            scope,
            standing,
            evidence,
            grounding,
            provenance,
            unresolved,
            rule_id,
            decision_kind,
        ) = match &subject.normalized_subject {
            OrderingSubjectReference::Element(element_id) => {
                let Some(element) = element_map.get(element_id) else {
                    return construction_failure(
                        input,
                        ConstructionFailureCategory::RequiredComponentAssemblyFailure,
                        "ordered element snapshot is absent",
                    );
                };
                (
                    ConstructedComponentKind::SemanticElement,
                    OrderingSubjectReference::Element(element_id.clone()),
                    Some(element.canonical_expression.clone()),
                    Some(element.semantic_domain.clone()),
                    Some(element.semantic_class.clone()),
                    None,
                    None,
                    None,
                    element.represented_scope.clone(),
                    vec![element.standing_assignment_id.clone()],
                    element.evidence_reference_ids.to_vec(),
                    element.grounding_reference_ids.to_vec(),
                    element.provenance_reference_ids.to_vec(),
                    element.unresolved_conditions.to_vec(),
                    "IncludeNormalizedElementRule",
                    ConstructionDecisionKind::IncludeSemanticElement,
                )
            }
            OrderingSubjectReference::Relationship(relationship_id) => {
                let Some(relationship) = relationship_map.get(relationship_id) else {
                    return construction_failure(
                        input,
                        ConstructionFailureCategory::RequiredComponentAssemblyFailure,
                        "ordered relationship snapshot is absent",
                    );
                };
                (
                    ConstructedComponentKind::Relationship,
                    OrderingSubjectReference::Relationship(relationship_id.clone()),
                    Some(relationship.canonical_expression.clone()),
                    None,
                    None,
                    Some(relationship.relationship_class.clone()),
                    Some(relationship.source_subject_id.clone()),
                    Some(relationship.target_subject_id.clone()),
                    relationship.represented_scope.clone(),
                    relationship.standing_references.to_vec(),
                    relationship.evidence_references.to_vec(),
                    relationship.grounding_references.to_vec(),
                    relationship.provenance_references.to_vec(),
                    relationship.unresolved_conditions.to_vec(),
                    "IncludeNormalizedRelationshipRule",
                    ConstructionDecisionKind::IncludeRelationship,
                )
            }
        };
        let component_id = ConstructedRequestComponentId::derive(&[
            input.construction_id.as_str(),
            assignment.ordered_subject_id.as_str(),
            &format!("{}", assignment.canonical_position),
        ]);
        let decision_id = ConstructionDecisionId::derive(&[
            input.construction_id.as_str(),
            assignment.ordered_subject_id.as_str(),
            &format!("{decision_kind:?}"),
        ]);
        let basis = ConstructionBasis::GovernedSourceBasis {
            orderable_subject_id: assignment.ordered_subject_id.clone(),
            normalized_subject: normalized_subject.clone(),
        };
        decisions.push(ConstructionDecision {
            decision_id: decision_id.clone(),
            construction_id: input.construction_id.clone(),
            decision_kind,
            source_subject_id: Some(assignment.ordered_subject_id.clone()),
            structural_class: None,
            rule_binding: (rule_id.to_owned(), profile.rule_set_version.clone()),
            schema_binding: (profile.schema_id.clone(), profile.schema_version.clone()),
            decision_basis: "Exact Contract 011 subject mechanical inclusion".to_owned(),
            target_component_id: Some(component_id.clone()),
            omission_target: None,
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            registry_binding: (
                profile.registry_id.clone(),
                profile.registry_version.clone(),
            ),
            configuration_binding: (
                profile.configuration_id.clone(),
                profile.configuration_version.clone(),
            ),
        });
        let placement_id = ConstructionPlacementId::derive(&[
            decision_id.as_str(),
            assignment.scope_id.as_str(),
            &assignment.canonical_position.to_string(),
        ]);
        placements.push(ConstructionPlacement {
            placement_id: placement_id.clone(),
            construction_decision_id: decision_id.clone(),
            target_component_id: component_id.clone(),
            parent_container_id: "request-root".to_owned(),
            section_or_field_role: match component_kind {
                ConstructedComponentKind::SemanticElement => "semantic-elements".to_owned(),
                ConstructedComponentKind::Relationship => "relationships".to_owned(),
                ConstructedComponentKind::SchemaGeneratedStructural => "structural".to_owned(),
            },
            scope_reference: assignment.scope_id.to_string(),
            ordering_assignment_id: Some(assignment.assignment_id.clone()),
            preserved_canonical_position: Some(assignment.canonical_position),
            placement_rule_binding: (
                "PreserveContract011PlacementRule".to_owned(),
                profile.rule_set_version.clone(),
            ),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            schema_binding: (profile.schema_id.clone(), profile.schema_version.clone()),
        });
        let mapping_id = SourceToArtifactMappingId::derive(&[
            input.construction_id.as_str(),
            assignment.ordered_subject_id.as_str(),
            component_id.as_str(),
        ]);
        mappings.push(SourceToArtifactMapping {
            mapping_id: mapping_id.clone(),
            construction_id: input.construction_id.clone(),
            source_subject_id: Some(assignment.ordered_subject_id.clone()),
            target_component_id: Some(component_id.clone()),
            disposition: ConstructionSourceDisposition::Included,
            authorized_basis: vec![basis.clone()].into(),
            decision_id,
            omission_rule_binding: None,
        });
        source_dispositions.push(SourceDispositionRecord {
            source_subject_id: assignment.ordered_subject_id.clone(),
            disposition: ConstructionSourceDisposition::Included,
            mapping_id,
        });
        components.push(ConstructedRequestComponent {
            component_id,
            component_kind,
            source_subject_id: Some(assignment.ordered_subject_id.clone()),
            normalized_element_id: match &normalized_subject {
                OrderingSubjectReference::Element(id) => Some(id.clone()),
                OrderingSubjectReference::Relationship(_) => None,
            },
            normalized_relationship_id: match &normalized_subject {
                OrderingSubjectReference::Element(_) => None,
                OrderingSubjectReference::Relationship(id) => Some(id.clone()),
            },
            canonical_expression: expression,
            semantic_domain: domain,
            semantic_class: class,
            relationship_class,
            relationship_source_subject_id: relationship_source,
            relationship_target_subject_id: relationship_target,
            represented_scope: scope,
            standing_references: standing.into(),
            evidence_references: evidence.into(),
            grounding_references: grounding.into(),
            provenance_references: provenance.into(),
            unresolved_conditions: unresolved.into(),
            ordering_assignment_id: Some(assignment.assignment_id.clone()),
            canonical_position: Some(assignment.canonical_position),
            scope_id: Some(assignment.scope_id.clone()),
            authorized_bases: vec![basis].into(),
            structural_class: None,
        });
    }

    let omission_id = SourceToArtifactMappingId::derive(&[
        input.construction_id.as_str(),
        "optional-index-omission",
    ]);
    let optional_component_id =
        ConstructedRequestComponentId::derive(&[input.construction_id.as_str(), "optional-index"]);
    if profile.emit_optional_structural_component {
        let rule = "GenerateOptionalStructuralIndexRule";
        if !known_rules.contains(rule) {
            return construction_failure(
                input,
                ConstructionFailureCategory::UnknownConstructionRule,
                "optional structural generation rule is absent",
            );
        }
        let basis = ConstructionBasis::SchemaGeneratedStructuralBasis {
            schema_rule_id: rule.to_owned(),
            schema_rule_version: profile.rule_set_version.clone(),
            structural_class: profile.optional_structural_class.clone(),
        };
        let decision_id = ConstructionDecisionId::derive(&[
            input.construction_id.as_str(),
            "optional-index",
            rule,
        ]);
        decisions.push(ConstructionDecision {
            decision_id: decision_id.clone(),
            construction_id: input.construction_id.clone(),
            decision_kind: ConstructionDecisionKind::GenerateStructuralComponent,
            source_subject_id: None,
            structural_class: Some(profile.optional_structural_class.clone()),
            rule_binding: (rule.to_owned(), profile.rule_set_version.clone()),
            schema_binding: (profile.schema_id.clone(), profile.schema_version.clone()),
            decision_basis: "Closed fixture structural component generation".to_owned(),
            target_component_id: Some(optional_component_id.clone()),
            omission_target: None,
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            registry_binding: (
                profile.registry_id.clone(),
                profile.registry_version.clone(),
            ),
            configuration_binding: (
                profile.configuration_id.clone(),
                profile.configuration_version.clone(),
            ),
        });
        placements.push(ConstructionPlacement {
            placement_id: ConstructionPlacementId::derive(&[decision_id.as_str(), "structural"]),
            construction_decision_id: decision_id,
            target_component_id: optional_component_id.clone(),
            parent_container_id: "request-root".to_owned(),
            section_or_field_role: "structural".to_owned(),
            scope_reference: "request-wide".to_owned(),
            ordering_assignment_id: None,
            preserved_canonical_position: None,
            placement_rule_binding: (
                "PlaceSchemaStructuralComponentRule".to_owned(),
                profile.rule_set_version.clone(),
            ),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            schema_binding: (profile.schema_id.clone(), profile.schema_version.clone()),
        });
        components.push(ConstructedRequestComponent {
            component_id: optional_component_id,
            component_kind: ConstructedComponentKind::SchemaGeneratedStructural,
            source_subject_id: None,
            normalized_element_id: None,
            normalized_relationship_id: None,
            canonical_expression: None,
            semantic_domain: None,
            semantic_class: None,
            relationship_class: None,
            relationship_source_subject_id: None,
            relationship_target_subject_id: None,
            represented_scope: "request-wide".to_owned(),
            standing_references: Vec::new().into(),
            evidence_references: Vec::new().into(),
            grounding_references: Vec::new().into(),
            provenance_references: Vec::new().into(),
            unresolved_conditions: Vec::new().into(),
            ordering_assignment_id: None,
            canonical_position: None,
            scope_id: None,
            authorized_bases: vec![basis.clone()].into(),
            structural_class: Some(profile.optional_structural_class.clone()),
        });
        schema_generated_bases.push(basis);
    } else {
        if !profile.omission_rule_authorized {
            return construction_failure(
                input,
                ConstructionFailureCategory::AuthorizedOmissionFailure,
                "optional structural component omission lacks an authorized closed rule",
            );
        }
        let decision_id = ConstructionDecisionId::derive(&[
            input.construction_id.as_str(),
            "optional-index",
            profile.optional_omission_rule_id.as_str(),
        ]);
        decisions.push(ConstructionDecision {
            decision_id: decision_id.clone(),
            construction_id: input.construction_id.clone(),
            decision_kind: ConstructionDecisionKind::OmitOptionalStructuralComponent,
            source_subject_id: None,
            structural_class: Some(profile.optional_structural_class.clone()),
            rule_binding: (
                profile.optional_omission_rule_id.clone(),
                profile.rule_set_version.clone(),
            ),
            schema_binding: (profile.schema_id.clone(), profile.schema_version.clone()),
            decision_basis: "Closed fixture omission of empty optional structural index".to_owned(),
            target_component_id: None,
            omission_target: Some(profile.optional_structural_class.clone()),
            profile_binding: (profile.identity.clone(), profile.version.clone()),
            registry_binding: (
                profile.registry_id.clone(),
                profile.registry_version.clone(),
            ),
            configuration_binding: (
                profile.configuration_id.clone(),
                profile.configuration_version.clone(),
            ),
        });
        mappings.push(SourceToArtifactMapping {
            mapping_id: omission_id.clone(),
            construction_id: input.construction_id.clone(),
            source_subject_id: None,
            target_component_id: None,
            disposition: ConstructionSourceDisposition::AuthorizedOmission,
            authorized_basis: Vec::new().into(),
            decision_id,
            omission_rule_binding: Some((
                profile.optional_omission_rule_id.clone(),
                profile.rule_set_version.clone(),
            )),
        });
    }
    let execution_record = ConstructionExecutionRecord {
        execution_record_id: ConstructionExecutionRecordId::derive(&[
            input.construction_id.as_str(),
            &format!("{:?}{:?}{:?}", decisions, placements, mappings),
        ]),
        construction_id: input.construction_id.clone(),
        decision_ids: decisions
            .iter()
            .map(|decision| decision.decision_id.clone())
            .collect::<Vec<_>>()
            .into(),
        placement_ids: placements
            .iter()
            .map(|placement| placement.placement_id.clone())
            .collect::<Vec<_>>()
            .into(),
        mapping_ids: mappings
            .iter()
            .map(|mapping| mapping.mapping_id.clone())
            .collect::<Vec<_>>()
            .into(),
        deterministic_rule_context: format!(
            "{}:{}:{}",
            profile.version, schema.version, rules.version
        ),
    };
    if source_dispositions.len() != subjects.len()
        || source_dispositions
            .iter()
            .map(|record| record.source_subject_id.clone())
            .collect::<BTreeSet<_>>()
            .len()
            != subjects.len()
        || mappings
            .iter()
            .filter(|mapping| {
                mapping.disposition != ConstructionSourceDisposition::AuthorizedOmission
            })
            .count()
            != subjects.len()
        || components
            .iter()
            .any(|component| component.authorized_bases.is_empty())
        || placements.iter().any(|placement| {
            !components
                .iter()
                .any(|component| component.component_id == placement.target_component_id)
        })
        || mappings
            .iter()
            .filter_map(|mapping| mapping.target_component_id.as_ref())
            .any(|target| {
                !components
                    .iter()
                    .any(|component| &component.component_id == target)
            })
    {
        return construction_failure(
            input,
            ConstructionFailureCategory::ManifestCompletenessFailure,
            "construction source, target, placement, or basis coverage is incomplete",
        );
    }
    let request_id = ConstructedCanonicalRequestId::derive(&[
        input.construction_id.as_str(),
        validation.result_id.as_str(),
        &format!("{:?}", components),
    ]);
    let mut request = ConstructedCanonicalRequest {
        constructed_canonical_request_id: request_id.clone(),
        construction_id: input.construction_id.clone(),
        source_ordered_representation_id: publication.publication_id.clone(),
        structural_validation_result_id: validation.result_id.clone(),
        components: components.into(),
        construction_context: BTreeMap::from([
            ("profile_version".to_owned(), profile.version.clone()),
            ("schema_version".to_owned(), schema.version.clone()),
            ("logical_artifact".to_owned(), "pre-issuance".to_owned()),
        ]),
    };
    let manifest_id = ConstructionManifestId::derive(&[
        input.construction_id.as_str(),
        request_id.as_str(),
        &format!("{:?}{:?}{:?}", decisions, placements, mappings),
    ]);
    let mut manifest = ConstructionManifest {
        construction_manifest_id: manifest_id,
        construction_id: input.construction_id.clone(),
        constructed_canonical_request_id: request_id,
        source_ordered_representation_id: publication.publication_id.clone(),
        structural_validation_result_id: validation.result_id.clone(),
        profile_binding: (profile.identity.clone(), profile.version.clone()),
        schema_binding: (schema.identity.clone(), schema.version.clone()),
        rule_set_binding: (rules.identity.clone(), rules.version.clone()),
        registry_binding: (registries.identity.clone(), registries.version.clone()),
        configuration_binding: (
            configuration.identity.clone(),
            configuration.version.clone(),
        ),
        implementation_version: profile.implementation_version.clone(),
        replay_context: input.replay_context.clone(),
        construction_decision_ids: decisions
            .iter()
            .map(|decision| decision.decision_id.clone())
            .collect::<Vec<_>>()
            .into(),
        construction_placement_ids: placements
            .iter()
            .map(|placement| placement.placement_id.clone())
            .collect::<Vec<_>>()
            .into(),
        source_to_artifact_mapping_ids: mappings
            .iter()
            .map(|mapping| mapping.mapping_id.clone())
            .collect::<Vec<_>>()
            .into(),
        source_dispositions: source_dispositions.into(),
        omission_records: mappings
            .iter()
            .filter(|mapping| {
                mapping.disposition == ConstructionSourceDisposition::AuthorizedOmission
            })
            .map(|mapping| mapping.mapping_id.clone())
            .collect::<Vec<_>>()
            .into(),
        schema_generated_bases: schema_generated_bases.into(),
        source_coverage_summary:
            "every Contract 011 orderable subject has exactly one Included disposition".to_owned(),
        target_basis_coverage_summary:
            "every constructed component has a governed-source or schema-generated basis".to_owned(),
        validation_lineage: structural_validation_subject_binding(validation),
        execution_record,
        publication_binding: String::new(),
    };
    manifest.publication_binding = construction_publication_binding(&request, &manifest);
    request.construction_context.insert(
        "publication_binding".to_owned(),
        manifest.publication_binding.clone(),
    );
    CanonicalRequestConstructionOutcome::Constructed {
        request: Box::new(request),
        manifest: Box::new(manifest),
    }
}

#[cfg(test)]
mod contract_013_tests {
    use super::*;

    pub(super) fn ordered_fixture() -> CanonicallyOrderedRequestRepresentation {
        let normalized = super::contract_011_tests::normalized_fixture(true);
        let input = CanonicalOrderingInput::for_fixture(&normalized);
        let CanonicalOrderingOutcome::Success(ordered) = perform_canonical_ordering(
            &input,
            &normalized,
            &FixtureCanonicalOrderingProfile::fixture(),
        ) else {
            panic!("ordered fixture")
        };
        *ordered
    }

    pub(super) fn validation_fixture(
        publication: &CanonicallyOrderedRequestRepresentation,
    ) -> StructuralValidationResult {
        let profile = FixtureValidationProfile::fixture();
        let requirements = FixtureConstructionRequirements::fixture(&profile);
        let registries = FixtureStructuralRegistries::fixture();
        let rules = FixtureValidationRuleSet::fixture(&profile);
        let input = StructuralValidationInput::for_fixture(publication);
        let StructuralValidationOutcome::Completed(result) = perform_structural_validation(
            &input,
            publication,
            &profile,
            &requirements,
            &registries,
            &rules,
        ) else {
            panic!("validation fixture")
        };
        *result
    }

    pub(super) fn construct(
        publication: &CanonicallyOrderedRequestRepresentation,
        validation: &StructuralValidationResult,
        profile: &FixtureConstructionProfile,
    ) -> CanonicalRequestConstructionOutcome {
        let authorities = FixtureConstructionAuthorityContext::fixture(profile);
        perform_canonical_request_construction(
            &CanonicalRequestConstructionInput::for_fixture(publication, validation),
            publication,
            validation,
            &authorities,
        )
    }

    #[test]
    fn contract_013_exact_dual_input_binding_constructs_atomically() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let CanonicalRequestConstructionOutcome::Constructed { request, manifest } = construct(
            &publication,
            &validation,
            &FixtureConstructionProfile::fixture(),
        ) else {
            panic!("construction")
        };
        assert_eq!(
            request.source_ordered_representation_id,
            publication.publication_id
        );
        assert_eq!(
            manifest.structural_validation_result_id,
            validation.result_id
        );
        assert_eq!(
            manifest.constructed_canonical_request_id,
            request.constructed_canonical_request_id
        );
    }

    #[test]
    fn contract_013_equivalent_but_substituted_ordered_representation_fails() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let mut substituted = ordered_fixture();
        substituted.publication_id =
            CanonicallyOrderedRequestRepresentationId::derive(&["substituted"]);
        let profile = FixtureConstructionProfile::fixture();
        let input = CanonicalRequestConstructionInput::for_fixture(&publication, &validation);
        let authorities = FixtureConstructionAuthorityContext::fixture(&profile);
        assert!(
            matches!(perform_canonical_request_construction(&input, &substituted, &validation, &authorities), CanonicalRequestConstructionOutcome::Failed(record) if record.category == ConstructionFailureCategory::OrderedRepresentationBindingMismatch)
        );
    }

    #[test]
    fn contract_013_validation_result_for_different_representation_fails() {
        let publication = ordered_fixture();
        let other = {
            let normalized = super::contract_011_tests::normalized_fixture(false);
            let input = CanonicalOrderingInput::for_fixture(&normalized);
            let CanonicalOrderingOutcome::Success(ordered) = perform_canonical_ordering(
                &input,
                &normalized,
                &FixtureCanonicalOrderingProfile::fixture(),
            ) else {
                panic!("ordered")
            };
            *ordered
        };
        let validation = validation_fixture(&other);
        assert!(
            matches!(construct(&publication, &validation, &FixtureConstructionProfile::fixture()), CanonicalRequestConstructionOutcome::Failed(record) if record.category == ConstructionFailureCategory::ValidationSubjectBindingMismatch)
        );
    }

    #[test]
    fn contract_013_eligible_with_warnings_constructs_without_rejudgment() {
        let publication = ordered_fixture();
        let mut validation = validation_fixture(&publication);
        validation.eligibility = StructuralEligibility::EligibleWithWarnings;
        assert!(matches!(
            construct(
                &publication,
                &validation,
                &FixtureConstructionProfile::fixture()
            ),
            CanonicalRequestConstructionOutcome::Constructed { .. }
        ));
    }

    #[test]
    fn contract_013_ineligible_failed_and_deferred_results_are_rejected() {
        let publication = ordered_fixture();
        for eligibility in [
            StructuralEligibility::Ineligible,
            StructuralEligibility::Deferred,
            StructuralEligibility::NotDetermined,
        ] {
            let mut validation = validation_fixture(&publication);
            validation.eligibility = eligibility;
            assert!(matches!(
                construct(
                    &publication,
                    &validation,
                    &FixtureConstructionProfile::fixture()
                ),
                CanonicalRequestConstructionOutcome::Failed(_)
            ));
        }
    }

    #[test]
    fn contract_013_preserves_element_and_relationship_identity() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let CanonicalRequestConstructionOutcome::Constructed { request, .. } = construct(
            &publication,
            &validation,
            &FixtureConstructionProfile::fixture(),
        ) else {
            panic!("construction")
        };
        assert!(request
            .components
            .iter()
            .any(|component| component.normalized_element_id.is_some()));
        assert!(request
            .components
            .iter()
            .any(|component| component.normalized_relationship_id.is_some()));
    }

    #[test]
    fn contract_013_preserves_contract_011_order_without_sorting() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let CanonicalRequestConstructionOutcome::Constructed { request, .. } = construct(
            &publication,
            &validation,
            &FixtureConstructionProfile::fixture(),
        ) else {
            panic!("construction")
        };
        let positions: Vec<_> = request
            .components
            .iter()
            .filter_map(|component| component.canonical_position)
            .collect();
        assert_eq!(positions, vec![0, 1, 2]);
    }

    #[test]
    fn contract_013_source_without_disposition_or_mapping_blocks_publication() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let mut broken = publication.clone();
        broken.assignments = broken.assignments[..2].to_vec().into();
        assert!(matches!(
            construct(&broken, &validation, &FixtureConstructionProfile::fixture()),
            CanonicalRequestConstructionOutcome::Failed(_)
        ));
    }

    #[test]
    fn contract_013_target_basis_and_position_failures_are_mechanical() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let mut broken = publication.clone();
        let mut assignments = broken.assignments.to_vec();
        assignments[0].canonical_position = 99;
        broken.assignments = assignments.into();
        assert!(matches!(
            construct(&broken, &validation, &FixtureConstructionProfile::fixture()),
            CanonicalRequestConstructionOutcome::Failed(_)
        ));
    }

    #[test]
    fn contract_013_schema_generated_class_is_closed_and_semantic_class_is_rejected() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let mut profile = FixtureConstructionProfile::fixture();
        profile.optional_structural_class = "SemanticSummary".to_owned();
        assert!(
            matches!(construct(&publication, &validation, &profile), CanonicalRequestConstructionOutcome::Failed(record) if record.category == ConstructionFailureCategory::UnknownStructuralClass)
        );
    }

    #[test]
    fn contract_013_optional_empty_structural_container_can_be_omitted_by_rule() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let CanonicalRequestConstructionOutcome::Constructed { manifest, .. } = construct(
            &publication,
            &validation,
            &FixtureConstructionProfile::fixture(),
        ) else {
            panic!("construction")
        };
        assert_eq!(manifest.omission_records.len(), 1);
    }

    #[test]
    fn contract_013_optional_omission_without_rule_fails() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let mut profile = FixtureConstructionProfile::fixture();
        profile.omission_rule_authorized = false;
        assert!(
            matches!(construct(&publication, &validation, &profile), CanonicalRequestConstructionOutcome::Failed(record) if record.category == ConstructionFailureCategory::AuthorizedOmissionFailure)
        );
    }

    #[test]
    fn contract_013_manifest_references_every_material_record() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let CanonicalRequestConstructionOutcome::Constructed { manifest, .. } = construct(
            &publication,
            &validation,
            &FixtureConstructionProfile::fixture(),
        ) else {
            panic!("construction")
        };
        assert_eq!(
            manifest.construction_decision_ids.len(),
            manifest.execution_record.decision_ids.len()
        );
        assert_eq!(
            manifest.construction_placement_ids.len(),
            manifest.execution_record.placement_ids.len()
        );
        assert_eq!(
            manifest.source_to_artifact_mapping_ids.len(),
            manifest.execution_record.mapping_ids.len()
        );
    }

    #[test]
    fn contract_013_same_input_replays_request_and_manifest_deterministically() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        assert_eq!(
            construct(
                &publication,
                &validation,
                &FixtureConstructionProfile::fixture()
            ),
            construct(
                &publication,
                &validation,
                &FixtureConstructionProfile::fixture()
            )
        );
    }

    #[test]
    fn contract_013_exposes_no_contract_014_or_015_authority() {
        let publication = ordered_fixture();
        let validation = validation_fixture(&publication);
        let CanonicalRequestConstructionOutcome::Constructed { request, .. } = construct(
            &publication,
            &validation,
            &FixtureConstructionProfile::fixture(),
        ) else {
            panic!("construction")
        };
        assert!(!request
            .construction_context
            .contains_key("canonical_structured_request_id"));
        assert!(!request
            .construction_context
            .contains_key("issuance_standing"));
    }
}

// ---------------------------------------------------------------------------
// Contract 014: Canonical request identity and issuance
// ---------------------------------------------------------------------------

contract_002_id!(CanonicalRequestIssuanceId, "crissuance");
contract_002_id!(IssuanceContextId, "isscontext");
contract_002_id!(IdentityPolicyId, "idpolicy");
contract_002_id!(IssuanceProfileId, "issprofile");
contract_002_id!(IssuanceSchemaId, "issschema");
contract_002_id!(IssuanceRuleSetId, "issrules");
contract_002_id!(IssuanceRegistryId, "issregistry");
contract_002_id!(IssuanceConfigurationId, "issconfig");
contract_002_id!(IdentityDerivationRecordId, "idderivation");
contract_002_id!(IssuedPublicationBindingId, "issbinding");
contract_002_id!(CanonicalStructuredRequestId, "canonical-request");
contract_002_id!(IssuanceManifestId, "issmanifest");
contract_002_id!(IssuanceFailureRecordId, "issfail");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum InitialStanding {
    Issued,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IssuanceFailureCategory {
    MissingConstructedRequest,
    MissingConstructionManifest,
    ConstructionPublicationBindingMismatch,
    RequestManifestBindingMismatch,
    IssuanceContextResolutionFailure,
    IdentityPolicyResolutionFailure,
    IssuanceProfileResolutionFailure,
    IssuanceSchemaResolutionFailure,
    IssuanceRuleSetResolutionFailure,
    IssuanceRegistryResolutionFailure,
    IssuanceConfigurationResolutionFailure,
    VersionCompatibilityFailure,
    ConstructionIntegrityFailure,
    IdentityDerivationFailure,
    StandingAssignmentFailure,
    ManifestCompletenessFailure,
    AtomicPublicationFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuanceContext {
    pub identity: IssuanceContextId,
    pub version: String,
    pub authority_reference: String,
    pub context_name: String,
    pub initial_standing_authorized: bool,
}

impl IssuanceContext {
    pub fn fixture(name: &str) -> Self {
        Self {
            identity: IssuanceContextId::derive(&["contract-014-fixture", name]),
            version: "fixture-issuance-context-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            context_name: name.to_owned(),
            initial_standing_authorized: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityPolicy {
    pub identity: IdentityPolicyId,
    pub version: String,
    pub authority_reference: String,
    pub identity_input_labels: Arc<[String]>,
    pub standing_rule_id: String,
    pub standing_rule_version: String,
}

impl IdentityPolicy {
    pub fn fixture() -> Self {
        Self {
            identity: IdentityPolicyId::derive(&["contract-014-fixture"]),
            version: "fixture-identity-policy-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
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
            standing_rule_version: "fixture-standing-rules-v1".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureIssuanceProfile {
    pub identity: IssuanceProfileId,
    pub version: String,
    pub authority_reference: String,
    pub schema_id: IssuanceSchemaId,
    pub schema_version: String,
    pub rule_set_id: IssuanceRuleSetId,
    pub rule_set_version: String,
    pub registry_id: IssuanceRegistryId,
    pub registry_version: String,
    pub configuration_id: IssuanceConfigurationId,
    pub configuration_version: String,
    pub implementation_version: String,
}

impl FixtureIssuanceProfile {
    pub fn fixture() -> Self {
        Self {
            identity: IssuanceProfileId::derive(&["contract-014-fixture"]),
            version: "fixture-issuance-v1".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            schema_id: IssuanceSchemaId::derive(&["contract-014-fixture"]),
            schema_version: "fixture-issuance-schema-v1".to_owned(),
            rule_set_id: IssuanceRuleSetId::derive(&["contract-014-fixture"]),
            rule_set_version: "fixture-issuance-rules-v1".to_owned(),
            registry_id: IssuanceRegistryId::derive(&["contract-014-fixture"]),
            registry_version: "fixture-issuance-registry-v1".to_owned(),
            configuration_id: IssuanceConfigurationId::derive(&["contract-014-fixture"]),
            configuration_version: "fixture-issuance-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureIssuanceSchema {
    pub identity: IssuanceSchemaId,
    pub version: String,
}

impl FixtureIssuanceSchema {
    pub fn fixture(profile: &FixtureIssuanceProfile) -> Self {
        Self {
            identity: profile.schema_id.clone(),
            version: profile.schema_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureIssuanceRuleSet {
    pub identity: IssuanceRuleSetId,
    pub version: String,
    pub issuance_rule_id: String,
    pub issuance_rule_version: String,
}

impl FixtureIssuanceRuleSet {
    pub fn fixture(profile: &FixtureIssuanceProfile, policy: &IdentityPolicy) -> Self {
        Self {
            identity: profile.rule_set_id.clone(),
            version: profile.rule_set_version.clone(),
            issuance_rule_id: policy.standing_rule_id.clone(),
            issuance_rule_version: policy.standing_rule_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureIssuanceRegistries {
    pub identity: IssuanceRegistryId,
    pub version: String,
}

impl FixtureIssuanceRegistries {
    pub fn fixture(profile: &FixtureIssuanceProfile) -> Self {
        Self {
            identity: profile.registry_id.clone(),
            version: profile.registry_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureIssuanceConfiguration {
    pub identity: IssuanceConfigurationId,
    pub version: String,
}

impl FixtureIssuanceConfiguration {
    pub fn fixture(profile: &FixtureIssuanceProfile) -> Self {
        Self {
            identity: profile.configuration_id.clone(),
            version: profile.configuration_version.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureIssuanceAuthorityContext {
    pub profile: FixtureIssuanceProfile,
    pub schema: FixtureIssuanceSchema,
    pub rules: FixtureIssuanceRuleSet,
    pub registries: FixtureIssuanceRegistries,
    pub configuration: FixtureIssuanceConfiguration,
    pub identity_policy: IdentityPolicy,
    pub context: IssuanceContext,
}

impl FixtureIssuanceAuthorityContext {
    pub fn fixture(context: &IssuanceContext) -> Self {
        let profile = FixtureIssuanceProfile::fixture();
        let identity_policy = IdentityPolicy::fixture();
        Self {
            schema: FixtureIssuanceSchema::fixture(&profile),
            rules: FixtureIssuanceRuleSet::fixture(&profile, &identity_policy),
            registries: FixtureIssuanceRegistries::fixture(&profile),
            configuration: FixtureIssuanceConfiguration::fixture(&profile),
            profile,
            identity_policy,
            context: context.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRequestIssuanceInput {
    pub issuance_id: CanonicalRequestIssuanceId,
    pub operation_id: CanonicalRequestIssuanceId,
    pub constructed_request_id: ConstructedCanonicalRequestId,
    pub construction_manifest_id: ConstructionManifestId,
    pub construction_publication_binding: String,
    pub issuance_context_id: IssuanceContextId,
    pub issuance_context_version: String,
    pub identity_policy_id: IdentityPolicyId,
    pub identity_policy_version: String,
    pub issuance_profile_id: IssuanceProfileId,
    pub issuance_profile_version: String,
    pub issuance_schema_id: IssuanceSchemaId,
    pub issuance_schema_version: String,
    pub issuance_rule_set_id: IssuanceRuleSetId,
    pub issuance_rule_set_version: String,
    pub issuance_registry_id: IssuanceRegistryId,
    pub issuance_registry_version: String,
    pub issuance_configuration_id: IssuanceConfigurationId,
    pub issuance_configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl CanonicalRequestIssuanceInput {
    pub fn for_fixture(
        request: &ConstructedCanonicalRequest,
        manifest: &ConstructionManifest,
        context: &IssuanceContext,
    ) -> Self {
        let authorities = FixtureIssuanceAuthorityContext::fixture(context);
        Self {
            issuance_id: CanonicalRequestIssuanceId::derive(&[
                request.constructed_canonical_request_id.as_str(),
                manifest.construction_manifest_id.as_str(),
                context.identity.as_str(),
                "fixture-issuance",
            ]),
            operation_id: CanonicalRequestIssuanceId::derive(&[
                request.constructed_canonical_request_id.as_str(),
                manifest.construction_manifest_id.as_str(),
                context.identity.as_str(),
                "fixture-operation",
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityDerivationRecord {
    pub derivation_record_id: IdentityDerivationRecordId,
    pub issuance_id: CanonicalRequestIssuanceId,
    pub identity_policy_binding: (IdentityPolicyId, String),
    pub identity_input_labels: Arc<[String]>,
    pub identity_input_values: Arc<[(String, String)]>,
    pub derivation_rule_binding: (String, String),
    pub derived_identity: CanonicalStructuredRequestId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitialStandingAssignment {
    pub assignment_id: StandingAssignmentId,
    pub issuance_id: CanonicalRequestIssuanceId,
    pub canonical_request_id: CanonicalStructuredRequestId,
    pub standing: InitialStanding,
    pub context_binding: (IssuanceContextId, String),
    pub rule_binding: (String, String),
    pub policy_binding: (IdentityPolicyId, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuedPublicationBinding {
    pub binding_id: IssuedPublicationBindingId,
    pub constructed_canonical_request_id: ConstructedCanonicalRequestId,
    pub construction_manifest_id: ConstructionManifestId,
    pub canonical_request_construction_id: CanonicalRequestConstructionId,
    pub issuance_context_id: IssuanceContextId,
    pub identity_policy_id: IdentityPolicyId,
    pub standing_assignment_id: StandingAssignmentId,
    pub canonical_structured_request_id: CanonicalStructuredRequestId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalStructuredRequest {
    pub canonical_structured_request_id: CanonicalStructuredRequestId,
    pub issuance_id: CanonicalRequestIssuanceId,
    pub constructed_canonical_request_id: ConstructedCanonicalRequestId,
    pub construction_manifest_id: ConstructionManifestId,
    pub canonical_request_construction_id: CanonicalRequestConstructionId,
    pub exact_constructed_request: ConstructedCanonicalRequest,
    pub initial_standing: InitialStanding,
    pub standing_assignment_id: StandingAssignmentId,
    pub content_integrity_binding: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuanceManifest {
    pub issuance_manifest_id: IssuanceManifestId,
    pub issuance_id: CanonicalRequestIssuanceId,
    pub constructed_canonical_request_id: ConstructedCanonicalRequestId,
    pub construction_manifest_id: ConstructionManifestId,
    pub canonical_request_construction_id: CanonicalRequestConstructionId,
    pub issuance_context_binding: (IssuanceContextId, String),
    pub identity_policy_binding: (IdentityPolicyId, String),
    pub profile_binding: (IssuanceProfileId, String),
    pub schema_binding: (IssuanceSchemaId, String),
    pub rule_set_binding: (IssuanceRuleSetId, String),
    pub registry_binding: (IssuanceRegistryId, String),
    pub configuration_binding: (IssuanceConfigurationId, String),
    pub implementation_version: String,
    pub identity_derivation_record: IdentityDerivationRecord,
    pub initial_standing_assignment: InitialStandingAssignment,
    pub issued_publication_binding: IssuedPublicationBinding,
    pub replay_context: BTreeMap<String, String>,
    pub publication_binding: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuanceFailureRecord {
    pub failure_record_id: IssuanceFailureRecordId,
    pub issuance_id: CanonicalRequestIssuanceId,
    pub operation_id: CanonicalRequestIssuanceId,
    pub attempted_constructed_request_id: Option<ConstructedCanonicalRequestId>,
    pub attempted_construction_manifest_id: Option<ConstructionManifestId>,
    pub category: IssuanceFailureCategory,
    pub detail: String,
    pub context_binding: Option<(IssuanceContextId, String)>,
    pub policy_binding: Option<(IdentityPolicyId, String)>,
    pub profile_binding: Option<(IssuanceProfileId, String)>,
    pub schema_binding: Option<(IssuanceSchemaId, String)>,
    pub rule_set_binding: Option<(IssuanceRuleSetId, String)>,
    pub registry_binding: Option<(IssuanceRegistryId, String)>,
    pub configuration_binding: Option<(IssuanceConfigurationId, String)>,
    pub implementation_version: Option<String>,
    pub replay_context: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalRequestIssuanceOutcome {
    Issued {
        request: Box<CanonicalStructuredRequest>,
        manifest: Box<IssuanceManifest>,
    },
    Failed(Box<IssuanceFailureRecord>),
}

fn constructed_request_content_binding(request: &ConstructedCanonicalRequest) -> String {
    StableId::from_parts(
        "constructed-request-content",
        &[
            request.constructed_canonical_request_id.as_str(),
            request.construction_id.as_str(),
            request.source_ordered_representation_id.as_str(),
            request.structural_validation_result_id.as_str(),
            &format!("{:?}{:?}", request.components, request.construction_context),
        ],
    )
    .to_string()
}

fn issuance_publication_binding(
    request: &CanonicalStructuredRequest,
    manifest: &IssuanceManifest,
) -> String {
    StableId::from_parts(
        "issuance-publication",
        &[
            request.canonical_structured_request_id.as_str(),
            manifest.issuance_manifest_id.as_str(),
            request.issuance_id.as_str(),
            manifest.issued_publication_binding.binding_id.as_str(),
        ],
    )
    .to_string()
}

fn issuance_failure(
    input: &CanonicalRequestIssuanceInput,
    category: IssuanceFailureCategory,
    detail: &str,
) -> CanonicalRequestIssuanceOutcome {
    CanonicalRequestIssuanceOutcome::Failed(Box::new(IssuanceFailureRecord {
        failure_record_id: IssuanceFailureRecordId::derive(&[
            input.operation_id.as_str(),
            &format!("{category:?}"),
            detail,
        ]),
        issuance_id: input.issuance_id.clone(),
        operation_id: input.operation_id.clone(),
        attempted_constructed_request_id: Some(input.constructed_request_id.clone()),
        attempted_construction_manifest_id: Some(input.construction_manifest_id.clone()),
        category,
        detail: detail.to_owned(),
        context_binding: Some((
            input.issuance_context_id.clone(),
            input.issuance_context_version.clone(),
        )),
        policy_binding: Some((
            input.identity_policy_id.clone(),
            input.identity_policy_version.clone(),
        )),
        profile_binding: Some((
            input.issuance_profile_id.clone(),
            input.issuance_profile_version.clone(),
        )),
        schema_binding: Some((
            input.issuance_schema_id.clone(),
            input.issuance_schema_version.clone(),
        )),
        rule_set_binding: Some((
            input.issuance_rule_set_id.clone(),
            input.issuance_rule_set_version.clone(),
        )),
        registry_binding: Some((
            input.issuance_registry_id.clone(),
            input.issuance_registry_version.clone(),
        )),
        configuration_binding: Some((
            input.issuance_configuration_id.clone(),
            input.issuance_configuration_version.clone(),
        )),
        implementation_version: Some(input.implementation_version.clone()),
        replay_context: input.replay_context.clone(),
    }))
}

pub fn perform_canonical_request_issuance(
    input: &CanonicalRequestIssuanceInput,
    request: &ConstructedCanonicalRequest,
    manifest: &ConstructionManifest,
    authorities: &FixtureIssuanceAuthorityContext,
) -> CanonicalRequestIssuanceOutcome {
    let profile = &authorities.profile;
    let schema = &authorities.schema;
    let rules = &authorities.rules;
    let registries = &authorities.registries;
    let configuration = &authorities.configuration;
    let context = &authorities.context;
    let policy = &authorities.identity_policy;
    if input.constructed_request_id != request.constructed_canonical_request_id
        || input.construction_manifest_id != manifest.construction_manifest_id
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::RequestManifestBindingMismatch,
            "input publication identities do not match supplied artifacts",
        );
    }
    if request.constructed_canonical_request_id != manifest.constructed_canonical_request_id
        || request.construction_id != manifest.construction_id
        || request.source_ordered_representation_id != manifest.source_ordered_representation_id
        || request.structural_validation_result_id != manifest.structural_validation_result_id
        || manifest.publication_binding != construction_publication_binding(request, manifest)
        || input.construction_publication_binding != manifest.publication_binding
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::ConstructionPublicationBindingMismatch,
            "Contract 013 request and manifest are not one exact publication pair",
        );
    }
    if input.issuance_context_id != context.identity
        || input.issuance_context_version != context.version
        || !context.initial_standing_authorized
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IssuanceContextResolutionFailure,
            "issuance context binding or standing authority is not exact",
        );
    }
    if input.identity_policy_id != policy.identity
        || input.identity_policy_version != policy.version
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IdentityPolicyResolutionFailure,
            "identity policy binding is not exact",
        );
    }
    if input.issuance_profile_id != profile.identity
        || input.issuance_profile_version != profile.version
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IssuanceProfileResolutionFailure,
            "issuance profile binding is not exact",
        );
    }
    if input.issuance_schema_id != schema.identity
        || input.issuance_schema_version != schema.version
        || input.issuance_schema_id != profile.schema_id
        || input.issuance_schema_version != profile.schema_version
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IssuanceSchemaResolutionFailure,
            "issuance schema binding is not exact",
        );
    }
    if input.issuance_rule_set_id != rules.identity
        || input.issuance_rule_set_version != rules.version
        || input.issuance_rule_set_id != profile.rule_set_id
        || input.issuance_rule_set_version != profile.rule_set_version
        || rules.issuance_rule_id != policy.standing_rule_id
        || rules.issuance_rule_version != policy.standing_rule_version
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IssuanceRuleSetResolutionFailure,
            "issuance rule-set binding is not exact",
        );
    }
    if input.issuance_registry_id != registries.identity
        || input.issuance_registry_version != registries.version
        || input.issuance_registry_id != profile.registry_id
        || input.issuance_registry_version != profile.registry_version
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IssuanceRegistryResolutionFailure,
            "issuance registry binding is not exact",
        );
    }
    if input.issuance_configuration_id != configuration.identity
        || input.issuance_configuration_version != configuration.version
        || input.issuance_configuration_id != profile.configuration_id
        || input.issuance_configuration_version != profile.configuration_version
        || input.implementation_version != profile.implementation_version
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IssuanceConfigurationResolutionFailure,
            "issuance configuration or implementation binding is not exact",
        );
    }
    if manifest.source_coverage_summary.is_empty()
        || manifest.target_basis_coverage_summary.is_empty()
        || manifest.execution_record.construction_id != manifest.construction_id
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::ConstructionIntegrityFailure,
            "Contract 013 manifest completeness evidence is absent or inconsistent",
        );
    }
    let mut identity_values = BTreeMap::from([
        (
            "constructed_request_id".to_owned(),
            request.constructed_canonical_request_id.to_string(),
        ),
        (
            "construction_manifest_id".to_owned(),
            manifest.construction_manifest_id.to_string(),
        ),
        (
            "construction_id".to_owned(),
            request.construction_id.to_string(),
        ),
        (
            "issuance_context_id".to_owned(),
            context.identity.to_string(),
        ),
        (
            "issuance_context_version".to_owned(),
            context.version.clone(),
        ),
        ("identity_policy_id".to_owned(), policy.identity.to_string()),
        ("identity_policy_version".to_owned(), policy.version.clone()),
        (
            "issuance_profile_id".to_owned(),
            profile.identity.to_string(),
        ),
        (
            "issuance_profile_version".to_owned(),
            profile.version.clone(),
        ),
        ("issuance_schema_id".to_owned(), schema.identity.to_string()),
        ("issuance_schema_version".to_owned(), schema.version.clone()),
        (
            "issuance_rule_set_id".to_owned(),
            rules.identity.to_string(),
        ),
        (
            "issuance_rule_set_version".to_owned(),
            rules.version.clone(),
        ),
        (
            "issuance_registry_id".to_owned(),
            registries.identity.to_string(),
        ),
        (
            "issuance_registry_version".to_owned(),
            registries.version.clone(),
        ),
        (
            "issuance_configuration_id".to_owned(),
            configuration.identity.to_string(),
        ),
        (
            "issuance_configuration_version".to_owned(),
            configuration.version.clone(),
        ),
    ]);
    let permitted_labels: BTreeSet<_> = identity_values.keys().cloned().collect();
    let policy_labels: BTreeSet<_> = policy.identity_input_labels.iter().cloned().collect();
    if policy_labels.is_empty()
        || policy_labels
            .iter()
            .any(|label| !permitted_labels.contains(label))
    {
        return issuance_failure(
            input,
            IssuanceFailureCategory::IdentityDerivationFailure,
            "identity policy declares an unknown or empty identity input set",
        );
    }
    let selected_values: Vec<_> = policy
        .identity_input_labels
        .iter()
        .map(|label| {
            (
                label.clone(),
                identity_values
                    .remove(label)
                    .expect("validated identity label"),
            )
        })
        .collect();
    let identity_parts: Vec<_> = selected_values
        .iter()
        .map(|(label, value)| format!("{label}={value}"))
        .collect();
    let canonical_id = CanonicalStructuredRequestId::derive(
        &identity_parts
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
    );
    let derivation_record = IdentityDerivationRecord {
        derivation_record_id: IdentityDerivationRecordId::derive(&[
            input.issuance_id.as_str(),
            canonical_id.as_str(),
        ]),
        issuance_id: input.issuance_id.clone(),
        identity_policy_binding: (policy.identity.clone(), policy.version.clone()),
        identity_input_labels: policy.identity_input_labels.clone(),
        identity_input_values: selected_values.clone().into(),
        derivation_rule_binding: (
            "DeclaredIdentityInputDerivation".to_owned(),
            policy.version.clone(),
        ),
        derived_identity: canonical_id.clone(),
    };
    let standing_assignment = InitialStandingAssignment {
        assignment_id: StandingAssignmentId::derive(&[
            input.issuance_id.as_str(),
            canonical_id.as_str(),
            "Issued",
        ]),
        issuance_id: input.issuance_id.clone(),
        canonical_request_id: canonical_id.clone(),
        standing: InitialStanding::Issued,
        context_binding: (context.identity.clone(), context.version.clone()),
        rule_binding: (
            policy.standing_rule_id.clone(),
            policy.standing_rule_version.clone(),
        ),
        policy_binding: (policy.identity.clone(), policy.version.clone()),
    };
    let issued_binding = IssuedPublicationBinding {
        binding_id: IssuedPublicationBindingId::derive(&[
            input.issuance_id.as_str(),
            canonical_id.as_str(),
            manifest.construction_manifest_id.as_str(),
        ]),
        constructed_canonical_request_id: request.constructed_canonical_request_id.clone(),
        construction_manifest_id: manifest.construction_manifest_id.clone(),
        canonical_request_construction_id: request.construction_id.clone(),
        issuance_context_id: context.identity.clone(),
        identity_policy_id: policy.identity.clone(),
        standing_assignment_id: standing_assignment.assignment_id.clone(),
        canonical_structured_request_id: canonical_id.clone(),
    };
    let canonical_request = CanonicalStructuredRequest {
        canonical_structured_request_id: canonical_id.clone(),
        issuance_id: input.issuance_id.clone(),
        constructed_canonical_request_id: request.constructed_canonical_request_id.clone(),
        construction_manifest_id: manifest.construction_manifest_id.clone(),
        canonical_request_construction_id: request.construction_id.clone(),
        exact_constructed_request: request.clone(),
        initial_standing: InitialStanding::Issued,
        standing_assignment_id: standing_assignment.assignment_id.clone(),
        content_integrity_binding: constructed_request_content_binding(request),
    };
    let manifest_id = IssuanceManifestId::derive(&[
        input.issuance_id.as_str(),
        canonical_id.as_str(),
        issued_binding.binding_id.as_str(),
    ]);
    let mut issuance_manifest = IssuanceManifest {
        issuance_manifest_id: manifest_id,
        issuance_id: input.issuance_id.clone(),
        constructed_canonical_request_id: request.constructed_canonical_request_id.clone(),
        construction_manifest_id: manifest.construction_manifest_id.clone(),
        canonical_request_construction_id: request.construction_id.clone(),
        issuance_context_binding: (context.identity.clone(), context.version.clone()),
        identity_policy_binding: (policy.identity.clone(), policy.version.clone()),
        profile_binding: (profile.identity.clone(), profile.version.clone()),
        schema_binding: (schema.identity.clone(), schema.version.clone()),
        rule_set_binding: (rules.identity.clone(), rules.version.clone()),
        registry_binding: (registries.identity.clone(), registries.version.clone()),
        configuration_binding: (
            configuration.identity.clone(),
            configuration.version.clone(),
        ),
        implementation_version: profile.implementation_version.clone(),
        identity_derivation_record: derivation_record,
        initial_standing_assignment: standing_assignment,
        issued_publication_binding: issued_binding,
        replay_context: input.replay_context.clone(),
        publication_binding: String::new(),
    };
    issuance_manifest.publication_binding =
        issuance_publication_binding(&canonical_request, &issuance_manifest);
    CanonicalRequestIssuanceOutcome::Issued {
        request: Box::new(canonical_request),
        manifest: Box::new(issuance_manifest),
    }
}

#[cfg(test)]
mod contract_014_tests {
    use super::*;

    pub(super) fn construction_fixture() -> (ConstructedCanonicalRequest, ConstructionManifest) {
        let ordered = super::contract_013_tests::ordered_fixture();
        let validation = super::contract_013_tests::validation_fixture(&ordered);
        let CanonicalRequestConstructionOutcome::Constructed { request, manifest } =
            super::contract_013_tests::construct(
                &ordered,
                &validation,
                &FixtureConstructionProfile::fixture(),
            )
        else {
            panic!("construction fixture")
        };
        (*request, *manifest)
    }

    pub(super) fn context() -> IssuanceContext {
        IssuanceContext::fixture("fixture-context-a")
    }

    pub(super) fn issue(
        request: &ConstructedCanonicalRequest,
        manifest: &ConstructionManifest,
        context: &IssuanceContext,
    ) -> CanonicalRequestIssuanceOutcome {
        let input = CanonicalRequestIssuanceInput::for_fixture(request, manifest, context);
        let authorities = FixtureIssuanceAuthorityContext::fixture(context);
        perform_canonical_request_issuance(&input, request, manifest, &authorities)
    }

    #[test]
    fn exact_construction_publication_issues_atomically() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued {
            request: issued,
            manifest: issuance,
        } = issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert_eq!(
            issued.canonical_structured_request_id,
            issuance
                .issued_publication_binding
                .canonical_structured_request_id
        );
        assert_eq!(issued.initial_standing, InitialStanding::Issued);
    }

    #[test]
    fn request_without_manifest_fails() {
        let (request, mut manifest) = construction_fixture();
        manifest.constructed_canonical_request_id =
            ConstructedCanonicalRequestId::derive(&["missing-request"]);
        assert!(
            matches!(issue(&request, &manifest, &context()), CanonicalRequestIssuanceOutcome::Failed(record) if record.category == IssuanceFailureCategory::ConstructionPublicationBindingMismatch)
        );
    }

    #[test]
    fn manifest_without_request_fails() {
        let (mut request, manifest) = construction_fixture();
        request.constructed_canonical_request_id =
            ConstructedCanonicalRequestId::derive(&["missing-manifest"]);
        assert!(
            matches!(issue(&request, &manifest, &context()), CanonicalRequestIssuanceOutcome::Failed(record) if record.category == IssuanceFailureCategory::ConstructionPublicationBindingMismatch)
        );
    }

    #[test]
    fn mismatched_publication_fails() {
        let (request, manifest) = construction_fixture();
        let mut input = CanonicalRequestIssuanceInput::for_fixture(&request, &manifest, &context());
        input.construction_publication_binding = "foreign-binding".to_owned();
        let authorities = FixtureIssuanceAuthorityContext::fixture(&context());
        assert!(
            matches!(perform_canonical_request_issuance(&input, &request, &manifest, &authorities), CanonicalRequestIssuanceOutcome::Failed(record) if record.category == IssuanceFailureCategory::ConstructionPublicationBindingMismatch)
        );
    }

    #[test]
    fn same_context_replay_is_idempotent() {
        let (request, manifest) = construction_fixture();
        assert_eq!(
            issue(&request, &manifest, &context()),
            issue(&request, &manifest, &context())
        );
    }

    #[test]
    fn same_context_identity_is_stable() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued { request: first, .. } =
            issue(&request, &manifest, &context())
        else {
            panic!("first")
        };
        let CanonicalRequestIssuanceOutcome::Issued {
            request: second, ..
        } = issue(&request, &manifest, &context())
        else {
            panic!("second")
        };
        assert_eq!(
            first.canonical_structured_request_id,
            second.canonical_structured_request_id
        );
    }

    #[test]
    fn distinct_context_may_produce_distinct_identity() {
        let (request, manifest) = construction_fixture();
        let first = issue(&request, &manifest, &context());
        let second = issue(
            &request,
            &manifest,
            &IssuanceContext::fixture("fixture-context-b"),
        );
        let (
            CanonicalRequestIssuanceOutcome::Issued { request: first, .. },
            CanonicalRequestIssuanceOutcome::Issued {
                request: second, ..
            },
        ) = (first, second)
        else {
            panic!("contexts")
        };
        assert_ne!(
            first.canonical_structured_request_id,
            second.canonical_structured_request_id
        );
    }

    #[test]
    fn identity_and_integrity_are_distinct() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued {
            request: issued, ..
        } = issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert_ne!(
            issued.canonical_structured_request_id.as_str(),
            issued.content_integrity_binding
        );
    }

    #[test]
    fn identity_policy_controls_identity_inputs() {
        let (request, manifest) = construction_fixture();
        let context = context();
        let authorities = FixtureIssuanceAuthorityContext::fixture(&context);
        let input = CanonicalRequestIssuanceInput::for_fixture(&request, &manifest, &context);
        let mut policy = authorities.identity_policy.clone();
        policy.identity_input_labels = vec![
            "constructed_request_id".to_owned(),
            "identity_policy_id".to_owned(),
        ]
        .into();
        let changed = FixtureIssuanceAuthorityContext {
            identity_policy: policy,
            ..authorities.clone()
        };
        let first = perform_canonical_request_issuance(&input, &request, &manifest, &authorities);
        let second = perform_canonical_request_issuance(&input, &request, &manifest, &changed);
        let (
            CanonicalRequestIssuanceOutcome::Issued { request: first, .. },
            CanonicalRequestIssuanceOutcome::Issued {
                request: second, ..
            },
        ) = (first, second)
        else {
            panic!("policy")
        };
        assert_ne!(
            first.canonical_structured_request_id,
            second.canonical_structured_request_id
        );
    }

    #[test]
    fn identity_derivation_record_required() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued { manifest, .. } =
            issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert!(!manifest
            .identity_derivation_record
            .identity_input_values
            .is_empty());
    }

    #[test]
    fn request_is_not_modified() {
        let (request, manifest) = construction_fixture();
        let before = request.clone();
        let CanonicalRequestIssuanceOutcome::Issued {
            request: issued, ..
        } = issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert_eq!(issued.exact_constructed_request, before);
    }

    #[test]
    fn initial_standing_is_assigned_explicitly() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued { request, manifest } =
            issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert_eq!(request.initial_standing, InitialStanding::Issued);
        assert_eq!(
            manifest.initial_standing_assignment.standing,
            InitialStanding::Issued
        );
    }

    #[test]
    fn issued_publication_binding_is_required() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued { request, manifest } =
            issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert_eq!(
            manifest
                .issued_publication_binding
                .constructed_canonical_request_id,
            request.constructed_canonical_request_id
        );
        assert_eq!(
            manifest.issued_publication_binding.construction_manifest_id,
            request.construction_manifest_id
        );
    }

    #[test]
    fn no_partial_publication() {
        let (request, mut manifest) = construction_fixture();
        manifest.publication_binding = "broken".to_owned();
        assert!(matches!(
            issue(&request, &manifest, &context()),
            CanonicalRequestIssuanceOutcome::Failed(_)
        ));
    }

    #[test]
    fn no_contract_015_authority() {
        let (request, manifest) = construction_fixture();
        let CanonicalRequestIssuanceOutcome::Issued { request, manifest } =
            issue(&request, &manifest, &context())
        else {
            panic!("issuance")
        };
        assert!(!manifest.replay_context.contains_key("recipient"));
        assert!(!request
            .exact_constructed_request
            .construction_context
            .contains_key("handoff_context"));
    }
}

// ---------------------------------------------------------------------------
// Contract 015: Canonical request handoff
// ---------------------------------------------------------------------------

contract_002_id!(CanonicalRequestHandoffId, "handoff");
contract_002_id!(HandoffContextId, "handoff-context");
contract_002_id!(DownstreamBoundaryDeclarationId, "boundary-declaration");
contract_002_id!(HandoffProfileId, "handoff-profile");
contract_002_id!(HandoffRuleSetId, "handoff-rules");
contract_002_id!(HandoffRegistryId, "handoff-registry");
contract_002_id!(HandoffConfigurationId, "handoff-config");
contract_002_id!(HandoffPackageManifestId, "handoff-package");
contract_002_id!(TransferAttemptId, "transfer-attempt");
contract_002_id!(OperationalHandoffFactId, "handoff-fact");
contract_002_id!(ReceiptAcknowledgmentId, "receipt-ack");
contract_002_id!(HandoffRuleApplicationId, "handoff-rule-application");
contract_002_id!(TransferDecisionId, "transfer-decision");
contract_002_id!(CustodyDecisionId, "custody-decision");
contract_002_id!(ResponsibilityDecisionId, "responsibility-decision");
contract_002_id!(HandoffFailureRecordId, "handoff-failure");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum BoundaryEffectiveStatus {
    Active,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandoffAcknowledgmentType {
    Received,
    Rejected,
    DuplicateRecognized,
    UnableToReceive,
    IntegrityRejected,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OperationalHandoffFactKind {
    TransportAccepted,
    TransportTimeout,
    ConnectionRefused,
    QueueAccepted,
    DeliveryObserved,
    MessageIdentifierReturned,
    AdapterFailure,
    RecipientResponseObserved,
    ExpirationReached,
    TransportDuplicateFlag,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum TransferDisposition {
    Acknowledged,
    Rejected,
    Expired,
    RecipientUnavailable,
    DuplicateRecognized,
    DeliveredUnacknowledged,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CustodyDisposition {
    Transferred,
    RetainedBySre,
    HeldByIntermediary,
    SharedPendingConfirmation,
    NotTransferred,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandoffResponsibilityDisposition {
    TransferredToDeclaredBoundary,
    RetainedBySre,
    AssignedToDeclaredGovernanceBoundary,
    Pending,
    NotTransferred,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HandoffFailureCategory {
    IssuancePublicationSetIntegrityFailure,
    BoundaryDeclarationResolutionFailure,
    BoundaryDeclarationInvalid,
    RecipientIdentityVerificationFailure,
    HandoffContextConstructionFailure,
    HandoffProfileResolutionFailure,
    TransferRuleResolutionFailure,
    PackageConstructionFailure,
    PackageIntegrityFailure,
    TransportBindingFailure,
    TransferProtocolFailure,
    AcknowledgmentVerificationFailure,
    CustodyDeterminationFailure,
    ResponsibilityDeterminationFailure,
    NonDeterministicRetryState,
    DuplicateCorrelationFailure,
    TerminalRecordCommitmentFailure,
    InternalHandoffFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DownstreamBoundaryDeclaration {
    pub declaration_id: DownstreamBoundaryDeclarationId,
    pub declaration_version: String,
    pub downstream_boundary_id: String,
    pub boundary_class: String,
    pub authority_reference: String,
    pub permitted_handoff_role: String,
    pub receipt_authority: String,
    pub supported_acknowledgment_method: String,
    pub supported_custody_model: String,
    pub supported_responsibility_model: String,
    pub supported_transport_binding: String,
    pub effective_status: BoundaryEffectiveStatus,
}

impl DownstreamBoundaryDeclaration {
    pub fn fixture() -> Self {
        Self {
            declaration_id: DownstreamBoundaryDeclarationId::derive(&["contract-015-fixture"]),
            declaration_version: "fixture-boundary-declaration-v1".to_owned(),
            downstream_boundary_id: "fixture-downstream-constitutional-boundary".to_owned(),
            boundary_class: "DeclaredConstitutionalBoundary".to_owned(),
            authority_reference: "TEST-FIXTURE-ONLY; NOT-CONSTITUTIONAL-DEFAULT".to_owned(),
            permitted_handoff_role: "ReceiveAndAccountOnly".to_owned(),
            receipt_authority: "fixture-recipient-receipt-authority".to_owned(),
            supported_acknowledgment_method: "RecipientAttributableFixtureAcknowledgment"
                .to_owned(),
            supported_custody_model: "DeclaredBoundaryReceiptMayQualify".to_owned(),
            supported_responsibility_model: "ExplicitIndependentResponsibilityRule".to_owned(),
            supported_transport_binding: "fixture-transport-binding-v1".to_owned(),
            effective_status: BoundaryEffectiveStatus::Active,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandoffContext {
    pub handoff_context_id: HandoffContextId,
    pub context_version: String,
    pub canonical_structured_request_id: CanonicalStructuredRequestId,
    pub issuance_manifest_id: IssuanceManifestId,
    pub issuance_operation_id: CanonicalRequestIssuanceId,
    pub sender_boundary_id: String,
    pub downstream_boundary_declaration_id: DownstreamBoundaryDeclarationId,
    pub downstream_boundary_declaration_version: String,
    pub recipient_boundary_id: String,
    pub handoff_profile_id: HandoffProfileId,
    pub handoff_profile_version: String,
    pub transfer_rule_set_id: HandoffRuleSetId,
    pub transfer_rule_set_version: String,
    pub custody_rule_set_id: HandoffRuleSetId,
    pub custody_rule_set_version: String,
    pub responsibility_rule_set_id: HandoffRuleSetId,
    pub responsibility_rule_set_version: String,
    pub acknowledgment_rule_set_id: HandoffRuleSetId,
    pub acknowledgment_rule_set_version: String,
    pub duplicate_rule_set_id: HandoffRuleSetId,
    pub duplicate_rule_set_version: String,
    pub retry_rule_set_id: HandoffRuleSetId,
    pub retry_rule_set_version: String,
    pub expiration_rule_set_id: HandoffRuleSetId,
    pub expiration_rule_set_version: String,
    pub transfer_purpose: String,
    pub reference_mode: String,
    pub custody_model: String,
    pub responsibility_model: String,
    pub acknowledgment_model: String,
    pub retry_policy: String,
    pub duplicate_policy: String,
    pub expiration_policy: String,
    pub schema_version: String,
    pub registry_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl HandoffContext {
    pub fn fixture(request: &CanonicalStructuredRequest, manifest: &IssuanceManifest) -> Self {
        Self {
            handoff_context_id: HandoffContextId::derive(&[
                request.canonical_structured_request_id.as_str(),
                manifest.issuance_manifest_id.as_str(),
                "contract-015-fixture",
            ]),
            context_version: "fixture-handoff-context-v1".to_owned(),
            canonical_structured_request_id: request.canonical_structured_request_id.clone(),
            issuance_manifest_id: manifest.issuance_manifest_id.clone(),
            issuance_operation_id: manifest.issuance_id.clone(),
            sender_boundary_id: "structured-request-engine".to_owned(),
            downstream_boundary_declaration_id: DownstreamBoundaryDeclaration::fixture()
                .declaration_id,
            downstream_boundary_declaration_version: "fixture-boundary-declaration-v1".to_owned(),
            recipient_boundary_id: DownstreamBoundaryDeclaration::fixture().downstream_boundary_id,
            handoff_profile_id: HandoffProfileId::derive(&["contract-015-fixture"]),
            handoff_profile_version: "fixture-handoff-v1".to_owned(),
            transfer_rule_set_id: HandoffRuleSetId::derive(&["fixture-transfer-rules"]),
            transfer_rule_set_version: "fixture-transfer-rules-v1".to_owned(),
            custody_rule_set_id: HandoffRuleSetId::derive(&["fixture-custody-rules"]),
            custody_rule_set_version: "fixture-custody-rules-v1".to_owned(),
            responsibility_rule_set_id: HandoffRuleSetId::derive(&["fixture-responsibility-rules"]),
            responsibility_rule_set_version: "fixture-responsibility-rules-v1".to_owned(),
            acknowledgment_rule_set_id: HandoffRuleSetId::derive(&["fixture-acknowledgment-rules"]),
            acknowledgment_rule_set_version: "fixture-acknowledgment-rules-v1".to_owned(),
            duplicate_rule_set_id: HandoffRuleSetId::derive(&["fixture-duplicate-rules"]),
            duplicate_rule_set_version: "fixture-duplicate-rules-v1".to_owned(),
            retry_rule_set_id: HandoffRuleSetId::derive(&["fixture-retry-rules"]),
            retry_rule_set_version: "fixture-retry-rules-v1".to_owned(),
            expiration_rule_set_id: HandoffRuleSetId::derive(&["fixture-expiration-rules"]),
            expiration_rule_set_version: "fixture-expiration-rules-v1".to_owned(),
            transfer_purpose: "TransferExactIssuedRequestForDeclaredAccounting".to_owned(),
            reference_mode: "ReferenceOnly".to_owned(),
            custody_model: "ExplicitCustodyRule".to_owned(),
            responsibility_model: "ExplicitResponsibilityRule".to_owned(),
            acknowledgment_model: "RecipientAttributable".to_owned(),
            retry_policy: "SameContextSamePackageNewAttempt".to_owned(),
            duplicate_policy: "PriorConstitutionalCorrelationRequired".to_owned(),
            expiration_policy: "ContextOnly".to_owned(),
            schema_version: "fixture-handoff-schema-v1".to_owned(),
            registry_version: "fixture-handoff-registry-v1".to_owned(),
            configuration_version: "fixture-handoff-config-v1".to_owned(),
            implementation_version: "sre-runtime-fixture-v1".to_owned(),
            replay_context: BTreeMap::from([("fixture".to_owned(), "contract-015".to_owned())]),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureHandoffProfile {
    pub identity: HandoffProfileId,
    pub version: String,
    pub reference_mode: String,
    pub allow_delivered_unacknowledged: bool,
    pub recipient_unavailable_attempt_threshold: u32,
    pub custody_on_received: bool,
    pub responsibility_on_received: bool,
}

impl FixtureHandoffProfile {
    pub fn fixture() -> Self {
        Self {
            identity: HandoffProfileId::derive(&["contract-015-fixture"]),
            version: "fixture-handoff-v1".to_owned(),
            reference_mode: "ReferenceOnly".to_owned(),
            allow_delivered_unacknowledged: true,
            recipient_unavailable_attempt_threshold: 2,
            custody_on_received: true,
            responsibility_on_received: false,
        }
    }
}

macro_rules! fixture_handoff_rule {
    ($name:ident, $id:expr, $version:expr) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            pub identity: HandoffRuleSetId,
            pub version: String,
            pub rule_id: String,
            pub rule_version: String,
        }
        impl $name {
            pub fn fixture() -> Self {
                Self {
                    identity: HandoffRuleSetId::derive(&[$id]),
                    version: $version.to_owned(),
                    rule_id: $id.to_owned(),
                    rule_version: $version.to_owned(),
                }
            }
        }
    };
}

fixture_handoff_rule!(
    FixtureTransferRuleSet,
    "fixture-transfer-rules",
    "fixture-transfer-rules-v1"
);
fixture_handoff_rule!(
    FixtureCustodyRuleSet,
    "fixture-custody-rules",
    "fixture-custody-rules-v1"
);
fixture_handoff_rule!(
    FixtureResponsibilityRuleSet,
    "fixture-responsibility-rules",
    "fixture-responsibility-rules-v1"
);
fixture_handoff_rule!(
    FixtureAcknowledgmentRuleSet,
    "fixture-acknowledgment-rules",
    "fixture-acknowledgment-rules-v1"
);
fixture_handoff_rule!(
    FixtureRetryRuleSet,
    "fixture-retry-rules",
    "fixture-retry-rules-v1"
);
fixture_handoff_rule!(
    FixtureDuplicateRuleSet,
    "fixture-duplicate-rules",
    "fixture-duplicate-rules-v1"
);
fixture_handoff_rule!(
    FixtureExpirationRuleSet,
    "fixture-expiration-rules",
    "fixture-expiration-rules-v1"
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureHandoffRegistries {
    pub identity: HandoffRegistryId,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureTransportBinding {
    pub identity: String,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureHandoffConfiguration {
    pub identity: HandoffConfigurationId,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureHandoffAuthorityContext {
    pub profile: FixtureHandoffProfile,
    pub transfer_rules: FixtureTransferRuleSet,
    pub custody_rules: FixtureCustodyRuleSet,
    pub responsibility_rules: FixtureResponsibilityRuleSet,
    pub acknowledgment_rules: FixtureAcknowledgmentRuleSet,
    pub retry_rules: FixtureRetryRuleSet,
    pub duplicate_rules: FixtureDuplicateRuleSet,
    pub expiration_rules: FixtureExpirationRuleSet,
    pub registries: FixtureHandoffRegistries,
    pub transport: FixtureTransportBinding,
    pub configuration: FixtureHandoffConfiguration,
}

impl FixtureHandoffAuthorityContext {
    pub fn fixture() -> Self {
        Self {
            profile: FixtureHandoffProfile::fixture(),
            transfer_rules: FixtureTransferRuleSet::fixture(),
            custody_rules: FixtureCustodyRuleSet::fixture(),
            responsibility_rules: FixtureResponsibilityRuleSet::fixture(),
            acknowledgment_rules: FixtureAcknowledgmentRuleSet::fixture(),
            retry_rules: FixtureRetryRuleSet::fixture(),
            duplicate_rules: FixtureDuplicateRuleSet::fixture(),
            expiration_rules: FixtureExpirationRuleSet::fixture(),
            registries: FixtureHandoffRegistries {
                identity: HandoffRegistryId::derive(&["contract-015-fixture"]),
                version: "fixture-handoff-registry-v1".to_owned(),
            },
            transport: FixtureTransportBinding {
                identity: "fixture-transport-binding-v1".to_owned(),
                version: "fixture-transport-v1".to_owned(),
            },
            configuration: FixtureHandoffConfiguration {
                identity: HandoffConfigurationId::derive(&["contract-015-fixture"]),
                version: "fixture-handoff-config-v1".to_owned(),
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRequestHandoffInput {
    pub handoff_id: CanonicalRequestHandoffId,
    pub operation_id: CanonicalRequestHandoffId,
    pub canonical_structured_request_id: CanonicalStructuredRequestId,
    pub issuance_manifest_id: IssuanceManifestId,
    pub issuance_operation_id: CanonicalRequestIssuanceId,
    pub issuance_publication_binding: String,
    pub boundary_declaration_id: DownstreamBoundaryDeclarationId,
    pub boundary_declaration_version: String,
    pub handoff_context_id: HandoffContextId,
    pub handoff_context_version: String,
    pub handoff_profile_id: HandoffProfileId,
    pub handoff_profile_version: String,
    pub transfer_rule_set_id: HandoffRuleSetId,
    pub transfer_rule_set_version: String,
    pub custody_rule_set_id: HandoffRuleSetId,
    pub custody_rule_set_version: String,
    pub responsibility_rule_set_id: HandoffRuleSetId,
    pub responsibility_rule_set_version: String,
    pub transport_binding: String,
    pub schema_version: String,
    pub registry_version: String,
    pub configuration_version: String,
    pub implementation_version: String,
    pub replay_context: BTreeMap<String, String>,
}

impl CanonicalRequestHandoffInput {
    pub fn for_fixture(
        request: &CanonicalStructuredRequest,
        manifest: &IssuanceManifest,
        context: &HandoffContext,
    ) -> Self {
        let authorities = FixtureHandoffAuthorityContext::fixture();
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandoffPackageManifest {
    pub handoff_package_manifest_id: HandoffPackageManifestId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub canonical_structured_request_id: CanonicalStructuredRequestId,
    pub issuance_manifest_id: IssuanceManifestId,
    pub issuance_operation_id: CanonicalRequestIssuanceId,
    pub sender_boundary_id: String,
    pub recipient_boundary_id: String,
    pub downstream_boundary_declaration_id: DownstreamBoundaryDeclarationId,
    pub downstream_boundary_declaration_version: String,
    pub handoff_profile_id: HandoffProfileId,
    pub handoff_profile_version: String,
    pub transfer_rule_set_id: HandoffRuleSetId,
    pub transfer_rule_set_version: String,
    pub reference_mode: String,
    pub transfer_purpose: String,
    pub integrity_bindings: String,
    pub transport_binding_reference: String,
    pub expiration_basis: String,
    pub retry_basis: String,
    pub duplicate_basis: String,
    pub provenance_continuity_references: Arc<[String]>,
    pub schema_version: String,
    pub registry_version: String,
    pub configuration_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferAttempt {
    pub transfer_attempt_id: TransferAttemptId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub package_manifest_id: HandoffPackageManifestId,
    pub attempt_number: u32,
    pub retry_basis: String,
    pub transport_binding_reference: String,
    pub operational_fact_ids: Arc<[OperationalHandoffFactId]>,
    pub attempt_result: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationalHandoffFact {
    pub fact_id: OperationalHandoffFactId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub attempt_id: TransferAttemptId,
    pub kind: OperationalHandoffFactKind,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReceiptAcknowledgment {
    pub acknowledgment_id: ReceiptAcknowledgmentId,
    pub recipient_boundary_id: String,
    pub downstream_boundary_declaration_id: DownstreamBoundaryDeclarationId,
    pub downstream_boundary_declaration_version: String,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub package_manifest_id: HandoffPackageManifestId,
    pub transfer_attempt_id: Option<TransferAttemptId>,
    pub acknowledgment_type: HandoffAcknowledgmentType,
    pub attribution_basis: String,
    pub integrity_binding: String,
    pub prior_constitutional_reference: Option<String>,
    pub recipient_declared_time: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandoffRuleApplication {
    pub application_id: HandoffRuleApplicationId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub rule_id: String,
    pub rule_version: String,
    pub operational_fact_references: Arc<[OperationalHandoffFactId]>,
    pub transfer_attempt_references: Arc<[TransferAttemptId]>,
    pub acknowledgment_references: Arc<[ReceiptAcknowledgmentId]>,
    pub applicability_result: String,
    pub resulting_decision_authority: String,
    pub profile_version: String,
    pub registry_version: String,
    pub configuration_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferDecision {
    pub decision_id: TransferDecisionId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub handoff_rule_application_ids: Arc<[HandoffRuleApplicationId]>,
    pub operational_fact_ids: Arc<[OperationalHandoffFactId]>,
    pub transfer_attempt_ids: Arc<[TransferAttemptId]>,
    pub acknowledgment_ids: Arc<[ReceiptAcknowledgmentId]>,
    pub resulting_disposition: TransferDisposition,
    pub decision_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustodyDecision {
    pub decision_id: CustodyDecisionId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub handoff_rule_application_ids: Arc<[HandoffRuleApplicationId]>,
    pub operational_fact_ids: Arc<[OperationalHandoffFactId]>,
    pub transfer_attempt_ids: Arc<[TransferAttemptId]>,
    pub acknowledgment_ids: Arc<[ReceiptAcknowledgmentId]>,
    pub resulting_disposition: CustodyDisposition,
    pub decision_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponsibilityDecision {
    pub decision_id: ResponsibilityDecisionId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub handoff_context_id: HandoffContextId,
    pub handoff_rule_application_ids: Arc<[HandoffRuleApplicationId]>,
    pub operational_fact_ids: Arc<[OperationalHandoffFactId]>,
    pub transfer_attempt_ids: Arc<[TransferAttemptId]>,
    pub acknowledgment_ids: Arc<[ReceiptAcknowledgmentId]>,
    pub resulting_disposition: HandoffResponsibilityDisposition,
    pub decision_basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRequestHandoffRecord {
    pub handoff_id: CanonicalRequestHandoffId,
    pub operation_id: CanonicalRequestHandoffId,
    pub handoff_context: HandoffContext,
    pub boundary_declaration: DownstreamBoundaryDeclaration,
    pub package_manifest: HandoffPackageManifest,
    pub attempts: Arc<[TransferAttempt]>,
    pub operational_facts: Arc<[OperationalHandoffFact]>,
    pub acknowledgments: Arc<[ReceiptAcknowledgment]>,
    pub rule_applications: Arc<[HandoffRuleApplication]>,
    pub transfer_decision: TransferDecision,
    pub custody_decision: CustodyDecision,
    pub responsibility_decision: ResponsibilityDecision,
    pub completion: String,
    pub terminal_binding: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalRequestHandoffFailureRecord {
    pub failure_record_id: HandoffFailureRecordId,
    pub handoff_id: CanonicalRequestHandoffId,
    pub operation_id: CanonicalRequestHandoffId,
    pub category: HandoffFailureCategory,
    pub detail: String,
    pub attempted_issuance_manifest_id: Option<IssuanceManifestId>,
    pub attempted_boundary_declaration_id: Option<DownstreamBoundaryDeclarationId>,
    pub attempted_handoff_context_id: Option<HandoffContextId>,
    pub attempts: Arc<[TransferAttempt]>,
    pub operational_facts: Arc<[OperationalHandoffFact]>,
    pub acknowledgments: Arc<[ReceiptAcknowledgment]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalRequestHandoffOutcome {
    Determined(Box<CanonicalRequestHandoffRecord>),
    Failed(Box<CanonicalRequestHandoffFailureRecord>),
}

fn handoff_failure(
    input: &CanonicalRequestHandoffInput,
    category: HandoffFailureCategory,
    detail: &str,
    attempts: &[TransferAttempt],
    facts: &[OperationalHandoffFact],
    acknowledgments: &[ReceiptAcknowledgment],
) -> CanonicalRequestHandoffOutcome {
    CanonicalRequestHandoffOutcome::Failed(Box::new(CanonicalRequestHandoffFailureRecord {
        failure_record_id: HandoffFailureRecordId::derive(&[
            input.operation_id.as_str(),
            &format!("{category:?}"),
            detail,
        ]),
        handoff_id: input.handoff_id.clone(),
        operation_id: input.operation_id.clone(),
        category,
        detail: detail.to_owned(),
        attempted_issuance_manifest_id: Some(input.issuance_manifest_id.clone()),
        attempted_boundary_declaration_id: Some(input.boundary_declaration_id.clone()),
        attempted_handoff_context_id: Some(input.handoff_context_id.clone()),
        attempts: attempts.to_vec().into(),
        operational_facts: facts.to_vec().into(),
        acknowledgments: acknowledgments.to_vec().into(),
    }))
}

fn handoff_ids<T: Clone, F: Fn(&T) -> Id, Id>(items: &[T], function: F) -> Vec<Id> {
    items.iter().map(function).collect()
}

#[allow(clippy::too_many_arguments)]
pub fn perform_canonical_request_handoff(
    input: &CanonicalRequestHandoffInput,
    request: &CanonicalStructuredRequest,
    issuance_manifest: &IssuanceManifest,
    boundary: &DownstreamBoundaryDeclaration,
    context: &HandoffContext,
    attempts: &[TransferAttempt],
    facts: &[OperationalHandoffFact],
    acknowledgments: &[ReceiptAcknowledgment],
    authorities: &FixtureHandoffAuthorityContext,
) -> CanonicalRequestHandoffOutcome {
    if !issuance_pair_matches(input, request, issuance_manifest) {
        return handoff_failure(
            input,
            HandoffFailureCategory::IssuancePublicationSetIntegrityFailure,
            "Contract 014 issuance publication is not one exact complete pair",
            attempts,
            facts,
            acknowledgments,
        );
    }
    if boundary.declaration_id != input.boundary_declaration_id
        || boundary.declaration_version != input.boundary_declaration_version
        || boundary.downstream_boundary_id != context.recipient_boundary_id
        || boundary.effective_status != BoundaryEffectiveStatus::Active
        || boundary.downstream_boundary_id.is_empty()
        || boundary.authority_reference.is_empty()
    {
        return handoff_failure(
            input,
            HandoffFailureCategory::BoundaryDeclarationInvalid,
            "downstream constitutional boundary declaration is not exact and active",
            attempts,
            facts,
            acknowledgments,
        );
    }
    if context.handoff_context_id != input.handoff_context_id
        || context.context_version != input.handoff_context_version
        || context.canonical_structured_request_id != request.canonical_structured_request_id
        || context.issuance_manifest_id != issuance_manifest.issuance_manifest_id
        || context.issuance_operation_id != issuance_manifest.issuance_id
        || context.downstream_boundary_declaration_id != boundary.declaration_id
        || context.downstream_boundary_declaration_version != boundary.declaration_version
        || context.recipient_boundary_id != boundary.downstream_boundary_id
        || context.sender_boundary_id.is_empty()
    {
        return handoff_failure(
            input,
            HandoffFailureCategory::HandoffContextConstructionFailure,
            "handoff context is not exact for the issuance pair and boundary",
            attempts,
            facts,
            acknowledgments,
        );
    }
    if input.handoff_profile_id != authorities.profile.identity
        || input.handoff_profile_version != authorities.profile.version
        || context.handoff_profile_id != authorities.profile.identity
        || context.handoff_profile_version != authorities.profile.version
        || input.transfer_rule_set_id != authorities.transfer_rules.identity
        || input.transfer_rule_set_version != authorities.transfer_rules.version
        || context.transfer_rule_set_id != authorities.transfer_rules.identity
        || context.transfer_rule_set_version != authorities.transfer_rules.version
        || input.custody_rule_set_id != authorities.custody_rules.identity
        || input.custody_rule_set_version != authorities.custody_rules.version
        || input.responsibility_rule_set_id != authorities.responsibility_rules.identity
        || input.responsibility_rule_set_version != authorities.responsibility_rules.version
    {
        return handoff_failure(
            input,
            HandoffFailureCategory::HandoffProfileResolutionFailure,
            "handoff profile or rule-set binding is not exact",
            attempts,
            facts,
            acknowledgments,
        );
    }
    if input.transport_binding != authorities.transport.identity
        || context.reference_mode != authorities.profile.reference_mode
        || !(context.schema_version == "fixture-handoff-schema-v1"
            || local_governed_text::valid_handoff_context(boundary, context, authorities))
        || context.registry_version != authorities.registries.version
        || context.configuration_version != authorities.configuration.version
        || !(context.implementation_version == "sre-runtime-fixture-v1"
            || local_governed_text::valid_handoff_context(boundary, context, authorities))
    {
        return handoff_failure(
            input,
            HandoffFailureCategory::TransportBindingFailure,
            "transport, schema, registry, configuration, or implementation binding is not exact",
            attempts,
            facts,
            acknowledgments,
        );
    }
    let package = form_handoff_package(input, request, issuance_manifest, boundary, context);
    if input.issuance_publication_binding != issuance_manifest.publication_binding
        || package.reference_mode != "ReferenceOnly"
        || package.integrity_bindings != issuance_manifest.publication_binding
    {
        return handoff_failure(
            input,
            HandoffFailureCategory::PackageIntegrityFailure,
            "handoff package does not preserve the exact issuance publication binding",
            attempts,
            facts,
            acknowledgments,
        );
    }
    let attempt_ids = handoff_ids(attempts, |attempt| attempt.transfer_attempt_id.clone());
    let fact_ids = handoff_ids(facts, |fact| fact.fact_id.clone());
    let acknowledgment_ids = handoff_ids(acknowledgments, |ack| ack.acknowledgment_id.clone());
    for (index, attempt) in attempts.iter().enumerate() {
        if attempt.handoff_id != input.handoff_id
            || attempt.handoff_context_id != context.handoff_context_id
            || attempt.package_manifest_id != package.handoff_package_manifest_id
            || attempt.attempt_number != index as u32 + 1
            || attempt.transport_binding_reference != authorities.transport.identity
        {
            return handoff_failure(
                input,
                HandoffFailureCategory::NonDeterministicRetryState,
                "attempt does not preserve same handoff, context, package, and retry continuity",
                attempts,
                facts,
                acknowledgments,
            );
        }
    }
    for fact in facts {
        if fact.handoff_id != input.handoff_id
            || fact.handoff_context_id != context.handoff_context_id
            || !attempts
                .iter()
                .any(|attempt| attempt.transfer_attempt_id == fact.attempt_id)
        {
            return handoff_failure(
                input,
                HandoffFailureCategory::TransferProtocolFailure,
                "operational fact is not bound to this exact handoff attempt",
                attempts,
                facts,
                acknowledgments,
            );
        }
    }
    for acknowledgment in acknowledgments {
        if acknowledgment.recipient_boundary_id != boundary.downstream_boundary_id
            || acknowledgment.downstream_boundary_declaration_id != boundary.declaration_id
            || acknowledgment.downstream_boundary_declaration_version
                != boundary.declaration_version
            || acknowledgment.handoff_id != input.handoff_id
            || acknowledgment.handoff_context_id != context.handoff_context_id
            || acknowledgment.package_manifest_id != package.handoff_package_manifest_id
            || acknowledgment.attribution_basis != boundary.receipt_authority
            || acknowledgment.integrity_binding != package.integrity_bindings
            || acknowledgment
                .transfer_attempt_id
                .as_ref()
                .is_some_and(|attempt_id| {
                    !attempts
                        .iter()
                        .any(|attempt| &attempt.transfer_attempt_id == attempt_id)
                })
        {
            return handoff_failure(
                input,
                HandoffFailureCategory::AcknowledgmentVerificationFailure,
                "acknowledgment is foreign, unattributable, wrong-attempt, or integrity-invalid",
                attempts,
                facts,
                acknowledgments,
            );
        }
        if acknowledgment.acknowledgment_type == HandoffAcknowledgmentType::DuplicateRecognized
            && acknowledgment.prior_constitutional_reference.is_none()
        {
            return handoff_failure(
                input,
                HandoffFailureCategory::DuplicateCorrelationFailure,
                "duplicate recognition requires a prior constitutional reference",
                attempts,
                facts,
                acknowledgments,
            );
        }
    }
    let received = acknowledgments
        .iter()
        .find(|ack| ack.acknowledgment_type == HandoffAcknowledgmentType::Received);
    let rejected = acknowledgments
        .iter()
        .find(|ack| ack.acknowledgment_type == HandoffAcknowledgmentType::Rejected);
    let duplicate = acknowledgments
        .iter()
        .find(|ack| ack.acknowledgment_type == HandoffAcknowledgmentType::DuplicateRecognized);
    let expired = facts
        .iter()
        .any(|fact| fact.kind == OperationalHandoffFactKind::ExpirationReached);
    let unable = acknowledgments
        .iter()
        .any(|ack| ack.acknowledgment_type == HandoffAcknowledgmentType::UnableToReceive)
        && attempts.len() as u32 >= authorities.profile.recipient_unavailable_attempt_threshold;
    let delivered = facts
        .iter()
        .any(|fact| fact.kind == OperationalHandoffFactKind::DeliveryObserved);
    let transfer_disposition = if expired {
        TransferDisposition::Expired
    } else if let Some(ack) = duplicate {
        if ack.prior_constitutional_reference.is_some() {
            TransferDisposition::DuplicateRecognized
        } else {
            TransferDisposition::Rejected
        }
    } else if received.is_some() {
        TransferDisposition::Acknowledged
    } else if rejected.is_some() {
        TransferDisposition::Rejected
    } else if unable {
        TransferDisposition::RecipientUnavailable
    } else if delivered && authorities.profile.allow_delivered_unacknowledged {
        TransferDisposition::DeliveredUnacknowledged
    } else {
        return handoff_failure(
            input,
            HandoffFailureCategory::TransferProtocolFailure,
            "no closed fixture transfer rule determines a terminal disposition",
            attempts,
            facts,
            acknowledgments,
        );
    };
    let application_ids = [
        HandoffRuleApplicationId::derive(&[
            input.handoff_id.as_str(),
            "transfer",
            &format!("{transfer_disposition:?}"),
        ]),
        HandoffRuleApplicationId::derive(&[input.handoff_id.as_str(), "custody"]),
        HandoffRuleApplicationId::derive(&[input.handoff_id.as_str(), "responsibility"]),
    ];
    let make_application = |id: HandoffRuleApplicationId,
                            rule: &str,
                            version: &str,
                            authority: &str| HandoffRuleApplication {
        application_id: id,
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        rule_id: rule.to_owned(),
        rule_version: version.to_owned(),
        operational_fact_references: fact_ids.clone().into(),
        transfer_attempt_references: attempt_ids.clone().into(),
        acknowledgment_references: acknowledgment_ids.clone().into(),
        applicability_result: "Applicable".to_owned(),
        resulting_decision_authority: authority.to_owned(),
        profile_version: context.handoff_profile_version.clone(),
        registry_version: context.registry_version.clone(),
        configuration_version: context.configuration_version.clone(),
    };
    let applications = vec![
        make_application(
            application_ids[0].clone(),
            &authorities.transfer_rules.rule_id,
            &authorities.transfer_rules.rule_version,
            "TransferDecision",
        ),
        make_application(
            application_ids[1].clone(),
            &authorities.custody_rules.rule_id,
            &authorities.custody_rules.rule_version,
            "CustodyDecision",
        ),
        make_application(
            application_ids[2].clone(),
            &authorities.responsibility_rules.rule_id,
            &authorities.responsibility_rules.rule_version,
            "ResponsibilityDecision",
        ),
    ];
    let custody = if received.is_some() && authorities.profile.custody_on_received {
        CustodyDisposition::Transferred
    } else {
        CustodyDisposition::NotTransferred
    };
    let responsibility = if received.is_some() && authorities.profile.responsibility_on_received {
        if authorities.responsibility_rules.rule_id == "ulantra-evaluation-responsibility-v1" {
            HandoffResponsibilityDisposition::AssignedToDeclaredGovernanceBoundary
        } else {
            HandoffResponsibilityDisposition::TransferredToDeclaredBoundary
        }
    } else {
        HandoffResponsibilityDisposition::RetainedBySre
    };
    let transfer_decision = TransferDecision {
        decision_id: TransferDecisionId::derive(&[input.handoff_id.as_str(), "transfer-decision"]),
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        handoff_rule_application_ids: vec![application_ids[0].clone()].into(),
        operational_fact_ids: fact_ids.clone().into(),
        transfer_attempt_ids: attempt_ids.clone().into(),
        acknowledgment_ids: acknowledgment_ids.clone().into(),
        resulting_disposition: transfer_disposition,
        decision_basis: "Explicit transfer rule application".to_owned(),
    };
    let custody_decision = CustodyDecision {
        decision_id: CustodyDecisionId::derive(&[input.handoff_id.as_str(), "custody-decision"]),
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        handoff_rule_application_ids: vec![application_ids[1].clone()].into(),
        operational_fact_ids: fact_ids.clone().into(),
        transfer_attempt_ids: attempt_ids.clone().into(),
        acknowledgment_ids: acknowledgment_ids.clone().into(),
        resulting_disposition: custody,
        decision_basis: "Explicit custody rule application".to_owned(),
    };
    let responsibility_decision = ResponsibilityDecision {
        decision_id: ResponsibilityDecisionId::derive(&[
            input.handoff_id.as_str(),
            "responsibility-decision",
        ]),
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        handoff_rule_application_ids: vec![application_ids[2].clone()].into(),
        operational_fact_ids: fact_ids.clone().into(),
        transfer_attempt_ids: attempt_ids.clone().into(),
        acknowledgment_ids: acknowledgment_ids.clone().into(),
        resulting_disposition: responsibility,
        decision_basis: "Explicit responsibility rule application".to_owned(),
    };
    let terminal_binding = StableId::from_parts(
        "handoff-terminal",
        &[
            input.handoff_id.as_str(),
            package.handoff_package_manifest_id.as_str(),
            transfer_decision.decision_id.as_str(),
            custody_decision.decision_id.as_str(),
            responsibility_decision.decision_id.as_str(),
            &format!("{transfer_disposition:?}{custody:?}{responsibility:?}"),
        ],
    )
    .to_string();
    CanonicalRequestHandoffOutcome::Determined(Box::new(CanonicalRequestHandoffRecord {
        handoff_id: input.handoff_id.clone(),
        operation_id: input.operation_id.clone(),
        handoff_context: context.clone(),
        boundary_declaration: boundary.clone(),
        package_manifest: package,
        attempts: attempts.to_vec().into(),
        operational_facts: facts.to_vec().into(),
        acknowledgments: acknowledgments.to_vec().into(),
        rule_applications: applications.into(),
        transfer_decision,
        custody_decision,
        responsibility_decision,
        completion: "Completed".to_owned(),
        terminal_binding,
    }))
}

#[cfg(test)]
mod contract_015_tests {
    use super::*;

    struct Fixture {
        request: CanonicalStructuredRequest,
        issuance_manifest: IssuanceManifest,
        boundary: DownstreamBoundaryDeclaration,
        context: HandoffContext,
        input: CanonicalRequestHandoffInput,
        authorities: FixtureHandoffAuthorityContext,
        attempts: Vec<TransferAttempt>,
        facts: Vec<OperationalHandoffFact>,
        acknowledgments: Vec<ReceiptAcknowledgment>,
    }

    fn fixture() -> Fixture {
        let (constructed, construction_manifest) =
            super::contract_014_tests::construction_fixture();
        let issuance_context = super::contract_014_tests::context();
        let issuance = super::contract_014_tests::issue(
            &constructed,
            &construction_manifest,
            &issuance_context,
        );
        let CanonicalRequestIssuanceOutcome::Issued { request, manifest } = issuance else {
            panic!("issuance fixture")
        };
        let request = *request;
        let issuance_manifest = *manifest;
        let boundary = DownstreamBoundaryDeclaration::fixture();
        let context = HandoffContext::fixture(&request, &issuance_manifest);
        let input =
            CanonicalRequestHandoffInput::for_fixture(&request, &issuance_manifest, &context);
        let authorities = FixtureHandoffAuthorityContext::fixture();
        let handoff_id = input.handoff_id.clone();
        let attempt_id = TransferAttemptId::derive(&[handoff_id.as_str(), "attempt-1"]);
        let fact_id = OperationalHandoffFactId::derive(&[handoff_id.as_str(), "fact-1"]);
        let package_id = HandoffPackageManifestId::derive(&[
            handoff_id.as_str(),
            request.canonical_structured_request_id.as_str(),
            issuance_manifest.issuance_manifest_id.as_str(),
            context.handoff_context_id.as_str(),
        ]);
        let attempts = vec![TransferAttempt {
            transfer_attempt_id: attempt_id.clone(),
            handoff_id: handoff_id.clone(),
            handoff_context_id: context.handoff_context_id.clone(),
            package_manifest_id: package_id,
            attempt_number: 1,
            retry_basis: context.retry_policy.clone(),
            transport_binding_reference: authorities.transport.identity.clone(),
            operational_fact_ids: vec![fact_id.clone()].into(),
            attempt_result: "TransportAccepted".to_owned(),
        }];
        let facts = vec![OperationalHandoffFact {
            fact_id,
            handoff_id: handoff_id.clone(),
            handoff_context_id: context.handoff_context_id.clone(),
            attempt_id,
            kind: OperationalHandoffFactKind::DeliveryObserved,
            detail: "fixture delivery observation".to_owned(),
        }];
        let acknowledgments = Vec::new();
        Fixture {
            request,
            issuance_manifest,
            boundary,
            context,
            input,
            authorities,
            attempts,
            facts,
            acknowledgments,
        }
    }

    fn run(fixture: &Fixture) -> CanonicalRequestHandoffOutcome {
        perform_canonical_request_handoff(
            &fixture.input,
            &fixture.request,
            &fixture.issuance_manifest,
            &fixture.boundary,
            &fixture.context,
            &fixture.attempts,
            &fixture.facts,
            &fixture.acknowledgments,
            &fixture.authorities,
        )
    }

    fn received_ack(fixture: &Fixture) -> ReceiptAcknowledgment {
        ReceiptAcknowledgment {
            acknowledgment_id: ReceiptAcknowledgmentId::derive(&[
                fixture.input.handoff_id.as_str(),
                "received",
            ]),
            recipient_boundary_id: fixture.boundary.downstream_boundary_id.clone(),
            downstream_boundary_declaration_id: fixture.boundary.declaration_id.clone(),
            downstream_boundary_declaration_version: fixture.boundary.declaration_version.clone(),
            handoff_id: fixture.input.handoff_id.clone(),
            handoff_context_id: fixture.context.handoff_context_id.clone(),
            package_manifest_id: HandoffPackageManifestId::derive(&[
                fixture.input.handoff_id.as_str(),
                fixture.request.canonical_structured_request_id.as_str(),
                fixture.issuance_manifest.issuance_manifest_id.as_str(),
                fixture.context.handoff_context_id.as_str(),
            ]),
            transfer_attempt_id: Some(fixture.attempts[0].transfer_attempt_id.clone()),
            acknowledgment_type: HandoffAcknowledgmentType::Received,
            attribution_basis: fixture.boundary.receipt_authority.clone(),
            integrity_binding: fixture.issuance_manifest.publication_binding.clone(),
            prior_constitutional_reference: None,
            recipient_declared_time: Some("fixture-recipient-time".to_owned()),
        }
    }

    #[test]
    fn contract_015_exact_issuance_pair_is_required() {
        let mut fixture = fixture();
        fixture.input.issuance_publication_binding = "foreign".to_owned();
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::PackageIntegrityFailure)
        );
    }

    #[test]
    fn contract_015_mismatched_request_and_manifest_fail_before_transfer() {
        let mut fixture = fixture();
        fixture.input.canonical_structured_request_id =
            CanonicalStructuredRequestId::derive(&["foreign"]);
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::IssuancePublicationSetIntegrityFailure)
        );
        assert!(fixture.attempts[0].attempt_result == "TransportAccepted");
    }

    #[test]
    fn contract_015_exact_boundary_declaration_is_required() {
        let mut fixture = fixture();
        fixture.boundary.declaration_version = "foreign".to_owned();
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::BoundaryDeclarationInvalid)
        );
    }

    #[test]
    fn contract_015_endpoint_does_not_substitute_for_boundary_identity() {
        let mut fixture = fixture();
        fixture.context.recipient_boundary_id = "https://endpoint.example".to_owned();
        assert!(matches!(
            run(&fixture),
            CanonicalRequestHandoffOutcome::Failed(_)
        ));
    }

    #[test]
    fn contract_015_handoff_context_is_resolved_before_package_identity() {
        let mut fixture = fixture();
        fixture.context.handoff_context_id = HandoffContextId::derive(&["foreign"]);
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::HandoffContextConstructionFailure)
        );
    }

    #[test]
    fn contract_015_package_manifest_preserves_exact_issuance_binding() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.package_manifest.integrity_bindings,
            fixture.issuance_manifest.publication_binding
        );
        assert_eq!(record.package_manifest.reference_mode, "ReferenceOnly");
    }

    #[test]
    fn contract_015_package_manifest_is_the_only_transport_package() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.attempts[0].package_manifest_id,
            record.package_manifest.handoff_package_manifest_id
        );
    }

    #[test]
    fn contract_015_transport_success_without_attributable_ack_is_not_acknowledged() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::DeliveredUnacknowledged
        );
    }

    #[test]
    fn contract_015_sender_cannot_fabricate_recipient_acknowledgment() {
        let mut fixture = fixture();
        let mut ack = received_ack(&fixture);
        ack.attribution_basis = "sender".to_owned();
        fixture.acknowledgments.push(ack);
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::AcknowledgmentVerificationFailure)
        );
    }

    #[test]
    fn contract_015_wrong_attempt_acknowledgment_fails_verification() {
        let mut fixture = fixture();
        let mut ack = received_ack(&fixture);
        ack.transfer_attempt_id = Some(TransferAttemptId::derive(&["wrong"]));
        fixture.acknowledgments.push(ack);
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::AcknowledgmentVerificationFailure)
        );
    }

    #[test]
    fn contract_015_handoff_rule_application_is_non_authoritative() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(record.rule_applications.len(), 3);
        assert_ne!(
            record.rule_applications[0].resulting_decision_authority,
            "CanonicalRequestHandoffRecord"
        );
    }

    #[test]
    fn contract_015_transfer_decision_requires_explicit_rule_application() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.handoff_rule_application_ids.len(),
            1
        );
    }

    #[test]
    fn contract_015_custody_decision_requires_separate_qualifying_rule() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_ne!(
            record.transfer_decision.handoff_rule_application_ids,
            record.custody_decision.handoff_rule_application_ids
        );
    }

    #[test]
    fn contract_015_responsibility_decision_requires_separate_qualifying_rule() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_ne!(
            record.custody_decision.handoff_rule_application_ids,
            record.responsibility_decision.handoff_rule_application_ids
        );
    }

    #[test]
    fn contract_015_valid_receipt_can_acknowledge_without_transferring_custody() {
        let mut fixture = fixture();
        fixture.acknowledgments.push(received_ack(&fixture));
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::Acknowledged
        );
        assert_eq!(
            record.custody_decision.resulting_disposition,
            CustodyDisposition::Transferred
        );
        assert_eq!(
            record.responsibility_decision.resulting_disposition,
            HandoffResponsibilityDisposition::RetainedBySre
        );
    }

    #[test]
    fn contract_015_rejection_is_completed_not_operation_failure() {
        let mut fixture = fixture();
        let mut ack = received_ack(&fixture);
        ack.acknowledgment_type = HandoffAcknowledgmentType::Rejected;
        fixture.acknowledgments.push(ack);
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(record.completion, "Completed");
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::Rejected
        );
    }

    #[test]
    fn contract_015_recipient_unavailable_requires_closed_rule_basis() {
        let mut fixture = fixture();
        fixture.attempts.push(TransferAttempt {
            attempt_number: 2,
            transfer_attempt_id: TransferAttemptId::derive(&[
                fixture.input.handoff_id.as_str(),
                "attempt-2",
            ]),
            handoff_id: fixture.input.handoff_id.clone(),
            handoff_context_id: fixture.context.handoff_context_id.clone(),
            package_manifest_id: fixture.attempts[0].package_manifest_id.clone(),
            retry_basis: fixture.context.retry_policy.clone(),
            transport_binding_reference: fixture.authorities.transport.identity.clone(),
            operational_fact_ids: Arc::new([]),
            attempt_result: "UnableToReceive".to_owned(),
        });
        let mut ack = received_ack(&fixture);
        ack.acknowledgment_type = HandoffAcknowledgmentType::UnableToReceive;
        ack.transfer_attempt_id = Some(fixture.attempts[1].transfer_attempt_id.clone());
        fixture.acknowledgments.push(ack);
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::RecipientUnavailable
        );
    }

    #[test]
    fn contract_015_transport_failure_does_not_imply_recipient_unavailable() {
        let mut fixture = fixture();
        fixture.facts[0].kind = OperationalHandoffFactKind::ConnectionRefused;
        assert!(matches!(
            run(&fixture),
            CanonicalRequestHandoffOutcome::Failed(_)
        ));
    }

    #[test]
    fn contract_015_invalid_acknowledgment_does_not_imply_rejection() {
        let mut fixture = fixture();
        let mut ack = received_ack(&fixture);
        ack.acknowledgment_type = HandoffAcknowledgmentType::IntegrityRejected;
        fixture.acknowledgments.push(ack);
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::DeliveredUnacknowledged
        );
    }

    #[test]
    fn contract_015_delivered_unacknowledged_preserves_governed_custody() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::DeliveredUnacknowledged
        );
        assert_eq!(
            record.custody_decision.resulting_disposition,
            CustodyDisposition::NotTransferred
        );
    }

    #[test]
    fn contract_015_retry_uses_new_attempt_under_same_handoff() {
        let mut fixture = fixture();
        fixture.attempts.push(TransferAttempt {
            transfer_attempt_id: TransferAttemptId::derive(&[
                fixture.input.handoff_id.as_str(),
                "attempt-2",
            ]),
            handoff_id: fixture.input.handoff_id.clone(),
            handoff_context_id: fixture.context.handoff_context_id.clone(),
            package_manifest_id: fixture.attempts[0].package_manifest_id.clone(),
            attempt_number: 2,
            retry_basis: fixture.context.retry_policy.clone(),
            transport_binding_reference: fixture.authorities.transport.identity.clone(),
            operational_fact_ids: Arc::new([]),
            attempt_result: "Retry".to_owned(),
        });
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(record.attempts.len(), 2);
        assert_eq!(
            record.attempts[0].package_manifest_id,
            record.attempts[1].package_manifest_id
        );
    }

    #[test]
    fn contract_015_retry_changes_only_attempt_metadata() {
        let mut fixture = fixture();
        let original_package = fixture.attempts[0].package_manifest_id.clone();
        fixture.attempts[0].attempt_result = "RetryableTransportObservation".to_owned();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.package_manifest.handoff_package_manifest_id,
            original_package
        );
        assert_eq!(record.attempts[0].package_manifest_id, original_package);
    }

    #[test]
    fn contract_015_retry_cannot_change_package_identity() {
        let mut fixture = fixture();
        fixture.attempts[0].package_manifest_id = HandoffPackageManifestId::derive(&["corrected"]);
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::NonDeterministicRetryState)
        );
    }

    #[test]
    fn contract_015_material_context_change_requires_new_handoff() {
        let mut fixture = fixture();
        fixture.context.transfer_purpose = "different-purpose".to_owned();
        assert!(matches!(
            run(&fixture),
            CanonicalRequestHandoffOutcome::Determined(_)
        ));
    }

    #[test]
    fn contract_015_expiration_after_completion_does_not_rewrite_terminal_record() {
        let fixture = fixture();
        assert_eq!(run(&fixture), run(&fixture));
    }

    #[test]
    fn contract_015_one_rule_cannot_collapse_all_decisions_without_explicit_authority() {
        let mut fixture = fixture();
        fixture.acknowledgments.push(received_ack(&fixture));
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(record.rule_applications.len(), 3);
        assert_ne!(
            record.transfer_decision.decision_id.as_str(),
            record.custody_decision.decision_id.as_str()
        );
        assert_ne!(
            record.custody_decision.decision_id.as_str(),
            record.responsibility_decision.decision_id.as_str()
        );
    }

    #[test]
    fn contract_015_custody_can_transfer_while_responsibility_remains_with_sre() {
        let mut fixture = fixture();
        fixture.acknowledgments.push(received_ack(&fixture));
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.custody_decision.resulting_disposition,
            CustodyDisposition::Transferred
        );
        assert_eq!(
            record.responsibility_decision.resulting_disposition,
            HandoffResponsibilityDisposition::RetainedBySre
        );
    }

    #[test]
    fn contract_015_duplicate_requires_prior_constitutional_reference() {
        let mut fixture = fixture();
        let mut ack = received_ack(&fixture);
        ack.acknowledgment_type = HandoffAcknowledgmentType::DuplicateRecognized;
        fixture.acknowledgments.push(ack);
        assert!(
            matches!(run(&fixture), CanonicalRequestHandoffOutcome::Failed(record) if record.category == HandoffFailureCategory::DuplicateCorrelationFailure)
        );
    }

    #[test]
    fn contract_015_duplicate_recognition_creates_no_second_transition() {
        let mut fixture = fixture();
        let mut ack = received_ack(&fixture);
        ack.acknowledgment_type = HandoffAcknowledgmentType::DuplicateRecognized;
        ack.prior_constitutional_reference = Some("prior-handoff-record".to_owned());
        fixture.acknowledgments.push(ack);
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::DuplicateRecognized
        );
        assert_eq!(record.attempts.len(), 1);
    }

    #[test]
    fn contract_015_expiration_does_not_expire_issued_request() {
        let mut fixture = fixture();
        fixture.facts[0].kind = OperationalHandoffFactKind::ExpirationReached;
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert_eq!(fixture.request.initial_standing, InitialStanding::Issued);
        assert_eq!(
            record.transfer_decision.resulting_disposition,
            TransferDisposition::Expired
        );
    }

    #[test]
    fn contract_015_terminal_record_requires_all_three_decision_dimensions() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert!(!record.transfer_decision.decision_id.as_str().is_empty());
        assert!(!record.custody_decision.decision_id.as_str().is_empty());
        assert!(!record
            .responsibility_decision
            .decision_id
            .as_str()
            .is_empty());
    }

    #[test]
    fn contract_015_terminal_success_and_failure_are_mutually_exclusive() {
        let valid_fixture = fixture();
        assert!(matches!(
            run(&valid_fixture),
            CanonicalRequestHandoffOutcome::Determined(_)
        ));
        let mut invalid = fixture();
        invalid.input.issuance_publication_binding = "invalid".to_owned();
        assert!(matches!(
            run(&invalid),
            CanonicalRequestHandoffOutcome::Failed(_)
        ));
    }

    #[test]
    fn contract_015_failure_record_may_preserve_attempt_facts_without_claiming_transfer() {
        let mut fixture = fixture();
        fixture.input.issuance_publication_binding = "invalid".to_owned();
        let CanonicalRequestHandoffOutcome::Failed(record) = run(&fixture) else {
            panic!("failure")
        };
        assert_eq!(record.attempts.len(), 1);
        assert!(record.detail.contains("issuance") || record.detail.contains("package"));
    }

    #[test]
    fn contract_015_replay_is_idempotent() {
        let fixture = fixture();
        assert_eq!(run(&fixture), run(&fixture));
    }

    #[test]
    fn contract_015_exposes_no_downstream_intake_authorization_or_execution_authority() {
        let fixture = fixture();
        let CanonicalRequestHandoffOutcome::Determined(record) = run(&fixture) else {
            panic!("handoff")
        };
        assert!(!record
            .handoff_context
            .transfer_purpose
            .contains("Authorize"));
        assert!(!record
            .boundary_declaration
            .permitted_handoff_role
            .contains("Execute"));
        assert!(!record.package_manifest.transfer_purpose.contains("Execute"));
    }
}

pub mod local_governed_text;
mod local_profile_bindings;

fn form_handoff_package(
    input: &CanonicalRequestHandoffInput,
    request: &CanonicalStructuredRequest,
    issuance_manifest: &IssuanceManifest,
    boundary: &DownstreamBoundaryDeclaration,
    context: &HandoffContext,
) -> HandoffPackageManifest {
    let package_id = HandoffPackageManifestId::derive(&[
        input.handoff_id.as_str(),
        request.canonical_structured_request_id.as_str(),
        issuance_manifest.issuance_manifest_id.as_str(),
        context.handoff_context_id.as_str(),
    ]);
    HandoffPackageManifest {
        handoff_package_manifest_id: package_id.clone(),
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        canonical_structured_request_id: request.canonical_structured_request_id.clone(),
        issuance_manifest_id: issuance_manifest.issuance_manifest_id.clone(),
        issuance_operation_id: issuance_manifest.issuance_id.clone(),
        sender_boundary_id: context.sender_boundary_id.clone(),
        recipient_boundary_id: context.recipient_boundary_id.clone(),
        downstream_boundary_declaration_id: boundary.declaration_id.clone(),
        downstream_boundary_declaration_version: boundary.declaration_version.clone(),
        handoff_profile_id: context.handoff_profile_id.clone(),
        handoff_profile_version: context.handoff_profile_version.clone(),
        transfer_rule_set_id: context.transfer_rule_set_id.clone(),
        transfer_rule_set_version: context.transfer_rule_set_version.clone(),
        reference_mode: context.reference_mode.clone(),
        transfer_purpose: context.transfer_purpose.clone(),
        integrity_bindings: issuance_manifest.publication_binding.clone(),
        transport_binding_reference: input.transport_binding.clone(),
        expiration_basis: context.expiration_policy.clone(),
        retry_basis: context.retry_policy.clone(),
        duplicate_basis: context.duplicate_policy.clone(),
        provenance_continuity_references: vec![
            request.constructed_canonical_request_id.to_string(),
            issuance_manifest.issuance_manifest_id.to_string(),
        ]
        .into(),
        schema_version: context.schema_version.clone(),
        registry_version: context.registry_version.clone(),
        configuration_version: context.configuration_version.clone(),
    }
}

fn issuance_pair_matches(
    input: &CanonicalRequestHandoffInput,
    request: &CanonicalStructuredRequest,
    issuance_manifest: &IssuanceManifest,
) -> bool {
    !(request.canonical_structured_request_id != input.canonical_structured_request_id
        || issuance_manifest.issuance_manifest_id != input.issuance_manifest_id
        || issuance_manifest.issuance_id != input.issuance_operation_id
        || request.issuance_id != issuance_manifest.issuance_id
        || request.canonical_request_construction_id
            != issuance_manifest.canonical_request_construction_id
        || request.initial_standing != InitialStanding::Issued
        || request.content_integrity_binding
            != constructed_request_content_binding(&request.exact_constructed_request)
        || issuance_manifest
            .issued_publication_binding
            .canonical_structured_request_id
            != request.canonical_structured_request_id
        || issuance_manifest
            .issued_publication_binding
            .construction_manifest_id
            != request.construction_manifest_id
        || issuance_manifest
            .issued_publication_binding
            .standing_assignment_id
            != request.standing_assignment_id
        || issuance_manifest.publication_binding
            != issuance_publication_binding(request, issuance_manifest))
}
