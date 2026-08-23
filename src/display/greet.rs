use core::arch::x86_64::__cpuid;

use alloc::string::String;

use crate::{
    boot::info::{ENTRIES, FRAMEBUFFER},
    println,
};

pub fn welcome() {
    println!("Welcome to Aether!\n");

    let mut bytes = [0u8; 48];

    for i in 0..3 {
        let r = __cpuid(0x80000002 + i);

        let offset = i as usize * 16;

        bytes[offset..offset + 4].copy_from_slice(&r.eax.to_le_bytes());
        bytes[offset + 4..offset + 8].copy_from_slice(&r.ebx.to_le_bytes());
        bytes[offset + 8..offset + 12].copy_from_slice(&r.ecx.to_le_bytes());
        bytes[offset + 12..offset + 16].copy_from_slice(&r.edx.to_le_bytes());
    }

    let brand = String::from_utf8_lossy(&bytes);
    println!("CPU: {brand}");

    let mut memsize = 0usize;

    for entry in *ENTRIES {
        memsize += entry.length as usize;
    }

    memsize = memsize / 1024 / 1024;

    println!("Memory: {}MiB", memsize);

    let (width, height) = {
        let framebuffer = FRAMEBUFFER.lock();
        (framebuffer.width(), framebuffer.height())
    };

    println!("Framebuffer: {width}x{height}\n");

    println!("Report issues at https://github.com/ouzuka-m/aether/issues");
}
