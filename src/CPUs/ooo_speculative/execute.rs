use crate::binary::{signed_to_unsigned_bitcast, unsigned_to_signed_bitcast};
use crate::components::ALU::{ALUOperation, CalcResult, ALU};
use crate::components::shift::{shift_with_carry, ShiftType};
use super::*;
use crate::IT::*;
use crate::model::Memory;

impl OoOSpeculative {
    pub(super) fn execute(&mut self) {
        if let Some(rs) = self.rs_alu_shift.get_one_ready() {
            self.execute_alu_shift(&rs.clone());
        }

        if let Some(rs) = self.rs_mul.get_one_ready() {
            self.execute_alu_shift(&rs.clone());
        }
        
        for rs in self.rs_ls.get_all_ready(&self.rob) {
            
        }
    }
    
    fn execute_mul(&mut self, rs: &RS) {
        let j = unsigned_to_signed_bitcast(Self::get_data(rs.j).unwrap());
        let k = unsigned_to_signed_bitcast(Self::get_data(rs.k).unwrap());
        
        assert_eq!(rs.i.it, MUL);
        
        let result = j * k;
        let aspr_update = ASPRUpdate {
            n: Some(result < 1),
            z: Some(result == 0),
            c: None,
            v: None,
        };
        let result = signed_to_unsigned_bitcast(j * k);
        
        // Multiplier has a delay of 2 cycles
        self.to_broadcast.push((2, CDBRecord {
            valid: false,
            result,
            aspr_update,
            rob_number: rs.rob_dest
        }))
    }
    
    fn execute_load_store(&mut self, rs: &RS) {
        
    }

    fn execute_alu_shift(&mut self, rs: &RS) {
        let j = Self::get_data(rs.j);
        let k = Self::get_data(rs.k);
        let l = Self::get_data(rs.l);

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

        let CalcResult {delay, result, aspr_update} = match op {
            ALU_Shift::ALU_OP(op) => ALU(op, n, m, c != 0),
            ALU_Shift::SHIFT_OP(op) => shift_with_carry(op, n, m as u8, c as u8),
        };
        // The delay should always be 1 for this bit
        assert_eq!(delay, 1);

        self.to_broadcast.push((delay, CDBRecord {
            valid: false,
            result,
            aspr_update,
            rob_number: rs.rob_dest
        }))
    }

    fn get_data(x: RSData) -> Option<u32> {
        if let RSData::Data(n) = x {
            Some(n)
        } else {
            None
        }
    }
}
