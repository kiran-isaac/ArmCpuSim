use std::io::Write;

use crate::{model::Registers, I, IT::*};

pub struct Logger {
    file: std::fs::File,
    previous_regs: Registers,
}

impl Logger {
    pub fn new(filename: &str, initialstate: &Registers) -> Logger {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write(b"PC                 ,Instruction   ,Rd   ,Rn   ,Rm   ,Rt   ,Rl                 ,Immu     ,Imms\n")
            .unwrap();
        Logger {
            file,
            previous_regs: initialstate.clone(),
        }
    }

    fn rl_to_string(rl: u16) -> String {
        if rl == 0 {
            "0".to_string()
        } else {
            format!("{:018b}", rl)
        }
    }

    fn reg_id_to_str(id: u8) -> String {
        match id {
            0..=12 => format!("{:0>2}", id),
            13 => "SP".to_string(),
            14 => "LR".to_string(),
            15 => "PC".to_string(),
            _ => panic!("Invalid register index"),
        }
    }

    // Will log in order: IT, Rd, Rn, Rm, Rt, Rl, Immu, Imms
    fn instr_args_log(&self, i: I, new_regs: &Registers) -> String {
        format!(
            "{:<2} {:<17}   ,{:<2} {:<17}   ,{:<2} {:<17}   ,{:<2} {:<17}   ,{} ,{:#08X} ,{:#08X}",
            Self::reg_id_to_str(i.rd),
            self.reg_change_log(i.rd, new_regs),
            Self::reg_id_to_str(i.rn),
            self.reg_change_log(i.rn, new_regs),
            Self::reg_id_to_str(i.rm),
            self.reg_change_log(i.rm, new_regs),
            Self::reg_id_to_str(i.rt),
            self.reg_change_log(i.rt, new_regs),
            Self::rl_to_string(i.rl),
            i.immu,
            i.imms
        )
    }

    fn reg_change_log(&self, reg: u8, new_regs: &Registers) -> String {
        let previous = self.previous_regs.get(reg);
        let current = new_regs.get(reg);
        if previous == current {
            "".to_string()
        } else {
            format!("{:X}>{:X}", previous, current)
        }
    }

    fn reg_change_logs(&self, i: I, new_regs: &Registers) -> String {
        format!(
            "{:<17}, {:<17}, {:<17}, {:<17}",
            self.reg_change_log(i.rd, new_regs),
            self.reg_change_log(i.rn, new_regs),
            self.reg_change_log(i.rm, new_regs),
            self.reg_change_log(i.rt, new_regs)
        )
    }

    pub fn log(&mut self, i: I, new_regs: &Registers) {
        self.file
            .write(
                format!(
                    "{:<17} ,{:<11}   ,{}\n",
                    self.reg_change_log(15, new_regs),
                    format!("{:?}", i.it),
                    self.instr_args_log(i, new_regs),
                    // self.reg_change_logs(i, new_regs)
                )
                .as_bytes(),
            )
            .unwrap();

        self.previous_regs = new_regs.clone()
    }
}
