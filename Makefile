.PHONY: all build fmt init lint pre-commit test full-test

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
ifeq ($(shell uname -s),Linux)
	@echo Linux detected..
	pin-toolchain-linux
else ifeq ($(shell uname -s),Darwin)
	@echo Macos detected..
	make pin-toolchain-mac-m1
endif

pin-toolchain-mac-m1:
	@rustup toolchain install nightly-2023-03-20 --component llvm-tools-preview
	@rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-20
	@rm -rf ~/.rustup/toolchains/nightly-aarch64-apple-darwin
	@ln -s ~/.rustup/toolchains/nightly-2023-03-20-aarch64-apple-darwin ~/.rustup/toolchains/nightly-aarch64-apple-darwin

pin-toolchain-linux:
	@rustup toolchain install nightly-2023-03-20 --component llvm-tools-preview
	@rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-20
	@rm -rf ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
	@ln -s ~/.rustup/toolchains/nightly-2023-03-20-x86_64-unknown-linux-gnu ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
	@rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint full-test

test:
	@echo ⚙️ Running unit tests...
	@cargo +nightly t

full-test:
	@echo ⚙️ Running all tests...
	@cargo +nightly t -- --include-ignored --test-threads=1
