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

#[derive(Clone, Copy)]
pub struct ASPRUpdate {
    pub n: Option<bool>,
    pub z: Option<bool>,
    pub c: Option<bool>,
    pub v: Option<bool>,
}

impl ASPRUpdate {
    pub fn no_update() -> Self {
        ASPRUpdate {
            n: None,
            z: None,
            c: None,
            v: None,
        }
    }
}

impl ASPR {
    fn apply_aspr_update(&mut self, update: &ASPRUpdate) {
        if let Some(update) = update.n {
            self.n = update;
        }
        if let Some(update) = update.z {
            self.z = update;
        }
        if let Some(update) = update.c {
            self.c = update;
        }
        if let Some(update) = update.v {
            self.v = update;
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
            0..=12 => format!("r{}", id),
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

    pub fn apply_aspr_update(&mut self, ASPR: &ASPRUpdate) {
        self.apsr.apply_aspr_update(ASPR);
    }
}

impl std::fmt::Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "R[r0: {:08X?}, r1: {:08X?}, r2: {:08X?}, r3: {:08X?}, r4: {:08X?}, r5: {:08X?}, r6: {:08X?}, r7: {:08X?}, r8: {:08X?}, r9: {:08X?}, r10: {:08X?}, r11: {:08X?}, r12: {:08X?}, sp: {:08X?}, lr: {:08X?}, pc: {:08X?}]", self.gp[0], self.gp[1], self.gp[2], self.gp[3], self.gp[4], self.gp[5], self.gp[6], self.gp[7], self.gp[8], self.gp[9], self.gp[10], self.gp[11], self.gp[12], self.sp, self.lr, self.pc)
    }
}
