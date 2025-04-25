/// We need to split some of the CISCy instructions into multiple instructions
use super::*;
use crate::binary::{bit_as_bool, hamming_weight};

// enum MOps {
//     // ALU instructions
//     ADD,
//     AND,
//     OR,
//     EOR,
//     REV,
//     REV16,
//     REVSH,
//     UXTH,
//     UXTB,
//     SXTB,
//     SXTH,
//     LSL,
//     LSR,
//     ASR,
//
//     // Multiply
//     MUL,
//
//     // Load store
//     LDB,
//     STB,
//     LDH,
//     STH,
//     LDW,
//     STW,
//
//     // Branch
//     B,
//     BL,
//
//     // All of the silly instructions
//     SVC,
//     DSB,
//     DMB,
//     ISB,
// }

pub fn decode2(i: I) -> Vec<I> {
    let mut vec = Vec::new();
    match i.it {
        PUSH | STMIA => {
            let n = hamming_weight(i.rl as u32);
            let target = match i.it {
                PUSH => 13,
                STMIA => i.rn as u32,
                _ => unreachable!(),
            } as u8;
            // SP subtraction
            vec.push(I {
                it: SUBImm,
                rd: target,
                immu: n << 2,
                imms: 0,
                rm: 0,
                rn: 13,
                rt: 0,
                rl: 0,
                setsflags: false,
            });
            let mut sp_offset = 0;
            for r in 0..15 {
                if bit_as_bool(i.rl as u32, r) {
                    vec.push(I {
                        it: STRImm,
                        rt: r as u8,
                        immu: sp_offset,
                        imms: 0,
                        rn: target,
                        rd: 0,
                        rl: 0,
                        rm: 0,
                        setsflags: false,
                    });
                    sp_offset += 4
                }
            }
        }
        POP | LDMIA => {
            let n = hamming_weight(i.rl as u32);
            let target = match i.it {
                POP => 13,
                LDMIA => i.rn as u32,
                _ => unreachable!(),
            } as u8;
            let mut sp_offset = 0;
            for r in 0..8 {
                if bit_as_bool(i.rl as u32, r) {
                    vec.push(I {
                        it: IT::LDRImm,
                        rt: r as u8,
                        immu: sp_offset,
                        imms: 0,
                        rn: target,
                        rd: 0,
                        rl: 0,
                        rm: 0,
                        setsflags: false,
                    });
                    sp_offset += 4
                }
            }
            if bit_as_bool(i.rl as u32, 15) {
                vec.push(I {
                    it: IT::LDRImm,
                    rt: 9,
                    immu: sp_offset,
                    imms: 0,
                    rn: target,
                    rd: 0,
                    rl: 0,
                    rm: 0,
                    setsflags: false,
                });
                // SP addition ( must be before BX or will be ignored)
                vec.push(I {
                    it: IT::ADDSpImm,
                    rd: target,
                    immu: n << 2,
                    imms: 0,
                    rm: 0,
                    rn: target,
                    rt: 0,
                    rl: 0,
                    setsflags: false,
                });
                vec.push(I {
                    it: IT::BX,
                    rn: 0,
                    rt: 15,
                    immu: 0,
                    imms: 0,
                    rl: 0,
                    rm: 9,
                    rd: 0,
                    setsflags: false,
                })
            } else {
                // SP addition
                vec.push(I {
                    it: IT::ADDSpImm,
                    rd: target,
                    immu: n << 2,
                    imms: 0,
                    rm: 0,
                    rn: 0,
                    rt: 0,
                    rl: 0,
                    setsflags: false,
                });
            }
        }
        _ => {
            vec.push(i);
        }
    }
    vec
}

#[cfg(test)]
mod decode2_test {
    use super::*;

    #[test]
    fn push() {
        let i = I {
            it: IT::PUSH,
            rl: 0b0100000011111111,
            rn: 0,
            rd: 0,
            rt: 0,
            rm: 0,
            immu: 0,
            imms: 0,
            setsflags: false,
        };
        println!("{:?}", decode2(i));
    }

    #[test]
    fn pop() {
        let i = I {
            it: IT::POP,
            rl: 0b1000000011111111,
            rn: 0,
            rd: 0,
            rt: 0,
            rm: 0,
            immu: 0,
            imms: 0,
            setsflags: false,
        };
        println!("{:?}", decode2(i));
    }
}
