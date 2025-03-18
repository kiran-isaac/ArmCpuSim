use crate::decode::{IssueType, I, IT::*};
use crate::model::Registers;
use ratatui::layout::Constraint::Fill;
use ratatui::layout::Rect;
use ratatui::prelude::{Layout, Widget};
use ratatui::widgets::{Block, Borders, Paragraph};
use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum RSData {
    ROB(u32),
    Data(u32),
    None,
}

impl Display for RSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSData::ROB(r) => write!(f, "#{:09}", r),
            RSData::Data(r) => write!(f, "{:#010x}", r),
            RSData::None => write!(f, "----------"),
        }
    }
}

#[derive(Clone)]
pub struct RS {
    pub busy: bool,

    // 3 needed as branch instructions can rely on up to 3 NZCV flags
    pub j: RSData,
    pub k: RSData,
    pub l: RSData,

    /// The ROB entry to write to after execution
    pub rob_dest: usize,

    pub i_str: String,

    setsflags: bool,
}

impl RS {
    pub fn new() -> Self {
        RS {
            busy: false,
            j: RSData::None,
            k: RSData::None,
            l: RSData::None,
            setsflags: false,
            rob_dest: 0,
            i_str: String::new(),
        }
    }

    pub fn is_ready(&self) -> bool {
        if !self.busy {
            false
        } else {
            // if either is waiting for ROB then not ready
            match (&self.j, &self.k) {
                (RSData::ROB(_), _) | (_, RSData::ROB(_)) => false,
                _ => true,
            }
        }
    }
}

pub struct RSSet {
    pub buf: Vec<RS>,
    issue_type: IssueType,
    n: usize,
}

impl RSSet {
    pub fn new(issue_type: IssueType, n: usize) -> RSSet {
        let vec = vec![RS::new(); n];
        RSSet {
            buf: vec,
            issue_type,
            n,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// if RST shows ROB entry then this, else get data from ARF
    fn get_rs_data(rn: u8, arf: &Registers, register_status: &[Option<usize>; 20]) -> RSData {
        if let Some(rob_entry_num) = register_status[rn as usize] {
            RSData::ROB(rob_entry_num as u32)
        } else {
            RSData::Data(arf.get(rn))
        }
    }

    fn get_alloc(&self) -> Option<usize> {
        // get index of first one that's not empty
        Some(self.buf.iter().enumerate().find(|(_, rs)| !rs.busy)?.0)
    }

    fn get_dependencies(
        &mut self,
        i: &I,
        arf: &Registers,
        register_status: &[Option<usize>; 20],
    ) -> (RSData, RSData, RSData) {
        let mut j = RSData::None;
        let mut k = RSData::None;
        let mut l = RSData::None;

        // Set dependencies
        match self.issue_type {
            IssueType::ALUSHIFT | IssueType::MUL => {
                match i.it {
                    // Dual register
                    ADC | MUL | ADDReg | AND | BIC | ASRReg | CMN | CMPReg | EOR | LSLReg
                    | LSRReg | MOVReg | ORR | ROR | SBC | SUBReg => {
                        j = Self::get_rs_data(i.rn, arf, register_status);
                        k = Self::get_rs_data(i.rm, arf, register_status);
                    }

                    // register immediate (rn)
                    ADDImm | ADDSpImm | CMPImm | SUBImm => {
                        j = Self::get_rs_data(i.rn, arf, register_status);
                        k = RSData::Data(i.immu);
                    }

                    // Immediate Only
                    MOVImm => {
                        j = RSData::Data(i.immu);
                    }

                    // Single Register (rm)
                    MVN | REV | REV16 | REVSH | SXTB | SXTH | UXTB | UXTH => {
                        j = Self::get_rs_data(i.rm, arf, register_status);
                    }

                    // Shift (rm is used as 1st operator not rn???)
                    ASRImm | LSLImm | LSRImm => {
                        j = Self::get_rs_data(i.rm, arf, register_status);
                        k = RSData::Data(i.immu);
                    }

                    _ => panic!(
                        "{:?} should not have been issued here. This is the res stations for {:?}",
                        i, self.issue_type
                    ),
                }
            }
            IssueType::LoadStore => match i.it {
                // rn + imm offset
                STRImm | STRBImm | STRHImm | LDRImm | LDRBImm | LDRHImm => {
                    j = Self::get_rs_data(i.rn, arf, register_status);
                    k = RSData::Data(i.immu);
                }

                // rn + rm offset
                STRReg | STRBReg | STRHReg | LDRReg | LDRBReg | LDRHReg | LDRSH | LDRSB => {
                    j = Self::get_rs_data(i.rn, arf, register_status);
                    k = Self::get_rs_data(i.rm, arf, register_status);
                }
                _ => panic!("{:?} should not have been issued here", i),
            },
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
                            0b0000 | 0b0001 => {
                                // (false, true, false, false),
                                j = Self::get_rs_data(17, arf, register_status);
                            }
                            // CS | CC
                            0b0010 | 0b0011 => {
                                //(false, false, true, false),
                                j = Self::get_rs_data(18, arf, register_status);
                            }
                            // MI | PL
                            0b0100 | 0b0101 => {
                                // (true, false, false, false)
                                j = Self::get_rs_data(16, arf, register_status);
                            }
                            // VS | VC
                            0b0110 | 0b0111 => {
                                // (false, false, false, true)
                                j = Self::get_rs_data(19, arf, register_status);
                            }
                            // HI | LS
                            0b1000 | 0b1001 => {
                                // (false, true, true, false)
                                j = Self::get_rs_data(17, arf, register_status);
                                k = Self::get_rs_data(18, arf, register_status);
                            }
                            // GE | LT
                            0b1010 | 0b1011 => {
                                // (true, false, false, true)
                                j = Self::get_rs_data(16, arf, register_status);
                                k = Self::get_rs_data(19, arf, register_status);
                            }
                            // GT | LE
                            0b1100 | 0b1101 => {
                                // (true, true, false, true),
                                j = Self::get_rs_data(16, arf, register_status);
                                k = Self::get_rs_data(17, arf, register_status);
                                l = Self::get_rs_data(19, arf, register_status);
                            }
                            // AL | NV
                            0b1110 | 0b1111 => {}
                            _ => unreachable!("invalid cond code"),
                        }
                    }
                    BL | BX | BLX => {}
                    // Supervisor calls always read from r0
                    SVC => {
                        j = Self::get_rs_data(0, arf, register_status);
                    }
                    _ => panic!(
                        "{:?} should not have been issued here. This is the res stations for {:?}",
                        i, self.issue_type
                    ),
                };
            }
        }

        (j, k, l)
    }

    pub fn issue_receive(
        &mut self,
        i: &I,
        dest: usize,
        arf: &Registers,
        register_status: &[Option<usize>; 20],
    ) -> Option<usize> {
        // will return none if it cannot allocate
        let alloc = self.get_alloc()?;

        let (j, k, l) = self.get_dependencies(i, arf, register_status);

        self.buf[alloc] = RS {
            busy: true,
            j,
            k,
            l,
            rob_dest: dest,
            i_str: i.to_string(),
            setsflags: i.setsflags,
        };
        Some(alloc)
    }
}
