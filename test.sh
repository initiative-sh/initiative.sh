#!/bin/bash
set -euxo pipefail

cargo test --workspace

cargo clippy --workspace --tests -- --deny warnings

git ls-files '*.rs' | xargs rustfmt --check --edition 2021
if git grep ',)' '*.rs'; then
  exit 1
fi

# (cd web && wasm-pack test --firefox --headless)
