use x86_64::PhysAddr;
use x86_64::structures::paging::PageTable;
use kernel::proc::mem::MemoryModel;

pub struct IdentityMappedPageTable(PhysAddr);

impl IdentityMappedPageTable {
    pub fn new(phys: PhysAddr) -> Self {
        Self(phys)
    }
}

impl MemoryModel for IdentityMappedPageTable {}