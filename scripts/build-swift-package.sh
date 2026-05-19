#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FFI_CRATE="$ROOT/crates/trafilatura-ffi"
SWIFT_PACKAGE="$ROOT/swift/TrafilaturaSwift"
FRAMEWORKS_DIR="$SWIFT_PACKAGE/Frameworks"
OUTPUT="$FRAMEWORKS_DIR/TrafilaturaFFI.xcframework"
UNIVERSAL_DIR="$ROOT/target/universal-apple-darwin/release"
UNIVERSAL_LIB="$UNIVERSAL_DIR/libtrafilatura_ffi.a"

if ! command -v xcodebuild >/dev/null 2>&1; then
  echo "error: xcodebuild is required to create TrafilaturaFFI.xcframework" >&2
  exit 1
fi

rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

cargo build --release --package trafilatura-ffi --target aarch64-apple-darwin
cargo build --release --package trafilatura-ffi --target x86_64-apple-darwin

rm -rf "$OUTPUT"
mkdir -p "$FRAMEWORKS_DIR" "$UNIVERSAL_DIR"

lipo -create \
  "$ROOT/target/aarch64-apple-darwin/release/libtrafilatura_ffi.a" \
  "$ROOT/target/x86_64-apple-darwin/release/libtrafilatura_ffi.a" \
  -output "$UNIVERSAL_LIB"

xcodebuild -create-xcframework \
  -library "$UNIVERSAL_LIB" \
  -headers "$FFI_CRATE/include" \
  -output "$OUTPUT"

echo "Created: $OUTPUT"
