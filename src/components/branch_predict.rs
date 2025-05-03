use std::collections::HashMap;
use crate::cpu::{PREDICT, PredictionAlgorithms::*};

pub struct BTB {
    // PC: (pred pc, direction)
    hm: HashMap<u32, (u32, u32)>
}

impl BTB {
    pub fn new() -> Self { 
        Self {
            hm: HashMap::new()
        }
    }
    
    pub fn make_prediction(&self, pc: u32) -> Option<u32> {
        if let Some((taken_pc, counter)) = self.hm.get(&pc) {
            let taken_pc = *taken_pc;
            let counter = *counter;
            
            match PREDICT {
                OneBit => {
                    assert!(counter < 2);
                    if counter == 1 {
                        Some(taken_pc)
                    } else {
                        None
                    }
                }
                TwoBit => {
                    assert!(counter < 4);
                    if counter >= 2 {
                        Some(taken_pc)
                    } else {
                        None
                    }
                }
                _ => panic!()
            }
        } else {
            match PREDICT {
                OneBit | TwoBit => {
                    None
                }
                _ => panic!()
            }
        }
    }
}