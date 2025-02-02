mod binary;
mod decode;
mod execute;
mod log;
mod model;
mod system;
#[cfg(test)]
mod test;

use binary::is_32_bit;
use decode::*;
use execute::ExecutorPool;
use log::Tracer;
use model::*;

fn main() {
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();

    let config = RunConfig {
        executors: 1,
        pipelined: false,
    };

    let mut registers = Registers::new();
    let app_path = std::env::args().nth(1).unwrap();

    // Load ELF and initialise register values
    let memory: Memory = Memory::from_elf(&app_path, &mut registers);

    let mut state = ProcessorState {
        regs: registers,
        mem: memory,
        halting: -1,
    };

    state.regs.pc = state.mem.entrypoint as u32;

    let mut runner = Runner::from_config(&config, state);

    loop {
        let (pc, executed_count) = runner.tick();

        if executed_count >= 77 {
            print!("")
        }
    }
}
