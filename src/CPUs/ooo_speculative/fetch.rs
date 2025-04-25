use std::cmp::PartialEq;
use crate::binary::briz;
use crate::decode::{decode_b1, decode_b2, decode_bl};
use super::*;


impl OoOSpeculative {
    pub(super) fn fetch(&mut self) {
        if self.fb.is_none() {
            let fetched = self.state.mem.get_instruction(self.spec_pc);
            let pc_increment = if is_32_bit(fetched) { 4 } else { 2 };
            self.fb = Some((self.spec_pc + pc_increment, fetched));
            
            if PREDICT == PredictionAlgorithms::AlwaysTaken {
                // we predict taken if its a B or a BL, if its BX or BLX
                if let Some(target) = Self::pre_decode(self.spec_pc, fetched) {
                    self.spec_pc = target;
                    return;
                }
            }
            self.spec_pc += pc_increment;
        }
    }
    
    // For branch prediction
    fn pre_decode(pc: u32, i: u32) -> Option<u32> {
        // If its BL
        if (i & 0b1111_1000_0000_0000_1101_0000_0000_0000) == 0b1111_0000_0000_0000_1101_0000_0000_0000 {
            let i = decode_bl(i);
            return Some(pc.wrapping_add((i.imms + 4) as u32))
        }
        
        // B (T1)
        // if its 0b1111 then its svc
        if ((i & 0b1111_0000_0000_0000) == 0b1101_0000_0000_0000) && briz(i, 8, 11) != 0b1111 {
            let i = decode_b1(i);
            return Some(pc.wrapping_add((i.imms + 4) as u32))
        }
        
        // B (T2)
        if ((i & 0b1111_1000_0000_0000) == 0b1110_0000_0000_0000) {
            let i = decode_b2(i);
            return Some(pc.wrapping_add((i.imms + 4) as u32))
        }
        
        None
    }
}
