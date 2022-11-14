use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::slice_from_raw_parts_mut;

use lolid::Uuid;
use uart_16550::SerialPort;
use uefi::prelude::*;
use uefi::table::boot::MemoryType;
use uefi::table::runtime::TimeCapabilities;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use kernel::log::set_log_out;
use kernel::proc::ProcessState;

use crate::{kernel_main, logln};

mod allocator;
mod int;
mod mem;
mod panic;
mod proc;


#[entry]
fn x86_64_entrypoint(handle: Handle, system_table: SystemTable<Boot>) -> Status {

    // allocate buf for memory map
    let buf_size = system_table
        .boot_services()
        .memory_map_size();

    let buf_store = system_table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, buf_size.map_size + buf_size.entry_size)
        .expect("failed to allocate memory map storage");

    let buf = unsafe {
        &mut *slice_from_raw_parts_mut(buf_store, buf_size.map_size + buf_size.entry_size)
    };

    // retrieve memory map, exit boot services, and add to allocator
    let (_, descriptors) = system_table
        .exit_boot_services(handle, buf)
        .expect("failed to exit boot services");

    // register conventional memory
    for descriptor in descriptors.clone() {
        match descriptor.ty {
            MemoryType::CONVENTIONAL => unsafe { allocator::register_descriptor(*descriptor) },
            _ => {}
        }
    }

    unsafe {
        let logger = box SerialPort::new(0x03f8);
        set_log_out(logger);
    }

    // register now unused acpi memory (as it is no longer needed)
    for descriptor in descriptors.clone() {
        match descriptor.ty {
            MemoryType::ACPI_RECLAIM => unsafe { allocator::register_descriptor(*descriptor) },
            _ => {}
        }
    }

    unsafe {
        let idt = int::KernelInterruptModel::new();
        let pt = mem::KernelMemoryModel::from_cr3();

        let proc = box proc::KernelProcessState::new(idt, pt);
        // proc.continue_execution();

        kernel_main(&*proc);
    }

    panic!("execution endpoint")
}