use super::{I, IT, IT::*};
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

        let args = match self.it {
            // RD, RN RM register register
            ADDReg | AND | ASRReg | BIC | EOR | LSLReg | LSRReg | ROR | MUL | ORR | SBC
            | SUBReg => format!("{} {} {}", self.rd, self.rn, self.rm),

            // RD RN immu
            ADC | ADDImm | ADDSpImm | RSB | SUBImm => {
                format!("{} {} #{}", self.rd, self.rn, self.immu)
            }

            // RD immu
            MOVImm => format!("{} #{}", self.rd, self.immu),

            // RD RM
            MOVReg | MVN | REV | REV16 | REVSH | SXTB | SXTH | UXTB | UXTH => {
                format!("{} {}", self.rd, self.rm)
            }

            // RD RM immu
            ASRImm | LSRImm | LSLImm => format!("{} {} #{}", self.rd, self.rn, self.immu),

            // RN RM
            CMN | CMPReg | TST => format!("{} {}", self.rn, self.rm),

            // RT RN immu
            LDRImm | LDRBImm | LDRHImm => format!("{} {} #{}", self.rt, self.rn, self.immu),

            // RT RN RM
            LDRReg | LDRBReg | LDRHReg | LDRSH | LDRSB => {
                format!("{} {} {}", self.rt, self.rn, self.rm)
            }

            // RN RT RM
            STRBReg | STRHReg | STRReg => format!("{} {} {}", self.rn, self.rt, self.rm),

            // RN RT immu
            STRImm | STRHImm | STRBImm => format!("{} {} #{}", self.rn, self.rt, self.immu),

            // RN immu
            CMPImm => format!("{} {}", self.rn, self.immu),

            // B PC + imms
            BL | B => format!("#{}", self.imms),

            // BX RM
            BX | BLX => format!("{}", self.rm),

            // immu
            SVC => format!("#{}", self.immu),
            
            SetPC => "".to_string(),

            _ => unimplemented!("tostring for {:?}", self.it),
        };
        str += args.as_str();
        write!(f, "{}", str)
    }
}
