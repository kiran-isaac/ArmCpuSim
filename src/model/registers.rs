use std::ops::Index;

pub struct Registers {
    // R0-R12
    // Main Stack Pointer (theres also psp but probably not needed), R13
    // Link Register, R14
    // Program Counter, R15
    pub gp: [u32; 13],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,

    pub apsr: u32,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            gp: [0; 13],
            sp: 0,
            lr: 0,
            pc: 0,
            apsr: 0,
        }
    }

    pub fn set_register(&mut self, index: u32, value: u32) {
        match index {
            0...12 => self.gp[index] = value,
            13 => self.sp = value,
            14 => self.lr = value,
            15 => self.pc = value,
            _ => panic!("Invalid register index"),
        }
    }

    pub fn get_register(&mut self, index: u32) -> u32 {
        match index {
            0...12 => self.gp[index],
            13 => self.sp,
            14 => self.lr,
            15 => self.pc,
            _ => panic!("Invalid register index"),
        }
    }
}
