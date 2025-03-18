mod fetch;
mod decode;
mod issue;
mod execute;
mod wb;
mod commit;

use super::*;
use std::collections::VecDeque;
use sdl2::libc::exit;

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
        self.commit();
        self.wb();
        self.execute();
        self.issue();
        self.decode();
        self.fetch();
    }
}