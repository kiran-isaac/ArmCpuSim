use super::*;
use crate::binary::is_32_bit;
use std::collections::VecDeque;

pub struct OoOSpeculative {
    state: ProcessorState,

    tracer: Tracer,

    /// Only single fetch buffer space needed, as decode buffer will always produce
    /// same or more num of mops, so fetch is never limiting factor
    fb_current: Option<u32>,
    fb_next: Option<u32>,
    
    iq_current: VecDeque<(I, u8)>,
    iq_next: VecDeque<(I, u8)>,
    
    rob: ROB,
}

impl CPU for OoOSpeculative {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self {
        Self {
            tracer: Tracer::new(trace_file, &state.regs),
            state,
            fb: None,
            iq_current: VecDeque::new(),
            rob: ROB::new(),
        }
    }

    fn tick(&mut self) {
        // 6 stage pipeline:
        // Fetch
        // Decode
        // Issue
        // Execute
        // WB
        // Commit

        // The pipeline stages are simulated forwards, so all pipeline registers have a
        // current and next value to avoid instantaneous updates
        
        // --------------------------------------------------------
        // FETCH
        if self.fb_current.is_none() {
            self.fb_current = self.fb_next;
            self.fb_next = Some(self.state.mem.get_instruction(self.state.regs.pc));
        }

        // --------------------------------------------------------
        // DECODE
        if self.iq_current.len() < 10 {
            if let Some(fb_instr) = self.fb_current {
                let i = decode(fb_instr);
                let i_as_mops = decode2(i);
                let mops_len = i_as_mops.len();
                for (i, mop) in i_as_mops.into_iter().enumerate() {
                    // Only increment PC if this is the last MOP in the stream
                    if i < mops_len - 1 {
                        self.iq_current.push_back((mop, 0));
                    } else {
                        self.iq_current
                            .push_back((mop, if is_32_bit(fb_instr) { 4 } else { 2 }));
                    }
                }
                
                // Consume from buffer
                self.fb_current = None;
            }
        }
        
        // Fetch
        
    }
}
