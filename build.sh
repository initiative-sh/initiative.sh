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

if [[ -v DISCORD_RELEASE_WEBHOOK_URL ]] && git diff --name-only HEAD^ | grep -xq data/changelog.md; then
  read -r -d '' release_announce <<'JS'
    const https = require('https');
    if (process.env.message) {
      const data = JSON.stringify({content: process.env.message});
      https.request(
        process.env.DISCORD_RELEASE_WEBHOOK_URL,
        {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Content-Length': data.length,
          },
        },
        (res) => res.on('data', (d) => {
          process.stdout.write(d);
          if (res.statusCode >= 400) {
            process.exit(1);
          }
        })
      ).write(data);
    }
JS

  changelog="$(git diff HEAD^ data/changelog.md | awk '/^+[^+]/{ sub(/^[+ ]+/, ""); sub(/^\* /, "\n- "); printf "%s ", $0 }')"
  stats="$(git diff --shortstat HEAD^)" 
  export message="There's a new release on initiative.sh!"$'\n'"$changelog"$'\n\n'"$stats"

  node -e "$release_announce"
fi
