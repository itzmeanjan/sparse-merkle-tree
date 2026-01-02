default: fmt clippy check test example

BACKTRACE=RUST_BACKTRACE=1

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

check:
	cargo check --no-default-features

test:
	$(BACKTRACE) cargo test --profile test-release --all --all-features

coverage:
	cargo tarpaulin -t 600 --profile test-release --out Html

example:
	cargo run --example smt_example --no-default-features --features "turboshake"

clean:
	cargo clean

bench:
	RUSTFLAGS="-C target-cpu=native" cargo bench --profile optimized --bench sparse_merkle_tree
