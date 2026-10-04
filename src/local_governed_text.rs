//! Approved local, literal-utterance profile; request provenance is never runtime authority.
use super::*;
pub const PROFILE: &str = "ulantra-governed-text-sre-v1";
pub const VERSION: &str = "1.0.0";
pub const SOURCE: &str = "mineard-local-ulantra-sre-assignments-v1";
pub const RECIPIENT: &str = "ibos-local-governed-text-intake-v1";
const IMPLEMENTATION: &str = "ulantra-sre-ibos-local-integration-v1";
fn sid(label: &str) -> StableId {
    StableId::from_parts("local-profile", &[PROFILE, label])
}
fn strings(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|x| (*x).to_owned()).collect()
}
macro_rules! binding {
    ($ty:ident, $label:expr) => {
        $ty {
            identity: sid($label),
            version: VERSION.into(),
            authority_reference: SOURCE.into(),
        }
    };
}
macro_rules! success {
    ($operation:expr, $variant:path) => {
        match $operation {
            $variant(value) => value,
            other => return Err(format!("{:?}", other)),
        }
    };
}
/// Mechanically bind externally supplied proposal content to the approved identity/profile.
/// This function does not produce interpretation content.
pub fn bind_proposal(
    request: &InterpretationRequest,
    content: ProposalContent,
) -> InterpretationProposal {
    InterpretationProposal::new(
        request,
        sid("ulantra-governed-text-interpretation-producer"),
        VERSION,
        request.interpreter_profile().clone(),
        ProposalProductionState::Returned,
        ProposalCompletionState::Complete,
        request.included_source_ids().to_vec(),
        Some(evidence_profile()),
        content,
        request.replay_context().clone(),
    )
}
fn evidence_profile() -> EvidenceCapabilityProfile {
    EvidenceCapabilityProfile {
        identity: sid("literal-source-evidence"),
        version: VERSION.into(),
        authority_reference: SOURCE.into(),
        scope: PROFILE.into(),
        required_forms: BTreeSet::new(),
        permitted_forms: BTreeSet::from([EvidenceForm::SourceLevel]),
        permitted_statuses: BTreeSet::from([EvidenceStatus::EvidenceProvided]),
        allow_no_result: false,
    }
}
/// In-memory exact publication. There is no persistence or restored authority.
pub struct IssuedText {
    pub request: CanonicalStructuredRequest,
    pub manifest: IssuanceManifest,
    pub source_intake_id: String,
    pub source_id: String,
    pub proposal_id: String,
    pub provenance_id: String,
    message: String,
}
/// Execute existing SRE operations under the approved local profile.
pub fn issue(
    message: &str,
    conversation: &str,
    producer: &dyn ExternalInterpreter,
) -> Result<IssuedText, String> {
    if message.trim().is_empty() || message.len() > 32768 || conversation.len() > 128 {
        return Err("invalid local source bounds".into());
    }
    let component = SourceComponentInput::new(
        SourceCategory::NaturalLanguage,
        SourceOrigin::InteractiveUser,
        SourcePayload::inline(message.as_bytes()),
        AdmissionState::Accepted,
    )
    .with_preservation(PreservationFacts::recoverable(
        "exact UTF-8 source retained for this local turn",
    ));
    let submission = SourceSubmission::new(vec![component])
        .with_declared_context("conversation_id", conversation)
        .with_declared_context("operational_profile", PROFILE);
    let intake = success!(
        admit(&submission, &CompositeAdmissionPolicy::Default).map_err(|x| format!("{x:?}"))?,
        IntakeOutcome::Success
    );
    let context = InterpretationOperationContext::from_intake(&intake);
    let request = success!(
        issue_interpretation_request(
            &context,
            InterpretationRequestSpec {
                included_source_ids: intake.source_ids(),
                excluded_source_ids: vec![],
                requested_scope: BTreeSet::from([ProposalDomain::Objectives]),
                contract_version: "0.1.0".into(),
                proposal_schema_version: "ulantra-literal-proposal-v1".into(),
                interpreter_profile: InterpreterProfileBinding {
                    identity: sid("ulantra-literal-utterance-v1"),
                    version: VERSION.into()
                },
                evidence_profile: evidence_profile().binding(),
                required_proposal_metadata: BTreeSet::new(),
                permitted_response_forms: strings(&["structured-proposal"]),
                completion_expectations: strings(&["returned"]),
                declared_bounds: strings(&["DirectQuotationOnly"]),
                declared_exclusions: strings(&["Inference", "Permission", "ExternalTruth"]),
                replay_context: BTreeMap::from([
                    ("profile".into(), PROFILE.into()),
                    ("version".into(), VERSION.into())
                ])
            }
        ),
        InterpretationRequestOutcome::Request
    );
    let mut proposals = producer.produce(&request);
    if proposals.len() != 1 {
        return Err("one literal proposal required".into());
    }
    let proposal = proposals.remove(0);
    let content = &proposal.content;
    if content.objective_elements.len() != 1
        || content.objective_elements[0].expression != message
        || content.objective_elements[0].basis != "DirectQuotation"
        || content.objective_elements[0].origin != "ExplicitSource"
        || content.objective_elements[0].form != "Atomic"
        || content.objective_elements[0].class_name != "RequestedOutcome"
        || content.objective_elements[0].scope.as_deref() != Some("request-wide")
        || !content.objective_elements[0].designations.is_empty()
        || !content.objective_elements[0].relationships.is_empty()
        || !content.objective_elements[0]
            .component_expressions
            .is_empty()
        || content.objective_elements[0]
            .representation_status
            .is_some()
        || content.objective_elements[0].evidence_status != Some(EvidenceStatus::EvidenceProvided)
        || content.objective_elements[0].evidence_references.as_ref()
            != content.evidence_references.as_ref()
        || content.evidence_references.len() != 1
        || content.evidence_references[0].form != EvidenceForm::SourceLevel
        || content.evidence_references[0].status != EvidenceStatus::EvidenceProvided
        || content.evidence_references[0].reference != intake.source_ids()[0].to_string()
        || !content.constraint_elements.is_empty()
        || !content.capability_elements.is_empty()
        || !content.clarification_elements.is_empty()
        || !content.evidence_elements.is_empty()
        || content.proposed_elements.as_ref() != [message]
        || !content.declared_assumptions.is_empty()
        || !content.declared_bounds.is_empty()
        || !content.declared_uncertainties.is_empty()
        || proposal.interpreter_identity != sid("ulantra-governed-text-interpretation-producer")
    {
        return Err("proposal violates approved literal scope".into());
    }
    let proposal_id = proposal.proposal_id.to_string();
    let decision = success!(
        admit_interpretation_proposal(&request, &proposal, VERSION),
        ProposalAdmissionOutcome::Decision
    );
    if decision.disposition() != AdmissionDisposition::Admitted {
        return Err(format!("{decision:?}"));
    }
    let admitted = success!(
        publish_admitted_proposal_set(
            &request,
            vec![(proposal, *decision)],
            SetPublicationPolicy {
                identity: sid("literal-publication"),
                version: VERSION.into(),
                criteria_version: VERSION.into(),
                authority_reference: SOURCE.into(),
                allow_zero_admitted: false
            }
        ),
        SetPublicationOutcome::Set
    );
    let mut op = FixtureObjectiveRepresentationProfile::local_governed_text();
    op.allow_empty = false;
    op.allow_decomposition = false;
    op.allowed_forms = BTreeSet::from([ObjectiveForm::Atomic]);
    op.allowed_origins = BTreeSet::from([ObjectiveOrigin::ExplicitSource]);
    op.allowed_bases = BTreeSet::from([RepresentationBasis::DirectQuotation]);
    let objectives = success!(
        represent_objectives(ObjectiveRepresentationInputs {
            admitted_proposal_set: &admitted,
            profile: &op,
            registries: &FixtureObjectiveRegistries::local_governed_text(),
            schema: &binding!(ObjectiveSchemaBinding, "objective-schema"),
            configuration: &binding!(ObjectiveConfigurationBinding, "objective-config")
        }),
        ObjectiveRepresentationOutcome::Set
    );
    let constraints = success!(
        represent_constraints(ConstraintRepresentationInputs {
            admitted_proposal_set: &admitted,
            objective_set: &objectives,
            profile: &FixtureConstraintRepresentationProfile::local_governed_text(),
            registries: &FixtureConstraintRegistries::local_governed_text(),
            schema: &binding!(ConstraintSchemaBinding, "constraint-schema"),
            configuration: &binding!(ConstraintConfigurationBinding, "constraint-config"),
            implementation_version: IMPLEMENTATION
        }),
        ConstraintRepresentationOutcome::Set
    );
    let capabilities = success!(
        represent_capabilities(CapabilityRepresentationInputs {
            admitted_proposal_set: &admitted,
            objective_set: &objectives,
            constraint_set: &constraints,
            profile: &FixtureCapabilityRepresentationProfile::local_governed_text(),
            registries: &FixtureCapabilityRegistries::local_governed_text(),
            schema: &binding!(CapabilitySchemaBinding, "capability-schema"),
            configuration: &binding!(CapabilityConfigurationBinding, "capability-config"),
            implementation_version: IMPLEMENTATION
        }),
        CapabilityRepresentationOutcome::Set
    );
    let clarifications = success!(
        represent_semantic_clarification(MeaningQualificationRepresentationInputs {
            admitted_proposal_set: &admitted,
            objective_set: &objectives,
            constraint_set: &constraints,
            capability_set: &capabilities,
            profile: &FixtureMeaningQualificationProfile::local_governed_text(),
            registries: &FixtureMeaningQualificationRegistries::local_governed_text(),
            schema: &binding!(MeaningQualificationSchemaBinding, "qualification-schema"),
            configuration: &binding!(
                MeaningQualificationConfigurationBinding,
                "qualification-config"
            ),
            implementation_version: IMPLEMENTATION
        }),
        MeaningQualificationOutcome::Set
    );
    let evidence = success!(
        represent_evidence(EvidenceRepresentationInputs {
            admitted_proposal_set: &admitted,
            objective_set: &objectives,
            constraint_set: &constraints,
            capability_set: &capabilities,
            clarification_set: &clarifications,
            profile: &FixtureEvidenceProfile::local_governed_text(),
            registries: &FixtureEvidenceRegistries::local_governed_text(),
            schema: &binding!(EvidenceSchemaBinding, "evidence-schema"),
            configuration: &binding!(EvidenceConfigurationBinding, "evidence-config"),
            implementation_version: IMPLEMENTATION
        }),
        EvidenceRepresentationOutcome::Set
    );
    let pp = FixtureProvenanceProfile::local_governed_text();
    let pr = FixtureProvenanceRegistries::local_governed_text();
    let pc = binding!(ProvenanceConfigurationBinding, "provenance-config");
    let source_id = intake.source_ids()[0].to_string();
    let source_intake_id = intake.record_id().to_string();
    let provenance_input = ProvenanceRepresentationInput {
        input_id: ProvenanceRepresentationInputId::derive(&[&source_intake_id, PROFILE]),
        schema_version: "provenance-input-v1".into(),
        profile_id: pp.identity.clone(),
        profile_version: pp.version.clone(),
        registry_version: pr.version.clone(),
        configuration_id: pc.identity.clone(),
        configuration_version: pc.version.clone(),
        implementation_version: IMPLEMENTATION.into(),
        admitted_proposal_set_id: admitted.set_id.clone(),
        objective_set_id: objectives.set_id.clone(),
        constraint_set_id: constraints.set_id.clone(),
        capability_set_id: capabilities.set_id.clone(),
        clarification_set_id: clarifications.set_id.clone(),
        evidence_set_id: evidence.set_id.clone(),
        source_intake_id: intake.record_id(),
        interpretation_operation_id: admitted.operation_id.clone(),
        input_proposal_ids: admitted.submitted_proposal_ids.clone(),
        input_decision_ids: admitted.decision_ids.clone(),
        subject_declarations: vec![ProvenanceSubjectDeclaration {
            declaration_id: ProvenanceDeclarationId::derive(&[&source_intake_id, "source"]),
            subject_id: source_intake_id.clone(),
            subject_class: "SourceIntakeRecord".into(),
            origin: "ApplicationSupplied".into(),
            participant_reference: Some("ulantra-governed-text-interpretation-producer".into()),
            source_reference: Some(source_id.clone()),
            basis: "ExactSourceReceipt".into(),
        }]
        .into(),
        event_declarations: vec![].into(),
        lineage_declarations: vec![LineageDeclaration {
            declaration_id: ProvenanceDeclarationId::derive(&[&proposal_id, "lineage"]),
            assertion_id: LineageAssertionId::derive(&[&proposal_id, &source_intake_id]),
            subject_artifact_id: admitted.set_id.to_string(),
            ancestor_or_related_artifact_id: source_intake_id.clone(),
            relationship: "DerivedFrom".into(),
            basis: "DirectQuotation".into(),
            origin: "ApplicationSupplied".into(),
            status: "Declared".into(),
        }]
        .into(),
        lifecycle_declarations: vec![].into(),
        external_identity_mappings: vec![].into(),
        conflict_declarations: vec![].into(),
        replay_context: request.replay_context.clone(),
    };
    let provenance = success!(
        represent_provenance(ProvenanceRepresentationInputs {
            input: &provenance_input,
            admitted_proposal_set: &admitted,
            objective_set: &objectives,
            constraint_set: &constraints,
            capability_set: &capabilities,
            clarification_set: &clarifications,
            evidence_set: &evidence,
            profile: &pp,
            registries: &pr,
            schema: &binding!(ProvenanceSchemaBinding, "provenance-schema"),
            configuration: &pc
        }),
        ProvenanceRepresentationOutcome::Success
    );
    let rp = FixtureSemanticReconciliationProfile::local_governed_text();
    let rr = FixtureSemanticReconciliationRegistries::local_governed_text();
    let subject = ReconciliationSubjectId::derive(&[&proposal_id, "literal"]);
    let element = ReconciledSemanticElementId::derive(&[&proposal_id, "literal"]);
    let reconciliation_input = SemanticReconciliationInput {
        input_id: SemanticReconciliationInputId::derive(&[&proposal_id, PROFILE]),
        operation_id: SemanticReconciliationOperationId::derive(&[&proposal_id, PROFILE]),
        admitted_proposal_set_id: admitted.set_id.clone(),
        objective_set_id: objectives.set_id.clone(),
        constraint_set_id: constraints.set_id.clone(),
        capability_set_id: capabilities.set_id.clone(),
        clarification_set_id: clarifications.set_id.clone(),
        evidence_set_id: evidence.set_id.clone(),
        provenance_set_id: provenance.set_id.clone(),
        source_intake_id: intake.record_id(),
        interpretation_operation_id: admitted.operation_id.clone(),
        input_proposal_ids: admitted.submitted_proposal_ids.clone(),
        input_decision_ids: admitted.decision_ids.clone(),
        declared_subjects: vec![ReconciliationSubjectDeclaration {
            subject_id: subject.clone(),
            upstream_publication_id: objectives.set_id.to_string(),
            upstream_representation_id: objectives.objectives[0].logical_objective_id.to_string(),
            original_expression: message.into(),
            semantic_class: "RequestedOutcome".into(),
            semantic_domain: "Objective".into(),
            represented_scope: "request-wide".into(),
            upstream_status: "Represented".into(),
            evidence_reference_ids: vec![source_id.clone()].into(),
            provenance_reference_ids: vec![provenance.set_id.to_string()].into(),
        }]
        .into(),
        declared_groups: vec![ReconciliationGroupDeclaration {
            group_id: ReconciliationGroupId::derive(&[&proposal_id, "literal"]),
            member_subject_ids: vec![subject.clone()].into(),
            semantic_domain_or_interaction_class: "Objective<->Objective".into(),
            represented_scope: "request-wide".into(),
            formation_rule_id: "ExplicitGroupDeclarationRule".into(),
            formation_rule_version: VERSION.into(),
            comparison_rule_id: "ExactIdentityRule".into(),
            comparison_rule_version: VERSION.into(),
            decision_rule_id: "ExplicitSelectionRule".into(),
            decision_rule_version: VERSION.into(),
            declared_relationship: ComparisonRelationship::ExactIdentity,
            declared_disposition: ReconciliationDisposition::Selected,
            standing_assignments: vec![
                ReconciliationStandingDeclaration {
                    assignment_id: StandingAssignmentId::derive(&[&proposal_id, "source-standing"]),
                    target_kind: "UpstreamSubject".into(),
                    target_id: subject.to_string(),
                    standing: Standing::Included,
                    basis: "DirectQuotationOnly".into(),
                },
                ReconciliationStandingDeclaration {
                    assignment_id: StandingAssignmentId::derive(&[&proposal_id, "literal"]),
                    target_kind: "ResultingReconciledElement".into(),
                    target_id: element.to_string(),
                    standing: Standing::Included,
                    basis: "DirectQuotationOnly".into(),
                },
            ]
            .into(),
            resulting_element_ids: vec![element.clone()].into(),
            result_representation_ids: vec![element.to_string()].into(),
            unresolved_conditions: vec![].into(),
        }]
        .into(),
        declared_relationships: vec![].into(),
        profile_id: rp.identity.clone(),
        profile_version: rp.version.clone(),
        grouping_registry_version: rr.version.clone(),
        comparison_registry_version: rr.version.clone(),
        decision_registry_version: rr.version.clone(),
        standing_registry_version: rr.version.clone(),
        schema_version: rp.schema_version.clone(),
        configuration_version: rp.configuration_version.clone(),
        implementation_version: IMPLEMENTATION.into(),
        replay_context: request.replay_context.clone(),
    };
    let reconciled = success!(
        represent_semantic_reconciliation(SemanticReconciliationInputs {
            input: &reconciliation_input,
            admitted_proposal_set: &admitted,
            objective_set: &objectives,
            constraint_set: &constraints,
            capability_set: &capabilities,
            clarification_set: &clarifications,
            evidence_set: &evidence,
            provenance_set: &provenance,
            profile: &rp,
            registries: &rr
        }),
        SemanticReconciliationOutcome::Success
    );
    let mut np = FixtureNormalizationProfile::local_governed_text();
    np.preservation_permissions.insert((
        "Objective".into(),
        "RequestedOutcome".into(),
        "expression".into(),
    ));
    let normalized = success!(
        normalize_semantic_request(
            &SemanticNormalizationInput::local_governed_text(&reconciled),
            &reconciled,
            &np,
            &FixtureCanonicalRegistry::local_governed_text(),
            &FixtureMappingRuleRegistry::local_governed_text()
        ),
        NormalizationOperationOutcome::Success
    );
    let ordered = success!(
        perform_canonical_ordering(
            &CanonicalOrderingInput::local_governed_text(&normalized),
            &normalized,
            &FixtureCanonicalOrderingProfile::local_governed_text()
        ),
        CanonicalOrderingOutcome::Success
    );
    let vp = FixtureValidationProfile::local_governed_text();
    let validation = success!(
        perform_structural_validation(
            &StructuralValidationInput::local_governed_text(&ordered),
            &ordered,
            &vp,
            &FixtureConstructionRequirements::local_governed_text(&vp),
            &FixtureStructuralRegistries::local_governed_text(),
            &FixtureValidationRuleSet::local_governed_text(&vp)
        ),
        StructuralValidationOutcome::Completed
    );
    let cp = FixtureConstructionProfile::local_governed_text();
    let (constructed, construction_manifest) = match perform_canonical_request_construction(
        &CanonicalRequestConstructionInput::local_governed_text(&ordered, &validation),
        &ordered,
        &validation,
        &FixtureConstructionAuthorityContext::local_governed_text(&cp),
    ) {
        CanonicalRequestConstructionOutcome::Constructed { request, manifest } => {
            (request, manifest)
        }
        other => return Err(format!("{other:?}")),
    };
    let ic = IssuanceContext::local_governed_text("ulantra-local-governed-conversation");
    let (request, manifest) = match perform_canonical_request_issuance(
        &CanonicalRequestIssuanceInput::local_governed_text(
            &constructed,
            &construction_manifest,
            &ic,
        ),
        &constructed,
        &construction_manifest,
        &FixtureIssuanceAuthorityContext::local_governed_text(&ic),
    ) {
        CanonicalRequestIssuanceOutcome::Issued { request, manifest } => (request, manifest),
        other => return Err(format!("{other:?}")),
    };
    Ok(IssuedText {
        request: *request,
        manifest: *manifest,
        source_intake_id,
        source_id,
        proposal_id,
        provenance_id: provenance.set_id.to_string(),
        message: message.into(),
    })
}

