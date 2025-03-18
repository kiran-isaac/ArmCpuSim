mod decode1;
mod decode2;

pub use decode1::*;
pub use decode2::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum IssueType {
    ALU,
    Shift,
    MUL,
    LoadStore,
    /// Writing to PC, branching, system calls
    Control,
}

use IT::*;

pub fn get_issue_type(it: IT) -> (IssueType) {
    match it {
        ADC | ADDImm | ADDReg | ADDSpImm | AND | BIC
        | CMN | CMPImm | CMPReg | EOR | MOVImm | MOVReg | MVN
        | ORR | REVSH | REV16 | REV | RSB | SBC | ROR | SUBImm
        | SUBReg | SXTB | SXTH | UXTB | UXTH | TST => IssueType::ALU,

        MUL => IssueType::MUL,

        B | LoadPc | BL | BLX | BX | SVC => IssueType::Control,

        STRImm | STRReg | STRBImm | STRBReg | STRHImm | STRHReg | LDRImm | LDRReg | LDRHImm | LDRHReg | LDRBImm | LDRBReg | LDRSB | LDRSH => IssueType::LoadStore,

        ASRImm | ASRReg | LSLImm | LSRImm | LSRReg | LSLReg => IssueType::Shift,

        // fails when hits these instructions, cannot issue
        NOP | DSB | ISB | DMB | BKPT | MRS | MSR | UNDEFINED | UNPREDICTABLE | SEV | WFE | WFI | YIELD  => panic!("Got instruction that shouldn't be in an application level binary, system level instructions not supported"),
        LDMIA | STMIA | POP | PUSH => panic!("Got ciscy instruction {:?} in issue, should have been broken down", it),
    }
}
