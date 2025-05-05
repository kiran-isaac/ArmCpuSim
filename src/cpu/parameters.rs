use crate::cpu::PredictionAlgorithms;

pub const N_ISSUE: usize = 2;
pub const FETCH_WIDTH: usize = 2;
pub const CDB_WIDTH: usize = 2;
pub const LQ_SIZE: usize = 32;
pub const N_LS_EXECS: usize = 2;
pub const N_ALUSHIFTERS: usize = 2;
pub const N_MULS: usize = 1;
pub const N_CONTROL: usize = 1;
pub const PREDICT: PredictionAlgorithms = PredictionAlgorithms::Bits(2);
pub const ROB_ENTRIES: usize = 64;
pub const FLUSH_DELAY: u32 = 3;
pub const N_ALUSHIFT_RS: usize = 12;
pub const N_MUL_RS: usize = 12;
pub const N_CNTRL_RS: usize = 12;
pub const N_LS_RS: usize = 12;
pub const STORE_LOAD_FORWARDING : bool = true;