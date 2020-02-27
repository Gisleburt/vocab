.PHONY: test docs

test:
	cargo test -- --test-threads=1

docs:
	cargo script docs
