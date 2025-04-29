#![allow(unused)]

pub struct BranchPredictor {
    speculative_pc: u32,
}

impl BranchPredictor {
    fn get_speculative_pc(&self) -> u32 {
        self.speculative_pc
    }
}
