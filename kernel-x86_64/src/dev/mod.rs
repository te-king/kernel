use alloc::string::String;
use kernel::dev::Device;
use kernel::dev::processor::{ProcessorDevice, Vendor};


pub struct Processor {
    vendor: Vendor,
}

impl Device for Processor {
    fn name(&self) -> &str { "Processor" }
}

impl ProcessorDevice for Processor {
    fn vendor(&self) -> Vendor { self.vendor }
}