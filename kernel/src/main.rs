#![feature(box_syntax)]
#![feature(abi_efiapi)]
#![feature(abi_x86_interrupt, )]
#![feature(alloc_error_handler)]

#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use core::fmt::Debug;

use x86_64::instructions::interrupts::int3;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use kernel::logln;
use kernel::proc::int::InterruptModel;
use kernel::proc::mem::MemoryModel;
use kernel::proc::ProcessState;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
pub mod arch;

#[cfg(target_arch = "x86_64")]
#[path = "arch/x86_64/mod.rs"]
pub mod arch;

// kernel main
pub fn kernel_main(proc: &impl ProcessState) -> usize {
    logln!("KERNEL  {}", env!("CARGO_PKG_NAME"));
    logln!("VERSION {}", env!("CARGO_PKG_VERSION"));
    0
}