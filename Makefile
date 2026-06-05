.PHONY: test-fast lean-build

test-fast:
	cargo test --workspace --test unified_field_theory_candidate_synthesis

lean-build:
	cd GPD/formal && lake build
