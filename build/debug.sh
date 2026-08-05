#!/usr/bin/env bash

set -e

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
