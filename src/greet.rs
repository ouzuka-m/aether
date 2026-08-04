use raw_cpuid::CpuId;

use crate::{display::framebuffer, memory, println};

pub fn welcome() {
    println!("Welcome to Aether!\n");

    let cpuid = CpuId::new();

    if let Some(brand) = cpuid.get_processor_brand_string() {
        println!("CPU: {}", brand.as_str());
    } else {
        println!("CPU: Unknown");
    }

    println!("Memory: {}MiB", memory::map::usable());

    let (width, height) = {
        let framebuffer = framebuffer().lock();
        (framebuffer.width(), framebuffer.height())
    };

    println!("Framebuffer: {width}x{height}\n");
}
