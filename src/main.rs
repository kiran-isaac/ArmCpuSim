mod decode;
mod dispatch;
mod execute;
mod instructions;
mod model;

#[cfg(test)]
mod test;

use model::Memory;
use model::Registers;

struct ProcessorState<'a> {
    pub regs: Registers,
    pub mem: Memory<'a>,
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let memory: Memory<'static> = Memory::load_elf(&path);

    let registers = Registers::new();
    let mut state = ProcessorState {
        regs: registers,
        mem: memory,
    };

    // Fetch is modeled by reading the instruction at the PC
    // decode is modeled by matching the instruction against a set of masks
    // execute is modeled by performing the operation on the registers

    let _text_start = state.mem.get_text_start();
    let _entry_point = state.mem.get_entry_point();

    println!("{:02X?}", state.mem.get_elf_text());
    state.mem.set_pc_to_program_start(&mut state.regs);
    let instruction = state.mem.get_instruction(state.regs.pc);
    println!("{}:{:08X?}", state.regs.pc, instruction);
}
