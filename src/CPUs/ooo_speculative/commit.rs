use crate::decode::IT::{STRBImm, STRBReg, STRHImm, STRHReg, STRImm, STRReg, SetPC, B, BL, BLX, BX};
use super::*;

impl OoOSpeculative{
    pub(super) fn commit(&mut self) {
        let head = self.rob.get_head();
        if !head.ready {return;}
        
        match head.i.it {
            // Maybe taken
            B => {

            }

            // Always Taken, so branch is mispredicted in "not taken"
            BL | BX | BLX | SetPC => {
                
            }
            
            _ => {}
        }
        
        match head.dest {
            ROBEntryDest::Address(addr) => {
                match head.i.it {
                    STRImm | STRReg => {
                        self.state.mem.set_byte(addr, head.value as u8).unwrap()
                    }
                        
                    STRHImm | STRHReg => {
                        self.state.mem.set_halfword(addr, head.value as u16).unwrap();
                    }
                    
                    STRBImm | STRBReg => {
                        self.state.mem.set_word(addr, head.value).unwrap();
                    }
                    
                    _ => unreachable!()
                }
            }
            ROBEntryDest::AwaitingAddress => unreachable!(),
            ROBEntryDest::Register(rn, aspr_update) => {
                self.state.regs.set(rn, head.value);

                if aspr_update {
                    self.state.regs.apply_aspr_update(&head.asprupdate)
                }
                
                self.rob.register_status[rn as usize] = None
            }
            _ => {}
        }

        self.rob.clear_head_and_increment()
    }
}
