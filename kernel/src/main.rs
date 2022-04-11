#![feature(custom_test_frameworks)]
#![feature(box_syntax)]
#![feature(abi_efiapi)]
#![feature(abi_x86_interrupt, )]
#![feature(alloc_error_handler)]
#![feature(asm)]

#![reexport_test_harness_main = "test_main"]
#![test_runner(test_runner)]

#![no_std]
#![no_main]

extern crate alloc;

mod arch;

use alloc::boxed::Box;
use alloc::vec;
use x86_64::instructions::interrupts::int3;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use kernel::logln;
use kernel::dev::Device;
use kernel::fs::{File, FileSystem};
use kernel::proc::ThreadState;


// kernel main
pub fn kernel_main() -> usize {
    logln!("KERNEL  {}", env!("CARGO_PKG_NAME"));
    logln!("VERSION {}", env!("CARGO_PKG_VERSION"));

    #[cfg(test)] crate::test_main();

    0
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    logln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}


// tests
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    tests.iter().for_each(|f| f());
}


#[test_case]
fn trivial_assert() {
    println!("[TRIVIAL_ASSERT]");

    if 1 == 1 {
        println!("[SUCCESS]")
    } else {
        println!("[FAILURE]")
    }
}