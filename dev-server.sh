#!/bin/bash
set -euxo pipefail

cd web
npm install
wasm-pack build
npm start
