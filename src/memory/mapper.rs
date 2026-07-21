use x86_64::{VirtAddr, registers::control::Cr3, structures::paging::OffsetPageTable};

pub fn init(hhdm: VirtAddr) -> OffsetPageTable<'static> {
    let (level_4_frame, _) = Cr3::read();

    let start_address = level_4_frame.start_address();
    let virt = hhdm + start_address.as_u64();

    unsafe {
        let level_4_table = &mut *virt.as_mut_ptr();
        OffsetPageTable::new(level_4_table, hhdm) // Both hhdm & phys offset same
    }
}
