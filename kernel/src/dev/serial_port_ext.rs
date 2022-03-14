use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use acpi::{AcpiHandler, AcpiTables};
use uart_16550::SerialPort;
use crate::dev::Device;


trait SerialPortExt<H>
    where
        H: AcpiHandler,
{
    fn from_tables(tables: &AcpiTables<H>) -> Vec<SerialPort>;
}

impl<H> SerialPortExt<H> for SerialPort
    where
        H: AcpiHandler,
{
    fn from_tables(tables: &AcpiTables<H>) -> Vec<Self> {
        vec![]
    }
}

impl Device for SerialPort {
    fn name(&self) -> String {
        "Serial Port".to_owned()
    }
}