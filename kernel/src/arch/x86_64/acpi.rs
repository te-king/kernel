use core::ptr::{NonNull, slice_from_raw_parts};
use acpi::{AcpiHandler, AcpiTables, HpetInfo, PhysicalMapping};
use uefi::prelude::*;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use alloc::vec;
use alloc::vec::Vec;
use aml::{AmlContext, AmlError, AmlName, DebugVerbosity, Handler, LevelType};
use uefi::table::Runtime;
use crate::dev::Device;


#[derive(Copy, Clone)]
pub struct IdentityMappedAcpiHandler;

impl AcpiHandler for IdentityMappedAcpiHandler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        let addr = NonNull::new_unchecked(physical_address as *mut T);
        PhysicalMapping::new(physical_address, addr, size, size, IdentityMappedAcpiHandler)
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
}

impl Handler for IdentityMappedAcpiHandler {
    fn read_u8(&self, _address: usize) -> u8 {
        todo!()
    }

    fn read_u16(&self, _address: usize) -> u16 {
        todo!()
    }

    fn read_u32(&self, _address: usize) -> u32 {
        todo!()
    }

    fn read_u64(&self, _address: usize) -> u64 {
        todo!()
    }

    fn write_u8(&mut self, _address: usize, _value: u8) {
        todo!()
    }

    fn write_u16(&mut self, _address: usize, _value: u16) {
        todo!()
    }

    fn write_u32(&mut self, _address: usize, _value: u32) {
        todo!()
    }

    fn write_u64(&mut self, _address: usize, _value: u64) {
        todo!()
    }

    fn read_io_u8(&self, _port: u16) -> u8 {
        todo!()
    }

    fn read_io_u16(&self, _port: u16) -> u16 {
        todo!()
    }

    fn read_io_u32(&self, _port: u16) -> u32 {
        todo!()
    }

    fn write_io_u8(&self, _port: u16, _value: u8) {
        todo!()
    }

    fn write_io_u16(&self, _port: u16, _value: u16) {
        todo!()
    }

    fn write_io_u32(&self, _port: u16, _value: u32) {
        todo!()
    }

    fn read_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u8 {
        todo!()
    }

    fn read_pci_u16(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u16 {
        todo!()
    }

    fn read_pci_u32(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16) -> u32 {
        todo!()
    }

    fn write_pci_u8(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u8) {
        todo!()
    }

    fn write_pci_u16(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u16) {
        todo!()
    }

    fn write_pci_u32(&self, _segment: u16, _bus: u8, _device: u8, _function: u8, _offset: u16, _value: u32) {
        todo!()
    }
}


pub fn read_acpi_tables(system_table: &SystemTable<Runtime>) -> Result<AmlContext, AmlError> {
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
            if let Ok(table) = HpetInfo::new(&table) {
                
            }
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


pub fn aml_devices(system_table: &SystemTable<Runtime>) -> Result<Vec<Device>, AmlError> {
    let mut result: Vec<Device> = vec![];

    let mut aml = read_acpi_tables(system_table)?;

    aml.namespace
        .clone()
        .traverse(|a, b| {
            match b.typ {
                LevelType::Device => {
                    let path = a.as_string();

                    let hid = aml.namespace
                        .search(&AmlName::from_str("_HID").unwrap(), a)
                        .and_then(|(a, b)| aml.namespace.get(b))
                        .and_then(|a| a.as_string(&aml))
                        .ok();

                    result.push(Device { path, hid });
                }
                _ => {}
            };

            Ok(true)
        });

    Ok(result)
}