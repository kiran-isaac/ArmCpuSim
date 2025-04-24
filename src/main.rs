#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
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

extern crate ratatui;

use decode::*;
use model::*;
use ratatui::crossterm::event::{self, Event, KeyCode};
use std::io;
use CPUs::*;

fn main() -> io::Result<()> {
    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("rust-sdl2 demo", 800, 600)
    //     .position_centered()
    //     .build()
    //     .unwrap();
    let mut terminal = ratatui::init();
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
        terminal.draw(|f| cpu.render(f))?;

        // Wait for enter
        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                match event::read()? {
                    Event::Key(key_event) => match key_event.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            std::process::exit(0);
                        }
                        KeyCode::Enter => {
                            break;
                        }
                        KeyCode::Up => {
                            cpu.rob_focus_down();
                            terminal.draw(|f| cpu.render(f))?;
                        }
                        KeyCode::Down => {
                            cpu.rob_focus_up();
                            terminal.draw(|f| cpu.render(f))?;
                        }
                        KeyCode::Char(c) => {
                            match c {
                                '1' => cpu.rs_current_display = IssueType::ALUSHIFT,
                                '2' => cpu.rs_current_display = IssueType::MUL,
                                '3' => cpu.rs_current_display = IssueType::LoadStore,
                                '4' => cpu.rs_current_display = IssueType::Control,
                                'r' => {}
                                _ => continue,
                            }
                            terminal.draw(|f| cpu.render(f))?;
                        }
                        _ => {}
                    },
                    Event::Resize(_, _) => {
                        terminal.draw(|f| cpu.render(f))?;
                    }
                    _ => {}
                }
            }
        }
    }
}
