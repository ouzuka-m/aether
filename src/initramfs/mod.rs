//! Filesystem subsystem.
//!
//! Currently provides a read-only tar archive filesystem ([`tarfs`])
//! backed by a Limine boot module.
//!
//!

pub mod entry;
pub mod error;
pub mod tar;

use spin::lazylock::LazyLock;

use crate::{
    boot::info::MODULES,
    initramfs::{entry::Entry, tar::Tar},
};

const BLOCK: usize = 512;
const NAME_SKIP: usize = 6;

pub static TAR: LazyLock<Tar> = LazyLock::new(|| {
    let module = MODULES
        .modules()
        .iter()
        .next()
        .expect("Module is empty, can't parse tar archive");

    let data = module.data();
    let mut offset = 0usize;

    let mut tarfs = Tar::new();

    while (offset + BLOCK) < data.len() {
        let end_header = offset + BLOCK;

        let header_block = &data[offset..end_header];
        if header_block.iter().all(|b| *b == 0) {
            break;
        }

        let name = str::from_utf8(&header_block[NAME_SKIP..100])
            .expect("Failed to parse filename")
            .trim_matches('\0');

        let size = {
            let mut value = 0u64;

            for &byte in &header_block[124..136] {
                if byte == 0 || byte == b' ' {
                    break;
                }

                value = value * 8 + (byte - b'0') as u64;
            }

            value
        };

        let data = &data[end_header..(end_header + size as usize)];

        tarfs.push(Entry::new(name, data));

        offset += (BLOCK + size as usize + 511) & !511;
    }

    tarfs
});
