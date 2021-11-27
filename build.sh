#!/usr/bin/env bash
set -euxo pipefail
cd "$(dirname "$0")/web"

if ! command -v rustup; then
  wget https://sh.rustup.rs -O rustup.sh
  chmod a+x rustup.sh
  ./rustup.sh -y
  rm rustup.sh

  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

if ! command -v wasm-pack; then
  npm install -g wasm-pack
fi

wasm-pack build --release
npm install
npm run build
