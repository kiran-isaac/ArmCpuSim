mod binary;
mod decode;
mod execute;
mod log;
mod model;
mod system;

#[cfg(test)]
mod test;

use std::fs::OpenOptions;
use std::io::Write;

use binary::is_32_bit;
use decode::*;
use execute::Executor;
use log::Tracer;
use model::Memory;
use model::Registers;

struct ProcessorState {
    pub regs: Registers,
    pub mem: Memory,
    pub halt: i32,
}

fn overwrite_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
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

    // let current_utc_time = chrono::Utc::now();

    // create trace dir if it doesn't exist
    std::fs::create_dir_all("traces").unwrap();
    let mut tracer = Tracer::new("traces/trace.csv", &state.regs);
    let logfile = "traces/log.txt";
    let mut log_file = OpenOptions::new()
        .write(true)
        .create(true).truncate(true)
        .open(logfile)
        .unwrap();
    let stack_dump_file = "traces/stack_dump.txt";

    loop {
        let mut logstr = String::new();

        let instruction = state.mem.get_instruction(state.regs.pc);
        let is_32_bit = is_32_bit(instruction);

        if state.regs.pc == 0x2a {
            print!("")
        }

        let decoded = decode(instruction);

        let _old_pc = state.regs.pc;

        // create std::fmt::Formatter to pass to execute
        executor0.assign(decoded);
        executor0.execute(&mut state, &mut logstr);
        log_file.write_all(logstr.as_bytes()).unwrap();

        overwrite_file(
            stack_dump_file,
            state.mem.dump_stack(state.regs.sp).as_str(),
        )
        .unwrap();

        // increment pc
        match decoded.it {
            IT::BL | IT::BLX | IT::B | IT::BX => {}
            _ => state.regs.pc += if is_32_bit { 4 } else { 2 },
        }

        tracer.log(decoded, &state.regs);

        if state.halt >= 0 {
            println!("Exiting with code: {}", state.halt);
            std::process::exit(state.halt)
        }
    }
}
