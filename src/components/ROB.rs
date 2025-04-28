use crate::components::ALU::ASPRUpdate;
use crate::components::ROB::ROBStatus::EMPTY;
use crate::decode::{I, IT, IT::*};
use crate::model::Registers;
use crate::CPUs::{LoadQueueEntry, ROB_ENTRIES, STALL_ON_BRANCH};
use std::fmt::Formatter;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ROBStatus {
    EMPTY,
    Pending,
    Execute,
    Commit,
    Write,

    /// Instruction had an exception
    Exception(u8),
}

impl std::fmt::Display for ROBStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ROBStatus::EMPTY => write!(f, "__"),
            ROBStatus::Execute => write!(f, "EX"),
            ROBStatus::Pending => write!(f, "PN"),
            ROBStatus::Commit => write!(f, "CO"),
            ROBStatus::Write => write!(f, "WB"),
            ROBStatus::Exception(v) => write!(f, "V{}", v),
        }
    }
}

pub struct ROB {
    queue: [ROBEntry; ROB_ENTRIES],
    pub head: usize,
    pub tail: usize,

    // None means not busy
    // Some(n) means ROB entry n holds the result. 16 arch registers, then N Z C V
    pub register_status: [Option<usize>; 20],

    // We don't know whether the res stations have space for this yet so put in temp buffers
    will_issue: ROBEntry,
    temp_register_status: [Option<usize>; 20],
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ROBEntryDest {
    None,
    AwaitingAddress,
    Address(u32),

    /// Boolean for if it updates cspr
    Register(u8, bool),
}
#[derive(Copy, Clone)]
pub struct ROBEntry {
    pub pc: u32,
    pub halt: bool,
    pub i: I,
    pub status: ROBStatus,
    pub value: u32,
    pub target_address: u32,
    pub asprupdate: ASPRUpdate,
    pub ready: bool,
    pub dest: ROBEntryDest,
}

impl ROBEntry {
    fn new() -> Self {
        ROBEntry {
            pc: 0,
            value: 0,
            target_address: 0,
            halt: false,
            status: ROBStatus::EMPTY,
            dest: ROBEntryDest::None,
            i: I::undefined(),
            asprupdate: ASPRUpdate::no_update(),
            ready: false,
        }
    }

