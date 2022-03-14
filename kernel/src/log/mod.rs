use core::fmt::Write;
use alloc::boxed::Box;
use spin::Mutex;


static LOG_OUT: Mutex<Option<Box<dyn Write + Send>>> = Mutex::new(None);


#[doc(hidden)]
pub fn _log(args: core::fmt::Arguments) {
    match &mut *LOG_OUT.lock() {
        None => {}
        Some(writer) => { writer.write_fmt(args); }
    }
}

pub fn install_logger(writer: Box<dyn Write + Send>) {
	*LOG_OUT.lock() = Some(writer);
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