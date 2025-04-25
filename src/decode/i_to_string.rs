use super::{I, IT, IT::*};
use crate::components::ROB::ROBEntryDest::Register;
use crate::model::Registers;
use std::fmt::Display;

impl IT {
    fn to_string_no_type(&self) -> String {
        let mut str = format!("{:?}", self).to_lowercase();
        str = str.replace("imm", "");
        str = str.replace("lit", "");
        str = str.replace("reg", "");
        str
    }
}

impl Display for I {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = self.it.to_string_no_type() + if self.setsflags { "s " } else { " " };

        let rd = Registers::reg_id_to_str(self.rd);
        let rn = Registers::reg_id_to_str(self.rn);
        let rm = Registers::reg_id_to_str(self.rm);
        let rt = Registers::reg_id_to_str(self.rt);

        let args = match self.it {
            // RD, RN RM register register
            ADDReg | AND | ASRReg | BIC | EOR | LSLReg | LSRReg | ROR | MUL | ORR | SBC
            | SUBReg => format!("{} {} {}", rd, rn, rm),

            // RD RN immu
            ADC | ADDImm | ADDSpImm | RSB | SUBImm => {
                format!("{} {} #{}", rd, rn, self.immu)
            }

            // RD immu
            MOVImm => format!("{} #{}", rd, self.immu),

            // RD RM
            MOVReg | MVN | REV | REV16 | REVSH | SXTB | SXTH | UXTB | UXTH => {
                format!("{} {}", rd, rm)
            }

            // RD RM immu
            ASRImm | LSRImm | LSLImm => format!("{} {} #{}", rd, rn, self.immu),

            // RN RM
            CMN | CMPReg | TST => format!("{} {}", rn, rm),

            // RT RN immu
            LDRImm | LDRBImm | LDRHImm => format!("{} {} #{}", rt, rn, self.immu),

            // RT RN RM
            LDRReg | LDRBReg | LDRHReg | LDRSH | LDRSB => {
                format!("{} {} {}", rt, rn, rm)
            }

            // RN RT RM
            STRBReg | STRHReg | STRReg => format!("{} {} {}", rn, rt, rm),

            // RN RT immu
            STRImm | STRHImm | STRBImm => format!("{} {} #{}", rn, rt, self.immu),

            // RN immu
            CMPImm => format!("{} {}", rn, self.immu),

            // B PC + imms
            BL | B => format!("#{}", self.imms),

            // BX RM
            BX | BLX => format!("{}", rm),

            // immu
            SVC => format!("#{}", self.immu),

            SetPC => "".to_string(),

            _ => unimplemented!("tostring for {:?}", self.it),
        };
        str += args.as_str();
        write!(f, "{}", str)
    }
}
