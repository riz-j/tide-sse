.PHONY: dev watch

dev:
	cargo run

watch:
	cargo watch -q -c -x run
