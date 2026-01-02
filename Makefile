default: fmt clippy test check

BACKTRACE=RUST_BACKTRACE=1

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

check:
	cargo check --no-default-features

test:
	$(BACKTRACE) cargo test --profile test-release --all --all-features

clean:
	cargo clean

bench:
	RUSTFLAGS="-C target-cpu=native" cargo bench --profile optimized --bench smt_benchmark
