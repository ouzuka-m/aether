#![allow(unused)]

use alloc::vec::Vec;
use limine::request::ModulesRequest;
use spin::once::Once;

const BLOCK: usize = 512;

#[derive(Debug)]
pub struct Entry<'a> {
    name: &'a str,
    data: &'a [u8],
}

static MODULES_REQUEST: ModulesRequest = ModulesRequest::new();

static TARFS: Once<Vec<Entry>> = Once::new();

pub fn init() {
    let modules = MODULES_REQUEST
        .response()
        .expect("Failed to get module response from bootloader")
        .modules();

    let tarfs = modules
        .iter()
        .next()
        .expect("Module is empty, can't parse tar archive");

    let entries = parse(tarfs.data());

    TARFS.call_once(|| entries);
}

pub fn open<'a>(path: &str) -> Option<&'a Entry<'a>> {
    let tarfs = tarfs();
    tarfs.iter().find(|entry| {
        assert_eq!(entry.name, path);

        entry.name == path
    })
}

pub fn read<'a>(entry: &'a Entry<'a>) -> &'a [u8] {
    entry.data
}

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

        crate::info!("{}", alloc::string::String::from_utf8_lossy(data));

        entries.push(Entry { name, data });

        offset += (BLOCK + size as usize + 511) & !511;
    }

    entries
}

fn tarfs<'a>() -> &'a Vec<Entry<'a>> {
    TARFS.get().expect("VFS hasn't been initialized")
}
