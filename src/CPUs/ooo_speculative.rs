use super::*;
use std::collections::VecDeque;

pub struct OoOSpeculative {
    state: ProcessorState,

    tracer: Tracer,
    instr_queue: VecDeque<(I, u8)>,
}
impl CPU for OoOSpeculative {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self {
        Self {
            tracer: Tracer::new(trace_file, &state.regs),
            state,
            instr_queue: VecDeque::new(),
        }
    }

    fn tick(&mut self) {}
}
