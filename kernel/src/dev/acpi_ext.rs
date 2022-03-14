use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::{format, vec};
use alloc::boxed::Box;
use core::fmt::{Debug, Formatter};
use core::ptr::NonNull;
use ::acpi::{HpetInfo, InterruptModel, PciConfigRegions, PlatformInfo};
use ::acpi::mcfg::McfgEntry;
use ::acpi::platform::{PmTimer, Processor};
use acpi::{AcpiHandler, PhysicalMapping};
use aml::Handler;
use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};
use crate::dev::Device;


#[derive(Copy, Clone)]
pub struct KernelHandler;

impl AcpiHandler for KernelHandler {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        PhysicalMapping::new(
            physical_address,
            NonNull::new_unchecked(physical_address as *mut T),
            size,
            size,
            KernelHandler,
        )
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {}
}

impl Handler for KernelHandler {
    fn read_u8(&self, _address: usize) -> u8 { unsafe { *(_address as *mut _) } }
    fn read_u16(&self, _address: usize) -> u16 { unsafe { *(_address as *mut _) } }
    fn read_u32(&self, _address: usize) -> u32 { unsafe { *(_address as *mut _) } }
    fn read_u64(&self, _address: usize) -> u64 { unsafe { *(_address as *mut _) } }

    fn write_u8(&mut self, _address: usize, _value: u8) { unsafe { *(_address as *mut _) = _value } }
    fn write_u16(&mut self, _address: usize, _value: u16) { unsafe { *(_address as *mut _) = _value } }
    fn write_u32(&mut self, _address: usize, _value: u32) { unsafe { *(_address as *mut _) = _value } }
    fn write_u64(&mut self, _address: usize, _value: u64) { unsafe { *(_address as *mut _) = _value } }

    fn read_io_u8(&self, _port: u16) -> u8 { unsafe { PortReadOnly::new(_port).read() } }
    fn read_io_u16(&self, _port: u16) -> u16 { unsafe { PortReadOnly::new(_port).read() } }
    fn read_io_u32(&self, _port: u16) -> u32 { unsafe { PortReadOnly::new(_port).read() } }

    fn write_io_u8(&self, _port: u16, _value: u8) { unsafe { PortWriteOnly::new(_port).write(_value) } }
    fn write_io_u16(&self, _port: u16, _value: u16) { unsafe { PortWriteOnly::new(_port).write(_value) } }
    fn write_io_u32(&self, _port: u16, _value: u32) { unsafe { PortWriteOnly::new(_port).write(_value) } }

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


///
/// acpi HpetInfo
impl Device for HpetInfo {
    fn name(&self) -> String {
        "HPET".to_owned()
    }
}

///
/// acpi InterruptModel
impl Device for InterruptModel {
    fn name(&self) -> String {
        match &self {
            InterruptModel::Apic(_) => "APIC Interrupt Model".to_owned(),
            _ => "Unknown Interrupt Model".to_owned(),
        }
    }
}

///
/// acpi PmTimer
impl Device for PmTimer {
    fn name(&self) -> String {
        if self.supports_32bit {
            "32bit PM Timer".to_owned()
        } else {
            "PM Timer".to_owned()
        }
    }
}

///
/// acpi Processor
impl Device for Processor {
    fn name(&self) -> String {
        format!("Processor {}", self.processor_uid)
    }
}

///
/// acpi PciConfigRegions
impl Device for PciConfigRegions {
    fn name(&self) -> String {
        "PCI Subsystem".to_owned()
    }
}

///
/// acpi PlatformInfo
impl Device for PlatformInfo {
    fn name(&self) -> String {
        "Platform".to_owned()
    }

    fn components(&self) -> Vec<&dyn Device> {
        let mut result: Vec<&dyn Device> = vec![];

        if let Some(proc) = &self.processor_info {
            result.push(&proc.boot_processor);

            for app_processor in &proc.application_processors {
                result.push(app_processor);
            }
        }

        result.push(&self.interrupt_model);

        if let Some(pm) = &self.pm_timer {
            result.push(pm);
        }

        result
    }
}