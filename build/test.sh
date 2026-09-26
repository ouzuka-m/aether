#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

if [ $# -ne 1 ]; then
    echo "Usage: $0 <OVMF firmware file>"
    exit 1
fi

OVMF_FILE="$1"

if [ ! -f "$OVMF_FILE" ]; then
    echo "OVMF firmware not found: $OVMF_FILE"
    exit 1
fi

echo "==> Checking formatting"
cargo fmt --check

echo "==> Running clippy"
cargo clippy -- -D warnings

echo "==> Building release ISO"
bash ./build/release.sh

echo "==> Launching QEMU"

set +e

qemu-system-x86_64 \
    -cdrom aether.iso \
    -bios "$OVMF_FILE" \
    -cpu host \
    -enable-kvm \
    -serial stdio \
    -display none \
    -device isa-debug-exit,iobase=0xf4

status=$?

set -e

echo "QEMU exit code: $status"

if [ "$status" -eq 1 ]; then
    echo "Test passed"
    exit 0
fi

echo "Test failed"
exit "$status"
