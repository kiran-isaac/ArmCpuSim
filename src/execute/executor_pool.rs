use super::*;

// mostly the same as cortex m0 delays
fn i_len_lookup(i: &I) -> usize {
    match i.it {
        // add/shift, 3 if pc relative
        ADDReg | MOVReg => {
            if i.rd == 15 {
                3
            } else {
                1
            }
        }
        // add/shift: 1
        ADC | ADDImm | ADDSpImm | ADR | AND | ASRImm | ASRReg | LSLImm | LSLReg | LSRImm
        | LSRReg | BIC | CMPReg | CMPImm | CMN | EOR | MOVImm | MVN | NOP | ORR | REV | REV16
        | REVSH | ROR | SBC | RSB | SUBImm | SUBReg | SUBSP | SXTB | SXTH | TST | UXTB | UXTH => 1,

        // load/store: 2
        LDRBImm | LDRHImm | LDRBReg | LDRHReg | LDRImm | LDRLit | LDRReg | LDRSB | LDRSH
        | STRBImm | STRBReg | STRHImm | STRHReg | STRImm | STRReg => 2,
        // Load/stoe multiple: 1 + number of load/stores
        LDMIA | STMIA | PUSH | POP => 1 + (hamming_weight(i.rl as u32) as usize),

        MUL => 2,

        B | BLX | BX => 3,

        BL | DMB | DSB | ISB | MRS | MSR | SVC => 4,

        WFI | WFE | YIELD | BKPT | SEV | UNDEFINED | UNPREDICTABLE => {
            unimplemented!("timings for {:?}", i.it)
        }
    }
}

impl Executor {
    fn assign(&mut self, i: I, is_32_bit: bool) {
        self.i = Some(i);
        // Lookup how long an instruction should take
        self.cycles_remaining = i_len_lookup(&i);
        
        self.is_32_bit = is_32_bit;
    }
}

impl ExecutorPool {
    pub fn assign(&mut self, i: I, is_32_bit: bool) -> bool {
        if self.pool[0].i.is_none() {
            self.pool[0].assign(i, is_32_bit);
            return true;
        }
        false
    }

    pub fn tick(&mut self, state: &mut ProcessorState, event_log: &mut String) {
        for executor in self.pool.iter_mut() {
            if executor.i.is_some() {
                if executor.cycles_remaining == 0 {
                    executor.execute_instruction(state, event_log);
                } else {
                    executor.cycles_remaining -= 1;
                }
            }
        }
    }
}
