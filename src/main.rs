mod decode;
mod dispatch;
mod execute;
mod instructions;
mod model;

#[cfg(test)]
mod test;

use instructions::is_32_bit;
use model::Memory;
use model::Registers;
use decode::*;

struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
}

fn main() {
    let os_path = std::env::args().nth(1).unwrap();
    let mut memory: Memory = Memory::from_os_elf(&os_path);
    let app_path = std::env::args().nth(2).unwrap();
    memory.load_additional_elf(&app_path);

    let registers = Registers::new();
    let mut state = ProcessorState {
        regs: registers,
        mem: memory,
    };

    // Fetch is modeled by reading the instruction at the PC
    // decode is modeled by matching the instruction against a set of masks
    // execute is modeled by performing the operation on the registers

    let _entry_point = state.mem.os_entrypoint;
    state.regs.pc = _entry_point as u32;

    loop {
        let instruction = state.mem.get_instruction(state.regs.pc);
        let is_32_bit = is_32_bit(instruction);
        let decoded = decode(instruction);
        println!("{:04X?}:{:08X?}:{:?}", state.regs.pc, instruction, decoded.it);
        state.regs.pc += if is_32_bit { 4 } else { 2 };
    }

}
