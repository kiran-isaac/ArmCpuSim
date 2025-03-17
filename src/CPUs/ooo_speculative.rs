use super::*;
use std::collections::VecDeque;

pub struct OoOSpeculative {
    state: ProcessorState,

    tracer: Tracer,

    // Only single fetch buffer space needed, as decode buffer will always produce
    // same or more num of mops, so fetch is never limiting factor
    fb: Option<u32>,
    iq: VecDeque<(I, u8)>,
    rob: ROB,

    // Reservation stations
    rs_mul: RSSet<4>,
    // Split across both ALUs
    rs_alu: RSSet<8>,
    rs_ls: RSSet<4>,
    rs_control: RSSet<4>,

    // The branch predictor stores the speculative PC
    branch_predictor: BranchPredictor,
}

impl CPU for OoOSpeculative {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self {
        Self {
            tracer: Tracer::new(trace_file, &state.regs),
            state,
            fb: None,
            iq: VecDeque::new(),
            rob: ROB::new(),

            rs_alu: RSSet::new(),
            rs_mul: RSSet::new(),
            rs_control: RSSet::new(),
            rs_ls: RSSet::new(),
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

        // The pipeline stages are simulated backwards to avoid instantaneous updates

        // --------------------------------------------------------
        // Issue
        if !self.rob.is_full() && self.iq.len() > 0 {
            let mop = self.iq.pop_front().unwrap();
        }

        // --------------------------------------------------------
        // DECODE
        if self.iq.len() < 10 {
            if let Some(fb_instr) = self.fb {
                let i = decode(fb_instr);
                let i_as_mops = decode2(i);
                let mops_len = i_as_mops.len();
                for (i, mop) in i_as_mops.into_iter().enumerate() {
                    // Only increment PC if this is the last MOP in the stream
                    if i < mops_len - 1 {
                        self.iq.push_back((mop, 0));
                    } else {
                        self.iq
                            .push_back((mop, if is_32_bit(fb_instr) { 4 } else { 2 }));
                    }
                }

                // Consume from buffer
                self.fb = None;
            }
        }

        // --------------------------------------------------------
        // FETCH
        if self.fb.is_none() {
            let fetched = self.state.mem.get_instruction(self.state.regs.pc);
            self.fb = Some(fetched);
            self.fetch_speculative_pc =
                self.branch_predictor + if is_32_bit(fetched) { 4 } else { 2 };
        }
    }
}
