use crate::components::ALU::{ALUOperation, ALU};
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
            
            let (op, n, m, c) = match rs.i.it {
                ADC => (ALUOperation::ADD, j.unwrap(), k.unwrap(), l.unwrap()),
                SBC => (ALUOperation::ADD, j.unwrap(), !k.unwrap(), l.unwrap()),
                
                // Add
                ADDReg | ADDImm | ADDSpImm => (ALUOperation::ADD, j.unwrap(), k.unwrap(), 0),
                
                _ => unreachable!()
            };
            
            let (result, aspr_update) = ALU(op, n, m, c != 0);
        };
    }
}
