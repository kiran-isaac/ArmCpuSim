mod ooo_speculative;

use crate::components::{ROB, RS};
use crate::decode::{decode, decode2, get_issue_type, I, IT, IssueType};
use crate::log::Tracer;
use crate::model::{Memory, ProcessorState, Registers};
pub use ooo_speculative::OoOSpeculative;

pub trait CPU {
    fn new(state: ProcessorState, trace_file: &str, log_file: &str, stack_dump_file: &str) -> Self;

    fn tick(&mut self);
}
