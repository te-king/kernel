use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::pin::Pin;
use x86_64::PhysAddr;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;

use kernel::proc::mem::MemoryModel;
use kernel::proc::ProcessState;

/// Implements an identity mapped memory model using a PhysAddr.
///
/// This is the MemoryModel for the kernel process.
/// It is used to manage the kernel process.
///
pub struct KernelMemoryModel {
    /// The memory model for the kernel process.
    addr: Cow<'static, PageTable>,
}

impl KernelMemoryModel {
    /// Creates a new KernelMemoryModel.
    ///
    /// This creates a new KernelMemoryModel.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the page table.
    ///
    pub fn new(page_table: PageTable) -> Self {
        Self {
            addr: Cow::Owned(page_table),
        }
    }

    /// Creates a new KernelMemoryModel.
    ///
    /// This creates a new KernelMemoryModel.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the page table.
    ///
    pub fn from(addr: &'static PageTable) -> Self {
        Self {
            addr: Cow::Borrowed(addr),
        }
    }

    /// Creates a new KernelMemoryModel from by reading the CR3 register.
    ///
    /// This creates a new KernelMemoryModel from by reading the CR3 register.
    ///
    /// # Unsafe
    /// This function is unsafe because it reads the CR3 register.
    /// There is no guarantee that the CR3 register refers to an identity mapped page table.
    pub unsafe fn from_cr3() -> Self {
        let (frame, _) = Cr3::read();
        let addr = frame.start_address().as_u64() as *const PageTable;
        Self::from(&*addr)
    }
}

impl MemoryModel for KernelMemoryModel {
    /// The memory model must be pinned while it is loaded.
    fn load(&'static self) {
        todo!("Load the kernel memory model into the CR3 register.")
    }
}


/// Implements a MemoryModel for a user-space process.
///
/// This is the MemoryModel for a user-space process.
/// It is used to manage a user-space process.
///
pub struct UserMemoryModel {
    /// The memory model for the kernel process.
    page_table: PageTable,
}

impl UserMemoryModel {
    pub fn new(page_table: PageTable) -> Self {
        Self { page_table }
    }
}

impl MemoryModel for UserMemoryModel {
    fn load(&'static self) {
        todo!("Load the user memory model into the CR3 register.")
    }
}