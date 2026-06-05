use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, canonical_ufts002_records, paper18_skeleton_marker,
    ufts002_finite_candidate_synthesis_record_closed, Paper18SkeletonCertificate,
    UFTS001UpstreamBinding, PAPER17_FINAL_CERTIFICATE, PAPER17_FORMAL_ENDPOINT,
    PAPER17_FROZEN_COMMIT,
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
fn docs_keep_ufts002_active_and_uft_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/unified_field_theory_candidate_synthesis_theorem.md");

    assert_eq!(active_obligation(), "UFTS-003");
    assert!(state.contains("\"active_obligation\": \"UFTS-003\""));
    assert!(state.contains("\"unified_field_theory_candidate_synthesis_theorem_closed\": false"));
    assert!(state.contains("\"ufts002_finite_candidate_synthesis_record_closed\": true"));
    assert!(state.contains("\"candidate_synthesis_success_claim\": false"));
    assert!(state.contains("\"unified_field_theory_claim\": false"));
    assert!(state.contains("\"physical_nature_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state_md.contains("`UFTS-002` is closed"));
    assert!(theorem.contains("UFTS-003"));
    assert!(theorem.contains("no dependency sufficiency or gate success"));
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
