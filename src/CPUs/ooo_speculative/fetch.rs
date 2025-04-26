use super::*;
use crate::binary::briz;
use crate::decode::{decode_b1, decode_b2, decode_bl};
use std::cmp::PartialEq;

impl OoOSpeculative {
    pub(super) fn fetch(&mut self) {
        if self.fb.is_none() && !self.fetch_stall {
            let fetched = self.state.mem.get_instruction(self.spec_pc);
            let pc_increment = if is_32_bit(fetched) { 4 } else { 2 };
            self.fb = Some((self.spec_pc + pc_increment, fetched));

            if let Some((control_instruction, control_offset)) = Self::pre_decode(self.spec_pc, fetched) {
                if control_instruction.is_serializing() {
                    self.fetch_stall = true;
                    return;
                }
                
                // BL or B
                if PREDICT == PredictionAlgorithms::AlwaysTaken {
                    self.spec_pc = self.spec_pc.wrapping_add(control_offset);
                    return;
                }
            }
            self.spec_pc += pc_increment;
        }
    }
    
    fn pre_decode(pc: u32, i: u32) -> Option<(IT, u32)> {
        // If its BL
        if (i & 0b1111_1000_0000_0000_1101_0000_0000_0000)
            == 0b1111_0000_0000_0000_1101_0000_0000_0000
        {
            let i = decode_bl(i);
            return Some((IT::BL, i.imms as u32));
        }

        // B (T1)
        // if its 0b1111 then its svc
        if ((i & 0b1111_0000_0000_0000) == 0b1101_0000_0000_0000) {
            return if briz(i, 8, 11) != 0b1111 {
                Some((IT::SVC, 0))
            } else {
                let i = decode_b1(i);
                Some((IT::B, i.imms as u32))
            }
        }

        // B (T2)
        if ((i & 0b1111_1000_0000_0000) == 0b1110_0000_0000_0000) {
            let i = decode_b2(i);
            return Some((IT::B, i.imms as u32));
        }

        // BX
        if ((i & 0b1111_1111_1000_0000) == 0b0100_0111_0000_0000) {
            return Some((IT::BX, 0));
        }

        // BX
        if ((i & 0b1111_1111_1000_0000) == 0b0100_0111_1000_0000) {
            return Some((IT::BLX, 0));
        }

        None
    }
}
