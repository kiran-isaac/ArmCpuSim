mod binary;
mod decode;
mod dispatch;
mod execute;
mod model;
mod system;
mod log;

#[cfg(test)]
mod test;

use std::clone;

use binary::is_32_bit;
use decode::*;
use execute::Executor;
use model::Memory;
use model::Registers;
use log::Logger;

struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halt: i32,
}

impl ProcessorState {
    #[cfg(test)]
    pub fn new() -> Self {
        ProcessorState {
            regs: Registers::new(),
            mem: Memory::empty(),
            halt: -1,
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
        halt: -1,
    };

    let _entry_point = state.mem.entrypoint;
    state.regs.pc = _entry_point as u32;
    let mut executor0 = Executor::new();

    let current_utc_time = chrono::Utc::now();

    // create trace dir if it doesn't exist
    std::fs::create_dir_all("traces").unwrap();
    // let mut logger = Logger::new(format!("traces/trace{}.csv", current_utc_time.timestamp_micros() / 1000).as_str(), &state.regs);
    let mut logger = Logger::new("traces/trace.csv", &state.regs);

    loop {
        let instruction = state.mem.get_instruction(state.regs.pc);
        let is_32_bit = is_32_bit(instruction);

        #[cfg(debug_assertions)]
        if state.regs.pc >= 0x40 {
            let _x = 1;
        }

        let decoded = decode(instruction);

        executor0.assign(decoded);
        executor0.execute(&mut state);
        
        state.regs.pc = state.regs.pc.wrapping_add(if is_32_bit {4} else {2});

        logger.log(decoded, &state.regs);

                if state.halt >= 0 {
            println!("Exiting with code: {}", state.halt);
            std::process::exit(state.halt)
        }
    }
}
