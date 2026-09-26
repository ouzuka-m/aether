# Aether

**Experimental 64-bit Operating System Kernel in Rust**

[![Rust Kernel CI](https://github.com/ouzuka-m/aether/actions/workflows/rust.yml/badge.svg)](https://github.com/ouzuka-m/aether/actions/workflows/rust.yml)
![License](https://img.shields.io/badge/license-GPL--2.0-blue)
![Architecture](https://img.shields.io/badge/arch-x86__64-orange)
![Rust](https://img.shields.io/badge/rust-nightly-red)
![Boot](https://img.shields.io/badge/boot-UEFI%20%2B%20Limine-green)

## What is Aether?

Aether is an experimental operating system kernel written in Rust, targeting x86_64 systems with UEFI boot via the [Limine](https://github.com/limine-bootloader/limine) boot protocol. The project focuses on safety and modern kernel design.

> [!NOTE]
> Aether is in early, active development. Expect frequent breaking changes.

## Hardware Requirements

- 64-bit Intel CPU with **TSC-Deadline** timer support
- **UEFI** firmware (legacy BIOS is **not** supported)
- **KVM** for virtualized testing (QEMU TCG is insufficient — TSC-Deadline is not fully emulated)

## Prerequisites

| Tool                     | Purpose                                |
| ------------------------ | -------------------------------------- |
| `rustc` + `cargo`        | Rust compiler and build system (nightly, pinned via `rust-toolchain.toml`) |
| `xorriso`                | Creating bootable ISO images           |
| `tar`                    | Packing the initial root filesystem    |
| `qemu-system-x86_64`    | Running the kernel in a virtual machine |
| OVMF                     | UEFI firmware for QEMU                 |

### Host Platform

> [!IMPORTANT]
> **Linux** is the only supported build and development platform.
> Windows and macOS have not been tested and are not supported.

### Installing Prerequisites (Arch Linux)

```bash
sudo pacman -S qemu-full xorriso edk2-ovmf
```

### Installing Prerequisites (Debian / Ubuntu)

```bash
sudo apt install qemu-system-x86 xorriso ovmf
```

Rust nightly is managed automatically via [`rust-toolchain.toml`](rust-toolchain.toml) — simply install [rustup](https://rustup.rs/) and the correct toolchain will be used.

## Building

Build scripts are located in the [`build/`](build/) directory.

### Debug Build

```bash
./build/debug.sh
```

Compiles the kernel in debug mode and creates `aether.iso`.

### Release Build

```bash
./build/release.sh
```

Compiles the kernel with optimizations and creates `aether.iso`.

Both scripts will:
1. Pack `rootfs/` into `iso/boot/initramfs.tar`
2. Compile the kernel ELF binary
3. Generate a bootable ISO with Limine

## Running with QEMU

After building, launch the ISO with QEMU + OVMF:

```bash
qemu-system-x86_64 \
  -bios /usr/share/edk2/x64/OVMF.fd \
  -cdrom aether.iso \
  -cpu host \
  -enable-kvm
```

To capture serial output in the terminal:

```bash
qemu-system-x86_64 \
  -bios /usr/share/edk2/x64/OVMF.fd \
  -cdrom aether.iso \
  -cpu host \
  -enable-kvm \
  -serial stdio
```

> [!TIP]
> The OVMF firmware path varies by distro. Common paths:
> - Arch Linux — `/usr/share/edk2/x64/OVMF.fd`
> - Ubuntu/Debian — `/usr/share/OVMF/OVMF_CODE.fd`

## Directory Layout

```
aether/
├── build/          Build scripts (debug.sh, release.sh)
├── iso/            ISO structure with Limine bootloader binaries
├── rootfs/         Root filesystem files packed into initramfs
├── src/            Kernel source code
│   ├── arch/         Architecture-specific code (x86_64)
│   ├── boot/         Boot initialization
│   ├── display/      Framebuffer and text rendering
│   ├── drivers/      Device drivers
│   ├── fs/           Filesystem (tarfs)
│   ├── log/          Kernel logging
│   ├── memory/       Frame allocator, mapper, heap
│   ├── qemu/         QEMU integration utilities
│   ├── config.rs     Kernel configuration
│   └── main.rs       Kernel entry point
├── targets/        Custom target specification (x86_64-unknown-none.json)
├── linker.ld       Linker script for kernel section layout
├── Cargo.toml      Package manifest and dependencies
└── rust-toolchain.toml  Pinned nightly Rust toolchain
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on code quality, toolchain management, and contribution workflow.

## License

Aether is licensed under the [GNU General Public License v2.0 (GPL-2.0)](LICENSE).
