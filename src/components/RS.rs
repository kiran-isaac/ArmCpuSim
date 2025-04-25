use crate::components::ROB::ROB;
use crate::decode::{IssueType, I, IT::*};
use crate::model::Registers;
use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum RSData {
    ROB(usize, u8),
    Data(u32),
    None,
}

impl Display for RSData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RSData::ROB(rob, r) => write!(f, "#{:09}:{}", rob, Registers::reg_id_to_str(*r)),
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

    pub i: I,

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
            i: I::undefined(),
        }
    }

    pub fn is_ready(&self) -> bool {
        if !self.busy {
            false
        } else {
            // if either is waiting for ROB then not ready
            match (&self.j, &self.k) {
                (RSData::ROB(_, _), _) | (_, RSData::ROB(_, _)) => false,
                _ => true,
            }
        }
    }

    fn receive_cdb_broadcast(&mut self, rob_entry: usize, rn: u8, result: u32) {
        match self.j {
            RSData::ROB(rob_entry_2, rn_2) => {
                if rn == rn_2 && rob_entry_2 == rob_entry {
                    self.j = RSData::Data(result);
                }
            }
            _ => {}
        }
        match self.k {
            RSData::ROB(rob_entry_2, rn_2) => {
                if rn == rn_2 && rob_entry_2 == rob_entry {
                    self.k = RSData::Data(result);
                }
            }
            _ => {}
        }
        match self.l {
            RSData::ROB(rob_entry_2, rn_2) => {
                if rn == rn_2 && rob_entry_2 == rob_entry {
                    self.l = RSData::Data(result);
                }
            }
            _ => {}
        }
    }
    
    fn assert_not_waiting_for_rob(&self, rob_entry: usize) {
        match self.j {
            RSData::ROB(rob_entry_2, _) => {
                assert_ne!(rob_entry_2, rob_entry);
            }
            _ => {}
        }
        match self.k {
            RSData::ROB(rob_entry_2, _) => {
                assert_ne!(rob_entry_2, rob_entry);
            }
            _ => {}
        }
        match self.l {
            RSData::ROB(rob_entry_2, _) => {
                assert_ne!(rob_entry_2, rob_entry);
            }
            _ => {}
        }
    }
}

pub struct RSSet {
    pub vec: Vec<RS>,
    issue_type: IssueType,
    n: usize,
}

