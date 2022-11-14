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


pub trait ProcessState {
    type InterruptModel: InterruptModel;
    type MemoryModel: MemoryModel;

    fn id(&self) -> Uuid;
    fn memory(&self) -> &Self::MemoryModel;
    fn memory_mut(&mut self) -> &mut Self::MemoryModel;
    fn interrupts(&self) -> &Self::InterruptModel;
    fn interrupts_mut(&mut self) -> &mut Self::InterruptModel;

    fn continue_execution(&'static self);
}