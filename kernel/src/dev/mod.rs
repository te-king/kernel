use alloc::string::String;


pub mod processor;


pub trait Device {
    fn name(&self) -> &str;
}