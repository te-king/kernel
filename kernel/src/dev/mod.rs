use alloc::string::String;


#[derive(Debug)]
pub struct Device {
    pub path: String,
    pub hid: Option<String>,
}