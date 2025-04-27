use std::cmp::Ordering;
use std::collections::HashSet;
use super::*;
use crate::binary::{bit_as_bool, briz, signed_to_unsigned_bitcast, unsigned_to_signed_bitcast};
use crate::components::shift::{shift_with_carry, ShiftType};
use crate::components::ALU::{ALUOperation, CalcResult, ALU};
use crate::IT::*;

impl<'a> OoOSpeculative<'a> {
    pub(super) fn execute(&mut self) {
        let mut can_go = Vec::with_capacity(N_LS_EXECS);
        for (i, entry) in self.load_queue.iter().enumerate() {
            if self.rob.load_can_go(entry)   {
                can_go.push((i, entry.clone()));
            }
        }
        
        // Sort by ROB entry
        can_go.sort_by(|a, b| {
            if self.rob.entry_is_before(a.1.rob_entry, b.1.rob_entry) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        
        let mut went = HashSet::new();
        for (i, ready_entry) in can_go {
            went.insert(i);
            let lqe_head = ready_entry.clone();
            let load_address = lqe_head.address;
            self.rob
                .set_target_address(lqe_head.rob_entry, lqe_head.address);

            let result = match lqe_head.load_type {
                LDRBImm | LDRBReg => match self.state.mem.get_byte(load_address) {
                    Ok(byte) => Ok(byte as u32),
                    Err(e) => Err(e),
                },
                LDRHReg | LDRHImm => match self.state.mem.get_halfword(load_address) {
                    Ok(byte) => Ok(byte as u32),
                    Err(e) => Err(e),
                },
                LDRImm | LDRReg => self.state.mem.get_word(load_address),
                LDRSB => match self.state.mem.get_byte(load_address) {
                    Ok(byte) => Ok(briz(byte as u32, 0, 6)
                        + (if bit_as_bool(byte as u32, 7) {
                        0x80000000
                    } else {
                        0
                    })),
                    Err(e) => Err(e),
                },
                LDRSH => match self.state.mem.get_halfword(load_address) {
                    Ok(byte) => Ok(briz(byte as u32, 0, 14)
                        + (if bit_as_bool(byte as u32, 15) {
                        0x80000000
                    } else {
                        0
                    })),
                    Err(e) => Err(e),
                },
                _ => unreachable!(),
            };

            let result = match result {
                Ok(result) => result,
                Err(e) => panic!("Memory error {:?}", e),
            };

            // Load store has a delay of 1 cycles on top of the 1 cycle for addr calc
            self.to_broadcast.push((
                1,
                CDBRecord {
                    is_branch_target: false,
                    valid: false,
                    result,
                    aspr_update: ASPRUpdate::no_update(),
                    rob_number: lqe_head.rob_entry,
                    halt: false,
                },
            ));
        }
        
        let mut new_load_queue = VecDeque::new();
        for (i, e) in self.load_queue.iter().enumerate() {
            if !went.contains(&i) {
                new_load_queue.push_back(e.clone());
            }
        }
        self.load_queue = new_load_queue;
        
        for _ in 0..N_ALUSHIFTERS {
            if let Some(rs_index) = self.rs_alu_shift.get_oldest_ready(&self.rob, false) {
                self.execute_alu_shift(&self.rs_alu_shift.vec[rs_index].clone(), &mut 0);
                self.rs_alu_shift.vec[rs_index].busy = false;
            }
        }
        
        for _ in 0..N_MULS {
            if let Some(rs_index) = self.rs_mul.get_oldest_ready(&self.rob, false) {
                self.execute_mul(&self.rs_mul.vec[rs_index].clone(), &mut 0);
                self.rs_mul.vec[rs_index].busy = false;
            }
        }
        
        for _ in 0..N_CONTROL {
            if let Some(rs_index) = self.rs_control.get_oldest_ready(&self.rob, false) {
                self.execute_control(&self.rs_control.vec[rs_index].clone(), &mut 0);
                self.rs_control.vec[rs_index].busy = false;
            }
        }
        
        for _ in 0..N_LS_EXECS {
            // No loads if the queue is full
            let no_loads = self.load_queue.len() >= LQ_SIZE;
            if let Some(rs_index) = self.rs_ls.get_oldest_ready(&self.rob, no_loads) {
                self.execute_load_store(&self.rs_ls.vec[rs_index].clone(), &mut 0);
                self.rs_ls.vec[rs_index].busy = false;
            }
        }
    }

    fn execute_control(&mut self, rs: &RS, num_broadcast: &mut usize) {
        if rs.i.it == SVC {
            let svc_num = Self::get_data(rs.j).unwrap();
            let r0 = Self::get_data(rs.k).unwrap();
            match svc_num {
                0 => {
                    self.to_broadcast.push((
                        1,
                        CDBRecord {
                            is_branch_target: false,
                            valid: false,
                            result: r0,
                            aspr_update: ASPRUpdate::no_update(),
                            rob_number: rs.rob_dest,
                            halt: true,
                        },
                    ));
                    *num_broadcast += 1;
                    return;
                }
                1 => {
                    let mut addr = r0;
                    loop {
                        let c = self.state.mem.get_byte_nolog(addr);
                        if c == 0 {
                            break;
                        }
                        print!("{}", c as char);
                        addr += 1;
                    }
                }
                // 2 => {
                //     // flush incase of output on same line
                //     std::io::stdout().flush().unwrap();
                //     let addr = self.state.regs.get(0) as u32;
                //     let mut i = 0;
                //     loop {
                //         let c = std::io::stdin().bytes().next().unwrap().unwrap();
                //         if c == 10 {
                //             break;
                //         }
                //         state.mem.set_byte_nolog(addr + i, c);
                //         i += 1;
                //     }
                //     // add null terminator
                //     state.mem.set_byte_nolog(addr + i, 0);
                // }
                3 => {
                    let value = r0;
                    print!("{}", value);
                }
                _ => panic!("Invalid svc"),
            }

            self.to_broadcast.push((
                1,
                CDBRecord {
                    is_branch_target: false,
                    valid: false,
                    result: 0,
                    aspr_update: ASPRUpdate::no_update(),
                    rob_number: rs.rob_dest,
                    halt: false,
                },
            ));
            return;
        }
        // BX, BLX and SetPc require RM
        // SetPC, BX and BLX are absolute
        // B and BL are relative, and require an immediate
        let mut target = match rs.i.it {
            SetPC | BX | BLX => Self::get_data(rs.j).unwrap(),
            BL | B => {
                let pc = self.rob.get(rs.rob_dest).pc;
                let offset = rs.i.imms as u32;
                pc.wrapping_add(offset)
            }
            _ => unreachable!(),
        };

        // Not necessary as the PC is already incremented by 4
        match rs.i.it {
            B => target += 2,
            _ => {}
        }

        let taken = match rs.i.it {
            SetPC | BX | BL | BLX => true,
            B => {
                let j = Self::get_data(rs.j).map(|x| x != 0);
                let k = Self::get_data(rs.k).map(|x| x != 0);
                let l = Self::get_data(rs.l).map(|x| x != 0);
                match rs.i.rn {
                    // These all just require one flag that should be in j
                    // EQ | CS |  MI | VS
                    0b0000 | 0b0010 | 0b0100 | 0b0110 => j.unwrap(),
                    // NE | CC | PL | VC
                    0b0001 | 0b0011 | 0b0101 | 0b0111 => !j.unwrap(),
                    // HI
                    0b1000 => k.unwrap() == true && j.unwrap() == false,
                    // LS
                    0b1001 => !(k.unwrap() == true && j.unwrap() == false),
                    // GE
                    0b1010 => j.unwrap() == k.unwrap(),
                    // LT
                    0b1011 => !(j.unwrap() == k.unwrap()),
                    // GT
                    0b1100 => k.unwrap() == false && j.unwrap() == l.unwrap(),
                    // LE
                    0b1101 => !(k.unwrap() == false && j.unwrap() == l.unwrap()),
                    // AL
                    0b1110 => true,
                    // NV
                    0b1111 => false,
                    _ => panic!("Invalid condition code"),
                }
            }
            _ => unreachable!(),
        };

        // since B can only reach an even address, we may use the bottom bit for taken or untaken
        // All the other control instrs are always taken
        if rs.i.it == B {
            assert_eq!(target % 2, 0);
            target += taken as u32;
        }

        self.to_broadcast.push((
            1,
            CDBRecord {
                is_branch_target: true,
                valid: false,
                result: target,
                aspr_update: ASPRUpdate::no_update(),
                rob_number: rs.rob_dest,
                halt: false,
            },
        ));
        *num_broadcast += 1;
    }

    fn execute_load_store(&mut self, rs: &RS, num_broadcast: &mut usize) {
        // Address calc
        let j = Self::get_data(rs.j).unwrap();
        let k = Self::get_data(rs.k).unwrap();

        let address = j.wrapping_add(k);

        match rs.i.it {
            LDRBImm | LDRBReg | LDRHReg | LDRHImm | LDRImm | LDRReg | LDRSB | LDRSH => {
                self.load_queue.push_back(LoadQueueEntry {
                    address,
                    rob_entry: rs.rob_dest,
                    load_type: rs.i.it,
                });
            }
            STRBImm | STRBReg | STRHImm | STRHReg | STRImm | STRReg => {
                self.to_broadcast.push((
                    1,
                    CDBRecord {
                        is_branch_target: false,
                        valid: true,
                        rob_number: rs.rob_dest,
                        result: address,
                        aspr_update: ASPRUpdate::no_update(),
                        halt: false,
                    },
                ));
                // L has the actual data to be stored
                self.rob
                    .set_value(rs.rob_dest, Self::get_data(rs.l).unwrap())
            }
            _ => unreachable!(),
        }
    }

    fn execute_mul(&mut self, rs: &RS, num_broadcast: &mut usize) {
        let j = unsigned_to_signed_bitcast(Self::get_data(rs.j).unwrap());
        let k = unsigned_to_signed_bitcast(Self::get_data(rs.k).unwrap());

        assert_eq!(rs.i.it, MUL);

        let (result, _) = j.overflowing_mul(k);
        let aspr_update = ASPRUpdate {
            n: Some(result < 1),
            z: Some(result == 0),
            c: None,
            v: None,
        };
        let result = signed_to_unsigned_bitcast(result);

        // Multiplier has a delay of 2 cycles
        self.to_broadcast.push((
            2,
            CDBRecord {
                is_branch_target: false,
                valid: false,
                result,
                aspr_update,
                rob_number: rs.rob_dest,
                halt: false,
            },
        ));
        *num_broadcast += 1;
    }

    fn execute_alu_shift(&mut self, rs: &RS, num_broadcast: &mut usize) {
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
            ADC => (
                ALU_Shift::ALU_OP(ALUOperation::ADD),
                j.unwrap(),
                k.unwrap(),
                l.unwrap(),
            ),
            SBC => (
                ALU_Shift::ALU_OP(ALUOperation::ADD),
                j.unwrap(),
                !k.unwrap(),
                l.unwrap(),
            ),
            // All the adds that dont require aspr c
            ADDReg | ADDImm | ADDSpImm | CMN => (
                ALU_Shift::ALU_OP(ALUOperation::ADD),
                j.unwrap(),
                k.unwrap(),
                0,
            ),

            // All the subs that dont require aspr c
            SUBReg | SUBImm | CMPImm | CMPReg => (
                ALU_Shift::ALU_OP(ALUOperation::ADD),
                j.unwrap(),
                !k.unwrap(),
                1,
            ),
            RSB => (
                ALU_Shift::ALU_OP(ALUOperation::ADD),
                !j.unwrap(),
                k.unwrap(),
                1,
            ),

            AND | TST => (
                ALU_Shift::ALU_OP(ALUOperation::AND),
                j.unwrap(),
                k.unwrap(),
                0,
            ),
            BIC => (
                ALU_Shift::ALU_OP(ALUOperation::AND),
                j.unwrap(),
                !k.unwrap(),
                0,
            ),
            ORR => (
                ALU_Shift::ALU_OP(ALUOperation::OR),
                j.unwrap(),
                !k.unwrap(),
                0,
            ),
            EOR => (
                ALU_Shift::ALU_OP(ALUOperation::EOR),
                j.unwrap(),
                !k.unwrap(),
                0,
            ),

            // Rev (unary)
            REV => (ALU_Shift::ALU_OP(ALUOperation::REV), j.unwrap(), 0, 0),
            REV16 => (ALU_Shift::ALU_OP(ALUOperation::REV16), j.unwrap(), 0, 0),
            REVSH => (ALU_Shift::ALU_OP(ALUOperation::REVSH), j.unwrap(), 0, 0),

            // Mov adds 0
            MOVReg | MOVImm => (
                ALU_Shift::ALU_OP(ALUOperation::AND),
                j.unwrap(),
                j.unwrap(),
                0,
            ),
            MVN => (ALU_Shift::ALU_OP(ALUOperation::ADD), !j.unwrap(), 0, 0),

            // The shifts all take ASPR C
            ASRReg | ASRImm => (
                ALU_Shift::SHIFT_OP(ShiftType::ASR),
                j.unwrap(),
                k.unwrap(),
                l.unwrap(),
            ),
            LSLImm | LSLReg => (
                ALU_Shift::SHIFT_OP(ShiftType::LSL),
                j.unwrap(),
                k.unwrap(),
                l.unwrap(),
            ),
            LSRReg | LSRImm => (
                ALU_Shift::SHIFT_OP(ShiftType::LSR),
                j.unwrap(),
                k.unwrap(),
                l.unwrap(),
            ),
            ROR => (
                ALU_Shift::SHIFT_OP(ShiftType::ROR),
                j.unwrap(),
                k.unwrap(),
                l.unwrap(),
            ),

            SXTB => (ALU_Shift::ALU_OP(ALUOperation::SXTB), j.unwrap(), 0, 0),
            SXTH => (ALU_Shift::ALU_OP(ALUOperation::SXTH), j.unwrap(), 0, 0),
            UXTB => (ALU_Shift::ALU_OP(ALUOperation::UXTB), j.unwrap(), 0, 0),
            UXTH => (ALU_Shift::ALU_OP(ALUOperation::UXTH), j.unwrap(), 0, 0),

            NOP => (ALU_Shift::ALU_OP(ALUOperation::AND), 0, 0, 0),

            _ => unreachable!("{:?}", rs.i.it),
        };

        let CalcResult {
            delay,
            result,
            aspr_update,
        } = match op {
            ALU_Shift::ALU_OP(op) => ALU(op, n, m, c != 0),
            ALU_Shift::SHIFT_OP(op) => shift_with_carry(op, n, m as u8, c as u8),
        };
        // The delay should always be 1 for this bit
        assert_eq!(delay, 1);

        self.to_broadcast.push((
            delay,
            CDBRecord {
                is_branch_target: false,
                valid: false,
                result,
                aspr_update,
                rob_number: rs.rob_dest,
                halt: false,
            },
        ));
        *num_broadcast += 1;
    }

    fn get_data(x: RSData) -> Option<u32> {
        if let RSData::Data(n) = x {
            Some(n)
        } else {
            None
        }
    }
}
