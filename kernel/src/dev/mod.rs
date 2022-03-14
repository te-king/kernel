use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::vec;
use ::acpi::{AcpiTables, HpetInfo, PciConfigRegions, PlatformInfo};
use uefi::prelude::SystemTable;
use uefi::table::cfg::{ACPI2_GUID, ACPI_GUID};
use uefi::table::Runtime;
use crate::dev::acpi_ext::KernelHandler;

mod acpi_ext;
mod serial_port_ext;


pub trait Device {
    fn name(&self) -> String;

    fn components(&self) -> Vec<&dyn Device> {
        vec![]
    }
}


pub fn read_acpi_tables(system_table: &SystemTable<Runtime>) -> Vec<Box<dyn Device>> {

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
            .and_then(|acpi| AcpiTables::from_rsdp(KernelHandler, acpi.address as usize).ok())
    };


    let mut result: Vec<Box<dyn Device>> = vec![];

    if let Some(table) = table {
        if let Ok(platform) = PlatformInfo::new(&table) {
            result.push(Box::new(platform))
        }
        if let Ok(hpet) = HpetInfo::new(&table) {
            result.push(Box::new(hpet))
        }
        if let Ok(pci) = PciConfigRegions::new(&table) {
            result.push(Box::new(pci))
        }
    }

    result
}