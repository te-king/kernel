///
/// Represents a generic table that stores predefined actions
/// for handling events.
/// An example implementation is the x86_64 IDT
///
pub trait EventRegister {}


///
/// Represents a generic mapping from kernel address space
/// to process address space.
/// An example implementation is the x86_64 PageTable.
///
pub trait VirtualMemory {}


///
/// Represents a process running on the machine.
///
pub struct Process<ER: EventRegister, VM: VirtualMemory> {
    id: u64,
    event_register: ER,
    virtual_memory: VM,
}