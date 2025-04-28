mod memory;
mod registers;

pub use memory::Memory;
pub use registers::{ASPRUpdate, Registers};

#[derive(Clone)]
pub struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halting: Option<u8>,
}
