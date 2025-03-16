use std::{
    fmt::Write,
    fs::{File, OpenOptions},
};

use crate::{binary::*, log::Tracer, system::syscall, ProcessorState, I, IT::*};

mod ALU;
mod execute_instruction;
mod executor_pool;
mod shift;

#[derive(Clone, Copy)]
pub struct Executor {
    i: Option<I>,
    cycles_remaining: usize,
    is_32_bit: bool,
}

#[allow(unused)]
pub struct ExecutorPool {
    pool: Vec<Executor>,
    scoreboard: [bool; 16],
    tracer: Tracer,
    log_file: File,
    stack_file: File,
    pub(super) executed_count: u32,
}

impl ExecutorPool {
    pub fn new(n: usize, tracer: Tracer, log_file_path: &str, stack_file_path: &str) -> Self {
        ExecutorPool {
            pool: vec![Executor::new(); n],
            scoreboard: [false; 16],
            tracer,
            log_file: OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(log_file_path)
                .unwrap(),
            stack_file: OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(stack_file_path)
                .unwrap(),
            executed_count: 0,
        }
    }
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            i: None,
            cycles_remaining: 0,
            is_32_bit: false,
        }
    }
}
