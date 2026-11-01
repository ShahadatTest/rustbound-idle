#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

wasm-pack build \
  --release \
  --target web \
  --out-dir web/pkg \
  --out-name rustbound_idle

rm -rf dist
mkdir -p dist/pkg

cp web/index.html web/idle-game.js web/styles.css dist/
cp web/pkg/rustbound_idle.js web/pkg/rustbound_idle_bg.wasm dist/pkg/

printf '\nBuilt static site in %s/dist\n' "$ROOT_DIR"
printf 'wasm size: %s bytes\n' "$(wc -c < dist/pkg/rustbound_idle_bg.wasm)"
