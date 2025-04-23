mod memory;
mod registers;

pub use memory::Memory;
pub use registers::Registers;

pub struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halting: Option<u8>,
}
