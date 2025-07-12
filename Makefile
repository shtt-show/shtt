test: small_tests big_tests

small_tests:
	cargo test

big_tests: release
	@find tests -type f -name "test_*.sh" \
		| sed 's/\.sh/\.test/' \
		| xargs make SHTT_BINARY=$(PWD)/target/release/shtt

%.test:
	. $*.sh

release:
	cargo build --release
