mod commit;
mod decode;
mod execute;
mod fetch;
mod issue;
mod wb;

use super::*;
use std::collections::{HashMap, VecDeque};

use ratatui::layout::Margin;
use ratatui::prelude::Alignment;
use ratatui::widgets::{Borders, Padding, Paragraph};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
use crate::components::ALU::ASPRUpdate;

const CDB_WIDTH: usize = 1;

#[derive(Clone, Copy)]
struct CDBRecord {
    valid: bool,
    rob_number: usize,
    result: u32,
    aspr_update: ASPRUpdate
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
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
    rs_mul: RSSet,
    // Split across both ALUs
    rs_alu_shift: RSSet,
    rs_ls: RSSet,
    rs_control: RSSet,

    fetch_pc: u32,

    cdb: [CDBRecord; CDB_WIDTH],

    // Render Info
    stalls: Vec<StallReason>,
    epoch: usize,
    pub rs_current_display: IssueType,
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

            rs_alu_shift: RSSet::new(IssueType::ALUSHIFT, 8),
            rs_mul: RSSet::new(IssueType::MUL, 4),
            rs_control: RSSet::new(IssueType::Control, 4),
            rs_ls: RSSet::new(IssueType::LoadStore, 8),

            stalls: Vec::new(),
            epoch: 0,
            rs_current_display: IssueType::ALUSHIFT,
            cdb: [CDBRecord {valid: false, rob_number: 0, result: 0, aspr_update: ASPRUpdate::no_update()}; CDB_WIDTH],
        }
    }

    fn tick(&mut self) {
        // 6 stage pipeline
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
        let horizontal = Layout::horizontal([Length(20), Fill(20)]);
        let [left_area, right_area] = horizontal.areas(main_area);

        let (rs_to_display, rs_to_display_n, rs_to_display_name) = match self.rs_current_display {
            IssueType::ALUSHIFT => (&self.rs_alu_shift, 1, "ALU/Shift"),
            IssueType::MUL => (&self.rs_mul, 2, "MUL"),
            IssueType::LoadStore => (&self.rs_ls, 3, "Load/Store"),
            IssueType::Control => (&self.rs_control, 4, "Control"),
        };

        frame.render_widget(Block::bordered().title("Stats"), left_area);
        frame.render_widget(Block::bordered().title("Pipeline"), right_area);

        // Get the area in the boxes
        let left_area = left_area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });
        let right_area = right_area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        let [fb_area, iq_area, rs_area] =
            Layout::vertical([Length(3), Length(5), Length(10)]).areas(right_area);
        let [epoch_area, rst_area, stall_area] =
            Layout::vertical([Length(3), Length(22), Fill(1)]).areas(left_area);

        let bottom_border = |name| {
            Block::bordered()
                .borders(Borders::BOTTOM)
                .title(name)
                .title_alignment(Alignment::Center)
                .padding(Padding::left(2))
        };

        // Render epoch num
        frame.render_widget(
            Paragraph::new(format!("Epoch: {}", self.epoch)).block(bottom_border("")),
            epoch_area,
        );

        let rs_str = self
            .rob
            .register_status
            .iter()
            .enumerate()
            .map(|(i, rob_entry)| match rob_entry {
                Some(entry) => format!("{}: #{}", i, entry),
                None => format!(
                    "{:02} : {:08X}",
                    Registers::reg_id_to_str(i as u8),
                    self.state.regs.get(i as u8)
                ),
            })
            .collect::<Vec<String>>()
            .join("\n");

        frame.render_widget(
            Paragraph::new(format!("{rs_str}")).block(bottom_border("Register Status")),
            rst_area,
        );

        frame.render_widget(
            Paragraph::new(format!(
                "{}",
                match &self.fb {
                    Some(i) => format!("{:08X}", i.i),
                    None => "-".to_string(),
                }
            ))
            .block(bottom_border("Fetch Buffer")),
            fb_area,
        );

        let i_strs = self
            .iq
            .iter()
            .enumerate()
            .map(|(i, iqe)| format!("{}: {}", i, iqe.i))
            .collect::<Vec<String>>()
            .join("\n");

        frame.render_widget(
            Paragraph::new(format!("{}", i_strs)).block(bottom_border("Instruction Queue")),
            iq_area,
        );

        let mut stall_count: HashMap<StallReason, usize> = HashMap::new();
        for x in self.stalls.iter() {
            *stall_count.entry(*x).or_default() += 1;
        }

        let mut stall_string = String::new();
        for entry in stall_count {
            stall_string += &format!("{:?}: {}", entry.0, entry.1);
        }

        frame.render_widget(
            Paragraph::new(stall_string)
                .block(bottom_border("Stall Status").borders(Borders::NONE)),
            stall_area,
        );

        let rs_title = format!("RS {}/4: {}", rs_to_display_n, rs_to_display_name);

        let [rs_area, rs_area_inner] = Layout::vertical([Length(1), Fill(1)]).areas(rs_area);

        let [index_area, j_area, k_area, l_area, inst_area] =
            Layout::horizontal([Length(3), Length(11), Length(11), Length(11), Fill(1)])
                .areas(rs_area_inner);

        fn make_block_from_property(
            rs_to_display: &RSSet,
            property_getter: fn(&RS) -> String,
            name: String,
        ) -> Paragraph {
            Paragraph::new(
                rs_to_display
                    .vec
                    .iter()
                    .map(|rs| {
                        if rs.busy {
                            property_getter(rs)
                        } else {
                            String::new()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
            .block(
                Block::new()
                    .title(name)
                    .title_alignment(Alignment::Center)
                    .borders(Borders::RIGHT),
            )
        }

        let j_para =
            make_block_from_property(rs_to_display, |rs: &RS| rs.j.to_string(), String::from("j"));
        let k_para =
            make_block_from_property(rs_to_display, |rs: &RS| rs.k.to_string(), String::from("k"));
        let l_para =
            make_block_from_property(rs_to_display, |rs: &RS| rs.l.to_string(), String::from("l"));
        let inst_para =
            make_block_from_property(rs_to_display, |rs: &RS| rs.i.to_string(), String::from("I"));

        let index_block = Paragraph::new(
            (1..=rs_to_display.len())
                .into_iter()
                .map(|j| format!("{}", j))
                .collect::<Vec<String>>()
                .join("\n"),
        )
        .block(
            Block::new()
                .title("I")
                .title_alignment(Alignment::Center)
                .borders(Borders::RIGHT),
        );

        frame.render_widget(
            Block::new()
                .title(rs_title)
                .title_alignment(Alignment::Center),
            rs_area,
        );
        frame.render_widget(index_block, index_area);
        frame.render_widget(j_para, j_area);
        frame.render_widget(k_para, k_area);
        frame.render_widget(l_para, l_area);
        frame.render_widget(inst_para, inst_area);
    }
}

impl OoOSpeculative {
    fn stall(&mut self, reason: StallReason) {
        self.stalls.push(reason);
    }
    
    fn get_cdb_issue_slot(&self) -> Option<usize> {
        self.cdb.iter().position(|e| e.valid)
    }
    
    fn wipe_cdb(&mut self) {
        self.cdb = [CDBRecord {valid: false, rob_number: 0, result: 0, aspr_update: ASPRUpdate::no_update()}; CDB_WIDTH];
    }
}
