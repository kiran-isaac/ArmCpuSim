use super::*;
use crate::binary::unsigned_to_signed_bitcast;
use crate::components::ROB::ROBStatus::EMPTY;
use crate::decode::IT::{
    STRBImm, STRBReg, STRHImm, STRHReg, STRImm, STRReg, SetPC, B, BL, BLX, BX,
};
use std::process::exit;

impl OoOSpeculative {
    pub(super) fn commit(&mut self) {
        let head = self.rob.get_head().clone();
        if !head.ready || self.rob.is_empty() {
            return;
        }
        if head.halt {
            // r0 is the exit code, must have been committed by now
            self.halt = Some(unsigned_to_signed_bitcast(self.state.regs.gp[0]))
        }

        let predicted_taken = match PREDICT {
            PredictionAlgorithms::AlwaysUntaken => false,
            PredictionAlgorithms::AlwaysTaken => true,
        };

        match head.i.it {
            // Maybe taken
            B => {
                let taken = ((head.branch_target & 1) == 1);
                
                if taken && !predicted_taken {
                    self.spec_pc = head.branch_target - 1;
                    self.flush_on_mispredict();
                }
 
                if !taken && predicted_taken {
                    self.spec_pc = head.pc;
                    self.flush_on_mispredict();
                }
            }

            // Always Taken, so branch is mispredicted in "not taken"
            BL => {
                if predicted_taken == false {
                    self.spec_pc = head.branch_target;
                    self.flush_on_mispredict();
                }
            }

            // Always requires a flush
            BX | BLX | SetPC => {
                self.spec_pc = head.branch_target;
                self.flush_on_mispredict();
            }

            _ => {}
        }

        match head.dest {
            ROBEntryDest::Address(addr) => match head.i.it {
                STRImm | STRReg => {
                    if let Err(e) = self.state.mem.set_word(addr, head.value) {
                        panic!("{:?}: attempt to set halfword at {:08X?}", e, addr)
                    }
                }

                STRHImm | STRHReg => {
                    if let Err(e) = self.state.mem.set_halfword(addr, head.value as u16) {
                        panic!("{:?}: attempt to set halfword at {:08X?}", e, addr)
                    }
                }

                STRBImm | STRBReg => {
                    if let Err(e) = self.state.mem.set_byte(addr, head.value as u8) {
                        panic!("{:?}: attempt to set halfword at {:08X?}", e, addr)
                    }
                }

                _ => unreachable!(),
            },
            ROBEntryDest::AwaitingAddress => unreachable!(),
            ROBEntryDest::Register(rn, _) => {
                self.state.regs.set(rn, head.value);

                self.rob.register_status[rn as usize] = None
            }
            _ => {}
        }

        if head.i.setsflags {
            self.state.regs.apply_aspr_update(&head.asprupdate);
            self.rob
                .wipe_aspr_rob_dependencies_at_head(&head.asprupdate);
        }

        //
        match head.i.it {
            BL | BLX => self.state.regs.set(14, head.pc),
            _ => {}
        }

        self.rob.clear_head_and_increment();
        self.instructions_committed += 1;
    }

    fn flush_on_mispredict(&mut self) {
        self.iq.clear();
        self.fb = None;
        self.to_broadcast.clear();
        self.load_queue.clear();
        self.cdb.clear();
        for flush in self.rob.flush_on_mispredict().iter() {
            let flush = *flush;
            self.rs_alu_shift.flush_entries_corresponding_to_rob(flush);
            self.rs_control.flush_entries_corresponding_to_rob(flush);
            self.rs_ls.flush_entries_corresponding_to_rob(flush);
            self.rs_mul.flush_entries_corresponding_to_rob(flush);
            self.rob.set_status(flush, EMPTY);
        }
    }
}
