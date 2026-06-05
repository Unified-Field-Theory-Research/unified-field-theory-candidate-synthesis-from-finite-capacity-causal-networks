# Repository Guidelines

## Scope And Purpose

This repository is Paper 18 in the finite-capacity causal-network theorem
ladder. It starts only after the closed Paper 17 physical promotion attempt
theorem and defines the next isolated workspace: unified field theory candidate
synthesis.

Do not treat Paper 18 as a completed unified field theory, physical nature
realization, physical validation, empirical adequacy, physical promotion
success, or physical promotion attempt success. The initial rung, `UFTS-001`,
only records upstream binding and claim boundaries through the frozen Paper 17
certificate. The active next rung is `UFTS-002`: define finite candidate
synthesis records without importing synthesis success, unified-field success,
physical validation, promotion success, simulation-only promotion, or fit-only
calibration.

## Project Structure

- `README.md`: repository overview and claim boundary.
- `UPSTREAM-PAPERS.json`: frozen upstream commits and formal endpoints for
  Papers 1-17.
- `GPD/STATE.md`, `GPD/ROADMAP.md`, `GPD/state.json`: active theorem-ladder
  state and next obligation.
- `docs/unified_field_theory_candidate_synthesis_theorem.md`: theorem target
  and current closed/open rungs.
- `docs/open_proof_obligations.md`: active obligation list.
- `docs/proof_log.md`: chronological proof and guard log.
- `GPD/formal/FiniteCapacity/UnifiedFieldTheoryCandidateSynthesis.lean`: Lean
  scaffold for Paper 18 gates and fail-closed blockers.
- `rust/cclab_accel/src/lib.rs`: Rust audit surface for finite bindings and
  non-promotion flags.
- `rust/cclab_accel/tests/unified_field_theory_candidate_synthesis.rs`:
  regression guards for the Paper 18 skeleton.

## Commands

Run local checks from this repository root:

```bash
make test-fast
make lean-build
```

Use `cargo fmt --all` before committing Rust changes.

## Research Rules

Keep theorem claims conditional and local. Paper 18 may define finite synthesis
records, dependency maps, candidate-interface descriptors, compatibility
references, conflict rows, residual-risk rows, and audit rows, but it must not
claim a unified field theory unless a later explicit theorem, external evidence
packet, physical promotion gate, and review/reproduction gate close that claim.

Do not add Python artifacts. Runtime guards and audits are Rust-only; formal
scaffolding is Lean-only.

## Commit Style

Use concise Conventional Commit-style messages, for example:

- `chore: create paper 18 scaffold`
- `formal: add ufts002 synthesis-record contract`
- `test: guard paper 18 claim boundary`
