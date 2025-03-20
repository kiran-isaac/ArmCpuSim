use std::cmp::min;
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

    // We don't know whether the res stations have space for this yet so put in temp buffers
    will_issue: ROBEntry,
    temp_register_status: [Option<usize>; 20],
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
    pub i: I,
    status: ROBStatus,
    pub value: u32,
    dest: ROBEntryDest,
}

impl ROBEntry {
    fn new() -> Self {
        ROBEntry {
            pc: 0,
            value: 0,
            status: ROBStatus::EMPTY,
            dest: ROBEntryDest::None,
            i: I::undefined(),
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

            will_issue: ROBEntry::new(),
            temp_register_status: [None; 20],
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
            TST | CMPImm | CMN | CMPReg | B | BL | BLX | BX | SVC | NOP => ROBEntryDest::None,

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
        self.temp_register_status = self.register_status.clone();
        match rob_dest {
            Register(rd, setsflags) => {
                self.temp_register_status[rd as usize] = Some(insert_point);

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
                        self.temp_register_status[16] = Some(insert_point);
                    }
                    if z {
                        self.temp_register_status[17] = Some(insert_point);
                    }
                    if c {
                        self.temp_register_status[18] = Some(insert_point);
                    }
                    if v {
                        self.temp_register_status[19] = Some(insert_point);
                    }
                }
            }
            _ => {}
        }

        self.will_issue = ROBEntry {
            pc,
            status: ROBStatus::Execute,
            value: 0,
            i: i.clone(),
            dest: rob_dest,
        };

        // Return where it would go upon commit
        insert_point
    }

    pub fn get(&self, n: usize) -> &ROBEntry {
        &self.queue[n]
    }

    pub fn issue_commit(&mut self) {
        self.queue[self.tail] = self.will_issue;
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
    
    pub fn get_first_entry(&self, e1: usize, e2: usize) -> usize {
        #[cfg(debug_assertions)]
        assert!(self.head < ROB_ENTRIES);
        
        let mut e1s = e1 as i32 - self.head as i32;
        let mut e2s = e2 as i32 - self.head as i32;
        if e1s < 0 {e1s += 64};
        if e2s < 0 {e2s += 64};
        
        if e1s < e2s {e1} else {e2}
    }
}

#[cfg(test)]
mod ROBTests {
    use super::*;

    #[test]
    fn get_first_entry() {
        let mut rob = ROB::new();
        rob.head = 10;
        
        // if the head is at 10 then 9 is much later than 10
        assert_eq!(rob.get_first_entry(9, 10), 10);
        assert_eq!(rob.get_first_entry(10, 11), 10);
        
        rob.head = 0;
        assert_eq!(rob.get_first_entry(0, 63), 0);

        rob.head = 63;
        assert_eq!(rob.get_first_entry(9, 10), 9);
        assert_eq!(rob.get_first_entry(62, 0), 0);
    }
}