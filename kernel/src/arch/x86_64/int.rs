use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use kernel::logln;
use kernel::proc::int::InterruptModel;


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    logln!("BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, errcode: u64) -> ! {
    panic!("DOUBLE-FAULT\n{:?}\n{:#?}", errcode, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, errcode: PageFaultErrorCode) {
    panic!("PAGE-FAULT\n{:?}\n{:#?}", errcode, stack_frame)
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, errcode: u64) {
    panic!("GENERAL-PROTECTION-FAULT\n{:?}\n{:#?}", errcode, stack_frame);
}


/// Implements an IDT specifically for the kernel on x86_64.
///
/// This IDT is used to handle interrupts and exceptions that occur in the kernel.
/// It does not handle interrupts that occur in user-space.
///
#[derive(Debug)]
pub struct KernelInterruptModel {
    idt: InterruptDescriptorTable,
}

impl KernelInterruptModel {
    pub fn new() -> Self {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        Self { idt }
    }
}

impl InterruptModel for KernelInterruptModel {
    fn load(&'static self) {
        self.idt.load();
    }
}


/// Implements an IDT specifically for user-space on x86_64.
///
/// This IDT is used to handle interrupts and exceptions that occur in user-space.
/// It does not handle interrupts that occur in the kernel.
///
pub struct UserInterruptModel {
    idt: InterruptDescriptorTable,
}

impl UserInterruptModel {
    pub fn new() -> Self {
        let mut idt = InterruptDescriptorTable::new();
        Self { idt }
    }
}

impl InterruptModel for UserInterruptModel {
    fn load(&'static self) {
        self.idt.load();
    }
}