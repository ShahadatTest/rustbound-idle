#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

wasm-pack build \
  --dev \
  --target web \
  --out-dir web/pkg \
  --out-name rustbound_idle

printf '\nServing http://localhost:8080\n'
python3 -m http.server 8080 --directory web
