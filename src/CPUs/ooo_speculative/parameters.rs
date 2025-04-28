use crate::CPUs::PredictionAlgorithms;

pub const N_ISSUE: usize = 4;
pub const CDB_WIDTH: usize = 2;
pub const LQ_SIZE: usize = 16;
pub const N_LS_EXECS: usize = 2;
pub const N_ALUSHIFTERS: usize = 2;
pub const N_MULS: usize = 1;
pub const N_CONTROL: usize = 1;
pub const STALL_ON_BRANCH: bool = false;
pub const PREDICT: PredictionAlgorithms = PredictionAlgorithms::AlwaysUntaken;
pub const ROB_ENTRIES: usize = 64;
pub const FLUSH_DELAY: u32 = 2;