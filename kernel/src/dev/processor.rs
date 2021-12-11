use core::fmt::{Display, Formatter};
use crate::dev::Device;


#[derive(Copy, Clone, Debug)]
pub enum Vendor {
    Intel,
    AMD,
    Unknown
}


pub trait ProcessorDevice : Device {
    fn vendor(&self) -> Vendor;
}