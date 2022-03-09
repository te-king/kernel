#![feature(custom_test_frameworks)]
#![feature(box_syntax, abi_efiapi, abi_x86_interrupt, alloc_error_handler, asm)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(test_runner)]

#![no_std]
#![no_main]

extern crate alloc;

mod arch;
pub mod dev;
pub mod log;
pub mod proc;
pub mod fs;


// kernel main
pub fn kernel_main() -> usize {
    logln!("KERNEL  {}", env!("CARGO_PKG_NAME"));
    logln!("VERSION {}", env!("CARGO_PKG_VERSION"));

    #[cfg(test)] crate::test_main();

    0
}


// tests
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    tests.iter().for_each(|f| f());
}


#[test_case]
fn test_trivassert() {
    println!("[TRIVIAL_ASSERT]");

    if 1 == 1 {
        println!("[SUCCESS]")
    } else {
        println!("[FAILURE]")
    }
}

#[test_case]
fn test_alloc() {
    use alloc::vec::Vec;

    println!("[ALLOCATION]");

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    if v[0] == 1 {
        println!("[SUCCESS]")
    } else {
        println!("[FAILURE]")
    }

    if v[1] == 2 {
        println!("[SUCCESS]")
    } else {
        println!("[FAILURE]")
    }

    if v[2] == 3 {
        println!("[SUCCESS]")
    } else {
        println!("[FAILURE]")
    }
}
