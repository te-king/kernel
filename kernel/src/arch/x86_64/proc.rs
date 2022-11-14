use lolid::Uuid;
use kernel::proc::int::InterruptModel;
use kernel::proc::mem::MemoryModel;
use kernel::proc::ProcessState;
use crate::arch::int::KernelInterruptModel;
use crate::arch::mem::KernelMemoryModel;

/// Implements a ProcessState for a kernel process.
///
/// This is the ProcessState for the kernel process.
/// It is used to manage the kernel process.
///
pub struct KernelProcessState {
    /// The interrupt model for the kernel process.
    interrupt_model: KernelInterruptModel,
    /// The memory model for the kernel process.
    memory_model: KernelMemoryModel,
}

impl KernelProcessState {
    /// Creates a new KernelProcessState.
    ///
    /// This creates a new KernelProcessState.
    ///
    /// # Arguments
    ///
    /// * `interrupt_model` - The interrupt model for the kernel process.
    /// * `memory_model` - The memory model for the kernel process.
    ///
    pub fn new(interrupt_model: KernelInterruptModel, memory_model: KernelMemoryModel) -> Self {
        Self {
            interrupt_model,
            memory_model,
        }
    }
}

impl ProcessState for KernelProcessState {
    type InterruptModel = KernelInterruptModel;
    type MemoryModel = KernelMemoryModel;

    fn id(&self) -> Uuid {
        /// Kernel is always nil
        Uuid::nil()
    }

    fn memory(&self) -> &Self::MemoryModel {
        todo!()
    }

    fn memory_mut(&mut self) -> &mut Self::MemoryModel {
        todo!()
    }

    fn interrupts(&self) -> &Self::InterruptModel {
        todo!()
    }

    fn interrupts_mut(&mut self) -> &mut Self::InterruptModel {
        todo!()
    }

    fn continue_execution(&'static self) {
        /// as this function can only be called from
        /// the kernel process, all it does is load the
        /// kernel memory model into the CR3 register.
        /// and set the interrupt model to the kernel
        /// interrupt model.
        self.memory_model.load();
        self.interrupt_model.load();
    }
}