impl<'a> RSSet {
    pub fn new(issue_type: IssueType, n: usize) -> RSSet {
        let vec = vec![RS::new(); n];
        RSSet {
            vec,
            issue_type,
            n,
        }
    }

    pub fn receive_cdb_broadcast(&mut self, rob_entry: usize, rn: u8, result: u32) {
        for rs in self.vec.iter_mut() {
            rs.receive_cdb_broadcast(rob_entry, rn, result);
        }
    }
    
    pub fn assert_none_waiting_for_rob(&self, rob_entry: usize) {
        for rs in self.vec.iter() {
            rs.assert_not_waiting_for_rob(rob_entry);
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    /// if RST shows ROB entry then this, else get data from ARF
    fn get_rs_data(rn: u8, arf: &Registers, register_status: &[Option<usize>; 20], rob: &'a ROB) -> RSData {
        if let Some(rob_entry_num) = register_status[rn as usize] {
            let rob_entry = rob.get(rob_entry_num);
            if rob_entry.ready {
                RSData::Data(rob_entry.value)
            } else {
                RSData::ROB(rob_entry_num, rn)
            }
        } else {
            RSData::Data(arf.get(rn))
        }
    }

    fn get_alloc(&self) -> Option<usize> {
        // get index of first one that's not empty
        Some(self.vec.iter().enumerate().find(|(_, rs)| !rs.busy)?.0)
    }

    pub fn get_one_ready(&self) -> Option<usize> {
        for (index, entry) in self.vec.iter().enumerate() {
            if !entry.busy {
                continue;
            }

            // Ignore this entry if any still pending results
            match (entry.j, entry.k, entry.l) {
                (RSData::ROB(_, _), _, _) | (_, RSData::ROB(_, _), _) | (_, _, RSData::ROB(_, _)) => {
                    continue
                }
                _ => return Some(index),
            }
        }
        None
    }

    /// Get a ready to execute RS
    pub fn get_all_ready(&self, rob: &ROB) -> Vec<&RS> {
        let mut set = Vec::new();
        for entry in &self.vec {
            if !entry.busy {
                continue;
            }

            // Ignore this entry if any still pending results
            match (entry.j, entry.k, entry.l) {
                (RSData::ROB(_, _), _, _) | (_, RSData::ROB(_, _), _) | (_, _, RSData::ROB(_, _)) => {
                    continue
                }
                _ => set.push(entry),
            }
        }
        set.sort_by(|a, b| {
            if rob.entry_is_before(a.rob_dest, b.rob_dest) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        set
    }

    fn get_dependencies(
        &mut self,
        i: &I,
        arf: &Registers,
        register_status: &[Option<usize>; 20],
        rob: &'a ROB
    ) -> (RSData, RSData, RSData) {
        let mut j = RSData::None;
        let mut k = RSData::None;
        let mut l = RSData::None;

        // Set dependencies
        match self.issue_type {
            IssueType::ALUSHIFT | IssueType::MUL => {
                match i.it {
                    // Dual register and carry {
                    ADC | SBC => {
                        j = Self::get_rs_data(i.rn, arf, register_status, rob);
                        k = Self::get_rs_data(i.rm, arf, register_status, rob);
                        // 18 is carry
                        l = Self::get_rs_data(18, arf, register_status, rob);
                    }

                    // Dual register
                    MUL | ADDReg | AND | BIC | ASRReg | CMN | CMPReg | EOR | LSLReg | LSRReg
                    | MOVReg | ORR | ROR | SUBReg => {
                        j = Self::get_rs_data(i.rn, arf, register_status, rob);
                        k = Self::get_rs_data(i.rm, arf, register_status, rob);
                    }

                    // register immediate (rn)
                    ADDImm | ADDSpImm | CMPImm | SUBImm => {
                        j = Self::get_rs_data(i.rn, arf, register_status, rob);
                        k = RSData::Data(i.immu);
                    }

                    // Immediate Only
                    MOVImm => {
                        j = RSData::Data(i.immu);
                    }

                    // Single Register (rm)
                    MVN | REV | REV16 | REVSH | SXTB | SXTH | UXTB | UXTH => {
                        j = Self::get_rs_data(i.rm, arf, register_status, rob);
                    }

                    // Shift (rm is used as 1st operator not rn???)
                    ASRImm | LSLImm | LSRImm => {
                        j = Self::get_rs_data(i.rm, arf, register_status, rob);
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
                LDRImm | LDRBImm | LDRHImm => {
                    j = Self::get_rs_data(i.rn, arf, register_status, rob);
                    k = RSData::Data(i.immu);
                }

                STRImm | STRBImm | STRHImm => {
                    j = Self::get_rs_data(i.rn, arf, register_status, rob);
                    k = RSData::Data(i.immu);
                    l = Self::get_rs_data(i.rt, arf, register_status, rob);
                }

                // rn + rm offset
                LDRReg | LDRBReg | LDRHReg | LDRSH | LDRSB => {
                    j = Self::get_rs_data(i.rn, arf, register_status, rob);
                    k = Self::get_rs_data(i.rm, arf, register_status, rob);
                }

                STRReg | STRBReg | STRHReg => {
                    j = Self::get_rs_data(i.rn, arf, register_status, rob);
                    k = Self::get_rs_data(i.rm, arf, register_status, rob);
                    l = Self::get_rs_data(i.rt, arf, register_status, rob);
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
                                j = Self::get_rs_data(17, arf, register_status, rob);
                            }
                            // CS | CC
                            0b0010 | 0b0011 => {
                                // (n, z, c, v)
                                // (false, false, true, false),
                                j = Self::get_rs_data(18, arf, register_status, rob);
                            }
                            // MI | PL
                            0b0100 | 0b0101 => {
                                // (n, z, c, v)
                                // (true, false, false, false)
                                j = Self::get_rs_data(16, arf, register_status, rob);
                            }
                            // VS | VC
                            0b0110 | 0b0111 => {
                                // (n, z, c, v)
                                // (false, false, false, true)
                                j = Self::get_rs_data(19, arf, register_status, rob);
                            }
                            // HI | LS
                            0b1000 | 0b1001 => {
                                // (n, z, c, v)
                                // (false, true, true, false)
                                j = Self::get_rs_data(17, arf, register_status, rob);
                                k = Self::get_rs_data(18, arf, register_status, rob);
                            }
                            // GE | LT
                            0b1010 | 0b1011 => {
                                // (n, z, c, v)
                                // (true, false, false, true)
                                j = Self::get_rs_data(16, arf, register_status, rob);
                                k = Self::get_rs_data(19, arf, register_status, rob);
                            }
                            // GT | LE
                            0b1100 | 0b1101 => {
                                // (n, z, c, v)
                                // (true, true, false, true),
                                j = Self::get_rs_data(16, arf, register_status, rob);
                                k = Self::get_rs_data(17, arf, register_status, rob);
                                l = Self::get_rs_data(19, arf, register_status, rob);
                            }
                            // AL | NV
                            0b1110 | 0b1111 => {}
                            _ => unreachable!("invalid cond code"),
                        }
                    }
                    BL => {}
                    BX | BLX => {
                        j = Self::get_rs_data(i.rm, arf, register_status, rob);
                    }
                    // Sets PC from register value
                    SetPC => {
                        j = Self::get_rs_data(i.rn, arf, register_status, rob);
                    }
                    // Supervisor calls always read from r0
                    SVC => {
                        j = RSData::Data(i.immu);
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
        rob: &'a ROB
    ) -> Option<usize> {
        // will return none if it cannot allocate
        let alloc = self.get_alloc()?;

        let (j, k, l) = self.get_dependencies(i, arf, register_status, rob);

        self.vec[alloc] = RS {
            busy: true,
            j,
            k,
            l,
            rob_dest: dest,
            i: i.clone(),
            setsflags: i.setsflags,
        };
        Some(alloc)
    }
}
