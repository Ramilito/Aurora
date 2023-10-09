SHELL := /bin/bash

.PHONY dev:
dev:
	cargo watch -w src -x run

.PHONY bind-wasm:
bind-wasm:
	wasm-bindgen --out-name aurora \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/wasm-release/aurora.wasm

.PHONY build-wasm:
build-wasm:
	cargo build --profile wasm-release --target wasm32-unknown-unknown

.PHONY run-wasm:
run-wasm:
	basic-http-server

.PHONY build:
build: build-wasm bind-wasm

.PHONY run:
run:
	trunk build --release && trunk serve --open

.ONESHELL:
.PHONY run-wsl:
run-wsl:
	@echo run-wsl
	cargo build --target x86_64-pc-windows-gnu
	mkdir -p ./target/x86_64-pc-windows-gnu/debug/assets
	rsync ./assets ./target/x86_64-pc-windows-gnu/debug -r --exclude-from=.gitignore
	cargo run --target x86_64-pc-windows-gnu
