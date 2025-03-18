mod ooo_speculative;

use crate::binary::is_32_bit;
use crate::components::{BranchPredictor, ROB::*, RS::*};
use crate::decode::{decode, decode2, get_issue_type, IssueType, I, IT};
use crate::log::Tracer;
use crate::model::{Memory, ProcessorState, Registers};
pub use ooo_speculative::OoOSpeculative;
use ratatui::backend::CrosstermBackend;
use ratatui::Frame;

pub trait CPU {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self;

    fn tick(&mut self);

    fn render(&self, frame: &mut Frame);
}
