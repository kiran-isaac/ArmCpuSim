#[derive(Clone, Copy)]
pub struct Registers {
    // R0-R12
    // Main Stack Pointer (theres also psp but probably not needed), R13
    // Link Register, R14
    // Program Counter, R15
    pub gp: [u32; 13],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,

    pub apsr: ASPR,
}

#[derive(Clone, Copy)]
pub struct ASPR {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
}

impl ASPR {
    pub fn cond(&self, cond: u8) -> bool {
        match cond {
            // EQ
            0b0000 => self.z == true,
            // NE
            0b0001 => self.z == false,
            // CS
            0b0010 => self.c == true,
            // CC
            0b0011 => self.c == false,
            // MI
            0b0100 => self.n == true,
            // PL
            0b0101 => self.n == false,
            // VS
            0b0110 => self.v == true,
            // VC
            0b0111 => self.v == false,
            // HI
            0b1000 => self.c == true && self.z == false,
            // LS
            0b1001 => self.c == false || self.z == true,
            // GE
            0b1010 => self.n == self.v,
            // LT
            0b1011 => self.n != self.v,
            // GT
            0b1100 => self.z == false && self.n == self.v,
            // LE
            0b1101 => self.z == true || self.n != self.v,
            // AL
            0b1110 => true,
            // NV
            0b1111 => false,
            _ => panic!("Invalid condition code"),
        }
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            gp: [0; 13],
            sp: 0,
            lr: 0,
            pc: 0,
            apsr: ASPR {
                n: false,
                z: false,
                c: false,
                v: false,
            },
        }
    }

    pub fn set(&mut self, index: u8, value: u32) {
        match index {
            0..=12 => self.gp[index as usize] = value,
            13 => self.sp = value,
            14 => self.lr = value,
            15 => self.pc = value,
            _ => panic!("Invalid register index"),
        }
    }

    pub fn get(&self, index: u8) -> u32 {
        match index {
            0..=12 => self.gp[index as usize],
            13 => self.sp,
            14 => self.lr,
            15 => self.pc,
            16 => self.apsr.n as u32,
            17 => self.apsr.z as u32,
            18 => self.apsr.c as u32,
            19 => self.apsr.v as u32,
            _ => panic!("Invalid register index"),
        }
    }

    pub fn reg_id_to_str(id: u8) -> String {
        match id {
            0..=12 => format!("{}", id),
            13 => "SP".to_string(),
            14 => "LR".to_string(),
            15 => "PC".to_string(),
            16 => "N".to_string(),
            17 => "Z".to_string(),
            18 => "C".to_string(),
            19 => "V".to_string(),
            _ => panic!("Invalid register index: {}", id),
        }
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R[r0: {:08X?}, r1: {:08X?}, r2: {:08X?}, r3: {:08X?}, r4: {:08X?}, r5: {:08X?}, r6: {:08X?}, r7: {:08X?}, r8: {:08X?}, r9: {:08X?}, r10: {:08X?}, r11: {:08X?}, r12: {:08X?}, sp: {:08X?}, lr: {:08X?}, pc: {:08X?}]", self.gp[0], self.gp[1], self.gp[2], self.gp[3], self.gp[4], self.gp[5], self.gp[6], self.gp[7], self.gp[8], self.gp[9], self.gp[10], self.gp[11], self.gp[12], self.sp, self.lr, self.pc)
    }
}
