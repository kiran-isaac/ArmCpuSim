use super::*;
use crate::binary::is_32_bit;
use crate::decode::I;
use std::collections::VecDeque;

struct IONonPipelinedCPU {
    state: ProcessorState,

    tracer: Tracer,
    instr_queue: VecDeque<(I, u8)>,
}
impl CPU for IONonPipelinedCPU {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self {
        Self {
            tracer: Tracer::new(trace_file, &state.regs),
            state,
            instr_queue: VecDeque::new(),
        }
    }

    fn tick(&mut self) {
        // 3 stage pipeline, fetch, decode, execute
        
        // Max len of the instruction queue is 20, as this is double the len of the largest mop sequence
        // Only get more if the IQ holds less than half of this
        if self.instr_queue.len() < 10 {
            let instruction = self.state.mem.get_instruction(self.state.regs.pc);
            let i = decode(instruction);
            let i_as_mops = decode2(i);
            let mops_len = i_as_mops.len();
            for (i, mop) in i_as_mops.into_iter().enumerate() {
                // Only increment PC if this is the last MOP in the stream
                if i < mops_len - 1 {
                    self.instr_queue.push_back((mop, 0));
                } else {
                    self.instr_queue
                        .push_back((mop, if is_32_bit(instruction) { 4 } else { 2 }));
                }
            }
        }
    }
}
