#!/usr/bin/env bash

set -e

cargo build --release

cp target/x86_64-unknown-none/release/aether \
   iso/boot/aether.elf

xorriso -as mkisofs \
  -b boot/limine-bios-cd.bin \
  -no-emul-boot \
  -boot-load-size 4 \
  -boot-info-table \
  --efi-boot EFI/BOOT/BOOTX64.EFI \
  -efi-boot-part \
  --efi-boot-image \
  --protective-msdos-label \
  iso \
  -o aether.iso
