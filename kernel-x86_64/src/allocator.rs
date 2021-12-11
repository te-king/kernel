use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use uefi::table::boot::{MemoryDescriptor, MemoryType};
use buddy_system_allocator::LockedHeap;


#[global_allocator]
static ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::new();

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}


pub unsafe fn register_descriptor(descriptor: MemoryDescriptor) {
    ALLOCATOR
        .lock()
        .add_to_heap(
            descriptor.phys_start as usize,
            descriptor.phys_start as usize + descriptor.page_count as usize * 4096,
        )
}

pub unsafe fn register_descriptor_safe(descriptor: MemoryDescriptor) {
    let slc = &mut *slice_from_raw_parts_mut(
        descriptor.phys_start as usize as *mut u8,
        descriptor.page_count as usize * 4096,
    );

    slc.fill(0u8);

    register_descriptor(descriptor);
}