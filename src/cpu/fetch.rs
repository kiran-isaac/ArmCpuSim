use super::*;
use crate::binary::briz;
use crate::decode::{decode_b1, decode_b2, decode_bl};

impl<'a> OoOSpeculative<'a> {
    pub(super) fn fetch(&mut self) {
        for i in 0..N_ISSUE {
            if self.fb[i].is_none() && !self.fetch_stall {
                let fetched = self.state.mem.get_instruction(self.spec_pc);
                let pc_increment = if is_32_bit(fetched) { 4 } else { 2 };

                /* The use of 0b1111 as a register specifier is not normally permitted in Thumb instructions. When a value of 0b1111 is
                   permitted, a variety of meanings is possible. For register reads, these meanings are:
                       • Read the PC value, that is, the address of the current instruction + 4. Some instructions read the PC value
                       implicitly, without the use of a register specifier, for example the conditional branch instruction B<c>.
                       • Read the word-aligned PC value, that is, the address of the current instruction + 4, with bits [1:0] forced to
                       zero. This enables instructions such as ADR and LDR (literal) instructions to use PC-relative data addressing.
                       The register specifier is implicit in the ARMv6-M encodings of these instructions.
                */

                if let Some((control_instruction, control_offset)) =
                    Self::pre_decode(self.spec_pc, fetched)
                {
                    // if control_instruction == IT::B {
                    //     self.fb = Some((self.spec_pc + 4, fetched));
                    // } else {
                    self.fb[i] = Some((self.spec_pc + pc_increment, fetched));

                    if control_instruction.is_serializing() {
                        self.fetch_stall = true;
                        self.spec_pc += pc_increment;
                        continue;
                    }

                    if control_instruction != IT::B && control_instruction != IT::BL {
                        panic!()
                    }

                    // BL or B
                    if PREDICT == PredictionAlgorithms::AlwaysTaken {
                        self.spec_pc = self.spec_pc.wrapping_add(control_offset).wrapping_add(4);
                        continue;
                    } else {
                        self.spec_pc += 4;
                        // if its a BL only fetch 1
                        if control_instruction == IT::BL {
                            return;
                        } else if control_instruction == IT::B {
                            continue;
                        } else {
                            unreachable!()
                        }
                    }
                } else {
                    self.fb[i] = Some((self.spec_pc + pc_increment, fetched));
                }
                self.spec_pc += pc_increment;
            }
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
        if (i & 0b1111_0000_0000_0000) == 0b1101_0000_0000_0000 {
            return if briz(i, 8, 11) == 0b1111 {
                Some((IT::SVC, 0))
            } else {
                let i = decode_b1(i);
                Some((IT::B, i.imms as u32))
            };
        }

        // B (T2)
        if (i & 0b1111_1000_0000_0000) == 0b1110_0000_0000_0000 {
            let i = decode_b2(i);
            return Some((IT::B, i.imms as u32));
        }

        // BX or POP(15)
        if ((i & 0b1111_1111_1000_0000) == 0b0100_0111_0000_0000)
            || (i & 0b1111_1111_0000_0000) == 0b1011_1101_0000_0000
        {
            return Some((IT::BX, 0));
        }

        // BX
        if (i & 0b1111_1111_1000_0000) == 0b0100_0111_1000_0000 {
            return Some((IT::BLX, 0));
        }
        //
        // // Pop (15)
        // if (i & 0b1111_1111_0000_0000) == 0b1011_1101_0000_0000 {
        //     return Some((IT::POP, 0));
        // }

        None
    }
}
