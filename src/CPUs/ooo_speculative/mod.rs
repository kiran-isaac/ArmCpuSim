use itertools::Itertools;
mod commit;
mod decode;
mod execute;
mod fetch;
mod issue;
mod wb;

use super::*;
use crate::components::ROB::{ROBEntryDest, ROBStatus};
use crate::decode::IT;
use crate::model::Registers;
use crate::{components::ALU::ASPRUpdate, components::ROB::ROB};
use ratatui::layout::Margin;
use ratatui::prelude::Alignment;
use ratatui::widgets::{Borders, Padding, Paragraph};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::Block,
    Frame,
};
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq)]
pub enum PredictionAlgorithms {
    AlwaysTaken,
    AlwaysUntaken,
}

pub const N_ISSUE: usize = 4;
const CDB_WIDTH: usize = 10;
const LQ_SIZE: usize = 8;
pub const N_LS_EXECS: usize = 2;
pub const N_ALUSHIFTERS: usize = 2;
pub const N_MULS: usize = 1;
pub const N_CONTROL: usize = 1;
pub const STALL_ON_BRANCH: bool = false;
pub const PREDICT: PredictionAlgorithms = PredictionAlgorithms::AlwaysTaken;
pub const ROB_ENTRIES: usize = 64;
pub const FLUSH_DELAY: u32 = 2;

#[derive(Clone, Copy)]
struct CDBRecord {
    is_branch_target: bool,
    valid: bool,
    rob_number: usize,
    result: u32,
    aspr_update: ASPRUpdate,
    halt: bool,
}

#[derive(Clone, Copy)]
pub struct LoadQueueEntry {
    pub address: u32,
    pub rob_entry: usize,
    pub load_type: IT,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum StallReason {
    FullRob,
    IssueRSFull,
    ExecuteLSQFull,
    IStall,
}

struct InstructionQueueEntry {
    pub i: I,
    /// the pc value fetched from
    pub pc: u32,
}
pub struct OoOSpeculative<'a> {
    state: ProcessorState,
    log_fn: Box<dyn FnMut(String) + 'a>,

    tracer: Tracer,

    // Only single fetch buffer space needed, as decode buffer will always produce
    // same or more num of mops, so fetch is never limiting factor
    fb: [Option<(u32, u32)>; N_ISSUE],
    iq: VecDeque<InstructionQueueEntry>,
    rob: ROB,

    load_queue: VecDeque<LoadQueueEntry>,

    // Reservation stations
    rs_mul: RSSet,
    // Split across both ALUs
    rs_alu_shift: RSSet,
    rs_ls: RSSet,
    rs_control: RSSet,

    pub output: String,

    flush_delay: u32,
    flushing: bool,
    spec_pc: u32,

    fetch_stall: bool,
    pub mispredicts: u32,
    pub correct_predicts: u32,

    // only the first {CDB_WIDTH} are currently being broadcasted
    cdb: VecDeque<CDBRecord>,
    // Holds all the simulated delays of simulated operations, and
    // when they should be broadcast onto CDB
    to_broadcast: Vec<(u8, CDBRecord)>,

    // Render Info
    stalls: Vec<StallReason>,
    pub epoch: usize,
    pub instructions_committed: usize,
    pub rs_current_display: IssueType,
    pub rob_focus: usize,
    pub mem_bottom_offset: usize,
    pub display_focus: usize,

    // The pc of the fn and its name
    pub call_stack: Vec<(u32, String)>,

    pub halt: Option<i32>,
}

impl<'a> OoOSpeculative<'a> {
    pub fn new<F>(state: ProcessorState, trace_file: &str, log_fn: F, stack_dump_file: &str) -> Self
    where
        F: FnMut(String) + 'a,
    {
        let rob = ROB::new();
        Self {
            output: String::new(),
            log_fn: Box::new(log_fn),

            tracer: Tracer::new(trace_file, &state.regs),
            spec_pc: state.regs.pc,
            state: state.clone(),
            fb: [None; N_ISSUE],
            iq: VecDeque::new(),

            rs_alu_shift: RSSet::new(IssueType::ALUSHIFT, 8),
            rs_mul: RSSet::new(IssueType::MUL,  8),
            rs_control: RSSet::new(IssueType::Control, 8),
            rs_ls: RSSet::new(IssueType::LoadStore, 8),

            rob,
            flush_delay: 0,
            flushing: false,
            fetch_stall: false,
            load_queue: VecDeque::with_capacity(LQ_SIZE),

            stalls: Vec::new(),
            mispredicts: 0,
            correct_predicts: 0,
            epoch: 0,
            instructions_committed: 0,
            rs_current_display: IssueType::ALUSHIFT,
            rob_focus: 0,
            to_broadcast: Vec::new(),
            cdb: VecDeque::new(),
            mem_bottom_offset: 0,
            display_focus: 0,
            halt: None,
            call_stack: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        // 6 stage pipeline
        // The pipeline stages are simulated backwards to avoid instantaneous updates
        self.epoch += 1;

        if self.flushing {
            self.flush_delay -= 1;
            if self.flush_delay == 0 {
                self.flushing = false;
            }
        }

        for _ in 0..N_ISSUE {
            self.commit();
        }
        if self.flushing {
            return;
        }

        self.wb();
        self.execute();


        if self.rob.is_full() {
            self.stall(StallReason::FullRob);
            return;
        }

        // If last issued was serializing dont speculatively fetch or issue any more this
        // cycle
        if let Some(last_issued) = self.rob.get_last_issued() {
            if last_issued.is_serializing() {
                self.stall(StallReason::IStall);
                return;
            }
        }

        for _ in 0..N_ISSUE {
            self.issue();
        }
        self.decode();
        self.fetch();
    }

    // -----------------------------------------------------------------
    // Rendering stuff
    pub fn render(&self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

        let vertical = Layout::vertical([Min(0)]);
        let [main_area] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Length(20), Length(40), Fill(20)]);
        let [left_area, rob_area, right_area] = horizontal.areas(main_area);

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

