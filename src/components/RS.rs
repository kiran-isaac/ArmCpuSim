use crate::decode::{IssueType, I, IT::*};
use crate::model::Registers;

pub enum RSData {
    ROB(u32),
    Data(u32),
    None,
}

pub struct RS {
    busy: bool,

    // 3 needed as branch instructions can rely on up to 3 NZCV flags
    j: RSData,
    k: RSData,
    l: RSData,
    /// The ROB entry to write to after execution
    dest: Option<usize>,
}

impl RS {
    pub fn new() -> Self {
        RS {
            busy: false,
            j: RSData::None,
            k: RSData::None,
            dest: None,
        }
    }

    pub fn is_ready(&self) -> bool {
        if !self.busy {false} else {
            // if either is waiting for ROB then not ready
            match (&self.j, &self.k) {
                (RSData::ROB(_), _) | (_, RSData::ROB(_)) => false,
                _ => true,
            }
        }
    }
}

pub struct RSSet<const N: usize> {
    buf: [RS; N],
    issue_type: IssueType
}

impl<const N: usize> RSSet<N> {
    pub fn new(issue_type: IssueType) -> RSSet<N> {
        let vec = [RS::new(); N];
        RSSet { buf: vec, issue_type }
    }

    pub fn len(&self) -> usize {
        N
    }

    fn get_rs_data(rn: u8, arf: &Registers, register_status: &[Option<usize>; 20]) -> RSData {
        if let Some(rob_entry_num) = register_status[rn] {
            RSData::ROB(rob_entry_num)
        } else {
            RSData::Data(arf.get(rn))
        }
    }

    fn get_alloc(&self) -> Option<usize> {
        // get index of first one that's not empty
        Some(self.buf.iter().enumerate().find(|(i, rs)| !rs.busy)?.0)
    }

    pub fn issue_receive(&mut self, i: &I, dest: Option<usize>, arf: &Registers, register_status: &[Option<usize>; 20]) -> Option<usize> {
        // will return none if it cannot allocate
        let alloc = self.get_alloc()?;

        let mut j = RSData::None;
        let mut k = RSData::None;
        let mut l = RSData::None;

        // Set dependencies
        match self.issue_type {
            IssueType::ALU | IssueType::MUL => {
                j = Self::get_rs_data(i.rn, arf, register_status);
                match i.it {
                    // Unary alu instructions dont require rm
                    REV | REV16 | REVSH | SXTB | SXTH | UXTB | UXTH => {},
                    // Binary alu instructions and mul require rm
                    _ => {
                        k = Self::get_rs_data(i.rm, arf, register_status);
                    }
                }
            }
            IssueType::LoadStore => {
                match i.it {

                }
            }
            IssueType::Control => {
                match i.it {
                    // How NZCV maps to ARF
                    // N => 16
                    // Z => 17
                    // C => 18
                    // V => 19
                    B => {
                        match i.rn {
                            // EQ | NE
                            0b0000 | 0b0001 => { // (false, true, false, false),
                                j = Self::get_rs_data(17, arf, register_status);
                            }
                            // CS | CC
                            0b0010 | 0b0011 => { //(false, false, true, false),
                                j = Self::get_rs_data(18, arf, register_status);
                            }
                            // MI | PL
                            0b0100 | 0b0101  => { // (true, false, false, false)
                                j = Self::get_rs_data(16, arf, register_status);
                            },
                            // VS | VC
                            0b0110 | 0b0111 => { // (false, false, false, true)
                                j = Self::get_rs_data(19, arf, register_status);
                            },
                            // HI | LS
                            0b1000 | 0b1001 => { // (false, true, true, false)
                                j = Self::get_rs_data(17, arf, register_status);
                                k = Self::get_rs_data(18, arf, register_status);
                            },
                            // GE | LT
                            0b1010 | 0b1011 => { // (true, false, false, true)
                                j = Self::get_rs_data(16, arf, register_status);
                                k = Self::get_rs_data(19, arf, register_status);
                            },
                            // GT | LE
                            0b1100 | 0b1101 => {// (true, true, false, true),
                                j = Self::get_rs_data(16, arf, register_status);
                                k = Self::get_rs_data(17, arf, register_status);
                                l = Self::get_rs_data(19, arf, register_status);
                            }
                            // AL | NV
                            0b1110 | 0b1111 => {},
                            _ => unreachable!(),
                        }
                    }
                    BL | BX | BLX => {},
                    // Supervisor calls always read from r0
                    SVC => {
                        j = Self::get_rs_data(0, arf, register_status);
                    }
                };
            }
        }

        self[alloc] = RS {
            busy: true,
            j, k, l, dest
        };
        Some(alloc)
    }
}
