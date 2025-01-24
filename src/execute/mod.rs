use crate::{binary::*, system::syscall, ProcessorState, I, IT::*};

pub struct Executor {
    i: Option<I>,
    cycles_remaining: usize,
}

#[allow(unused)]
struct ExecutorPool {
    pool: [Executor; 8],
    scoreboard: [bool; 16],
}

impl Executor {
    pub fn new() -> Self {
        Executor {i: None, cycles_remaining: 0}
    }

    pub fn assign(&mut self, i: I) -> usize {
        self.i = Some(i);
        // Lookup how long an instruction should take
        self.cycles_remaining = 1;

        self.cycles_remaining
    }

    pub fn execute(&self, state: &mut ProcessorState) {
        let i = match self.i {
            None => panic!("Cannot execute: No instruction assigned"),
            Some(i) => i
        };
        
        match i.it {
            UNPREDICTABLE => panic!("Cannot execute unpredictable"),
            UNDEFINED => panic!("Cannot execute undefined"),

            SVC => syscall(&i, state),

            // ADD
            ADC | SBC | ADDReg | ADDImm | ADDSpImm | ADDSpReg | CMN | CMPReg | CMPImm | RSB | SUBImm | SUBReg | SUBSP => {
                let n = match i.it {
                    ADR => state.regs.pc,
                    ADDSpImm | ADDSpReg => state.regs.sp,
                    RSB => !state.regs.get(i.rn),
                    _ => state.regs.get(i.rn),
                };
                let m = match i.it {
                    ADC | ADDReg | ADDSpReg => state.regs.get(i.rm),
                    ADDImm | ADDSpImm | ADR => i.immu,
                    CMPImm => !i.immu,
                    CMPReg | SBC | SUBImm | SUBReg | SUBSP => !state.regs.get(i.rm),
                    RSB => 0,
                    _ => unreachable!(),
                };
                let carry = match i.it {
                    ADC | SBC => state.regs.apsr.c as u8,
                    CMPReg | RSB | SUBImm | SUBReg | SUBSP => 1,
                    _ => 0,
                };

                let (result, carry) = add_with_carry(n as u32, m, carry);

                match i.it {
                    CMN | CMPImm | CMPReg => {}
                    ADC | ADDImm | ADDReg | ADDSpImm | SBC | RSB | SUBImm | SUBReg | SUBSP => state.regs.set(i.rd, result),
                    _ => unreachable!(),
                }

                // If PC is not being written to, update flags
                if i.rd != 15 && i.setflags {
                    state.regs.apsr.n = bit_as_bool(result, 31);
                    state.regs.apsr.z = hamming_weight(result) == 0;
                    state.regs.apsr.c = carry;
                    state.regs.apsr.v = carry;
                }
            }
            MUL => {
                let n = state.regs.get(i.rn);
                let m = state.regs.get(i.rm);
                let result = n.wrapping_mul(m);
                state.regs.set(i.rd, result);
                state.regs.apsr.n = bit_as_bool(result, 31);
                state.regs.apsr.z = hamming_weight(result) == 0;
            }
            AND | BIC | EOR | ORR => {
                let n = state.regs.get(i.rn);
                let m = match i.it {
                    BIC => !state.regs.get(i.rm),
                    AND | EOR | ORR => state.regs.get(i.rm),
                    _ => unreachable!(),
                };
                let result = match i.it {
                    AND | BIC => n & m,
                    EOR => n ^ m,
                    ORR => n | m,
                    _ => unreachable!(),
                };
                state.regs.set(i.rd, result);

                if i.setflags {
                    state.regs.apsr.n = bit_as_bool(result, 31);
                    state.regs.apsr.z = hamming_weight(result) == 0;
                    state.regs.apsr.c = false;
                    // state.regs.apsr.v = false;
                }
            }
            ASRImm | ASRReg | LSLImm | LSLReg | LSRImm | LSRReg | MOVImm | MOVReg | MVN | ROR => {
                let amount = match i.it {
                    ASRReg | LSLReg | LSRReg | ROR => state.regs.get(i.rn),
                    ASRImm | LSLImm | LSRImm | MOVReg => state.regs.get(i.rm),
                    MVN => !state.regs.get(i.rm),
                    MOVImm => i.immu,
                    _ => unreachable!(),
                };
                let shift_n = match i.it {
                    ASRImm | LSRImm | LSLImm => i.immu as u8,
                    ASRReg | LSLReg | LSRReg | ROR => (state.regs.get(i.rm) & 0xff) as u8,
                    MOVImm | MOVReg | MVN => 0,
                    _ => unreachable!(),
                };
                let shift_type = match i.it {
                    ASRImm | ASRReg => ShiftType::ASR,
                    LSLImm | LSLReg | MVN | MOVImm | MOVReg => ShiftType::LSL,
                    LSRImm | LSRReg => ShiftType::LSR,
                    ROR => ShiftType::ROR,
                    _ => unreachable!(),
                };
                let (result, carry) =
                    shift_with_carry(shift_type, amount, shift_n as u8, state.regs.apsr.c as u8);
                state.regs.set(i.rd, result);

                if i.setflags {
                    state.regs.apsr.n = bit_as_bool(result, 31);
                    state.regs.apsr.z = hamming_weight(result) == 0;
                    state.regs.apsr.c = carry;
                    // state.regs.apsr.v = false;
                }
            }
            B => {
                match i.rn {
                    0b1110 | 0b1111 => unimplemented!(),
                    _ => {}
                }
                let offset = i.immu;
                let cond = state.regs.apsr.cond(i.rn);
                if cond {
                    state.regs.pc = state.regs.pc.wrapping_add(offset);
                }
            }
            BL | BLX | BX => {
                let target = match i.it {
                    BL => (state.regs.pc as i64).wrapping_add(i.imms as i64) as u32,
                    BLX => state.regs.get(i.rm),
                    _ => unreachable!(),
                };
                match i.it {
                    BL | BLX => {
                        state.regs.lr = (briz(state.regs.pc, 1, 31) << 2) + 1;
                    }
                    BX => {}
                    _ => unreachable!(),
                }
                state.regs.pc = target;
            }
            LDMIA => {
                let mut wback = true;
                let mut addr = state.regs.get(i.rn);
                for b in 0..7 {
                    if briz(i.rl as u32, b, b) == (i.rn as u32) {
                        wback = false;
                    }
                    state.regs.set(b as u8, state.mem.get_word(addr));
                    addr += 4;
                }
                if wback {
                    state.regs.set(i.rn, 4 * hamming_weight(i.rl as u32));
                }
            }
            STMIA => {
                let mut addr = state.regs.get(i.rn);
                for b in 0..15 {
                    if bit_as_bool(i.rl as u32, b) {
                        state.mem.set_word(addr, state.regs.get(b as u8));
                        addr += 4;
                    }
                }
                state.regs.set(i.rn, 4 * hamming_weight(i.rl as u32));
            }
            LDRImm | LDRReg | LDRLit | LDRBImm | LDRBReg | LDRHReg | LDRHImm | LDRSB | LDRSH => {
                let addr = match i.it {
                    LDRImm | LDRBImm | LDRHImm => state.regs.get(i.rn) + i.immu,
                    LDRLit => state.regs.pc + i.immu,
                    LDRReg | LDRBReg | LDRHReg => state.regs.get(i.rn) + state.regs.get(i.rm),
                    _ => unreachable!(),
                };
                let value = match i.it {
                    LDRImm | LDRReg | LDRLit => state.mem.get_word(addr),
                    LDRHImm | LDRHReg => state.mem.get_halfword(addr) as u32,
                    LDRBImm | LDRBReg => state.mem.get_byte(addr) as u32,
                    LDRSH => state.mem.get_halfword(addr) as i16 as i32 as u32,
                    LDRSB => state.mem.get_byte(addr) as i8 as i32 as u32,
                    _ => unreachable!(),
                };
                state.regs.set(i.rt, value);
            }
            STRImm | STRReg | STRBImm | STRBReg | STRHImm | STRHReg => {
                let addr = match i.it {
                    STRImm | STRBImm | STRHImm => state.regs.get(i.rn) + i.immu,
                    LDRReg | STRBReg | STRHReg => state.regs.get(i.rn) + state.regs.get(i.rm),
                    _ => unreachable!()
                };
                let value = state.regs.get(i.rt);
                match i.it {
                    STRImm | STRReg  => state.mem.set_word(addr, value),
                    STRHImm | STRHReg => state.mem.set_halfword(addr, value as u16),
                    STRBImm | STRBReg => state.mem.set_byte(addr, value as u8),
                    _ => unreachable!(),
                };
            }
            POP => {
                let mut addr = state.regs.sp;
                for b in 0..8 {
                    if bit_as_bool(i.rl as u32, b) {
                        state.regs.set(b as u8, state.mem.get_word(addr));
                        addr += 4;
                    }
                }
                if bit_as_bool(i.rl as u32, 15) {
                    state.regs.pc = state.mem.get_word(addr);
                }
                state.regs.sp = state.regs.sp + 4 * hamming_weight(i.rl as u32)
            }
            PUSH => {
                let mut addr = state.regs.sp - 4 * hamming_weight(i.rl as u32);
                for b in 0..15 {
                    if bit_as_bool( i.rl as u32, b) {
                        state.mem.set_word(addr, state.regs.get(b as u8));
                        addr += 4;
                    }
                }
                state.regs.sp -= 4 * hamming_weight(i.rl as u32); 
            }
            REV => {
                let value = state.regs.get(i.rm);
                let result = (value & 0xff) << 24
                    | (value & 0xff00) << 8
                    | (value & 0xff0000) >> 8
                    | (value & 0xff000000) >> 24;
                state.regs.set(i.rd, result);
            }
            REV16 => {
                let value = state.regs.get(i.rm);
                let result = (value & 0xff) << 8
                    | (value & 0xff00) >> 8
                    | (value & 0xff0000) << 8
                    | (value & 0xff000000) >> 8;
                state.regs.set(i.rd, result);
            }
            REVSH => {
                let value = state.regs.get(i.rm);
                let result = ((value & 0xff) << 8 | (value & 0xff00) >> 8) as i16 as i32 as u32;
                state.regs.set(i.rd, result);
            }
            NOP => {}
            _ => unimplemented!("Instruction execute not implemented: {:?}", i.it),
        }
    }
}
