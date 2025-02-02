mod memory;
mod registers;
mod run;

pub use memory::Memory;
pub use registers::Registers;
pub use run::*;

pub struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halting: i32,
}
