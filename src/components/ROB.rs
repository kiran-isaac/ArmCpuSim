use crate::components::ROB::ROBEntryDest::{AwaitingAddress, Register};
use crate::decode::{I, IT, IT::*};
use crate::model::ASPR;

enum ROBStatus {
    Pending,
    Execute,
    Commit,
    Write,

    /// Instruction had an exception
    Exception(u8),
}

const ROB_ENTRIES: usize = 64;

pub struct ROB {
    queue: circular_buffer,
    head: usize,
    tail: usize,
    op: IT,

    // None means not busy
    // Some(n) means ROB entry n holds the result. 16 arch registers, then N Z C V
    pub register_status: [Option<usize>; 20],
}

enum ROBEntryDest {
    None,
    AwaitingAddress,
    Address(usize),

    /// Boolean for if it updates cspr
    Register(u8, bool),
}

pub struct ROBEntry {
    pub pc: u32,
    pub status: ROBStatus,
    pub value: u32,
    pub dest: ROBEntryDest,
}

impl ROB {
    pub fn new() -> Self {
        Self {
            queue: [const { None }; ROB_ENTRIES],
            head: 0,
            tail: 0,
            register_status: [None; 20],
            op: UNDEFINED,
        }
    }

    pub fn issue_receive(&mut self, i: &I, pc: u32) {
        let rob_dest = match i.it {
            // All ALU instructions that write back to rd, and update CSPR
            ADC | ADDImm | ADDReg | ADDSpImm | AND | BIC | EOR | MOVImm | MOVReg | MVN
            | ORR | REVSH | REV16 | REV | RSB | SBC | ROR | SUBImm  | SUBReg | SXTB
            | SXTH | UXTB | UXTH => Register(i.rd, i.setsflags),

            // All ALU instructions that dont write back, as well as branches and system calls
            // Have none as a destination
            TST | CMPImm | CMN | CMPReg | B | BL | BLX | BX | SVC => ROBEntryDest::None,

            // All store instructions will be pending address calculation
            STRImm | STRReg | STRBImm | STRBReg | STRHImm | STRHReg => AwaitingAddress,

            // All load instructions write back to rt, and do not update CSPR
            // (no clue why they differentiate between rd and rt)
            LDRImm | LDRLit | LDRReg | LDRHImm | LDRHReg | LDRBImm | LDRBReg | LDRSB | LDRSH => {
                Register(i.rt, false)
            }

            // Special case
            LoadPc => Register(15, false),

            _ => panic!("ROB cannot add {:?}", i.rt),
        };

        let new_entry = ROBEntry {
            pc,
            status: ROBStatus::Execute,
            value: 0,
            dest: rob_dest,
        };

        // Should be checked by caller
        if self.is_full() {
            panic!("ROB is full, cannot issue. Should be handled by caller to ROB::issue_recieve");
        }

        // If its gonna write to a register, add this to the register status
        match new_entry.dest {
            Register(rd) => self.register_status[rd as usize] = Some(self.tail),
            _ => {}
        }
        self.queue[self.tail] = Some(new_entry);
        self.increment_tail();
    }

    /// Wrapping increment
    fn increment_tail(&mut self) {
        self.tail += 1;
        if self.tail == ROB_ENTRIES {
            self.tail = 0;
        }
    }

    /// Wrapping increment
    fn increment_head(&mut self) {
        self.head += 1;
        if self.head == ROB_ENTRIES {
            self.head = 0;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        self.head == self.tail + 1
    }
}
