use alloc::boxed::Box;
use core::pin::Pin;
use core::task::{Context, Poll};

///
/// The reasons why a process has yielded to the kernel.
/// These are the basis of syscall handling.
pub enum YieldReason {
    /// The process completed its time slice.
    TimeSlice,

    /// The process gave up its time slice
    Cooperative,

    /// The process needs memory to be allocated to continue
    Allocate {
        output: usize,
        length: usize,
    },

    /// The process needs memory to be freed to continue
    Free {
        input: usize,
        length: usize,
    },
}

///
/// A process represents a running program sandboxed from the rest of the system.
/// It is polled to completion like a future.
pub trait Process<'a> {
    fn resume(self: Pin<&mut Self>) -> Poll<YieldReason>;
}