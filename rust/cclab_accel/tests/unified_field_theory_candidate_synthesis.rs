use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, canonical_ufts002_records, canonical_ufts003_descriptors,
    canonical_ufts004_descriptors, canonical_ufts005_compatibility_rows,
    canonical_ufts006_rollback_rows, paper18_skeleton_marker,
    ufts002_finite_candidate_synthesis_record_closed,
    ufts003_assumption_dependency_gate_reference_closed, ufts004_consistency_conflict_risk_closed,
    ufts005_paper17_promotion_attempt_compatibility_closed,
    ufts006_stability_audit_rollback_closed, Paper18SkeletonCertificate, UFTS001UpstreamBinding,
    PAPER17_FINAL_CERTIFICATE, PAPER17_FORMAL_ENDPOINT, PAPER17_FROZEN_COMMIT,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|err| {
        panic!("failed to read {path}: {err}");
    })
}

#[test]
fn required_scaffold_files_exist() {
    for path in [
        "README.md",
        "UPSTREAM-PAPERS.json",
        "GPD/STATE.md",
        "GPD/ROADMAP.md",
        "GPD/state.json",
        "docs/unified_field_theory_candidate_synthesis_theorem.md",
        "docs/open_proof_obligations.md",
        "GPD/formal/FiniteCapacity/UnifiedFieldTheoryCandidateSynthesis.lean",
    ] {
        assert!(
            repo_root().join(path).exists(),
            "missing required file: {path}"
        );
    }
}

#[test]
fn ufts001_consumes_closed_paper17_chain_without_uft_claims() {
    let binding = UFTS001UpstreamBinding::canonical();
    assert!(binding.closes_ufts001());
    assert_eq!(binding.paper17_frozen_commit, PAPER17_FROZEN_COMMIT);
    assert_eq!(binding.paper17_formal_endpoint, PAPER17_FORMAL_ENDPOINT);
    assert_eq!(binding.paper17_final_certificate, PAPER17_FINAL_CERTIFICATE);
    assert!(binding
        .claim_boundary
        .all_unified_field_and_physical_success_claims_remain_false());
}

#[test]
fn initial_skeleton_keeps_paper18_theorem_open() {
    let skeleton = Paper18SkeletonCertificate::initial_ufts001_only();
    assert!(skeleton.ufts001_upstream_binding_closed);
    assert!(!skeleton.ufts002_finite_candidate_synthesis_record_closed);
    assert!(!skeleton.ufts008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper18_theorem());
    assert_eq!(
        paper18_skeleton_marker(),
        "paper18-unified-field-theory-candidate-synthesis-ufts001-nonpromoting-skeleton"
    );
}

#[test]
fn ufts002_defines_finite_candidate_synthesis_records_without_promotion() {
    assert!(ufts002_finite_candidate_synthesis_record_closed());
    let records = canonical_ufts002_records();
    assert_eq!(records.len(), 1);
    for record in records {
        assert!(record.is_bounded_auditable_non_promoting());
        assert_eq!(
            record.audit_status_descriptor,
            "bounded-auditable-nonpromoting"
        );
        assert!(record
            .claim_boundary
            .all_unified_field_and_physical_success_claims_remain_false());
    }

    let skeleton = Paper18SkeletonCertificate::after_ufts002();
    assert!(skeleton.ufts001_upstream_binding_closed);
    assert!(skeleton.ufts002_finite_candidate_synthesis_record_closed);
    assert!(!skeleton.ufts003_assumption_dependency_gate_reference_closed);
    assert!(!skeleton.closes_paper18_theorem());
}

#[test]
fn ufts003_defines_dependency_and_gate_descriptors_without_success() {
    assert!(ufts003_assumption_dependency_gate_reference_closed());
    let descriptors = canonical_ufts003_descriptors();
    assert_eq!(descriptors.len(), 1);
    for descriptor in descriptors {
        assert!(descriptor.is_finite_auditable_non_sufficient());
        assert!(!descriptor.assumption_discharge_claim);
        assert!(!descriptor.dependency_sufficiency_claim);
        assert!(!descriptor.gate_success_claim);
        assert!(descriptor
            .upstream_reference_labels
            .contains(&PAPER17_FROZEN_COMMIT));
        assert!(descriptor
            .upstream_reference_labels
            .contains(&PAPER17_FORMAL_ENDPOINT));
        assert!(descriptor
            .upstream_reference_labels
            .contains(&PAPER17_FINAL_CERTIFICATE));
    }

    let skeleton = Paper18SkeletonCertificate::after_ufts003();
    assert!(skeleton.ufts002_finite_candidate_synthesis_record_closed);
    assert!(skeleton.ufts003_assumption_dependency_gate_reference_closed);
    assert!(!skeleton.ufts004_consistency_conflict_risk_closed);
    assert!(!skeleton.closes_paper18_theorem());
}

