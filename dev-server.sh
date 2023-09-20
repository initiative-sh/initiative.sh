#!/bin/bash
set -euxo pipefail

cd web
npm install
pgrep webpack || npm start &
wasm-pack build
