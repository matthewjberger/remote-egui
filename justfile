set windows-shell := ["powershell.exe"]

export RUST_LOG := "info"

[private]
default:
  @just --list

# Build the editor static site
build-site:
  cd crates/editor; trunk build;

# Create a new widget by copying the template crate
create-widget name:
  cp -r ./crates/widgets/template ./crates/widgets/{{name}}

# Show the workspace documentation
docs: build
  cargo doc --open -p editor

# Build the editor / http server / static site bundle
build-bundle:
  @just build --features bundled

# Build the editor
build *args="": build-site
  cargo build -r --all {{args}}

# Serve the editor static site
serve address='127.0.0.1' port='9003':
  cd crates/editor; trunk serve --address {{address}} --port {{port}};

# Check the workspace
check:
  cargo check --all --tests
  cargo fmt --all --check

# Run the test suite
test:
  cargo test --workspace

# Run the editor as a desktop app or with custom cli args
run *args="desktop":
  cargo run -r -p editor -- {{args}}

# Run the editor in a web browser
run-web port='9002':
  cargo run -r -p editor -- browser --port {{ port }}

# Run the editor backend websocket server
run-server:
  cargo run -r -p editor -- server

# Fix all automatically resolvable lints with clippy
fix:
  cargo clippy --fix --allow-dirty

# Install wasm tooling
init-wasm:
  cargo install --locked trunk

# Install wasm tooling on an M1 mac
init-wasm-m1: init-wasm
  cargo install --locked wasm-bindgen-cli

# Autoformat the workspace
format:
  cargo fmt --all

# Lint the workspace
lint:
  cargo clippy --all --tests -- -D warnings

# Display toolchain versions
@versions:
  rustc --version
  cargo fmt -- --version
  cargo clippy -- --version
  rustup --version
