use crate::{ProcessorState, I, IT::*};

struct Executor {
    i: Option<I>,
    cycles_remaining: usize,
}

struct ExecutorPool {
    pool: [Executor; 8],
    scoreboard: [bool; 16]
}

impl Executor {
    pub fn execute(i : I, state: &mut ProcessorState) {
        match i.it {
            UNPREDICTABLE => panic!("Cannot execute unpredictable"),
            UNDEFINED => panic!("Cannot execute undefined"),

            ADC => {
                u32::overflowing_add(u32::overflowing_add(state.regs[i.rn], state.regs[i.rm]), 1);
            }
        }
    }
}