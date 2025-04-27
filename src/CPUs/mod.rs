mod ooo_speculative;

use crate::binary::is_32_bit;
use crate::components::RS::*;
use crate::decode::{decode, decode2, get_issue_type, IssueType, I};
use crate::log::Tracer;
use crate::model::ProcessorState;
pub use ooo_speculative::*;
use ratatui::Frame;
