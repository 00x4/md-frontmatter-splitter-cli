default: build-debug fmt clippy test

build-debug:
  cargo build

build-release:
  cargo build --release

fmt:
  cargo fmt

clippy:
  cargo clippy

test:
  cargo test

clean:
  cargo clean

run-sample:
  cargo run -- sample.md
