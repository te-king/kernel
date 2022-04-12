use alloc::boxed::Box;
use alloc::{format, vec};
use alloc::sync::Arc;
use core::alloc::Layout;
use core::fmt::{Debug, Formatter};
use lolid::Uuid;
use spin::RwLock;
use x86_64::structures::idt::InterruptDescriptorTable;
use mem::MemoryModel;
use int::InterruptModel;

pub mod mem;
pub mod int;

// a process can have any number of child processes.
// interactions between layers of the tree are done through syscalls.
// instead of the kernel handling all syscalls, it will first delegate to the processes parent.
// if a parent process has certain interrupts set, those interrupts will be triggered
// when a child makes a syscall. if not, it will be propagated up until it hits
// the kernel.
pub struct ProcessState<MM: MemoryModel, IM: InterruptModel> {
    // the identifier of a process is mostly for debugging purposes.
    id: Uuid,

    // when context switch to, a thread will take read access
    // to the memory and interrupt values. they may internally modify
    // the memory, the locks job is to make sure the memory model (and interrupts)
    // are not changed by the kernel while a thread is using it.
    memory: RwLock<MM>,
    interrupt: RwLock<IM>,
}

impl<MM: MemoryModel, IM: InterruptModel> Debug for ProcessState<MM, IM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ProcessState(id: {})", self.id)
    }
}

impl<MM: MemoryModel, IM: InterruptModel> ProcessState<MM, IM> {
    pub fn new(id: Uuid, memory: MM, interrupt: IM) -> Self {
        Self { id, memory, interrupt }
    }
}


pub struct ThreadState<MM: MemoryModel, IM: InterruptModel> {
    proc: Arc<ProcessState<MM, IM>>,
}


pub enum ThreadContinuation<MM: MemoryModel, IM: InterruptModel> {
    /// The thread has exited.
    Exited {
        exit_code: i32
    },

    /// The thread has yielded control.
    Waiting {
        state: ThreadState<MM, IM>,
        resume: Box<dyn Fn(ThreadState<MM, IM>) -> ThreadContinuation<MM, IM>>,
    },

    /// The thread is waiting for an allocation to be made,
    /// placed within its process state, and passed in.
    WaitingAlloc {
        layout: Layout,
        state: ThreadState<MM, IM>,
        resume: Box<dyn Fn(ThreadState<MM, IM>, *mut ()) -> ThreadContinuation<MM, IM>>,
    },

    /// The thread is waiting for an address to be freed.
    WaitingFree {
        address: *mut (),
        layout: Layout,
        state: ThreadState<MM, IM>,
        resume: Box<dyn Fn(ThreadState<MM, IM>) -> ThreadContinuation<MM, IM>>,
    },
}