use alloc::string::String;


#[derive(Debug)]
pub struct PlugAndPlayDevice {
    pub path: String,
    pub hid: Option<String>,
}