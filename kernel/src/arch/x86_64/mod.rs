use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::slice_from_raw_parts_mut;
use uart_16550::SerialPort;
use uefi::table::boot::MemoryType;
use uefi::prelude::*;
use uefi::table::runtime::TimeCapabilities;
use crate::log::install_logger;
use crate::{Device, kernel_main, logln};
use crate::dev::read_acpi_tables;

mod allocator;
mod interrupt;
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
        .allocate_pool(MemoryType::LOADER_DATA, buf_size.map_size + buf_size.entry_size)
        .expect_success("failed to allocate memory map storage");

    let buf = unsafe {
        &mut *slice_from_raw_parts_mut(buf_store, buf_size.map_size + buf_size.entry_size)
    };

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
        install_logger(box SerialPort::new(0x03f8));
    }

    // read aml
    // we dont know where the serial port is yet so we get no stdout
    let devices = read_acpi_tables(&system_table);

    for device in devices {
        logln!("{:?}", device.name());
        for component in device.components() {
            logln!("\t{:?}", component.name());
        }
    }

    // register now unused acpi memory (as it is no longer needed)
    for descriptor in descriptors.clone() {
        match descriptor.ty {
            MemoryType::ACPI_RECLAIM => unsafe { allocator::register_descriptor(*descriptor) },
            _ => {}
        }
    }

    kernel_main();
    panic!("execution endpoint")
}