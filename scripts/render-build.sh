#!/usr/bin/env bash
set -euo pipefail

# Render can provide a shared, read-only Rust installation. Keep toolchains and
# Cargo downloads in the build user's writable home instead.
export RUSTUP_HOME="$HOME/.rustup"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$HOME/.local/bin:$PATH"

WASM_PACK_VERSION="v0.15.0"

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --profile minimal --no-modify-path
  # shellcheck disable=SC1091
  source "$CARGO_HOME/env"
fi

rustup toolchain install stable --profile minimal --no-self-update
rustup default stable
rustup target add wasm32-unknown-unknown

if ! command -v wasm-pack >/dev/null 2>&1; then
  mkdir -p "$HOME/.local/bin"

  case "$(uname -m)" in
    x86_64) WASM_PACK_TARGET="x86_64-unknown-linux-musl" ;;
    aarch64|arm64) WASM_PACK_TARGET="aarch64-unknown-linux-musl" ;;
    *) echo "Unsupported build architecture: $(uname -m)" >&2; exit 1 ;;
  esac

  ARCHIVE="wasm-pack-${WASM_PACK_VERSION}-${WASM_PACK_TARGET}.tar.gz"
  URL="https://github.com/wasm-bindgen/wasm-pack/releases/download/${WASM_PACK_VERSION}/${ARCHIVE}"
  TMP_DIR="$(mktemp -d)"

  curl -L --fail --silent --show-error "$URL" -o "$TMP_DIR/$ARCHIVE"
  tar -xzf "$TMP_DIR/$ARCHIVE" -C "$TMP_DIR"
  install \
    "$TMP_DIR/wasm-pack-${WASM_PACK_VERSION}-${WASM_PACK_TARGET}/wasm-pack" \
    "$HOME/.local/bin/wasm-pack"
  rm -rf "$TMP_DIR"
fi

bash ./scripts/build.sh
