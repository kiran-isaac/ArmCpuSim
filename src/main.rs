mod CPUs;
mod binary;
mod components;
mod decode;
mod execute;
mod log;
mod model;
mod system;
#[cfg(test)]
mod test;
use binary::is_32_bit;
use decode::*;
use model::*;
use CPUs::*;

extern crate circular_buffer;

fn main() {
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();
    let mut registers = Registers::new();
    let app_path = std::env::args().nth(1).unwrap();

    // Load ELF and initialise register values
    let memory: Memory = Memory::from_elf(&app_path, &mut registers);

    let mut state = ProcessorState {
        regs: registers,
        mem: memory,
        halting: None,
    };

    state.regs.pc = state.mem.entrypoint as u32;

    let mut cpu = OoOSpeculative::new(
        state,
        "traces/trace.csv",
        "traces/log.txt",
        "traces/stack_dump.txt",
    );

    loop {
        cpu.tick();
    }
}
