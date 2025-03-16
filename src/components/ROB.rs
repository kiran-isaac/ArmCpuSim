use crate::components::ROB::ROBEntryDest::Register;
use crate::decode::{I, IT::*};

enum ROBStatus {
    Pending,
    Execute,
    Commit,
    Write,

    /// Instruction had an exception
    Exception(u8),
}

const ROB_ENTRIES: usize = 64;

struct ROB {
    queue: [Option<ROBEntry>; ROB_ENTRIES],
    head: usize,
    tail: usize,

    // None means not busy
    // Some(n) means ROB entry n holds the result
    register_status: [Option<u8>; 16],
}

enum ROBEntryDest {
    // Discards results. Only for CMP, CMN etc
    None,
    AwaitingAddress,
    Address(usize),
    Register(u8),
}

struct ROBEntry {
    pub pc: u32,
    pub status: ROBStatus,
    pub value: u32,
    pub dest: ROBEntryDest,
}

impl ROB {
    fn new() -> Self {
        Self {
            queue: [const { None }; ROB_ENTRIES],
            head: 0,
            tail: 0,
            register_status: [None; 16],
        }
    }

    fn issue_receive(&mut self, i: &I, pc: u32) {
        let mut dest = match i.it {
            ADC | ADDImm | ADDReg | ADDSpImm | AND | ASRImm => ROBEntryDest::Register(i.rd),

            LoadPc => ROBEntryDest::Register(15),
        };

        let new_entry = ROBEntry {
            pc,
            status: ROBStatus::Execute,
            value: 0,
        };
    }
}
