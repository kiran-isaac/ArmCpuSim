use std::io::Write;

use crate::{model::Registers, I, IT::*};


pub struct Logger {
    file: std::fs::File,
    previous_state: Registers,
}

impl Logger {
    pub fn new(filename : &str, initialstate: &Registers) -> Logger {
        let mut file = std::fs::File::create(filename).unwrap();
        file.write(b"Instruction,IT,Rd,Rn,Rm,Rt,Rl,Immu,Imms\n").unwrap();
        Logger { file, previous_state: initialstate.clone() }
    }

    fn reg_id_to_str(id: u8) -> String {
        match id {
            0..=12 => format!("r{}", id),
            13 => "sp".to_string(),
            14 => "lr".to_string(),
            15 => "pc".to_string(),
            _ => panic!("Invalid register index"),
        }
    }

    // Will log in order: IT, Rd, Rn, Rm, Rt, Rl, Immu, Imms
    fn instr_args_log(i: I) -> String {
        format!("{:?}, {}, {}, {}, {}, {:#018b}, {}, {}", i.it, i.rd, i.rn, i.rm, i.rt, i.rl, i.immu, i.imms)
        // match i.it {
        //     // Rd, Rn, Rm
        //     ADDReg => {
        //         #[cfg(debug_assertions)]
        //         assert_eq!(i.rd, i.rn);

        //         format!("{},{},{}", Self::reg_id_to_str(i.rd), Self::reg_id_to_str(i.rn), Self::reg_id_to_str(i.rm))
        //     }
        //     // Rdn. Rm
        //     ADC => {
        //         #[cfg(debug_assertions)]
        //         assert_eq!(i.rd, i.rn);

        //         format!("{},=, {}", Self::reg_id_to_str(i.rd), Self::reg_id_to_str(i.rm))
        //     }
        //     // Rd, Rn, immu
        //     ADDImm | ADDSpImm => {
        //         #[cfg(debug_assertions)]
        //         assert_eq!(i.rd, i.rn);

        //         format!("{},{},#{}", Self::reg_id_to_str(i.rd), Self::reg_id_to_str(i.rn), i.immu)
        //     }
        //     // Rd, Immu
        //     ADR => {
        //         format!("{},#{}", Self::reg_id_to_str(i.rd), i.immu)
        //     }
        //     _ => unimplemented!("Instruction tostr not implemented"),
        // }
    }

    pub fn log(&mut self, i: I, state: &Registers) {
        self.file.write(format!("{:?},{}\n", i.it, Self::instr_args_log(i)).as_bytes()).unwrap();

        // self.previous_state = state.clone()
    }
}
