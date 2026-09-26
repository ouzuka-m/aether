//! Tar archive filesystem (TarFS).
//!
//! Parses a USTAR-format tar archive loaded as a Limine boot module
//! and provides simple path-based file lookup. Used to load the initial
//! root filesystem (`initramfs.tar`) at boot.

#![allow(dead_code)]

use alloc::vec::Vec;
use spin::once::Once;

use crate::boot::info::MODULES;

const BLOCK: usize = 512;

static TARFS: Once<Vec<Entry>> = Once::new();

#[derive(Debug)]
pub struct Entry<'a> {
    name: &'a str,
    size: u64,
    data: &'a [u8],
}

/// Parses the first boot module as a tar archive and stores the entries.
///
/// # Panics
/// Panics if no boot module is available.
pub fn init() {
    let tarfs = MODULES
        .modules()
        .iter()
        .next()
        .expect("Module is empty, can't parse tar archive");

    let entries = parse(tarfs.data());

    crate::info!("TarFS initialized ({} entries)", entries.len());

    TARFS.call_once(|| entries);
}

/// Opens a file by path, returning a reference to its [`Entry`] if found.
pub fn open<'a>(path: &str) -> Option<&'a Entry<'a>> {
    let tarfs = tarfs();
    tarfs.iter().find(|entry| entry.name == path)
}

/// Returns the raw byte contents of a tar entry.
pub fn read<'a>(entry: &'a Entry<'a>) -> &'a [u8] {
    entry.data
}

/// Parses a raw byte slice as a USTAR tar archive into a list of entries.
fn parse(data: &[u8]) -> Vec<Entry<'_>> {
    let mut offset = 0usize;
    let mut entries = Vec::new();

    while (offset + BLOCK) < data.len() {
        let end_header = offset + BLOCK;

        let header_block = &data[offset..end_header];
        if header_block.iter().all(|b| *b == 0) {
            break;
        }

        let name = str::from_utf8(&header_block[0..100])
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

        entries.push(Entry { name, size, data });

        offset += (BLOCK + size as usize + 511) & !511;
    }

    entries
}

fn tarfs<'a>() -> &'a Vec<Entry<'a>> {
    TARFS.get().expect("VFS hasn't been initialized")
}
