///
/// x86_64 uefi panic handler
///
///

use core::panic::PanicInfo;
use x86_64::instructions::hlt;
use crate::logln;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logln!("KERNEL PANICKED AT {}", info);
    loop { hlt() }
}