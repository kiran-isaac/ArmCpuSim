mod ooo_speculative;

use crate::binary::is_32_bit;
use crate::components::RS::*;
use crate::decode::{decode, decode2, get_issue_type, IssueType, I};
use crate::log::Tracer;
use crate::model::{ProcessorState, Registers};
pub use ooo_speculative::*;
use ratatui::Frame;

pub trait CPU {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self;

    fn tick(&mut self);

    fn render(&self, frame: &mut Frame);
}
