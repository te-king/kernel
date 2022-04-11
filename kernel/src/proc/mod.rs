use alloc::boxed::Box;
use alloc::{format, vec};
use alloc::sync::Arc;
use core::alloc::Layout;
use core::fmt::{Debug, Formatter};
use lolid::Uuid;
use x86_64::structures::idt::InterruptDescriptorTable;
use mem::MemoryMap;
use int::InterruptMap;

mod mem;
mod int;


pub struct ThreadState {
    int: InterruptDescriptorTable,
    mem: Box<[u8]>,
}

impl ThreadState {
    pub fn new() -> Self {
        Self {
            int: InterruptDescriptorTable::new(),
            mem: vec![0u8; 4096].into_boxed_slice(),
        }
    }
}


pub enum ThreadContinuation {
    /// The thread has exited.
    Exited {
        exit_code: i32
    },

    /// The thread has yielded control.
    Waiting {
        state: ThreadState,
        resume: &'static dyn Fn(ThreadState) -> ThreadContinuation,
    },

    /// The thread is waiting for an allocation to be made,
    /// placed within its process state, and passed in.
    WaitingAlloc {
        layout: Layout,
        state: ThreadState,
        resume: &'static dyn Fn(ThreadState, *mut ()) -> ThreadContinuation,
    },

    /// The thread is waiting for an address to be freed.
    WaitingFree {
        address: *mut (),
        layout: Layout,
        state: ThreadState,
        resume: &'static dyn Fn(ThreadState) -> ThreadContinuation,
    },
}

fn scratch() {
    todo!()


    // -- create process

    // set thread idt
    // set page table
    // set tss
    // iret to process entry in ring3
    // (wait for return)
    // figure out what caused an interrupt,
    // create a continuation
    // enqueue continuation in whatever task pool is needed

    // repeat until continuation is "Exited"
}