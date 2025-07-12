help: #: Print this help menu
	@echo "USAGE:\n"
	@cat $(MAKEFILE_LIST) \
		| grep '#:' \
		| grep -v grep \
		| awk -F':' '{ OFS=":"; print $$1,$$3 }' \
		| sort \
		| column -t -s":"

test: tests.small tests.big #: Full test suite

tests.small: #: Unit and integration tests (via cargo)
	cargo test

tests.big: release #: Real workloads (shell scripts) for shtt
	@find tests -type f -name "test_*.sh" \
		| sed 's/\.sh/\.test/' \
		| xargs make SHTT_BINARY=$(PWD)/target/release/shtt

release: #: Create an optimized build
	cargo build --release

install: #: Install shtt locally (in your homedir/cargo path)
	cargo install --path .

%.test:
	. $*.sh

%.tgz: release
	mv target/release/shtt .
	tar czf $@ shtt
