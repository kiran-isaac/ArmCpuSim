use std::cmp::min;
use std::collections::HashMap;
use crate::cpu::{PREDICT, PredictionAlgorithms::*};

pub struct BTB {
    // PC: (pred pc, direction)
    hm: HashMap<u32, u32>
}

impl BTB {
    pub fn new() -> Self { 
        Self {
            hm: HashMap::new()
        }
    }
    
    pub fn make_prediction(&self, pc: u32) -> bool {
        if let Some(counter) = self.hm.get(&pc) {
            let counter = *counter;
            
            match PREDICT {
                Bits(n) => {
                    let pow = 2_u32.pow(n as u32);
                    assert!(counter < pow);
                    counter >= pow / 2 
                }
                _ => panic!()
            }
        } else {
            match PREDICT {
                Bits(_) => false,
                _ => panic!()
            }
        }
    }
    
    pub fn update(&mut self, pc: u32, taken: bool) {
        if let Some(counter) = self.hm.get_mut(&pc) {
            *counter += taken as u32;
            
            match PREDICT {
                Bits(n) => {
                    {
                        *counter = min(2_u32.pow(n as u32) - 1, *counter);
                    }
                }
                _ => unreachable!()
            }
        } else {
            self.hm.insert(pc, taken as u32);
        }
    }
}