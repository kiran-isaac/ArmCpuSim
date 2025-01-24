use crate::{ProcessorState, I};

pub enum Syscalls {
    HALT = 0,
    PUTS = 1,
    GETS = 2,
}

pub fn syscall(i: &I, state: &mut ProcessorState) {
    match i.immu {
        0 => {
            state.halt = state.regs.get(0) as i32;
        }
        _ => unimplemented!()
    }
}