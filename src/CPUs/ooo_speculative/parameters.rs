use crate::CPUs::PredictionAlgorithms;

pub const N_ISSUE: usize = 8;
pub const CDB_WIDTH: usize = 10;
pub const LQ_SIZE: usize = 16;
pub const N_LS_EXECS: usize = N_ISSUE;
pub const N_ALUSHIFTERS: usize = N_ISSUE;
pub const N_MULS: usize = N_ISSUE;
pub const N_CONTROL: usize = N_ISSUE;
pub const STALL_ON_BRANCH: bool = false;
pub const PREDICT: PredictionAlgorithms = PredictionAlgorithms::AlwaysTaken;
pub const ROB_ENTRIES: usize = 64;
pub const FLUSH_DELAY: u32 = 2;