    pub fn is_serializing(&self) -> bool {
        self.i.it.is_serializing()
    }
}

impl ROB {
    // Only wipe aspr if the thing currently committing is the thing that is currently responsible for
    // the register_status entry
    pub fn wipe_aspr_rob_dependencies_at_head(&mut self, asprupdate: &ASPRUpdate) {
        match self.register_status[16] {
            Some(n) => {
                if n == self.head {
                    self.register_status[16] = None
                }
            }
            _ => {}
        }
        match self.register_status[17] {
            Some(n) => {
                if n == self.head {
                    self.register_status[17] = None
                }
            }
            _ => {}
        }
        match self.register_status[18] {
            Some(n) => {
                if n == self.head {
                    self.register_status[18] = None
                }
            }
            _ => {}
        }
        match self.register_status[19] {
            Some(n) => {
                if n == self.head {
                    self.register_status[19] = None
                }
            }
            _ => {}
        }
    }
    pub fn new() -> Self {
        Self {
            queue: [ROBEntry::new(); ROB_ENTRIES],
            head: 0,
            tail: 0,
            register_status: [None; 20],

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

        // If this is a branch, set "value" to pc as this is the LR value
        let mut value = 0;

        let rob_dest = match i.it {
            // All ALU instructions (+ mul) that write back to rd, and update CSPR
            ADC | ADDImm | ADDReg | ADDSpImm | AND | BIC | EOR | MOVImm | MOVReg | MVN | ORR
            | REVSH | REV16 | REV | RSB | SBC | ROR | SUBImm | SUBReg | SXTB | SXTH | UXTB
            | UXTH | MUL | LSLImm | LSLReg | LSRReg | LSRImm | ASRReg | ASRImm => {
                ROBEntryDest::Register(i.rd, i.setsflags)
            }

            // All ALU instructions that dont write back, as well as branches and system calls
            // Have none as a destination
            TST | CMPImm | CMN | CMPReg | B | BX | SVC | NOP => ROBEntryDest::None,

            // Sets LR
            BL | BLX => {
                value = pc;
                ROBEntryDest::Register(14, i.setsflags)
            }

            // All store instructions will be pending address calculation
            STRImm | STRReg | STRBImm | STRBReg | STRHImm | STRHReg => {
                ROBEntryDest::AwaitingAddress
            }

            // All load instructions write back to rt, and do not update CSPR
            // (no clue why they differentiate between rd and rt)
            LDRImm | LDRReg | LDRHImm | LDRHReg | LDRBImm | LDRBReg | LDRSB | LDRSH => {
                ROBEntryDest::Register(i.rt, false)
            }

            _ => panic!("ROB cannot add {:?}", i),
        };

        // If its gonna write to a register, add this to the register status
        self.temp_register_status = self.register_status.clone();
        match rob_dest {
            ROBEntryDest::Register(rd, setsflags) => {
                self.temp_register_status[rd as usize] = Some(insert_point);
            }
            _ => {}
        }

        if i.setsflags {
            // Get what flags this updates
            let (n, z, c, v) = match i.it {
                // Adds and Subtracts: Sets all 4. All the add instructions pretty much
                ADC | ADDImm | ADDReg | CMN | CMPReg | CMPImm | SUBImm | SUBReg | RSB => {
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

        self.will_issue = ROBEntry {
            target_address: 0,
            pc,
            status: ROBStatus::Execute,
            value,
            i: i.clone(),
            halt: false,
            dest: rob_dest,
            ready: false,
            asprupdate: ASPRUpdate::no_update(),
        };

        // Return where it would go upon issue commit
        // Cant insert now because RS might be full
        insert_point
    }

    pub fn flush_on_mispredict(&mut self) -> Vec<usize> {
        let mut i = Self::increment_index(self.head);
        let mut flushed = vec![];
        while self.entry_is_before(i, self.tail) {
            for rn in 0..20 {
                if let Some(rs) = self.register_status[rn] {
                    if rs == i {
                        self.register_status[rn] = None
                    }
                }
            }
            flushed.push(i);
            i = Self::increment_index(i);
        }
        self.tail = Self::increment_index(self.head);
        flushed
    }

    pub fn clear(&mut self) {
        for i in 0..ROB_ENTRIES {
            self.queue[i].status = EMPTY
        }
    }

    pub fn get(&self, n: usize) -> &ROBEntry {
        &self.queue[n]
    }

    pub fn get_head(&self) -> &ROBEntry {
        &self.queue[self.head]
    }

    pub fn get_last_issued(&self) -> Option<&ROBEntry> {
        let last_issued = &self.queue[Self::decrement_index(self.tail)];
        if last_issued.status == ROBStatus::EMPTY {
            None
        } else {
            Some(last_issued)
        }
    }

    pub fn set_halt(&mut self, n: usize) {
        self.queue[n].halt = true;
    }

    pub fn set_value(&mut self, n: usize, value: u32) {
        self.queue[n].value = value;
    }

    pub fn set_target_address(&mut self, n: usize, target: u32) {
        self.queue[n].target_address = target;
    }

    pub fn set_aspr(&mut self, n: usize, asprupdate: ASPRUpdate) {
        self.queue[n].asprupdate = asprupdate;
    }

    pub fn set_status(&mut self, n: usize, robstatus: ROBStatus) {
        self.queue[n].status = robstatus;
    }

    pub fn set_address(&mut self, n: usize, address: u32) {
        if self.queue[n].dest != ROBEntryDest::AwaitingAddress {
            unreachable!()
        }

        self.queue[n].dest = ROBEntryDest::Address(address);
    }

    pub fn set_ready(&mut self, n: usize) {
        self.queue[n].ready = true;
    }

    pub fn clear_head_and_increment(&mut self) {
        self.queue[self.head].status = ROBStatus::EMPTY;
        self.head = Self::increment_index(self.head);
    }

    pub fn issue_commit(&mut self) {
        self.queue[self.tail] = self.will_issue;
        self.register_status = self.temp_register_status;
        self.tail = Self::increment_index(self.tail);
    }

    /// Wrapping increment
    fn increment_index(index: usize) -> usize {
        (index + 1) % ROB_ENTRIES
    }

    /// Wrapping increment
    fn decrement_index(index: usize) -> usize {
        if index == 0 {
            ROB_ENTRIES - 1
        } else {
            (index - 1) % ROB_ENTRIES
        }
    }

    pub fn is_empty(&self) -> bool {
        (self.head == self.tail) && !self.is_full()
    }

    pub fn is_full(&self) -> bool {
        let mut i = self.head;
        if self.get(i).status == ROBStatus::EMPTY {
            return false;
        }
        i = Self::increment_index(i);
        while i != self.head {
            if self.get(i).status == ROBStatus::EMPTY {
                return false;
            }
            i = Self::increment_index(i);
        }
        true
    }

    /// True if e1 is first, false otherwise
    pub fn entry_is_before(&self, e1: usize, e2: usize) -> bool {
        #[cfg(debug_assertions)]
        assert!(self.head < ROB_ENTRIES);

        let mut e1s = e1 as i32 - self.head as i32;
        let mut e2s = e2 as i32 - self.head as i32;
        if e1s < 0 {
            e1s += 64
        };
        if e2s < 0 {
            e2s += 64
        };

        e1s < e2s
    }

    pub fn load_can_go(&self, load: &LoadQueueEntry) -> bool {
        let mut i = self.head;
        while self.entry_is_before(i, load.rob_entry) {
            let thingy = self.queue[i];
            match thingy.status {
                ROBStatus::EMPTY => break, // we must have exceded tail?
                _ => match thingy.dest {
                    // If the address has not been calculated yet
                    ROBEntryDest::AwaitingAddress => return false,
                    // If the address is within a word of this one
                    ROBEntryDest::Address(store_addr) => {
                        if store_addr.abs_diff(load.address) <= 4 {
                            return false;
                        }
                    }
                    _ => {}
                },
            }
            i = Self::increment_index(i);
        }
        true
    }

    pub fn render(&self, focus: usize) -> String {
        let mut string = format!("head: {}\ntail: {}\n", self.head, self.tail);
        let mut looking_at = focus;
        for i in 0..ROB_ENTRIES {
            string = format!("{string} {looking_at:02}: {}", self.queue[looking_at]);
            if i < ROB_ENTRIES - 1 {
                string.push_str("\n");
            }
            looking_at += 1;
            if looking_at >= ROB_ENTRIES {
                looking_at = 0;
            }
        }
        string
    }
}

impl std::fmt::Display for ROBEntryDest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ROBEntryDest::Register(rn, _) => Registers::reg_id_to_str(*rn),
                ROBEntryDest::AwaitingAddress => "AA".to_string(),
                ROBEntryDest::Address(addr) => format!("{:08X?}", addr),
                ROBEntryDest::None => "None".to_string(),
            }
        )
    }
}
impl std::fmt::Display for ROBEntry {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.status == ROBStatus::EMPTY {
            return write!(f, "__");
        }
        write!(
            f,
            "{}, {}, {}, {:08X?}",
            self.status.to_string(),
            self.dest,
            self.i.to_string(),
            self.pc
        )
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
        assert_eq!(rob.entry_is_before(9, 10), false);
        assert_eq!(rob.entry_is_before(10, 11), true);

        rob.head = 0;
        assert_eq!(rob.entry_is_before(0, 63), true);

        rob.head = 63;
        assert_eq!(rob.entry_is_before(9, 10), true);
        assert_eq!(rob.entry_is_before(62, 0), false);
    }
}
