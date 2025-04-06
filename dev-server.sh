#!/bin/bash
set -euxo pipefail

cd web
npm install

# very messy workaround for https://github.com/rustwasm/wasm-pack/issues/1420
mv package.json package.json.bak
echo -n '{}' > package.json
wasm-pack build
mv -f package.json.bak package.json

npm start
