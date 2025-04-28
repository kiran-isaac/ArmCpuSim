use crate::cpu::PredictionAlgorithms;

pub const N_ISSUE: usize = 4;
pub const FETCH_WIDTH: u32 = 8;
pub const CDB_WIDTH: usize = 4;
pub const LQ_SIZE: usize = 16;
pub const N_LS_EXECS: usize = 4;
pub const N_ALUSHIFTERS: usize = 4;
pub const N_MULS: usize = 4;
pub const N_CONTROL: usize = 4;
pub const STALL_ON_BRANCH: bool = false;
pub const PREDICT: PredictionAlgorithms = PredictionAlgorithms::AlwaysTaken;
pub const ROB_ENTRIES: usize = 64;
pub const FLUSH_DELAY: u32 = 2;
pub const N_ALUSHIFT_RS: usize = 12;
pub const N_MUL_RS: usize = 8;
pub const N_CNTRL_RS: usize = 8;
pub const N_LS_RS: usize = 12;