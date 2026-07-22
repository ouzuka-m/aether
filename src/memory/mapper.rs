use x86_64::{registers::control::Cr3, structures::paging::OffsetPageTable};

use crate::address::{ext::PhysExt, hhdm::HHDM};

pub fn init() -> OffsetPageTable<'static> {
    let (level_4_frame, _) = Cr3::read();

    let phys = level_4_frame.start_address();
    let virt = phys.to_virt();

    unsafe {
        let level_4_table = &mut *virt.as_mut_ptr();

        OffsetPageTable::new(level_4_table, *HHDM) // Both hhdm & phys offset same
    }
}
