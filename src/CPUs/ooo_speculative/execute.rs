use crate::components::ALU::{ALUOperation, ALU};
use crate::components::shift::ShiftType;
use super::*;
use crate::IT::*;

impl OoOSpeculative {
    pub(super) fn execute(&mut self) {
        // Wipe cdb at beginning of execute. As we are executing backwards this means that 
        self.wipe_cdb();

        let alu_result = if let Some(rs) = self.rs_alu_shift.get_ready() {
            fn get_data(x: RSData) -> Option<u32> {
                if let RSData::Data(n) = x {
                    Some(n)
                } else {
                    None
                }
            }
            let j = get_data(rs.j);
            let k = get_data(rs.k);
            let l = get_data(rs.l);
            
            // Whether the alu or shift functionality should be used, as this res station works for both
            enum ALU_Shift {
                ALU_OP(ALUOperation),
                SHIFT_OP(ShiftType),
            }
            
            let (op, n, m, c) = match rs.i.it {
                // j = arg 1
                // k = arg 2
                // l = aspr carry
                ADC => (ALU_Shift::ALU_OP(ALUOperation::ADD), j.unwrap(), k.unwrap(), l.unwrap()),
                SBC => (ALU_Shift::ALU_OP(ALUOperation::ADD), j.unwrap(), !k.unwrap(), l.unwrap()),
                // All the adds that dont require aspr c
                ADDReg | ADDImm | ADDSpImm | CMN => (ALU_Shift::ALU_OP(ALUOperation::ADD), j.unwrap(), k.unwrap(), 0),
                
                // All the subs that dont require aspr c
                SUBReg | SUBImm | CMPImm | CMPReg => (ALU_Shift::ALU_OP(ALUOperation::ADD), j.unwrap(), !k.unwrap(), 1),
                RSB => (ALU_Shift::ALU_OP(ALUOperation::ADD), !j.unwrap(), k.unwrap(), 1),
                
                AND | TST => (ALU_Shift::ALU_OP(ALUOperation::AND), j.unwrap(), k.unwrap(), 0),
                ORR => (ALU_Shift::ALU_OP(ALUOperation::OR), j.unwrap(), !k.unwrap(), 0),
                EOR => (ALU_Shift::ALU_OP(ALUOperation::EOR), j.unwrap(), !k.unwrap(), 0),
                
                // Rev (unary)
                REV => (ALU_Shift::ALU_OP(ALUOperation::REV), j.unwrap(), 0, 0),
                REV16 => (ALU_Shift::ALU_OP(ALUOperation::REV16), j.unwrap(), 0, 0),
                REVSH => (ALU_Shift::ALU_OP(ALUOperation::REVSH), j.unwrap(), 0, 0),

                // Mov adds 0
                MOVReg | MOVImm => (ALU_Shift::ALU_OP(ALUOperation::ADD), j.unwrap(), 0, 0),
                MVN => (ALU_Shift::ALU_OP(ALUOperation::ADD), !j.unwrap(), 0, 0),
                
                // The shifts all take ASPR C
                ASRReg | ASRImm => (ALU_Shift::SHIFT_OP(ShiftType::ASR), j.unwrap(), k.unwrap(), l.unwrap()),
                LSLImm | LSLReg => (ALU_Shift::SHIFT_OP(ShiftType::LSL), j.unwrap(), k.unwrap(), l.unwrap()),
                LSRReg | LSRImm => (ALU_Shift::SHIFT_OP(ShiftType::LSR), j.unwrap(), k.unwrap(), l.unwrap()),
                ROR => (ALU_Shift::SHIFT_OP(ShiftType::ROR), j.unwrap(), k.unwrap(), l.unwrap()),
                
                SXTB => (ALU_Shift::ALU_OP(ALUOperation::SXTB), j.unwrap(), 0, 0),
                SXTH => (ALU_Shift::ALU_OP(ALUOperation::SXTH), j.unwrap(), 0, 0),
                UXTB => (ALU_Shift::ALU_OP(ALUOperation::UXTB), j.unwrap(), 0, 0),
                UXTH => (ALU_Shift::ALU_OP(ALUOperation::UXTH), j.unwrap(), 0, 0),
                
                NOP => (ALU_Shift::ALU_OP(ALUOperation::AND), 0, 0, 0),
                
                _ => unreachable!(),
            };
            
            let (result, aspr_update) = ALU(op, n, m, c != 0);
        };
    }
}
