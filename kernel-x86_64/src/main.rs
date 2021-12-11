#![feature(box_syntax, abi_efiapi, abi_x86_interrupt, alloc_error_handler, asm)]

#![no_main]
#![no_std]

extern crate alloc;

use core::ptr::slice_from_raw_parts_mut;
use aml::{AmlContext, AmlError};
use uart_16550::SerialPort;
use uefi::table::boot::{MemoryDescriptor, MemoryType};
use uefi::prelude::*;
use kernel::log::STDOUT;
use kernel::logln;
use kernel::proc::{EventRegister, Process, VirtualMemory};

mod acpi;
mod allocator;
mod devices;
mod interrupts;
mod memory;
mod panic;


#[entry]
fn x86_64_entrypoint(handle: Handle, system_table: SystemTable<Boot>) -> Status {
    // allocate buf for memory map
    let buf_size = system_table
        .boot_services()
        .memory_map_size();

    let buf_store = system_table
        .boot_services()
        .allocate_pool(MemoryType::LOADER_DATA, buf_size + 8 * core::mem::size_of::<MemoryDescriptor>())
        .expect_success("failed to allocate memory map storage");

    let buf = unsafe { &mut *slice_from_raw_parts_mut(buf_store, buf_size) };

    // retrieve memory map, exit boot services, and add to allocator
    let (system_table, descriptors) = system_table
        .exit_boot_services(handle, buf)
        .expect_success("failed to exit boot services");


    // register conventional memory
    for descriptor in descriptors.clone() {
        match descriptor.ty {
            MemoryType::CONVENTIONAL => unsafe { allocator::register_descriptor(*descriptor) },
            _ => {}
        }
    }

    // create static serial port.
    unsafe {
        // it looks like 0x03fa is defined in the ISA DSDT for QEMU.
        // see https://github.com/pebble/qemu/blob/master/hw/i386/acpi-dsdt-isa.dsl
        // this is looks like a standard.
        *STDOUT.lock() = Some(box SerialPort::new(0x03f8));
    }

    // dump device tree from aml
    let aml = acpi::debug_acpi_aml(&system_table);

    match aml {
        Ok(aml) => logln!("{:?}", aml.namespace),
        Err(aml_err) => logln!("AML Error: {:?}", aml_err)
    }

    // register now unused acpi memory (as it is no longer needed)
    for descriptor in descriptors.clone() {
        match descriptor.ty {
            MemoryType::ACPI_RECLAIM => unsafe { allocator::register_descriptor(*descriptor) },
            _ => {}
        }
    }

    kernel::kernel_main();
    panic!("execution endpoint")
}