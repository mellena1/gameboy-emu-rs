test-with-cov: export CARGO_INCREMENTAL = 0
test-with-cov: export RUSTFLAGS = -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
test-with-cov: export RUSTDOCFLAGS = -Cpanic=abort
test-with-cov: export RUSTC_BOOTSTRAP=1
test-with-cov:
	cargo build
	cargo test
	grcov ./target/debug/ -s . -t lcov --llvm --branch --ignore-not-existing -o lcov.info
