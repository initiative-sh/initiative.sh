#!/usr/bin/env bash
set -euxo pipefail

project_root="$(dirname "$(readlink -m "$0")")"


if ! command -v rustup; then
  wget https://sh.rustup.rs -O rustup.sh
  chmod a+x rustup.sh
  ./rustup.sh -y --target wasm32-unknown-unknown --profile minimal
  rm rustup.sh

  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

if ! command -v wasm-pack; then
  npm install -g wasm-pack
fi


cd "$project_root/web"

# very messy workaround for https://github.com/rustwasm/wasm-pack/issues/1420
mv package.json package.json.bak
echo -n '{}' > package.json
wasm-pack build --release
mv -f package.json.bak package.json

npm install
NODE_OPTIONS=--openssl-legacy-provider npm run build


cd "$project_root"
cargo doc --workspace --no-deps --document-private-items
rm -r web/dist/doc || true
mv target/doc web/dist/
