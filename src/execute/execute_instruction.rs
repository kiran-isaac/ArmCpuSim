use super::*;

impl Executor {
    pub fn execute_instruction(&mut self, state: &mut ProcessorState, event_log: &mut String) {
        let i = match self.i {
            None => panic!("Cannot execute: No instruction assigned"),
            Some(i) => i,
        };

        self.i = None;

        match i.it {
            UNPREDICTABLE => panic!("Cannot execute unpredictable"),
            UNDEFINED => panic!("Cannot execute undefined"),

            SVC => syscall(&i, state),

            // ADD
            ADC | SBC | ADDReg | ADDImm | ADDSpImm | CMN | CMPReg | CMPImm | RSB | SUBImm
            | SUBReg | SUBSP => {
                let n = match i.it {
                    ADR => state.regs.pc,
                    ADDSpImm | SUBSP => state.regs.sp,
                    RSB => !state.regs.get(i.rn),
                    _ => state.regs.get(i.rn),
                };
                let m = match i.it {
                    ADC | ADDReg => state.regs.get(i.rm),
                    ADDImm | ADDSpImm | ADR => i.immu,
                    CMPImm | SUBSP => !i.immu,
                    CMPReg | SBC | SUBImm | SUBReg => !state.regs.get(i.rm),
                    RSB => {
                        #[cfg(debug_assertions)]
                        assert_eq!(i.immu, 0);
                        i.immu
                    }
                    _ => unreachable!(),
                };
                let carry = match i.it {
                    ADC | SBC => state.regs.apsr.c as u8,
                    CMPReg | CMPImm | RSB | SUBImm | SUBReg | SUBSP => 1,
                    _ => 0,
                };

                let (result, carry, overflow) = add_with_carry(n as u32, m, carry);

                match i.it {
                    CMN | CMPImm | CMPReg => {}
                    ADC | ADDImm | ADDReg | ADDSpImm | SBC | RSB | SUBImm | SUBReg | SUBSP => {
                        state.regs.set(i.rd, result)
                    }
                    _ => unreachable!(),
                }

                // If PC is not being written to, update flags
                if i.rd != 15 && i.setflags {
                    state.regs.apsr.n = bit_as_bool(result, 31);
                    state.regs.apsr.z = hamming_weight(result) == 0;
                    state.regs.apsr.c = carry == 1;
                    match i.it {
                        ADC | SBC | RSB | CMPReg | CMPImm | CMN => {
                            state.regs.apsr.v = overflow == 1;
                        }
                        _ => {}
                    }
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
            MUL => {
                let n = state.regs.get(i.rn);
                let m = state.regs.get(i.rm);
                let result = n.wrapping_mul(m);
                state.regs.set(i.rd, result);
                state.regs.apsr.n = bit_as_bool(result, 31);
                state.regs.apsr.z = hamming_weight(result) == 0;
            }
            AND | BIC | EOR | ORR | TST => {
                let n = state.regs.get(i.rn);

                let m = match i.it {
                    BIC => !state.regs.get(i.rm),
                    AND | EOR | ORR | TST => state.regs.get(i.rm),
                    _ => unreachable!(),
                };
                let result = match i.it {
                    AND | BIC | TST => n & m,
                    EOR => n ^ m,
                    ORR => n | m,
                    _ => unreachable!(),
                };

                match i.it {
                    TST => {}
                    _ => state.regs.set(i.rd, result),
                }

                if i.setflags {
                    state.regs.apsr.n = bit_as_bool(result, 31);
                    state.regs.apsr.z = hamming_weight(result) == 0;
                    state.regs.apsr.c = false;
                    // state.regs.apsr.v = false;
                }
            }
            B | BL | BLX | BX => {
                let pc_value = state.regs.pc + 4;

                let target = match i.it {
                    BL => (pc_value as i64).wrapping_add(i.imms as i64) as u32,
                    BLX | BX => state.regs.get(i.rm),
                    B => {
                        if state.regs.apsr.cond(i.rn) {
                            (pc_value as i64).wrapping_add(i.imms as i64) as u32
                        } else {
                            state.regs.pc + 2
                        }
                    }
                    _ => unreachable!(),
                };

                // get if branching to a function
                let func = state.mem.get_function_at(target + 1);
                if func.is_some() {
                    // debug trap, faster than conditional breakpoint
                    #[cfg(debug_assertions)]
                    if func.unwrap() == "quick_sort" {
                        print!("")
                    }

                    write!(
                        event_log,
                        "Calling: {:?}({:#08X}, {:#08X}, {:#08X}, {:#08X})",
                        func.unwrap(),
                        state.regs.get(0),
                        state.regs.get(1),
                        state.regs.get(2),
                        state.regs.get(3)
                    )
                    .unwrap();
                    match i.it {
                        BL | BLX => {
                            write!(event_log, " Link: {:#08X}", pc_value).unwrap();
                        }
                        _ => {}
                    }
                    writeln!(event_log).unwrap();
                }

                match i.it {
                    BL => {
                        state.regs.lr = (pc_value & 0xfffffffe) + 1;
                    }
                    BLX => {
                        state.regs.lr = ((pc_value - 2) & 0xfffffffe) + 1;
                    }
                    B | BX => {}
                    _ => unreachable!(),
                }
                state.regs.pc = target & 0xfffffffe;
            }
            LDMIA => {
                let mut wback = true;
                let mut addr = state.regs.get(i.rn);
                #[allow(unused)]
                let info_str = "";
                #[cfg(debug_assertions)]
                let info_str = format!("I:LDMIA,PC:{:#X}", state.regs.pc);
                for b in 0..7 {
                    if bit_as_bool(i.rl as u32, b) {
                        if b as u8 == i.rn {
                            wback = false;
                        }
                        state
                            .regs
                            .set(b as u8, state.mem.get_word(addr, &info_str, event_log));
                        addr = addr.wrapping_add(4);
                    }
                }
                if wback {
                    state.regs.set(
                        i.rn,
                        state
                            .regs
                            .get(i.rn)
                            .wrapping_add(4 * hamming_weight(i.rl as u32)),
                    );
                }
            }
            STMIA => {
                let mut addr = state.regs.get(i.rn);
                #[allow(unused)]
                let info_str = "";
                #[cfg(debug_assertions)]
                let info_str = format!("I:STMIA,PC:{:#X}", state.regs.pc);
                for b in 0..15 {
                    if bit_as_bool(i.rl as u32, b) {
                        state
                            .mem
                            .set_word(addr, state.regs.get(b as u8), &info_str, event_log);
                        addr = addr.wrapping_add(4);
                    }
                }
                state
                    .regs
                    .set(i.rn, state.regs.get(i.rn) + 4 * hamming_weight(i.rl as u32));
            }
            LDRImm | LDRReg | LDRLit | LDRBImm | LDRBReg | LDRHReg | LDRHImm | LDRSB | LDRSH => {
                let addr = match i.it {
                    LDRImm | LDRBImm | LDRHImm => state.regs.get(i.rn).wrapping_add(i.immu),
                    LDRLit => ((state.regs.pc >> 2) << 2)
                        .wrapping_add(i.immu)
                        .wrapping_add(4),
                    LDRReg | LDRBReg | LDRHReg => {
                        state.regs.get(i.rn).wrapping_add(state.regs.get(i.rm))
                    }
                    _ => unreachable!(),
                };
                #[allow(unused)]
                let info_str = "";

                #[cfg(debug_assertions)]
                let info_str = &format!("I:{:?},PC:{:#X}", i.it, state.regs.pc);
                let value = match i.it {
                    LDRImm | LDRReg | LDRLit => state.mem.get_word(addr, info_str, event_log),
                    LDRHImm | LDRHReg => state.mem.get_halfword(addr, info_str, event_log) as u32,
                    LDRBImm | LDRBReg => state.mem.get_byte(addr, info_str, event_log) as u32,
                    LDRSH => state.mem.get_halfword(addr, info_str, event_log) as i16 as i32 as u32,
                    LDRSB => state.mem.get_byte(addr, info_str, event_log) as i8 as i32 as u32,
                    _ => unreachable!(),
                };
                state.regs.set(i.rt, value);
            }
            STRImm | STRReg | STRBImm | STRBReg | STRHImm | STRHReg => {
                let addr = match i.it {
                    STRImm | STRBImm | STRHImm => state.regs.get(i.rn).wrapping_add(i.immu),
                    STRReg | STRBReg | STRHReg => {
                        let n = state.regs.get(i.rn);
                        let offset = state.regs.get(i.rm);
                        n.wrapping_add(offset)
                    }
                    _ => unreachable!(),
                };
                let value = state.regs.get(i.rt);
                match i.it {
                    STRImm | STRReg => state.mem.set_word(
                        addr,
                        value,
                        &format!("I:{:?},PC:{:#X}", i.it, state.regs.pc),
                        event_log,
                    ),
                    STRHImm | STRHReg => state.mem.set_halfword(
                        addr,
                        value as u16,
                        &format!("I:{:?},PC:{:#X}", i.it, state.regs.pc),
                        event_log,
                    ),
                    STRBImm | STRBReg => state.mem.set_byte(
                        addr,
                        value as u8,
                        &format!("I:{:?},PC:{:#X}", i.it, state.regs.pc),
                        event_log,
                    ),
                    _ => unreachable!(),
                };
            }
            POP => {
                let mut addr = state.regs.sp;
                #[allow(unused)]
                let info_str = "";
                #[cfg(debug_assertions)]
                let info_str = format!("I:POP,PC:{:#X}", state.regs.pc);
                for b in 0..8 {
                    if bit_as_bool(i.rl as u32, b) {
                        state
                            .regs
                            .set(b as u8, state.mem.get_word(addr, &info_str, event_log));
                        addr = addr.wrapping_add(4);
                    }
                }
                if bit_as_bool(i.rl as u32, 15) {
                    // -1 to align + -2 to cancel out the pc increment
                    state.regs.pc = state
                        .mem
                        .get_word(
                            addr,
                            &format!("I:POPPING_PC,PC:{:#X}", state.regs.pc),
                            event_log,
                        )
                        .wrapping_sub(3);
                }
                state.regs.sp = state.regs.sp.wrapping_add(4 * hamming_weight(i.rl as u32))
            }
            PUSH => {
                let mut addr = state.regs.sp - 4 * hamming_weight(i.rl as u32);
                #[allow(unused)]
                let info_str = "";
                #[cfg(debug_assertions)]
                let info_str = format!("I:PUSH,PC:{:#X}", state.regs.pc);
                for b in 0..8 {
                    if bit_as_bool(i.rl as u32, b) {
                        state
                            .mem
                            .set_word(addr, state.regs.get(b as u8), &info_str, event_log);
                        addr = addr.wrapping_add(4);
                    }
                }
                // LR
                if bit_as_bool(i.rl as u32, 14) {
                    state
                        .mem
                        .set_word(addr, state.regs.get(14), &info_str, event_log);
                }

                state.regs.sp = state.regs.sp.wrapping_sub(4 * hamming_weight(i.rl as u32));
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
            UXTH | UXTB => {
                let value = state.regs.get(i.rm);
                let result = match i.it {
                    UXTH => (value & 0xffff) as u32,
                    UXTB => (value & 0xff) as u32,
                    _ => unreachable!(),
                };
                state.regs.set(i.rd, result);
            }
            _ => unimplemented!("Instruction execute not implemented: {:?}", i.it),
        }

        // increment pc
        match i.it {
            BL | BLX | B | BX => {}
            _ => state.regs.pc += if self.is_32_bit { 4 } else { 2 },
        }
    }
}
