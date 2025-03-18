mod commit;
mod decode;
mod execute;
mod fetch;
mod issue;
mod stalls;
mod wb;

use super::*;
use std::collections::VecDeque;

use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
    style::{Color, Modifier, Style, Stylize},
};
use ratatui::layout::Margin;
use ratatui::Viewport::Fixed;
use ratatui::widgets::Paragraph;

enum StallReason {
    IssueRobFull,
    IssueRSFull,
}

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

    // Render Info
    stalls: Vec<StallReason>,
    epoch: usize,
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

            stalls: Vec::new(),
            epoch: 0,
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

        self.epoch += 1;
    }

    fn render(&self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

        let vertical = Layout::vertical([Min(0)]);
        let [main_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Length(20), Fill(1)]);
        let [left_area, right_area] = horizontal.areas(main_area);

        frame.render_widget(Block::bordered().title("Stats"), left_area);
        frame.render_widget(Block::bordered().title("Pipeline"), right_area);

        // Get the area in the boxes
        let left_area = left_area.inner(Margin { horizontal: 1, vertical: 1 });
        let right_area = right_area.inner(Margin { horizontal: 1, vertical: 1 });
        let [fb_area, iq_area] = Layout::vertical([Length(1), Fill(1)]).areas(right_area);
        let [epoch_area, _,  rs_area] = Layout::vertical([Length(1), Length(1), Fill(1)]).areas(left_area); 
        // Render epoch num
        frame.render_widget(Paragraph::new(format!("Epoch: {}", self.epoch)), epoch_area);
        
        let rs_str = self.rob.register_status.iter().enumerate().map(|(i, rob_entry)| match rob_entry {
            Some(entry) => format!("{}: #{}", i, entry),
            None => format!("{:02} : {:08X}", Registers::reg_id_to_str(i as u8), self.state.regs.get(i as u8)),
        }).collect::<Vec<String>>().join("\n");
        
        frame.render_widget(Paragraph::new(format!("Register Status: \n{rs_str}")), rs_area);

        frame.render_widget(Paragraph::new(format!("Fetch Buffer: {}", match &self.fb {
            Some(i) => format!("{:08X}", i.i),
            None => "-".to_string(),
        })), fb_area);

        let i_strs = self.iq.iter().enumerate().map(| (i, iqe) | format!("  {}: {}", i, iqe.i)).collect::<Vec<String>>().join("\n");
        
        frame.render_widget(Paragraph::new(format!("IQ: \n{}", i_strs)), iq_area);
    }
}

impl OoOSpeculative {
    fn stall(&mut self, reason: StallReason) {
        self.stalls.push(reason);
    }
}
