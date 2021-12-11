use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use kernel::logln;
use kernel::proc::EventRegister;


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    logln!("BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, errcode: u64) -> ! {
    panic!("DOUBLE-FAULT\n{:?}\n{:#?}", errcode, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, errcode: PageFaultErrorCode) {
    panic!("PAGE-FAULT\n{:?}\n{:#?}", errcode, stack_frame)
}


// impl EventRegister for InterruptDescriptorTable {
//
// }