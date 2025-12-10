.PHONY: test
test: # Run all tests.
	cargo test --workspace -- --nocapture

.PHONY: fmt
fmt: # Run `rustfmt` on the entire workspace
	cargo +nightly fmt --all

.PHONY: clippy
clippy: # Run `clippy` on the entire workspace.
	cargo clippy --all --all-targets --no-deps -- --deny warnings

.PHONY: lint
lint: fmt clippy sort # Run all linters.

.PHONY: sort
sort: # Run `cargo sort` on the entire workspace.
	cargo sort --grouped --workspace

.PHONY: clean-deps
clean-deps: # Run `cargo udeps`
	cargo +nightly udeps --workspace --tests --all-targets --release

.PHONY: pr
pr: lint clean-deps test
