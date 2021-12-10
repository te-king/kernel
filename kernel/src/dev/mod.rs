use alloc::string::String;


pub trait Device {
    fn name(&self) -> String;
}