/// Only the exact approved local declaration/context can use operational version bindings.
pub(crate) fn valid_handoff_context(
    boundary: &DownstreamBoundaryDeclaration,
    context: &HandoffContext,
    authorities: &FixtureHandoffAuthorityContext,
) -> bool {
    boundary == &DownstreamBoundaryDeclaration::local_governed_text()
        && authorities == &FixtureHandoffAuthorityContext::local_governed_text()
        && context.schema_version == "ulantra-sre-handoff-schema-v1"
        && context.implementation_version == IMPLEMENTATION
        && context.handoff_profile_id == authorities.profile.identity
        && context.handoff_profile_version == authorities.profile.version
        && context.recipient_boundary_id == RECIPIENT
        && context.transfer_purpose == "ReceiveExactIssuedRequestForIndependentGovernedEvaluation"
}
pub fn prepare_handoff(issued: &IssuedText) -> Result<HandoffPackageManifest, String> {
    let boundary = DownstreamBoundaryDeclaration::local_governed_text();
    let context = HandoffContext::local_governed_text(&issued.request, &issued.manifest);
    let input = CanonicalRequestHandoffInput::local_governed_text(
        &issued.request,
        &issued.manifest,
        &context,
    );
    if !issuance_pair_matches(&input, &issued.request, &issued.manifest)
        || issued
            .request
            .exact_constructed_request
            .components
            .iter()
            .filter(|c| c.canonical_expression.is_some())
            .count()
            != 1
        || !issued
            .request
            .exact_constructed_request
            .components
            .iter()
            .any(|c| c.canonical_expression.as_deref() == Some(issued.message.as_str()))
    {
        return Err("SRE_ISSUANCE_BINDING_MISMATCH".into());
    }
    Ok(form_handoff_package(
        &input,
        &issued.request,
        &issued.manifest,
        &boundary,
        &context,
    ))
}
/// Called at recipient intake before it originates acknowledgment, never runtime permission.
pub fn verify_received<'a>(
    issued: &'a IssuedText,
    package: &HandoffPackageManifest,
    message: &str,
) -> Result<&'a str, String> {
    if &prepare_handoff(issued)? != package || issued.message != message {
        return Err("SRE_RECEIPT_BINDING_MISMATCH".into());
    }
    Ok(&issued.message)
}
/// Apply independent handoff rules to an acknowledgment originating at the recipient.
pub fn complete_handoff(
    issued: &IssuedText,
    package: &HandoffPackageManifest,
    ack: &ReceiptAcknowledgment,
) -> Result<CanonicalRequestHandoffRecord, String> {
    verify_received(issued, package, &issued.message)?;
    if ack.acknowledgment_type != HandoffAcknowledgmentType::Received {
        return Err("SRE_QUALIFIED_RECEIPT_REQUIRED".into());
    }
    let boundary = DownstreamBoundaryDeclaration::local_governed_text();
    let context = HandoffContext::local_governed_text(&issued.request, &issued.manifest);
    let input = CanonicalRequestHandoffInput::local_governed_text(
        &issued.request,
        &issued.manifest,
        &context,
    );
    let authorities = FixtureHandoffAuthorityContext::local_governed_text();
    let attempt_id = TransferAttemptId::derive(&[input.handoff_id.as_str(), "attempt-1"]);
    let fact_id =
        OperationalHandoffFactId::derive(&[input.handoff_id.as_str(), "recipient-receipt"]);
    let attempt = TransferAttempt {
        transfer_attempt_id: attempt_id.clone(),
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        package_manifest_id: package.handoff_package_manifest_id.clone(),
        attempt_number: 1,
        retry_basis: context.retry_policy.clone(),
        transport_binding_reference: authorities.transport.identity.clone(),
        operational_fact_ids: vec![fact_id.clone()].into(),
        attempt_result: "RecipientVerifiedPublication".into(),
    };
    let fact = OperationalHandoffFact {
        fact_id,
        handoff_id: input.handoff_id.clone(),
        handoff_context_id: context.handoff_context_id.clone(),
        attempt_id,
        kind: OperationalHandoffFactKind::DeliveryObserved,
        detail: "recipient generated exact-publication acknowledgment".into(),
    };
    match perform_canonical_request_handoff(
        &input,
        &issued.request,
        &issued.manifest,
        &boundary,
        &context,
        &[attempt],
        &[fact],
        std::slice::from_ref(ack),
        &authorities,
    ) {
        CanonicalRequestHandoffOutcome::Determined(record) => Ok(*record),
        other => Err(format!("{other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Literal<'a>(&'a str);
    impl ExternalInterpreter for Literal<'_> {
        fn produce(&self, request: &InterpretationRequest) -> Vec<InterpretationProposal> {
            let mut content = ProposalContent::empty();
            content.proposed_elements = vec![self.0.to_owned()].into();
            content.evidence_references = vec![EvidenceReference {
                form: EvidenceForm::SourceLevel,
                reference: request.included_source_ids()[0].to_string(),
                status: EvidenceStatus::EvidenceProvided,
            }]
            .into();
            content.objective_elements = vec![ProposalObjectiveElement {
                element_id: "literal-utterance".into(),
                expression: self.0.into(),
                form: "Atomic".into(),
                origin: "ExplicitSource".into(),
                basis: "DirectQuotation".into(),
                class_name: "RequestedOutcome".into(),
                scope: Some("request-wide".into()),
                designations: vec![].into(),
                evidence_references: vec![EvidenceReference {
                    form: EvidenceForm::SourceLevel,
                    reference: request.included_source_ids()[0].to_string(),
                    status: EvidenceStatus::EvidenceProvided,
                }]
                .into(),
                evidence_status: Some(EvidenceStatus::EvidenceProvided),
                representation_status: None,
                relationships: vec![].into(),
                component_expressions: vec![].into(),
            }]
            .into();
            vec![bind_proposal(request, content)]
        }
    }
    #[test]
    fn literal_publication_preserves_source_and_has_no_fixture_authority() {
        let text = "achieve outcome";
        let issued = issue(text, "integration-trace-001", &Literal(text)).expect("local issuance");
        assert!(issued
            .request
            .exact_constructed_request
            .components
            .iter()
            .any(|c| c.canonical_expression.as_deref() == Some(text)));
        assert!(!format!("{:?}", issued.manifest)
            .to_lowercase()
            .contains("fixture"));
    }
    #[test]
    fn producer_cannot_add_semantics_to_the_literal_utterance() {
        struct InferringProducer;
        impl ExternalInterpreter for InferringProducer {
            fn produce(&self, request: &InterpretationRequest) -> Vec<InterpretationProposal> {
                let mut proposals = Literal("literal source").produce(request);
                let mut content = proposals[0].content.clone();
                let mut objectives = content.objective_elements.to_vec();
                objectives[0].component_expressions = vec!["inferred goal".into()].into();
                content.objective_elements = objectives.into();
                proposals[0] = bind_proposal(request, content);
                proposals
            }
        }
        assert!(issue("literal source", "scope-test", &InferringProducer).is_err());
    }
}
