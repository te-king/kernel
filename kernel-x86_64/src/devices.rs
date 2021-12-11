use alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::String;
use acpi::InterruptModel;
use acpi::platform::interrupt::Apic;
use acpi::platform::ProcessorInfo;
use libertyos_kernel::dev::Device;


pub struct SerialPort(uart_16550::SerialPort);

impl Device for SerialPort {
    fn name(&self) -> String {
        "Serial Port".to_owned()
    }
}

impl SerialPort {
    pub unsafe fn from_base(base: u16) -> Self {
        SerialPort(uart_16550::SerialPort::new(base))
    }
}


pub struct InterruptController(InterruptModel);

impl Device for InterruptController {
    fn name(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl InterruptController {
    pub fn new(model: InterruptModel) -> Self {
        InterruptController(model)
    }
}


pub struct Processor(acpi::platform::Processor);

impl Device for Processor {
    fn name(&self) -> String {
        format!("{:?}", self.0)
    }
}

impl Processor {
    pub fn new(info: acpi::platform::Processor) -> Self {
        Processor(info)
    }
}
