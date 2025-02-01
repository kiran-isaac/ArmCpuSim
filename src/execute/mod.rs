use std::fmt::Write;

use crate::{binary::*, system::syscall, ProcessorState, I, IT::*};

mod execute_instruction;
mod executor_pool;

#[derive(Clone, Copy)]
pub struct Executor {
    i: Option<I>,
    cycles_remaining: usize,
    is_32_bit: bool
}

#[allow(unused)]
pub struct ExecutorPool {
    pool: Vec<Executor>,
    scoreboard: [bool; 16],
}

impl ExecutorPool {
    pub fn new(n: usize) -> Self {
        ExecutorPool {
            pool: vec![Executor::new(); n],
            scoreboard: [false; 16],
        }
    }
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            i: None,
            cycles_remaining: 0,
            is_32_bit: false
        }
    }
}
