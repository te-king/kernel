use core::fmt::Write;
use alloc::boxed::Box;
use spin::Mutex;




/// The current output writer for the kernel log.
///
/// This is a `Mutex` because it is possible for multiple threads to log at the same time.
///
/// This is an `Option` because it is possible for the kernel to log before the output writer is set.
///
/// This is a `Box` because it is possible for the output writer to be dynamically allocated.
static LOG_OUT: Mutex<Option<Box<dyn Write + Send>>> = Mutex::new(None);



/// atomically sets the log output writer.
///
pub fn set_log_out(writer: Box<dyn Write + Send>) {
	*LOG_OUT.lock() = Some(writer);
}


#[doc(hidden)]
pub fn _log(args: core::fmt::Arguments) {
	if let Some(writer) = &mut *LOG_OUT.lock() {
		writer
			.write_fmt(args)
			.unwrap();
	}
}


/// Logs a message to the kernel log.
#[macro_export]
macro_rules! log {
	($($arg:tt)*) => {
		$crate::log::_log(format_args!($($arg)*));
	};
}

/// Logs a message to the kernel log, followed by a newline.
#[macro_export]
macro_rules! logln {
	() => ($crate::log!("\n"));
	($fmt:expr) => ($crate::log!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::log!(concat!($fmt, "\n"), $($arg)*));
}


/// Logs a message to the kernel log with a `WARN` prefix.
#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => {
		$crate::log!("WARN: {}\n", format_args!($($arg)*));
	};
}

/// Logs a message to the kernel log with a `WARN` prefix, followed by a newline.
#[macro_export]
macro_rules! warnln {
	() => ($crate::log_warn!("\n"));
	($fmt:expr) => ($crate::log_warn!(concat!($fmt, "\n")));
	($fmt:expr, $($arg:tt)*) => ($crate::log_warn!(concat!($fmt, "\n"), $($arg)*));
}