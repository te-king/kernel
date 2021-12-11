use alloc::boxed::Box;
use core::ptr::{NonNull, slice_from_raw_parts};
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use uefi::prelude::*;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use alloc::vec;
use alloc::vec::Vec;
use aml::{AmlContext, AmlError, DebugVerbosity, Handler};
use uefi::table::Runtime;
use kernel::dev::Device;
use kernel::logln;
use crate::devices::{InterruptController, Processor, SerialPort};


#[derive(Copy, Clone)]
pub struct IdentityMappedAcpiHandler;

impl AcpiHandler for IdentityMappedAcpiHandler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        let addr = NonNull::new_unchecked(physical_address as *mut T);
        PhysicalMapping::new(physical_address, addr, size, size, IdentityMappedAcpiHandler)
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {}
}

impl Handler for IdentityMappedAcpiHandler {
    fn read_u8(&self, address: usize) -> u8 {
        todo!()
    }

    fn read_u16(&self, address: usize) -> u16 {
        todo!()
    }

    fn read_u32(&self, address: usize) -> u32 {
        todo!()
    }

    fn read_u64(&self, address: usize) -> u64 {
        todo!()
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        todo!()
    }

    fn write_u16(&mut self, address: usize, value: u16) {
        todo!()
    }

    fn write_u32(&mut self, address: usize, value: u32) {
        todo!()
    }

    fn write_u64(&mut self, address: usize, value: u64) {
        todo!()
    }

    fn read_io_u8(&self, port: u16) -> u8 {
        todo!()
    }

    fn read_io_u16(&self, port: u16) -> u16 {
        todo!()
    }

    fn read_io_u32(&self, port: u16) -> u32 {
        todo!()
    }

    fn write_io_u8(&self, port: u16, value: u8) {
        todo!()
    }

    fn write_io_u16(&self, port: u16, value: u16) {
        todo!()
    }

    fn write_io_u32(&self, port: u16, value: u32) {
        todo!()
    }

    fn read_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u8 {
        todo!()
    }

    fn read_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u16 {
        todo!()
    }

    fn read_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16) -> u32 {
        todo!()
    }

    fn write_pci_u8(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u8) {
        todo!()
    }

    fn write_pci_u16(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u16) {
        todo!()
    }

    fn write_pci_u32(&self, segment: u16, bus: u8, device: u8, function: u8, offset: u16, value: u32) {
        todo!()
    }
}


pub fn debug_acpi_aml(system_table: &SystemTable<Runtime>) -> Result<AmlContext, AmlError> {

    // create aml parser context
    let mut context = AmlContext::new(
        box IdentityMappedAcpiHandler,
        DebugVerbosity::All,
    );

    // load acpi tables
    let acpi_v2 = system_table
        .config_table()
        .iter()
        .find(|c| c.guid == ACPI2_GUID);

    let acpi_v1 = system_table
        .config_table()
        .iter()
        .find(|c| c.guid == ACPI_GUID);

    let table = unsafe {
        acpi_v2
            .or(acpi_v1)
            .and_then(|acpi| AcpiTables::from_rsdp(IdentityMappedAcpiHandler, acpi.address as usize).ok())
    };

    unsafe {
        if let Some(table) = table {
            if let Some(dsdt) = table.dsdt {
                let stream = slice_from_raw_parts(dsdt.address as *const u8, dsdt.length as usize);
                context.parse_table(&*stream)?;
            };
            for ssdt in table.ssdts {
                let stream = slice_from_raw_parts(ssdt.address as *const u8, ssdt.length as usize);
                context.parse_table(&*stream)?;
            }
        }
    }

    Ok(context)
}