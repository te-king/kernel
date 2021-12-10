use core::fmt::Write;
use alloc::boxed::Box;
use spin::Mutex;


pub static STDOUT: Mutex<Option<Box<dyn Write + Send>>> = Mutex::new(None);


#[doc(hidden)]
pub fn _log(args: core::fmt::Arguments) {
    match &mut *STDOUT.lock() {
        None => {}
        Some(writer) => { writer.write_fmt(args); }
    }
}


#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		$crate::log::_log(format_args!($($arg)*));
	};
}

#[macro_export]
macro_rules! logln {
	() => ($crate::log!("\n"));
	($fmt:expr) => ($crate::log!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::log!(concat!($fmt, "\n"), $($arg)*));
}