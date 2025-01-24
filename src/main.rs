mod binary;
mod decode;
mod dispatch;
mod execute;
mod model;
mod system;

#[cfg(test)]
mod test;

use binary::is_32_bit;
use decode::*;
use execute::Executor;
use model::Memory;
use model::Registers;

struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halt: bool,
}

impl ProcessorState {
    pub fn new() -> Self {
        ProcessorState {
            regs: Registers::new(),
            mem: Memory::empty(),
            halt: false,
        }
    }
}

fn main() {
    let mut registers = Registers::new();
    let app_path = std::env::args().nth(1).unwrap();

    // Load ELF and initialise register values
    let memory: Memory = Memory::from_elf(&app_path, &mut registers);

    let mut state = ProcessorState {
        regs: registers,
        mem: memory,
        halt: false,
    };

    // Fetch is modeled by reading the instruction at the PC
    // decode is modeled by matching the instruction against a set of masks
    // execute is modeled by performing the operation on the registers

    // let _entry_point = state.mem.entrypoint;
    // state.regs.pc = _entry_point as u32;
    let mut executor0 = Executor::new();

    loop {
        let instruction = state.mem.get_instruction(state.regs.pc);
        let is_32_bit = is_32_bit(instruction);

        #[cfg(debug_assertions)]
        if state.regs.pc >= 0x40 {
            let _x = 1;
        }

        let decoded = decode(instruction);
        println!(
            "0x{:04X?} : 0x{:08X?} : {:?}",
            state.regs.pc, instruction, decoded.it
        );
        state.regs.pc += if is_32_bit { 4 } else { 2 };
        // executor0.assign(decoded);
        // executor0.execute(&mut state);
    }
}