        let [fb_area, iq_area, rs_area, mem_top_border, mem_area] =
            Layout::vertical([Length((2 + N_ISSUE) as u16), Length(5), Length(10), Length(1), Fill(1)])
                .areas(right_area);
        let [epoch_area, rst_area, stall_area, call_stack_area] =
            Layout::vertical([Length(4), Length(22), Length(5), Fill(1)]).areas(left_area);

        let bottom_border = |name| {
            Block::bordered()
                .borders(Borders::BOTTOM)
                .title(name)
                .title_alignment(Alignment::Center)
                .padding(Padding::left(2))
        };

        // Render epoch num
        frame.render_widget(
            Paragraph::new(format!(
                "Epoch: {}\nCommitted: {}",
                self.epoch, self.instructions_committed
            ))
            .block(bottom_border("")),
            epoch_area,
        );

        let rs_str = self
            .rob
            .register_status
            .iter()
            .enumerate()
            .map(|(i, rob_entry)| match rob_entry {
                Some(entry) => format!("{:03} : #{}", Registers::reg_id_to_str(i as u8), entry),
                None => format!(
                    "{:03} : {:08X}",
                    Registers::reg_id_to_str(i as u8),
                    self.state.regs.get(i as u8)
                ),
            })
            .collect::<Vec<String>>()
            .join("\n");

        frame.render_widget(
            Paragraph::new(rs_str).block(bottom_border("Register Status")),
            rst_area,
        );

        let rob_str = self.rob.render(self.rob_focus);

        frame.render_widget(
            Paragraph::new(rob_str).block(Block::bordered().title(if self.display_focus == 0 {
                "#ROB#"
            } else {
                "ROB"
            })),
            rob_area,
        );

        let mem_string = self.state.mem.dump(
            mem_area.width.into(),
            (mem_area.height - 2).into(),
            0x22000000,
            self.mem_bottom_offset,
        );
        frame.render_widget(Block::new().borders(Borders::BOTTOM), mem_top_border);
        frame.render_widget(
            Paragraph::new(mem_string).block(
                Block::new()
                    .title(if self.display_focus == 1 {
                        "#Mem#"
                    } else {
                        "Mem"
                    })
                    .title_alignment(Alignment::Center),
            ),
            mem_area,
        );

        frame.render_widget(
            Paragraph::new(format!(
                "{}",
                (0..N_ISSUE).map(|j| {
                    match &self.fb[j] {
                        Some((pc, i)) => format!("{:08X}   Spec PC: {:08X?}", i, pc),
                        None => "-".to_string(),
                    }
                }).join("\n"),
            ))
            .block(bottom_border("Fetch Buffer")),
            fb_area,
        );

        let i_strs = self
            .iq
            .iter()
            .enumerate()
            .map(|(i, iqe)| format!("{}: {:<14}    {:08X?}", i, iqe.i.to_string(), iqe.pc))
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
            stall_string += &format!("{:?}: {}\n", entry.0, entry.1);
        }

        frame.render_widget(
            Paragraph::new(stall_string).block(bottom_border("Stall Status")),
            stall_area,
        );

        frame.render_widget(
            Paragraph::new(
                self.call_stack
                    .iter()
                    .fold(String::new(), |acc, x| acc + "\n" + &x.1),
            ),
            call_stack_area,
        );

        let rs_title = format!("RS {}/4: {}", rs_to_display_n, rs_to_display_name);

        let [rs_area, rs_area_inner] = Layout::vertical([Length(1), Fill(1)]).areas(rs_area);

        let [index_area, j_area, k_area, l_area, inst_area] =
            Layout::horizontal([Length(3), Length(11), Length(11), Length(11), Fill(1)])
                .areas(rs_area_inner);

        fn make_block_from_property<'a>(
            rs_to_display: &RSSet,
            property_getter: fn(&RS) -> String,
            name: String,
        ) -> Paragraph<'a> {
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

    fn stall(&mut self, reason: StallReason) {
        self.stalls.push(reason);
    }

    pub fn rob_focus_up(&mut self) {
        self.rob_focus += 1;
        if self.rob_focus >= ROB_ENTRIES {
            self.rob_focus = 0;
        }
    }

    pub fn rob_focus_down(&mut self) {
        if self.rob_focus == 0 {
            self.rob_focus = ROB_ENTRIES;
        }
        self.rob_focus -= 1;
    }

    pub fn reset(&mut self) {
        self.flush_on_mispredict();
        self.rob.clear();
        self.rs_mul.empty();
        self.rs_alu_shift.empty();
        self.rs_control.empty();
        self.rs_ls.empty();
        self.epoch = 0;
    }
}
