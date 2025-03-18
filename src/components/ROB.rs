use crate::components::ROB::ROBEntryDest::{AwaitingAddress, Register};
use crate::decode::{I, IT, IT::*};
use std::collections::VecDeque;

#[derive(Copy, Clone)]
enum ROBStatus {
    EMPTY,
    Pending,
    Execute,
    Commit,
    Write,

    /// Instruction had an exception
    Exception(u8),
}

const ROB_ENTRIES: usize = 64;

pub struct ROB {
    queue: [ROBEntry; ROB_ENTRIES],
    head: usize,
    tail: usize,
    op: IT,

    // None means not busy
    // Some(n) means ROB entry n holds the result. 16 arch registers, then N Z C V
    pub register_status: [Option<usize>; 20],
}

#[derive(Copy, Clone)]
enum ROBEntryDest {
    None,
    AwaitingAddress,
    Address(usize),

    /// Boolean for if it updates cspr
    Register(u8, bool),
}
#[derive(Copy, Clone)]
pub struct ROBEntry {
    pub pc: u32,
    pub status: ROBStatus,
    pub value: u32,
    pub dest: ROBEntryDest,
}

impl ROBEntry {
    fn new() -> Self {
        ROBEntry {
            pc: 0,
            value: 0,
            status: ROBStatus::EMPTY,
            dest: ROBEntryDest::None,
        }
    }
}

impl ROB {
    pub fn new() -> Self {
        Self {
            queue: [ROBEntry::new(); ROB_ENTRIES],
            head: 0,
            tail: 0,
            register_status: [None; 20],
            op: UNDEFINED,
        }
    }

    pub fn issue_receive(&mut self, i: &I, pc: u32) -> usize {
        // Should be checked by caller
        if self.is_full() {
            panic!("ROB is full, cannot issue. Should be handled by caller to ROB::issue_recieve");
        }

        let insert_point = self.tail;

        let rob_dest = match i.it {
            // All ALU instructions (+ mul) that write back to rd, and update CSPR
            ADC | ADDImm | ADDReg | ADDSpImm | AND | BIC | EOR | MOVImm | MOVReg | MVN | ORR
            | REVSH | REV16 | REV | RSB | SBC | ROR | SUBImm | SUBReg | SXTB | SXTH | UXTB
            | UXTH | MUL | LSLImm | LSLReg | LSRReg | LSRImm | ASRReg | ASRImm => {
                Register(i.rd, i.setsflags)
            }

            // All ALU instructions that dont write back, as well as branches and system calls
            // Have none as a destination
            TST | CMPImm | CMN | CMPReg | B | BL | BLX | BX | SVC => ROBEntryDest::None,

            // All store instructions will be pending address calculation
            STRImm | STRReg | STRBImm | STRBReg | STRHImm | STRHReg => AwaitingAddress,

            // All load instructions write back to rt, and do not update CSPR
            // (no clue why they differentiate between rd and rt)
            LDRImm | LDRReg | LDRHImm | LDRHReg | LDRBImm | LDRBReg | LDRSB | LDRSH => {
                Register(i.rt, false)
            }

            // Special case
            LoadPc => Register(15, false),

            _ => panic!("ROB cannot add {:?}", i),
        };
        
        // If its gonna write to a register, add this to the register status
        match rob_dest {
            Register(rd, setsflags) => {
                self.register_status[rd as usize] = Some(insert_point);

                if setsflags {
                    // Get what flags this updates
                    let (n, z, c, v) = match i.it {
                        // Adds and Subtracts: Sets all 4. All the add instructions pretty much
                        ADC | ADDImm | ADDReg | CMN | CMPReg | CMPImm | SUBImm | SUBReg => {
                            (true, true, true, true)
                        }

                        // Shifts: Sets all but v
                        ASRImm | ASRReg | LSLReg | LSLImm | LSRImm | LSRReg | ROR => {
                            (true, true, true, false)
                        }

                        // Logical ops + mul: Sets n, z. Some of these say they
                        // update c in the spec but that's because the dummy shift
                        AND | TST | BIC | MOVImm | MOVReg | MUL | MVN | EOR | ORR => {
                            (true, true, false, false)
                        }

                        _ => panic!("{:?} shouldn't update NZCV flags", i),
                    };
                    // Update NZCV RS flags to point to this as the last update
                    if n {
                        self.register_status[16] = Some(insert_point);
                    }
                    if z {
                        self.register_status[17] = Some(insert_point);
                    }
                    if c {
                        self.register_status[18] = Some(insert_point);
                    }
                    if v {
                        self.register_status[19] = Some(insert_point);
                    }
                }
            }
            _ => {}
        }

        self.queue[insert_point] = ROBEntry {
            pc,
            status: ROBStatus::Execute,
            value: 0,
            dest: rob_dest,
        };
        self.increment_tail();
        insert_point
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
