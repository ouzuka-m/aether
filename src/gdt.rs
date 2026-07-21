use lazy_static::lazy_static;
use x86_64::{
    instructions::tables,
    registers::segmentation::{CS, Segment},
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
};

use crate::tss::TASK_STATE_SEGMENT;

struct Selectors {
    tss: SegmentSelector,
    code: SegmentSelector,
}

lazy_static! {
    static ref PAIR: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();

        let tss = gdt.append(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
        let code = gdt.append(Descriptor::kernel_code_segment());

        let selectors = Selectors { tss, code };

        (gdt, selectors)
    };
}

pub fn init() {
    let (gdt, selectors) = &*PAIR;

    gdt.load();
    unsafe {
        CS::set_reg(selectors.code);
        tables::load_tss(selectors.tss);
    }
}
