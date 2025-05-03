use crate::cpu::PredictionAlgorithms;

pub const N_ISSUE: usize = 4;
pub const FETCH_WIDTH: usize = N_ISSUE * 2;
pub const CDB_WIDTH: usize = N_ISSUE * 2;
pub const LQ_SIZE: usize = 32;
pub const N_LS_EXECS: usize = N_ISSUE;
pub const N_ALUSHIFTERS: usize = N_ISSUE;
pub const N_MULS: usize = N_ISSUE;
pub const N_CONTROL: usize = N_ISSUE;
pub const PREDICT: PredictionAlgorithms = PredictionAlgorithms::AlwaysTaken;
pub const ROB_ENTRIES: usize = 64;
pub const FLUSH_DELAY: u32 = 2;
pub const N_ALUSHIFT_RS: usize = 12;
pub const N_MUL_RS: usize = 12;
pub const N_CNTRL_RS: usize = 12;
pub const N_LS_RS: usize = 12;