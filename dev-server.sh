#!/bin/bash
set -euxo pipefail

pgrep webpack || (cd web/www && npm start &)
(cd web && wasm-pack build)