#[test]
fn ufts004_defines_conflict_and_risk_rows_without_discharge() {
    assert!(ufts004_consistency_conflict_risk_closed());
    let descriptors = canonical_ufts004_descriptors();
    assert_eq!(descriptors.len(), 1);
    for descriptor in descriptors {
        assert!(descriptor.is_finite_unresolved_non_successful());
        assert!(!descriptor.consistency_success_claim);
        assert!(!descriptor.conflict_resolution_success_claim);
        assert!(!descriptor.risk_discharge_claim);
        assert!(descriptor.unresolved_status_labels.contains(&"open"));
        assert!(descriptor
            .unresolved_status_labels
            .contains(&"non-discharged"));
    }

    let skeleton = Paper18SkeletonCertificate::after_ufts004();
    assert!(skeleton.ufts003_assumption_dependency_gate_reference_closed);
    assert!(skeleton.ufts004_consistency_conflict_risk_closed);
    assert!(!skeleton.ufts005_paper17_promotion_attempt_compatibility_closed);
    assert!(!skeleton.closes_paper18_theorem());
}

#[test]
fn ufts005_keeps_paper17_compatibility_non_promoting() {
    assert!(ufts005_paper17_promotion_attempt_compatibility_closed());
    let rows = canonical_ufts005_compatibility_rows();
    assert_eq!(rows.len(), 1);
    for row in rows {
        assert!(row.is_paper17_compatible_without_promotion_success());
        assert_eq!(row.paper17_frozen_commit, PAPER17_FROZEN_COMMIT);
        assert_eq!(row.paper17_formal_endpoint, PAPER17_FORMAL_ENDPOINT);
        assert_eq!(row.paper17_final_certificate, PAPER17_FINAL_CERTIFICATE);
        assert!(!row.physical_promotion_attempt_success_claim);
        assert!(!row.physical_promotion_claim);
        assert!(!row.physical_validation_claim);
        assert!(!row.empirical_adequacy_claim);
    }

    let skeleton = Paper18SkeletonCertificate::after_ufts005();
    assert!(skeleton.ufts004_consistency_conflict_risk_closed);
    assert!(skeleton.ufts005_paper17_promotion_attempt_compatibility_closed);
    assert!(!skeleton.ufts006_stability_audit_rollback_closed);
    assert!(!skeleton.closes_paper18_theorem());
}

#[test]
fn ufts006_defines_auditable_rollback_without_empirical_stability() {
    assert!(ufts006_stability_audit_rollback_closed());
    let rows = canonical_ufts006_rollback_rows();
    assert_eq!(rows.len(), 1);
    for row in rows {
        assert!(row.is_auditable_rollback_non_empirical());
        assert!(row.rollback_available);
        assert!(!row.empirical_stability_claim);
        assert!(!row.falsification_success_claim);
        assert!(!row.physical_failure_claim);
        assert!(row
            .rollback_trigger_labels
            .contains(&"claim-boundary-violation"));
    }

    let skeleton = Paper18SkeletonCertificate::after_ufts006();
    assert!(skeleton.ufts005_paper17_promotion_attempt_compatibility_closed);
    assert!(skeleton.ufts006_stability_audit_rollback_closed);
    assert!(!skeleton.ufts007_no_hidden_unified_field_nature_validation_audit_closed);
    assert!(!skeleton.closes_paper18_theorem());
}

#[test]
fn upstream_json_records_paper17_certificate_and_non_uft_boundary() {
    let upstream = read_repo_file("UPSTREAM-PAPERS.json");
    assert!(upstream.contains(PAPER17_FROZEN_COMMIT));
    assert!(upstream.contains(PAPER17_FORMAL_ENDPOINT));
    assert!(upstream.contains(PAPER17_FINAL_CERTIFICATE));
    assert!(upstream
        .contains("\"paper17_conditional_physical_promotion_attempt_theorem_closed\": true"));
    assert!(upstream.contains("\"paper17_physical_promotion_attempt_success_claim\": false"));
    assert!(upstream.contains("\"paper17_physical_promotion_claim\": false"));
    assert!(upstream.contains("\"paper17_unified_field_theory_claim\": false"));
}

#[test]
fn docs_keep_active_rung_and_uft_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/unified_field_theory_candidate_synthesis_theorem.md");

    assert_eq!(active_obligation(), "UFTS-007");
    assert!(state.contains("\"active_obligation\": \"UFTS-007\""));
    assert!(state.contains("\"unified_field_theory_candidate_synthesis_theorem_closed\": false"));
    assert!(state.contains("\"ufts002_finite_candidate_synthesis_record_closed\": true"));
    assert!(state.contains("\"ufts003_assumption_dependency_gate_reference_closed\": true"));
    assert!(state.contains("\"ufts004_consistency_conflict_risk_closed\": true"));
    assert!(state.contains("\"ufts005_paper17_promotion_attempt_compatibility_closed\": true"));
    assert!(state.contains("\"ufts006_stability_audit_rollback_closed\": true"));
    assert!(state.contains("\"candidate_synthesis_success_claim\": false"));
    assert!(state.contains("\"unified_field_theory_claim\": false"));
    assert!(state.contains("\"physical_nature_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state_md.contains("`UFTS-006` is closed"));
    assert!(theorem.contains("UFTS-007"));
    assert!(theorem.contains("no hidden unified-field"));
}

#[test]
fn repository_contains_no_python_artifacts() {
    let forbidden = [
        "pyproject.toml",
        "requirements.txt",
        "setup.py",
        "setup.cfg",
    ];
    for path in forbidden {
        assert!(
            !repo_root().join(path).exists(),
            "unexpected Python artifact: {path}"
        );
    }
}
