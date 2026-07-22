use x86_64::{PhysAddr, VirtAddr};

use crate::address::hhdm::HHDM;

pub trait PhysExt {
    fn to_virt(self) -> VirtAddr;
}

pub trait VirtExt {
    fn to_phys(self) -> PhysAddr;

    fn offset(&self, offset: u64) -> VirtAddr;
}

impl PhysExt for PhysAddr {
    fn to_virt(self) -> VirtAddr {
        VirtAddr::new((*HHDM).as_u64() + self.as_u64())
    }
}

impl VirtExt for VirtAddr {
    fn to_phys(self) -> PhysAddr {
        PhysAddr::new(self.as_u64() - (*HHDM).as_u64())
    }

    fn offset(&self, offset: u64) -> VirtAddr {
        VirtAddr::new(self.as_u64() + offset)
    }
}
