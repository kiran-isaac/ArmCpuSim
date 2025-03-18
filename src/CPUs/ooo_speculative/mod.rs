mod commit;
mod decode;
mod execute;
mod fetch;
mod issue;
mod wb;

use super::*;
use std::collections::{HashMap, VecDeque};

use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
    style::{Color, Modifier, Style, Stylize},
};
use ratatui::layout::Margin;
use ratatui::prelude::Alignment;
use ratatui::Viewport::Fixed;
use ratatui::widgets::{Borders, Padding, Paragraph};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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
        
        let [fb_area,iq_area] = Layout::vertical([Length(3), Length(5)]).areas(right_area);
        let [epoch_area, rs_area, stall_area] = Layout::vertical([Length(3), Length(22), Fill(1)]).areas(left_area);

        let bottom_border = |name| Block::bordered()
            .borders(Borders::BOTTOM).title(name).title_alignment(Alignment::Center).padding(Padding::left(2));
        
        // Render epoch num
        frame.render_widget(Paragraph::new(format!("Epoch: {}", self.epoch)).block(bottom_border("")), epoch_area);
        
        let rs_str = self.rob.register_status.iter().enumerate().map(|(i, rob_entry)| match rob_entry {
            Some(entry) => format!("{}: #{}", i, entry),
            None => format!("{:02} : {:08X}", Registers::reg_id_to_str(i as u8), self.state.regs.get(i as u8)),
        }).collect::<Vec<String>>().join("\n");
        
        frame.render_widget(Paragraph::new(format!("{rs_str}")).block(bottom_border("Register Status")), rs_area);

        frame.render_widget(Paragraph::new(format!("{}", match &self.fb {
            Some(i) => format!("{:08X}", i.i),
            None => "-".to_string(),
        })).block(bottom_border("Fetch Buffer")), fb_area);

        let i_strs = self.iq.iter().enumerate().map(| (i, iqe) | format!("{}: {}", i, iqe.i)).collect::<Vec<String>>().join("\n");

        frame.render_widget(Paragraph::new(format!("{}", i_strs)).block(bottom_border("Instruction Queue")), iq_area);

        let mut stall_count: HashMap<StallReason, usize> = HashMap::new();
        for x in self.stalls.iter() {
            *stall_count.entry(*x).or_default() += 1;
        }

        for entry in stall_count {
            
        }
    }
}

impl OoOSpeculative {
    fn stall(&mut self, reason: StallReason) {
        self.stalls.push(reason);
    }
}
