/// An abstraction of memory management.
///
/// This trait is used to abstract the memory management of an architecture.
pub trait InterruptModel {
    /// Loads the interrupt model.
    fn load(&'static self);
}