#!/usr/bin/env sh

set -e

if ! command -v tar >/dev/null 2>&1; then
    echo "error: tar is not installed or not available in PATH" >&2
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo is not installed or not available in PATH" >&2
    exit 1
fi

if ! command -v xorriso >/dev/null 2>&1; then
    echo "error: xorriso is not installed or not available in PATH" >&2
    exit 1
fi

tar -cf iso/boot/initramfs.tar rootfs/*

cargo build

cp target/x86_64-unknown-none/debug/aether \
   iso/boot/aether.elf

xorriso -as mkisofs \
  --efi-boot boot/limine-uefi-cd.bin \
  -efi-boot-part \
  --efi-boot-image \
  --protective-msdos-label \
  iso \
  -o aether.iso
