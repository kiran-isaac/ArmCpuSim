mod memory;
mod registers;

pub use memory::Memory;
pub use registers::{Registers, ASPR};

pub struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halting: Option<u8>,
}
