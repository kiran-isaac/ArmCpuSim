use super::*;
use std::collections::VecDeque;

struct InstructionQueueEntry {
    pub i: I,
    /// the pc value fetched from
    pub pc: u32,
    /// how the pc should be updated after this instruction commits
    pub pc_increment: u8,
}

struct FetchBufferEntry {
    pub i: u32,
    pub pc: u32,
}

pub struct OoOSpeculative {
    state: ProcessorState,

    tracer: Tracer,

    // Only single fetch buffer space needed, as decode buffer will always produce
    // same or more num of mops, so fetch is never limiting factor
    fb: Option<FetchBufferEntry>,
    iq: VecDeque<InstructionQueueEntry>,
    rob: ROB,

    // Reservation stations
    rs_mul: RSSet<4>,
    // Split across both ALUs
    rs_alu: RSSet<8>,

    rs_shift: RSSet<4>,
    rs_ls: RSSet<4>,
    rs_control: RSSet<4>,

    fetch_pc: u32,
}

impl CPU for OoOSpeculative {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self {
        Self {
            tracer: Tracer::new(trace_file, &state.regs),
            fetch_pc: state.regs.pc,
            state,
            fb: None,
            iq: VecDeque::new(),
            rob: ROB::new(),

            rs_alu: RSSet::new(IssueType::ALU),
            rs_shift: RSSet::new(IssueType::Shift),
            rs_mul: RSSet::new(IssueType::MUL),
            rs_control: RSSet::new(IssueType::Control),
            rs_ls: RSSet::new(IssueType::LoadStore),
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
            let iqe = self.iq.pop_front().unwrap();
            let dest = self.rob.issue_receive(&iqe.i, iqe.pc);

            match get_issue_type(iqe.i.it.clone()) {
                IssueType::ALU => self.rs_alu.issue_receive(
                    &iqe.i,
                    dest,
                    &self.state.regs,
                    &self.rob.register_status,
                ),
                IssueType::MUL => self
                    .rs_mul
                    .issue_receive(
                        &iqe.i,
                        dest,
                        &self.state.regs,
                        &self.rob.register_status,
                    ),
                IssueType::LoadStore => self
                    .rs_ls
                    .issue_receive(
                        &iqe.i,
                        dest,
                        &self.state.regs,
                        &self.rob.register_status,
                    ),
                IssueType::Control => self
                    .rs_control
                    .issue_receive(
                        &iqe.i,
                        dest,
                        &self.state.regs,
                        &self.rob.register_status,
                    ),
                _ => unimplemented!("where to put shift"),
            };
        }

        // --------------------------------------------------------
        // DECODE
        if self.iq.len() < 10 {
            if let Some(fb_entry) = &self.fb {
                let i = decode(fb_entry.i);
                let pc = fb_entry.pc;
                let pc_increment = if is_32_bit(fb_entry.i) { 4 } else { 2 };
                let i_as_mops = decode2(i);
                let mops_len = i_as_mops.len();

                for (i, mop) in i_as_mops.into_iter().enumerate() {
                    // Only increment PC if this is the last MOP in the stream
                    if i < mops_len - 1 {
                        self.iq.push_back(InstructionQueueEntry {
                            i: mop,
                            pc,
                            pc_increment: 0,
                        });
                    } else {
                        self.iq.push_back(InstructionQueueEntry {
                            i: mop,
                            pc,
                            pc_increment,
                        });
                    }
                }

                // Consume from buffer
                self.fb = None;
            }
        }

        // --------------------------------------------------------
        // FETCH
        if self.fb.is_none() {
            let fetched = self.state.mem.get_instruction(self.fetch_pc);
            self.fb = Some(FetchBufferEntry {
                i: fetched,
                pc: self.fetch_pc,
            });
            self.fetch_pc += if is_32_bit(fetched) { 4 } else { 2 };
        }
    }
}
