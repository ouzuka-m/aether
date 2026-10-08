#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

LIMINE_BIN="$PROJECT_DIR/iso/boot/limine-uefi-cd.bin"
LIMINE_URL="https://github.com/Limine-Bootloader/Limine/releases/latest/download/limine-binary.tar.gz"

if [ -f "$LIMINE_BIN" ]; then
    echo "limine-uefi-cd.bin already exists, skipping download."
    exit 0
fi

if ! command -v curl >/dev/null 2>&1; then
    echo "error: curl is not installed or not available in PATH" >&2
    exit 1
fi

echo "downloading limine-binary.tar.gz..."

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

curl -fL -o "$TMPDIR/limine-binary.tar.gz" "$LIMINE_URL"

echo "extracting limine-uefi-cd.bin..."

tar -xzf "$TMPDIR/limine-binary.tar.gz" -C "$TMPDIR"

EXTRACTED_BIN=$(find "$TMPDIR" -name "limine-uefi-cd.bin" -type f | head -n 1)

if [ -z "$EXTRACTED_BIN" ]; then
    echo "error: limine-uefi-cd.bin not found in the archive" >&2
    exit 1
fi

mkdir -p "$PROJECT_DIR/iso/boot"
cp "$EXTRACTED_BIN" "$LIMINE_BIN"

echo "limine-uefi-cd.bin installed to iso/boot/"
