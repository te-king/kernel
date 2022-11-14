use core::pin::Pin;

/// An abstraction of memory management.
///
/// This trait is used to abstract the memory management of an architecture.
///
/// The memory model is responsible for allocating and freeing memory.
/// It is also responsible for mapping and unmapping memory.
///
pub trait MemoryModel {
    /// Loads the memory model into the CR3 register.
    /// This is used to switch to a new memory model.
    fn load(&'static self);
}