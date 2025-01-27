use std::io::Write;

use crate::{model::Registers, I};

pub struct Tracer {
    file: std::fs::File,
    previous_regs: Registers,
    i_count: usize,
}

impl Tracer {
    pub fn new(filename: &str, initialstate: &Registers) -> Tracer {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write(b"I, PC                ,Instruction, LR, SP  ,Rd   ,Rn   ,Rm   ,Rt   ,Rl                 ,Immu     ,Imms\n")
            .unwrap();
        Tracer {
            file,
            previous_regs: initialstate.clone(),
            i_count: 0,
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
            0..=12 => format!("{}", id),
            13 => "SP".to_string(),
            14 => "LR".to_string(),
            15 => "PC".to_string(),
            _ => panic!("Invalid register index: {}", id),
        }
    }

    // Will log in order: IT, Rd, Rn, Rm, Rt, Rl, Immu, Imms
    fn instr_args_log(&self, i: I, new_regs: &Registers) -> String {
        format!(
            "{} {},{} {}   ,{} {},{} {},{} ,{} ,{}",
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
            format!("{:X}", previous)
        } else {
            format!("{:X}>{:X}", previous, current)
        }
    }

    pub fn log(&mut self, i: I, new_regs: &Registers) {
        self.file
            .write(
                format!(
                    "{},{},{},{},{}   ,{}\n",
                    self.i_count,
                    self.reg_change_log(15, new_regs),
                    format!("{:?}", i.it),
                    self.reg_change_log(14, new_regs),
                    self.reg_change_log(13, new_regs),
                    self.instr_args_log(i, new_regs),
                    // self.reg_change_logs(i, new_regs)
                )
                .as_bytes(),
            )
            .unwrap();
        self.i_count += 1;

        self.previous_regs = new_regs.clone()
    }
}
