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
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::exit;
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

    let mut log_file = File::create(
        if STALL_ON_BRANCH {
            "traces/log_stall.txt"
        } else if PREDICT == PredictionAlgorithms::AlwaysTaken {
            "traces/log_taken.txt"
        } else {
            "traces/log_un.txt"
        }
    )?;

    let mut cpu = OoOSpeculative::new(
        state.clone(),
        "traces/trace.csv",
        |i: String| {
            log_file.write((i + "\n").as_bytes()).unwrap();
        },
        "traces/stack_dump.txt",
    );

    let mut complete = false;

    loop {
        cpu.tick();
        if let Some(exit_code) = cpu.halt {
            terminal.clear()?;
            terminal.flush()?;
            terminal.set_cursor_position((0, 0))?;
            println!("Program terminated with code {}", exit_code);
            let ipc = (cpu.instructions_committed as f64) / (cpu.epoch as f64);
            println!(
                "Cycles: {}\nInstructions: {}\nIPC: {}",
                cpu.epoch, cpu.instructions_committed, ipc
            );
            return Ok(());
        }
        terminal.draw(|f| cpu.render(f))?;

        if complete {
            continue;
        }

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
                            if cpu.display_focus == 0 {
                                cpu.rob_focus_down();
                            } else if cpu.display_focus == 1 {
                                cpu.mem_bottom_offset += 1;
                            }
                            terminal.draw(|f| cpu.render(f))?;
                        }
                        KeyCode::Down => {
                            if cpu.display_focus == 0 {
                                cpu.rob_focus_up();
                            } else if cpu.display_focus == 1 {
                                if cpu.mem_bottom_offset > 0 {
                                    cpu.mem_bottom_offset -= 1;
                                }
                            }
                            terminal.draw(|f| cpu.render(f))?;
                        }
                        KeyCode::Char(c) => {
                            match c {
                                '1' => cpu.rs_current_display = IssueType::ALUSHIFT,
                                '2' => cpu.rs_current_display = IssueType::MUL,
                                '3' => cpu.rs_current_display = IssueType::LoadStore,
                                '4' => cpu.rs_current_display = IssueType::Control,
                                'r' => {}
                                'l' => cpu.reset(),
                                'c' => {
                                    complete = true;
                                    break;
                                }
                                'f' => {
                                    let new_focus = if cpu.display_focus == 0 { 1 } else { 0 };
                                    cpu.display_focus = new_focus
                                }
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
